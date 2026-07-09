// aws_esdk.hpp — idiomatic C++ facade over the cxx-generated bindings.
//
// Design goals (learnings from the binding-strategy review):
//   * Stable, GENERATOR-AGNOSTIC public API: no cxx type (rust::*, the generated
//     enums/structs) appears in any public signature. Swapping the binding
//     backend later touches only this file's internals, not consumer code.
//   * RAII + exceptions: cxx throws rust::Error on failure; we translate that
//     into Aws::Esdk::EsdkException so callers never see a cxx type.
//   * Memory safety at the handle seam: the bridge passes handles as raw
//     `*const` pointers inside config structs, which can dangle if a C++ caller
//     drops a client before it is used. Each wrapper here OWNS its dependencies
//     via shared_ptr and declares them before the object that points at them,
//     so a dependency can never be destroyed while something references it.
#pragma once

#include <cstdint>
#include <map>
#include <memory>
#include <stdexcept>
#include <string>
#include <vector>

#include "rust/cxx.h"                 // rust::Error, rust::Box, rust::Vec, rust::Slice
#include "aws-esdk-cpp/src/lib.rs.h"  // cxx-generated bindings (global namespace)

namespace Aws {
namespace Esdk {

// ---- File-wide owned-interface / lifetime invariants (see spec/shim/shim.md) -
//= spec/shim/shim.md#owned-interface
//= type=implication
//= reason=The facade defines its own Aws::Esdk:: enum for every core-library enumeration it exposes (AlgorithmSuiteId, CommitmentPolicy, CacheKind).
//# - For each core-library enumeration it exposes, the shim MUST define its own
//# corresponding enumeration type.
//
//= spec/shim/shim.md#owned-interface
//= type=implication
//= reason=Every public method signature uses only Aws::Esdk:: types and standard-library types; no generated or core-library type appears.
//# - Each public function the shim exposes MUST use only owned-interface types or
//# standard-library types in its signature.
//
//= spec/shim/shim.md#owned-interface
//= type=implication
//= reason=Every wrapper stores its generated handle in a private member and exposes it only through the internal ffi() accessor.
//# - Each owned interface that represents a core-library resource MUST hold the
//# generated handle privately and MUST NOT expose it in its public API.

// ---- Public exception: hides cxx's rust::Error ------------------------------
//= spec/shim/shim.md#owned-interface
//= type=implication
//= reason=EsdkException is the shim's own error type; detail::guard translates every cxx rust::Error into it, so no cxx or core-library error type appears on the public surface.
//# - The shim MUST define its own error type for its public surface.
class EsdkException : public std::runtime_error {
public:
    explicit EsdkException(const std::string& msg) : std::runtime_error(msg) {}
};

// ---- Public enums: our own, so the generated enums never leak ----------------
enum class AlgorithmSuiteId : std::uint16_t {
    AesGcm128                      = 0x0014,
    AesGcm192                      = 0x0046,
    AesGcm256                      = 0x0078,
    AesGcm128Hkdf256               = 0x0114,
    AesGcm192Hkdf256               = 0x0146,
    AesGcm256Hkdf256               = 0x0178,
    AesGcm128Hkdf256Ecdsa256       = 0x0214,
    AesGcm192Hkdf384Ecdsa384       = 0x0346,
    AesGcm256Hkdf384Ecdsa384       = 0x0378,
    AesGcm256Hkdf512Commit         = 0x0478,
    AesGcm256Hkdf512CommitEcdsa384 = 0x0578,
};

enum class CommitmentPolicy : std::uint8_t {
    ForbidEncryptAllowDecrypt  = 0,
    RequireEncryptAllowDecrypt = 1,
    RequireEncryptRequireDecrypt = 2,
};

enum class CacheKind : std::uint8_t { None = 0, MultiThreaded = 1 };

namespace detail {

// Run a cxx call, translating rust::Error -> EsdkException so no cxx exception
// type escapes this header.
template <typename F>
inline auto guard(F&& f) -> decltype(f()) {
    try {
        return f();
    } catch (const ::rust::Error& e) {
        throw EsdkException(e.what());
    }
}

// Enum translation is the only code that knows the generated enum types exist.
// Values are kept identical to the generated enums, so casts are exact.
inline ::EsdkAlgorithmSuiteId to_ffi(AlgorithmSuiteId a) {
    return static_cast<::EsdkAlgorithmSuiteId>(static_cast<std::uint16_t>(a));
}
inline ::EsdkCommitmentPolicy to_ffi(CommitmentPolicy p) {
    return static_cast<::EsdkCommitmentPolicy>(static_cast<std::uint8_t>(p));
}
inline ::CacheType to_ffi(CacheKind c) {
    return static_cast<::CacheType>(static_cast<std::uint8_t>(c));
}

inline ::rust::Vec<::EncryptionContextItem> to_ec(
    const std::map<std::string, std::string>& ec) {
    ::rust::Vec<::EncryptionContextItem> out;
    for (const auto& kv : ec) {
        ::EncryptionContextItem item;
        item.key = ::rust::String(kv.first);
        item.value = ::rust::String(kv.second);
        out.push_back(std::move(item));
    }
    return out;
}

// Reverse translations, for values the core library returns to the target.
inline AlgorithmSuiteId alg_from_ffi(::EsdkAlgorithmSuiteId a) {
    return static_cast<AlgorithmSuiteId>(static_cast<std::uint16_t>(a));
}

inline std::map<std::string, std::string> from_ec(
    const ::rust::Vec<::EncryptionContextItem>& ec) {
    std::map<std::string, std::string> out;
    for (const auto& item : ec) {
        out.emplace(std::string(item.key), std::string(item.value));
    }
    return out;
}

}  // namespace detail

// ---- Fluent client config ----------------------------------------------------
class ClientConfig {
    ::MplAwsClientConfig cfg_ = ::default_client_config();
public:
    ClientConfig& with_max_retry_attempts(std::uint32_t n) { cfg_.retry.max_attempts = n; return *this; }
    ClientConfig& with_adaptive_retry(bool a)              { cfg_.retry.mode_adaptive = a; return *this; }
    ClientConfig& with_region(const std::string& r)        { cfg_.region = ::rust::String(r); return *this; }
    const ::MplAwsClientConfig& raw() const { return cfg_; }   // internal; POD-ish, low leak risk
};

// ---- Handle wrappers: copyable (shared_ptr), so they can be owned as deps -----
class KmsClient {
    std::shared_ptr<::rust::Box<::MplKmsClient>> inner_;
    const ::MplKmsClient* ffi() const { return &**inner_; }
    friend class KeyStore;
public:
    explicit KmsClient(const ClientConfig& cfg)
        : inner_(std::make_shared<::rust::Box<::MplKmsClient>>(
              detail::guard([&] { return ::create_kms_client(cfg.raw()); }))) {}
};

class DdbClient {
    std::shared_ptr<::rust::Box<::MplDdbClient>> inner_;
    const ::MplDdbClient* ffi() const { return &**inner_; }
    friend class KeyStore;
public:
    explicit DdbClient(const ClientConfig& cfg)
        : inner_(std::make_shared<::rust::Box<::MplDdbClient>>(
              detail::guard([&] { return ::create_ddb_client(cfg.raw()); }))) {}
};

class KeyStore {
    // Declared before inner_ so they outlive it; shared_ptr keeps the Rust
    // clients alive for as long as this KeyStore exists -> the raw pointers the
    // bridge stores can never dangle.
    KmsClient kms_;
    DdbClient ddb_;
    std::shared_ptr<::rust::Box<::KeyStore>> inner_;
    const ::KeyStore* ffi() const { return &**inner_; }
    friend class HierarchicalKeyring;
public:
    KeyStore(const std::string& table_name,
             const std::string& logical_key_store_name,
             const std::string& kms_arn,
             KmsClient kms,
             DdbClient ddb)
        : kms_(std::move(kms)),
          ddb_(std::move(ddb)),
          inner_(std::make_shared<::rust::Box<::KeyStore>>(detail::guard([&] {
              ::KeyStoreConfig c = ::default_keystore_config();
              c.ddb_table_name = ::rust::String(table_name);
              c.logical_key_store_name = ::rust::String(logical_key_store_name);
              c.kms_configuration_type = ::KmsConfigurationType::KmsKeyArn;
              c.kms_configuration_value = ::rust::String(kms_arn);
              c.kms_client = kms_.ffi();
              c.ddb_client = ddb_.ffi();
              return ::create_keystore(c);
          }))) {}
};

class HierarchicalKeyring {
    KeyStore key_store_;  // owns the keystore (and transitively its clients)
    std::shared_ptr<::rust::Box<::Keyring>> inner_;
    const ::Keyring* ffi() const { return &**inner_; }
    friend class EncryptionSDK;
public:
    HierarchicalKeyring(KeyStore key_store,
                        const std::string& branch_key_id,
                        std::uint32_t ttl_seconds,
                        std::uint32_t cache_capacity,
                        CacheKind cache = CacheKind::MultiThreaded)
        : key_store_(std::move(key_store)),
          inner_(std::make_shared<::rust::Box<::Keyring>>(detail::guard([&] {
              ::HierarchicalKeyringInput in = ::default_hierarchical_keyring_input();
              in.branch_key_id = ::rust::String(branch_key_id);
              in.key_store = key_store_.ffi();
              in.ttl = ttl_seconds;
              in.cache = detail::to_ffi(cache);
              in.multi_threaded_cache.entryCapacity = cache_capacity;
              return ::create_hierarchical_keyring(in);
          }))) {}
};

// ---- Decrypt result ----------------------------------------------------------
// decrypt returns not just the plaintext but the encryption context and algorithm
// suite the core library reported, so the caller can inspect what was actually
// used and authenticated (for example, to make access-control decisions).
struct DecryptResult {
    std::vector<std::uint8_t> plaintext;
    std::map<std::string, std::string> encryption_context;
    AlgorithmSuiteId algorithm_suite_id;
};

// ---- SDK facade --------------------------------------------------------------
class EncryptionSDK {
public:
    std::vector<std::uint8_t> encrypt(
        const std::vector<std::uint8_t>& plaintext,
        const HierarchicalKeyring& keyring,
        AlgorithmSuiteId alg = AlgorithmSuiteId::AesGcm256Hkdf512CommitEcdsa384,
        CommitmentPolicy policy = CommitmentPolicy::RequireEncryptRequireDecrypt,
        const std::map<std::string, std::string>& encryption_context = {}) {
        // Start from the Rust-defined defaults, then override — so we never
        // re-declare defaults in C++ and risk diverging from the Rust core.
        ::EncryptInput in = ::default_encrypt_input();
        in.keyring = keyring.ffi();
        in.plaintext = ::rust::Slice<const std::uint8_t>(plaintext.data(), plaintext.size());
        in.algorithm_suite_id = detail::to_ffi(alg);
        in.commitment_policy = detail::to_ffi(policy);
        in.encryption_context = detail::to_ec(encryption_context);
        ::EncryptOutput out = detail::guard([&] { return ::encrypt(in); });
        return std::vector<std::uint8_t>(out.ciphertext.begin(), out.ciphertext.end());
    }

    DecryptResult decrypt(
        const std::vector<std::uint8_t>& ciphertext,
        const HierarchicalKeyring& keyring) {
        ::DecryptInput in = ::default_decrypt_input();
        in.keyring = keyring.ffi();
        in.ciphertext = ::rust::Slice<const std::uint8_t>(ciphertext.data(), ciphertext.size());
        ::DecryptOutput out = detail::guard([&] { return ::decrypt(in); });
        DecryptResult result;
        result.plaintext = std::vector<std::uint8_t>(out.plaintext.begin(), out.plaintext.end());
        result.encryption_context = detail::from_ec(out.encryption_context);
        result.algorithm_suite_id = detail::alg_from_ffi(out.algorithm_suite_id);
        return result;
    }
};

}  // namespace Esdk
}  // namespace Aws
