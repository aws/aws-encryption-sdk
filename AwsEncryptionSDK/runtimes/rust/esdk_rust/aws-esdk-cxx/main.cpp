#include <iostream>
#include "aws-esdk-cxx/src/lib.rs.h"

int main()
{
    try
    {
        auto client_config = default_client_config();
        client_config.retry.max_attempts = 5;
        auto kms = create_kms_client(client_config);
        auto ddb = create_ddb_client(client_config);

        auto keystore_config = default_keystore_config();
        keystore_config.ddb_client = &*ddb;
        keystore_config.kms_client = &*kms;
        keystore_config.ddb_table_name = "KeyStoreDdbTable";
        keystore_config.logical_key_store_name = "KeyStoreDdbTable";
        keystore_config.kms_configuration_value = "arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126";
        auto keystore = create_keystore(keystore_config);

        auto keyring_input = default_hierarchical_keyring_input();
        keyring_input.key_store = &*keystore;
        keyring_input.branch_key_id = "3ce7656b-e166-40f3-8c9b-a920ce6596cd";
        auto keyring = create_hierarchical_keyring(keyring_input);

        auto encrypt_input = default_encrypt_input();
        encrypt_input.keyring = &*keyring;
        const unsigned char data[] = "Hello World";
        encrypt_input.plaintext = rust::Slice<const unsigned char>(&data[0], sizeof(data));
        auto encrypt_output = encrypt(encrypt_input);
        auto decrypt_input = default_decrypt_input();
        decrypt_input.keyring = &*keyring;
        decrypt_input.ciphertext = rust::Slice<const unsigned char>(&encrypt_output.ciphertext[0], encrypt_output.ciphertext.size());
        auto decrypt_output = decrypt(decrypt_input);
        if (encrypt_input.plaintext.size() != decrypt_output.plaintext.size())
            fprintf(stderr, "Decrypt size mismatch\n");
        if (memcmp(decrypt_output.plaintext.data(), encrypt_input.plaintext.data(), encrypt_input.plaintext.size()))
            fprintf(stderr, "Decrypt data mismatch\n");

        delete_keyring(std::move(keyring));
        delete_keystore(std::move(keystore));
        delete_kms_client(std::move(kms));
        delete_ddb_client(std::move(ddb));
        std::cout << "Success!\n";
    }
    catch (const std::exception &e)
    {
        std::cout << "Error: " << e.what() << "\n";
    }
    catch (const std::string &e)
    {
        std::cout << "Error String: " << e << "\n";
    }
    catch (...)
    {
        std::cout << "Other Exception \n";
    }
    return 0;
}
