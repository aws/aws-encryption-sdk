// test_esdk.cpp — integration tests for the idiomatic C++ facade (aws_esdk.hpp).
//
// These test the SHIM's own behavior through the public C++ API:
//   * encrypt/decrypt round-trip fidelity,
//   * encryption-context handling,
//   * failures surface as Aws::Esdk::EsdkException (not a crash),
//   * tampered ciphertext is rejected,
//   * handle lifetime safety (a keyring keeps its clients/keystore alive even
//     after the local wrappers go out of scope).
//
// They exercise the real crypto path, so they need real AWS resources. Provide
// them via environment variables; if any required var is unset the suite prints
// SKIP and exits 0 (so it never fails credential-less CI):
//
//   ESDK_TEST_KMS_KEY_ARN     KMS key ARN backing the key store
//   ESDK_TEST_DDB_TABLE       DynamoDB table name of the key store
//   ESDK_TEST_LOGICAL_KEYSTORE  logical key store name
//   ESDK_TEST_BRANCH_KEY_ID   branch key id provisioned in the key store
//   ESDK_TEST_REGION          (optional) AWS region, e.g. us-west-2
//
// Build: see the `test_esdk` target in the Makefile (requires -std=c++17).

#include <cstdlib>
#include <iostream>
#include <map>
#include <string>
#include <vector>

#include <aws/esdk/EncryptionSDK.h>

// ---- Tiny assertion harness (no external test framework dependency) ---------
static int g_tests = 0;
static int g_failures = 0;

#define CHECK(cond)                                                            \
    do {                                                                       \
        ++g_tests;                                                             \
        if (!(cond)) {                                                         \
            ++g_failures;                                                      \
            std::cerr << "FAIL: " << #cond << " (" << __FILE__ << ":"          \
                      << __LINE__ << ")\n";                                    \
        }                                                                      \
    } while (0)

#define CHECK_THROWS_ESDK(expr)                                                \
    do {                                                                       \
        ++g_tests;                                                             \
        bool threw = false;                                                    \
        try {                                                                  \
            expr;                                                              \
        } catch (const Aws::Esdk::EsdkException&) {                             \
            threw = true;                                                      \
        } catch (...) {                                                        \
        }                                                                      \
        if (!threw) {                                                          \
            ++g_failures;                                                      \
            std::cerr << "FAIL: expected EsdkException from " << #expr << "\n"; \
        }                                                                      \
    } while (0)

