#include <iostream>
#include <vector>
#include <aws/esdk/EncryptionSDK.h>  // Idiomatic C++ wrapper layer

int main() {
    try {
        // Create AWS clients
        auto client_config = Aws::Esdk::ClientConfig()
            .with_max_retry_attempts(5);
        
        auto kms = Aws::Esdk::KmsClient(client_config);
        auto ddb = Aws::Esdk::DdbClient(client_config);

        // Create keystore with constructor - all config in one place
        auto keystore = Aws::Esdk::KeyStore(
            "KeyStoreDdbTable",                                                    // table_name
            "KeyStoreDdbTable",                                                    // logical_key_store_name
            "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126",  // kms_arn
            kms,
            ddb
        );

        // Create hierarchical keyring with constructor
        auto keyring = Aws::Esdk::HierarchicalKeyring(
            keystore,
            "3ce7656b-e166-40f3-8c9b-a920ce6596cd",  // branch_key_id
            4242,                                     // ttl_seconds
            42                                        // cache_capacity
        );

        // Create encryption SDK client
        Aws::Esdk::EncryptionSDK sdk;
        
        // Encrypt with std::vector - idiomatic C++
        std::vector<uint8_t> plaintext = {'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd'};
        auto ciphertext = sdk.encrypt(plaintext, keyring);
        
        // Decrypt
        auto decrypted = sdk.decrypt(ciphertext, keyring);
        
        // Validate results
        if (plaintext != decrypted.plaintext) {
            std::cerr << "Decryption failed: plaintext mismatch\n";
            return 1;
        }

        std::cout << "Success! Encryption and decryption completed successfully.\n";
        
        // Automatic cleanup via RAII - no manual delete calls needed!
        
    } catch (const std::exception &e) {
        std::cerr << "Error: " << e.what() << "\n";
        return 1;
    }
    
    return 0;
}