namespace {

struct TestConfig {
    std::string kms_key_arn;
    std::string ddb_table;
    std::string logical_keystore;
    std::string branch_key_id;
    std::string region;
};

std::vector<std::uint8_t> to_bytes(const std::string& s) {
    return std::vector<std::uint8_t>(s.begin(), s.end());
}

// Build a keyring. Deliberately constructs the clients/keystore as locals and
// moves them in, so the returned keyring is the sole owner — used by the
// lifetime test to prove the wrappers keep their dependencies alive.
Aws::Esdk::HierarchicalKeyring make_keyring(const TestConfig& cfg) {
    auto client_config = Aws::Esdk::ClientConfig().with_max_retry_attempts(3);
    if (!cfg.region.empty()) {
        client_config.with_region(cfg.region);
    }
    Aws::Esdk::KmsClient kms(client_config);
    Aws::Esdk::DdbClient ddb(client_config);
    Aws::Esdk::KeyStore keystore(cfg.ddb_table, cfg.logical_keystore,
                                cfg.kms_key_arn, kms, ddb);
    return Aws::Esdk::HierarchicalKeyring(std::move(keystore), cfg.branch_key_id,
                                         /*ttl_seconds*/ 600, /*cache_capacity*/ 100);
}

void test_round_trip(const TestConfig& cfg) {
    auto keyring = make_keyring(cfg);
    Aws::Esdk::EncryptionSDK sdk;
    auto plaintext = to_bytes("Hello World");
    auto ciphertext = sdk.encrypt(plaintext, keyring);
    CHECK(ciphertext != plaintext);

    //= spec/shim/shim.md#delegation
    //= type=test
    //= reason=Encrypting then decrypting back to the plaintext through the C++ facade proves the shim invoked the core library's encrypt and decrypt operations.
    //# - Each operation the shim exposes MUST produce its result by invoking the
    //# corresponding core-library operation.
    //
    //= spec/shim/esdk-shim.md#encrypt-inputs
    //= type=test
    //= reason=Round-tripping back to the original plaintext proves encrypt passed the target plaintext to the core library unmodified.
    //# - `encrypt` MUST pass the target-supplied plaintext to the core ESDK unmodified.
    //
    //= spec/shim/esdk-shim.md#encrypt-outputs
    //= type=test
    //= reason=Decrypting the returned ciphertext back to the original plaintext proves encrypt returned the core library's ciphertext unmodified.
    //# - `encrypt` MUST return the core ESDK's ciphertext unmodified.
    //
    //= spec/shim/esdk-shim.md#decrypt-inputs
    //= type=test
    //= reason=Decrypting this exact ciphertext back to the original plaintext proves it reached the core library unmodified; any modification would fail authentication.
    //# - `decrypt` MUST pass the target-supplied ciphertext to the core ESDK unmodified.
    //
    //= spec/shim/esdk-shim.md#decrypt-outputs
    //= type=test
    //= reason=The recovered plaintext equals the original, proving decrypt returned the core library's plaintext unmodified.
    //# - `decrypt` MUST return the core ESDK's plaintext unmodified.
    auto decrypted = sdk.decrypt(ciphertext, keyring);
    CHECK(decrypted.plaintext == plaintext);
}

void test_round_trip_with_encryption_context(const TestConfig& cfg) {
    auto keyring = make_keyring(cfg);
    Aws::Esdk::EncryptionSDK sdk;
    auto plaintext = to_bytes("context matters");
    std::map<std::string, std::string> ec = {{"purpose", "test"}, {"origin", "cxx"}};
    auto ciphertext = sdk.encrypt(plaintext, keyring,
                                  Aws::Esdk::AlgorithmSuiteId::AesGcm256Hkdf512CommitEcdsa384,
                                  Aws::Esdk::CommitmentPolicy::RequireEncryptRequireDecrypt, ec);
    auto decrypted = sdk.decrypt(ciphertext, keyring);
    CHECK(decrypted.plaintext == plaintext);

    //= spec/shim/esdk-shim.md#decrypt-outputs
    //= type=test
    //= reason=The decrypt result exposes the encryption context the core library returned; it contains every pair supplied at encrypt time.
    //# - `decrypt` MUST return the result encryption context, converted as defined in
    //# [Encryption context](#encryption-context).
    for (const auto& kv : ec) {
        CHECK(decrypted.encryption_context.count(kv.first) == 1);
        CHECK(decrypted.encryption_context[kv.first] == kv.second);
    }

    //= spec/shim/esdk-shim.md#decrypt-outputs
    //= type=test
    //= reason=The decrypt result exposes the algorithm suite the core library used; it equals the suite selected at encrypt time.
    //# - `decrypt` MUST return the used algorithm suite, converted as defined in
    //# [Algorithm suite identifier](#algorithm-suite-identifier).
    CHECK(decrypted.algorithm_suite_id ==
          Aws::Esdk::AlgorithmSuiteId::AesGcm256Hkdf512CommitEcdsa384);
}

void test_decrypt_garbage_throws(const TestConfig& cfg) {
    auto keyring = make_keyring(cfg);
    Aws::Esdk::EncryptionSDK sdk;
    auto garbage = to_bytes("this is not a valid ESDK message");
    // A failure must surface as an exception, never a crash/abort.
    //= spec/shim/shim.md#delegation
    //= type=test
    //# - When a core-library operation returns an error, the shim MUST return an error to
    //# the target and MUST NOT report the operation as successful.
    CHECK_THROWS_ESDK(sdk.decrypt(garbage, keyring));

    // The surfaced error identifies which operation failed (SHOULD context):
    // EsdkException carries the core error string, which is prefixed "decrypt failed".
    std::string msg;
    try {
        sdk.decrypt(garbage, keyring);
    } catch (const Aws::Esdk::EsdkException& e) {
        msg = e.what();
    }
    //= spec/shim/shim.md#delegation
    //= type=test
    //# - An error the shim returns for a failed core-library operation SHOULD include
    //# context identifying which operation failed.
    CHECK(msg.find("decrypt") != std::string::npos);
}

void test_tampered_ciphertext_throws(const TestConfig& cfg) {
    auto keyring = make_keyring(cfg);
    Aws::Esdk::EncryptionSDK sdk;
    auto ciphertext = sdk.encrypt(to_bytes("tamper me"), keyring);
    CHECK(!ciphertext.empty());
    ciphertext[ciphertext.size() / 2] ^= 0xFF;  // flip a byte in the body
    CHECK_THROWS_ESDK(sdk.decrypt(ciphertext, keyring));
}

// #3: the keyring is built inside make_keyring(); all local KmsClient/DdbClient/
// KeyStore wrappers are destroyed when it returns. If the facade did NOT keep
// them alive, the raw pointers the bridge stored would dangle and this would
// crash. Succeeding proves the shared_ptr ownership works.
void test_lifetime_safety(const TestConfig& cfg) {
    Aws::Esdk::HierarchicalKeyring keyring = make_keyring(cfg);
    Aws::Esdk::EncryptionSDK sdk;
    auto plaintext = to_bytes("outlives its dependencies");
    //= spec/shim/shim.md#handles-and-lifetimes
    //= type=test
    //= reason=make_keyring builds the KmsClient/DdbClient/KeyStore as locals and returns only the keyring; a successful encrypt+decrypt after those locals are destroyed proves the keyring retained ownership of its dependencies, so a handle could not outlive the resource it refers to.
    //# - An owned interface that depends on another resource MUST retain ownership of
    //# that resource for its own lifetime, so that a handle passed across the boundary
    //# cannot outlive the resource it refers to.
    auto ciphertext = sdk.encrypt(plaintext, keyring);
    auto decrypted = sdk.decrypt(ciphertext, keyring);
    CHECK(decrypted.plaintext == plaintext);
}

const char* env_or_null(const char* name) {
    const char* v = std::getenv(name);
    return (v && *v) ? v : nullptr;
}

}  // namespace

int main() {
    const char* kms = env_or_null("ESDK_TEST_KMS_KEY_ARN");
    const char* ddb = env_or_null("ESDK_TEST_DDB_TABLE");
    const char* logical = env_or_null("ESDK_TEST_LOGICAL_KEYSTORE");
    const char* branch = env_or_null("ESDK_TEST_BRANCH_KEY_ID");
    const char* region = env_or_null("ESDK_TEST_REGION");

    if (!kms || !ddb || !logical || !branch) {
        std::cout << "SKIP: set ESDK_TEST_KMS_KEY_ARN, ESDK_TEST_DDB_TABLE, "
                     "ESDK_TEST_LOGICAL_KEYSTORE, and ESDK_TEST_BRANCH_KEY_ID "
                     "to run the C++ integration tests.\n";
        return 0;
    }

    TestConfig cfg{kms, ddb, logical, branch, region ? region : ""};

    // Each test builds its own keyring so a failure in one does not cascade.
    try {
        test_round_trip(cfg);
        test_round_trip_with_encryption_context(cfg);
        test_decrypt_garbage_throws(cfg);
        test_tampered_ciphertext_throws(cfg);
        test_lifetime_safety(cfg);
    } catch (const std::exception& e) {
        std::cerr << "FATAL: unexpected exception aborted the suite: " << e.what() << "\n";
        return 2;
    }

    std::cout << (g_tests - g_failures) << "/" << g_tests << " checks passed\n";
    if (g_failures > 0) {
        std::cerr << g_failures << " check(s) FAILED\n";
        return 1;
    }
    std::cout << "All C++ integration tests passed.\n";
    return 0;
}
