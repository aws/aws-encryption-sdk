#![allow(clippy::boxed_local, reason = "need to pass Box<T> to destructors")]
use aws_config::{AppName, Region, SdkConfig};
use std::sync::LazyLock;

static DAFNY_TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

#[cxx::bridge]
mod ffi {
    struct EncryptionContextItem {
        key: String,
        value: String,
    }
    enum EsdkAlgorithmSuiteId {
        AlgAes128GcmIv12Tag16NoKdf = 0x0014,
        AlgAes192GcmIv12Tag16NoKdf = 0x0046,
        AlgAes256GcmIv12Tag16NoKdf = 0x0078,
        AlgAes128GcmIv12Tag16HkdfSha256 = 0x0114,
        AlgAes192GcmIv12Tag16HkdfSha256 = 0x0146,
        AlgAes256GcmIv12Tag16HkdfSha256 = 0x0178,
        AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 = 0x0214,
        AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0346,
        AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 = 0x0378,
        AlgAes256GcmHkdfSha512CommitKey = 0x0478,
        AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 = 0x0578,
    }

    struct EncryptInput<'a> {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// Bytes of plaintext data per frame. Default 4096.
        pub frame_length: u32,
        /// The source of cryptographic materials
        pub keyring: *const Keyring,
        /// data to be encrypted
        pub plaintext: &'a [u8],
        /// default is no limit
        pub max_encrypted_data_keys: u32,
        /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
        pub commitment_policy: EsdkCommitmentPolicy,
    }
    struct EncryptOutput {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// data to be decrypted
        pub ciphertext: Vec<u8>,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
    }
    struct DecryptInput<'a> {
        /// data to be decrypted
        pub ciphertext: &'a [u8],
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// The source of cryptographic materials
        pub keyring: *const Keyring,
        /// default is no limit
        pub max_encrypted_data_keys: u32,
        /// default is `EsdkCommitmentPolicy::RequireEncryptRequireDecrypt`
        pub commitment_policy: EsdkCommitmentPolicy,
    }
    struct DecryptOutput {
        /// Algorithm Suite. See <https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/supported-algorithms.html>
        pub algorithm_suite_id: EsdkAlgorithmSuiteId,
        /// Key-Value pairs to associate with the encrypted data
        pub encryption_context: Vec<EncryptionContextItem>,
        /// decrypted data
        pub plaintext: Vec<u8>,
    }

    enum EsdkCommitmentPolicy {
        ForbidEncryptAllowDecrypt,
        RequireEncryptAllowDecrypt,
        RequireEncryptRequireDecrypt,
    }

    struct RetryConfig {
        mode_adaptive: bool,
        max_attempts: u32,
        initial_backoff_milli: u64,
        max_backoff_milli: u64,
        reconnect_all: bool,
        use_static_exponential_base: bool,
    }

    enum KmsConfigurationType {
        KmsKeyArn,
        KmsMrKeyArn,
        Discovery,
        MrDiscovery,
    }

    enum CacheType {
        NoCache,
        MultiThreadedCache,
    }

    struct MplAwsClientConfig {
        env: bool,
        region: String,
        retry: RetryConfig,
    }

    struct MultiThreadedCacheConfig {
        entryCapacity: u32,
        entryPruningTailSize: u32,
    }

    struct KeyStoreConfig {
        ddb_table_name: String,
        kms_configuration_type: KmsConfigurationType,
        kms_configuration_value: String,
        logical_key_store_name: String,
        id: String,
        grant_tokens: Vec<String>,
        ddb_client: *const MplDdbClient,
        kms_client: *const MplKmsClient,
    }

    struct HierarchicalKeyringInput {
        branch_key_id: String,
        key_store: *const KeyStore,
        ttl: u32,
        cache: CacheType,
        multi_threaded_cache: MultiThreadedCacheConfig,
        partition_id: String,
    }

    extern "Rust" {
        type MplDdbClient;
        fn create_ddb_client(value: &MplAwsClientConfig) -> Result<Box<MplDdbClient>>;
        fn delete_ddb_client(client: Box<MplDdbClient>) -> Result<()>;

        type KeyStore;
        fn create_keystore(value: &KeyStoreConfig) -> Result<Box<KeyStore>>;
        fn delete_keystore(client: Box<KeyStore>) -> Result<()>;

        type Keyring;
        fn create_hierarchical_keyring(value: &HierarchicalKeyringInput) -> Result<Box<Keyring>>;
        fn delete_keyring(client: Box<Keyring>) -> Result<()>;

        type MplKmsClient;
        fn create_kms_client(value: &MplAwsClientConfig) -> Result<Box<MplKmsClient>>;
        fn delete_kms_client(client: Box<MplKmsClient>) -> Result<()>;

        fn encrypt(input: &EncryptInput) -> Result<EncryptOutput>;
        fn decrypt(input: &DecryptInput) -> Result<DecryptOutput>;

        // fn default_retry_config() -> RetryConfig;
        fn default_client_config() -> MplAwsClientConfig;
        fn default_keystore_config() -> KeyStoreConfig;
        fn default_hierarchical_keyring_input() -> HierarchicalKeyringInput;
        fn default_encrypt_input() -> EncryptInput<'static>;
        fn default_decrypt_input() -> DecryptInput<'static>;
    }
}

struct MplKmsClient {
    client: aws_sdk_kms::Client,
}
struct MplDdbClient {
    client: aws_sdk_dynamodb::Client,
}

struct KeyStore {
    client: aws_mpl_legacy::deps::aws_cryptography_keyStore::client::Client,
}

struct Keyring {
    client: aws_mpl_legacy::types::keyring::KeyringRef,
}

fn convert_commit(x: ffi::EsdkCommitmentPolicy) -> aws_mpl_rs::commitment::EsdkCommitmentPolicy {
    use aws_mpl_rs::commitment::EsdkCommitmentPolicy as New;
    use ffi::EsdkCommitmentPolicy as Old;
    match x {
        Old::ForbidEncryptAllowDecrypt => New::ForbidEncryptAllowDecrypt,
        Old::RequireEncryptAllowDecrypt => New::RequireEncryptAllowDecrypt,
        Old::RequireEncryptRequireDecrypt => New::RequireEncryptRequireDecrypt,
        _ => panic!("Invalid EsdkCommitmentPolicy"),
    }
}

fn default_encrypt_input() -> ffi::EncryptInput<'static> {
    ffi::EncryptInput {
        algorithm_suite_id: ffi::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        encryption_context: Vec::default(),
        frame_length: 4096,
        keyring: std::ptr::null(),
        plaintext: &[],
        max_encrypted_data_keys: 0,
        commitment_policy: ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}

fn default_decrypt_input() -> ffi::DecryptInput<'static> {
    ffi::DecryptInput {
        ciphertext: &[],
        encryption_context: Vec::default(),
        keyring: std::ptr::null(),
        max_encrypted_data_keys: 0,
        commitment_policy: ffi::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
    }
}

fn default_hierarchical_keyring_input() -> ffi::HierarchicalKeyringInput {
    ffi::HierarchicalKeyringInput {
        branch_key_id: String::default(),
        key_store: std::ptr::null(),
        ttl: 300,
        cache: ffi::CacheType::MultiThreadedCache,
        multi_threaded_cache: ffi::MultiThreadedCacheConfig {
            entryCapacity: 1000,
            entryPruningTailSize: 1,
        },
        partition_id: String::default(),
    }
}

fn default_keystore_config() -> ffi::KeyStoreConfig {
    ffi::KeyStoreConfig {
        ddb_table_name: String::default(),
        kms_configuration_type: ffi::KmsConfigurationType::KmsKeyArn,
        kms_configuration_value: String::default(),
        logical_key_store_name: String::default(),
        id: String::default(),
        grant_tokens: Vec::default(),
        ddb_client: std::ptr::null(),
        kms_client: std::ptr::null(),
    }
}

fn default_client_config() -> ffi::MplAwsClientConfig {
    ffi::MplAwsClientConfig {
        env: true,
        region: String::default(),
        retry: default_retry_config(),
    }
}

fn default_retry_config() -> ffi::RetryConfig {
    ffi::RetryConfig {
        mode_adaptive: false,
        max_attempts: 0,
        initial_backoff_milli: 0,
        max_backoff_milli: 0,
        reconnect_all: false,
        use_static_exponential_base: false,
    }
}

fn convert_alg_id(e: ffi::EsdkAlgorithmSuiteId) -> aws_mpl_rs::suites::EsdkAlgorithmSuiteId {
    use aws_mpl_rs::suites::EsdkAlgorithmSuiteId as New;
    use ffi::EsdkAlgorithmSuiteId as Old;
    match e {
        Old::AlgAes128GcmIv12Tag16NoKdf => New::AlgAes128GcmIv12Tag16NoKdf,
        Old::AlgAes192GcmIv12Tag16NoKdf => New::AlgAes192GcmIv12Tag16NoKdf,
        Old::AlgAes256GcmIv12Tag16NoKdf => New::AlgAes256GcmIv12Tag16NoKdf,
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => New::AlgAes128GcmIv12Tag16HkdfSha256,
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => New::AlgAes192GcmIv12Tag16HkdfSha256,
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => New::AlgAes256GcmIv12Tag16HkdfSha256,
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        Old::AlgAes256GcmHkdfSha512CommitKey => New::AlgAes256GcmHkdfSha512CommitKey,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
        _ => panic!("Invalid AlgorithmSuiteId"),
    }
}

fn unconvert_alg_id(e: aws_mpl_rs::suites::EsdkAlgorithmSuiteId) -> ffi::EsdkAlgorithmSuiteId {
    use aws_mpl_rs::suites::EsdkAlgorithmSuiteId as Old;
    use ffi::EsdkAlgorithmSuiteId as New;
    match e {
        Old::AlgAes128GcmIv12Tag16NoKdf => New::AlgAes128GcmIv12Tag16NoKdf,
        Old::AlgAes192GcmIv12Tag16NoKdf => New::AlgAes192GcmIv12Tag16NoKdf,
        Old::AlgAes256GcmIv12Tag16NoKdf => New::AlgAes256GcmIv12Tag16NoKdf,
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => New::AlgAes128GcmIv12Tag16HkdfSha256,
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => New::AlgAes192GcmIv12Tag16HkdfSha256,
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => New::AlgAes256GcmIv12Tag16HkdfSha256,
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        Old::AlgAes256GcmHkdfSha512CommitKey => New::AlgAes256GcmHkdfSha512CommitKey,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
        _ => panic!("Invalid AlgorithmSuiteId"),
    }
}

// TODO, take ownership so we don't have to clone
fn list_from_map(x: &std::collections::HashMap<String, String>) -> Vec<ffi::EncryptionContextItem> {
    x.iter()
        .map(|(k, v)| ffi::EncryptionContextItem {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

fn map_from_list(x: &[ffi::EncryptionContextItem]) -> std::collections::HashMap<String, String> {
    x.iter().map(|x| (x.key.clone(), x.value.clone())).collect()
}

fn encrypt(input: &ffi::EncryptInput) -> Result<ffi::EncryptOutput, String> {
    if input.keyring.is_null() {
        return Err("keyring is null in encrypt".to_string());
    }

    let mut input2 = aws_esdk::EncryptInput::with_legacy_keyring(
        input.plaintext,
        map_from_list(&input.encryption_context),
        unsafe { (*input.keyring).client.clone() },
    );
    if input.max_encrypted_data_keys > 0 {
        input2.max_encrypted_data_keys =
            Some(std::num::NonZeroUsize::new(input.max_encrypted_data_keys as usize).unwrap());
    }
    input2.frame_length =
        aws_esdk::FrameLength::new(input.frame_length).map_err(|e| format!("{:?}", e))?;
    input2.commitment_policy = convert_commit(input.commitment_policy);
    input2.algorithm_suite_id = Some(convert_alg_id(input.algorithm_suite_id));
    let output = DAFNY_TOKIO_RUNTIME.block_on(aws_esdk::encrypt(&input2));
    let output = output.map_err(|e| format!("{:?}", e))?;
    Ok(ffi::EncryptOutput {
        algorithm_suite_id: unconvert_alg_id(output.algorithm_suite_id),
        ciphertext: output.ciphertext,
        encryption_context: list_from_map(&output.encryption_context),
    })
}
fn decrypt(input: &ffi::DecryptInput) -> Result<ffi::DecryptOutput, String> {
    if input.keyring.is_null() {
        return Err("keyring is null in decrypt".to_string());
    }

    let mut input2 = aws_esdk::DecryptInput::with_legacy_keyring(
        input.ciphertext,
        map_from_list(&input.encryption_context),
        unsafe { (*input.keyring).client.clone() },
    );
    if input.max_encrypted_data_keys > 0 {
        input2.max_encrypted_data_keys =
            Some(std::num::NonZeroUsize::new(input.max_encrypted_data_keys as usize).unwrap());
    }
    input2.commitment_policy = convert_commit(input.commitment_policy);
    let output = DAFNY_TOKIO_RUNTIME.block_on(aws_esdk::decrypt(&input2));
    let output = output.map_err(|e| format!("{:?}", e))?;
    Ok(ffi::DecryptOutput {
        algorithm_suite_id: unconvert_alg_id(output.algorithm_suite_id),
        plaintext: output.plaintext,
        encryption_context: list_from_map(&output.encryption_context),
    })
}

fn make_retry_config(config: &ffi::RetryConfig) -> aws_config::retry::RetryConfig {
    let mut out_config = if config.mode_adaptive {
        aws_config::retry::RetryConfig::adaptive()
    } else {
        aws_config::retry::RetryConfig::standard()
    };
    if config.max_attempts > 0 {
        out_config = out_config.with_max_attempts(config.max_attempts);
    }
    if config.initial_backoff_milli > 0 {
        out_config = out_config.with_initial_backoff(std::time::Duration::from_millis(
            config.initial_backoff_milli,
        ));
    }
    if config.max_backoff_milli > 0 {
        out_config =
            out_config.with_max_backoff(std::time::Duration::from_millis(config.max_backoff_milli));
    }
    if config.reconnect_all {
        out_config = out_config
            .with_reconnect_mode(aws_sdk_kms::config::retry::ReconnectMode::ReuseAllConnections);
    }
    if config.use_static_exponential_base {
        out_config = out_config.with_use_static_exponential_base(true);
    }
    out_config
}
fn delete_kms_client(_client: Box<MplKmsClient>) -> Result<(), String> {
    Ok(())
}
fn delete_ddb_client(_client: Box<MplDdbClient>) -> Result<(), String> {
    Ok(())
}
fn delete_keystore(_client: Box<KeyStore>) -> Result<(), String> {
    Ok(())
}
fn delete_keyring(_client: Box<Keyring>) -> Result<(), String> {
    Ok(())
}

fn make_cache_type(
    config: &ffi::HierarchicalKeyringInput,
) -> Result<aws_mpl_legacy::types::CacheType, String> {
    match config.cache {
        ffi::CacheType::NoCache => Ok(aws_mpl_legacy::types::CacheType::No(
            aws_mpl_legacy::types::NoCache::builder().build().unwrap(),
        )),
        ffi::CacheType::MultiThreadedCache => {
            let entry_capacity = config.multi_threaded_cache.entryCapacity;
            let entry_pruning_tail_size = config.multi_threaded_cache.entryPruningTailSize;
            Ok(aws_mpl_legacy::types::CacheType::MultiThreaded(
                aws_mpl_legacy::types::MultiThreadedCache::builder()
                    .entry_capacity(entry_capacity as i32)
                    .entry_pruning_tail_size(entry_pruning_tail_size as i32)
                    .build()
                    .unwrap(),
            ))
        }
        _ => Err("Invalid CacheType in HierarchicalKeyringInput".to_string()),
    }
}

fn make_kms_config(
    config: &ffi::KeyStoreConfig,
) -> Result<aws_mpl_legacy::deps::aws_cryptography_keyStore::types::KmsConfiguration, String> {
    match config.kms_configuration_type {
        ffi::KmsConfigurationType::KmsKeyArn => Ok(
            aws_mpl_legacy::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsKeyArn(
                config.kms_configuration_value.clone(),
            ),
        ),
        ffi::KmsConfigurationType::KmsMrKeyArn => Ok(
            aws_mpl_legacy::deps::aws_cryptography_keyStore::types::KmsConfiguration::KmsMrKeyArn(
                config.kms_configuration_value.clone(),
            ),
        ),
        ffi::KmsConfigurationType::Discovery => Ok(
            aws_mpl_legacy::deps::aws_cryptography_keyStore::types::KmsConfiguration::Discovery(
                aws_mpl_legacy::deps::aws_cryptography_keyStore::types::Discovery::builder()
                    .build()
                    .unwrap(),
            ),
        ),
        ffi::KmsConfigurationType::MrDiscovery => Ok(
            aws_mpl_legacy::deps::aws_cryptography_keyStore::types::KmsConfiguration::MrDiscovery(
                aws_mpl_legacy::deps::aws_cryptography_keyStore::types::MrDiscovery::builder()
                    .build()
                    .unwrap(),
            ),
        ),
        _ => Err("Invalid KmsConfigurationType".to_string()),
    }
}

fn create_hierarchical_keyring(
    input: &ffi::HierarchicalKeyringInput,
) -> Result<Box<Keyring>, String> {
    let mpl_config = aws_mpl_legacy::types::MaterialProvidersConfig::builder()
        .build()
        .unwrap();
    let mpl = aws_mpl_legacy::Client::from_conf(mpl_config).unwrap();
    if input.branch_key_id.is_empty() {
        return Err("branch_key_id must not be empty in create_hierarchical_keyring".to_string());
    }
    let mut builder = mpl
        .create_aws_kms_hierarchical_keyring()
        .cache(make_cache_type(input)?)
        .branch_key_id(input.branch_key_id.clone())
        .ttl_seconds(input.ttl);
    if input.key_store.is_null() {
        return Err("key_store is null in create_hierarchical_keyring".to_string());
    } else {
        builder = builder.key_store(unsafe { (*input.key_store).client.clone() })
    }
    if !input.partition_id.is_empty() {
        builder = builder.partition_id(input.partition_id.clone());
    }

    let keyring = DAFNY_TOKIO_RUNTIME
        .block_on(builder.send())
        .map_err(|e| format!("{:?}", e))?;

    let keyring = Keyring { client: keyring };
    Ok(Box::new(keyring))
}

fn create_keystore(input: &ffi::KeyStoreConfig) -> Result<Box<KeyStore>, String> {
    let mut builder = aws_mpl_legacy::deps::aws_cryptography_keyStore::types::key_store_config::KeyStoreConfig::builder();
    if input.kms_client.is_null() {
        return Err("kms_client is null in create_keystore".to_string());
    } else {
        builder = builder.kms_client(unsafe { (*input.kms_client).client.clone() });
    }
    if input.ddb_client.is_null() {
        return Err("ddb_client is null in create_keystore".to_string());
    } else {
        builder = builder.ddb_client(unsafe { (*input.ddb_client).client.clone() });
    }
    if input.ddb_table_name.is_empty() {
        return Err("ddb_table_name is empty in create_keystore".to_string());
    } else {
        builder = builder.ddb_table_name(input.ddb_table_name.clone());
    }
    if input.logical_key_store_name.is_empty() {
        return Err("logical_key_store_name is empty in create_keystore".to_string());
    } else {
        builder = builder.logical_key_store_name(input.logical_key_store_name.clone());
    }
    builder = builder.kms_configuration(make_kms_config(input)?);
    if !input.id.is_empty() {
        builder = builder.id(input.id.clone());
    }
    if !input.grant_tokens.is_empty() {
        builder = builder.grant_tokens(input.grant_tokens.clone());
    }
    let config = builder.build().map_err(|e| format!("{:?}", e))?;

    let store = aws_mpl_legacy::deps::aws_cryptography_keyStore::client::Client::from_conf(config)
        .map_err(|e| format!("{:?}", e))?;
    let store = KeyStore { client: store };
    Ok(Box::new(store))
}

fn create_kms_client(input: &ffi::MplAwsClientConfig) -> Result<Box<MplKmsClient>, String> {
    let sdk_config = create_sdk_config(input);
    let client = aws_sdk_kms::Client::new(&sdk_config);
    let client = MplKmsClient { client };
    Ok(Box::new(client))
}

fn create_ddb_client(input: &ffi::MplAwsClientConfig) -> Result<Box<MplDdbClient>, String> {
    let sdk_config = create_sdk_config(input);
    let client = aws_sdk_dynamodb::Client::new(&sdk_config);
    let client = MplDdbClient { client };
    Ok(Box::new(client))
}

fn create_sdk_config(input: &ffi::MplAwsClientConfig) -> SdkConfig {
    let shared_config = DAFNY_TOKIO_RUNTIME.block_on(aws_config::load_defaults(
        aws_config::BehaviorVersion::latest(),
    ));

    let user_agent_string = "AwsCryptographicMPL-C++-1.11.1";
    let current_app_name = shared_config
        .app_name()
        .map(|app_name| app_name.to_string())
        .unwrap_or_default();
    let new_app_name = if current_app_name.is_empty() {
        user_agent_string.to_string()
    } else {
        format!("{} {} ", current_app_name, user_agent_string)
    };
    let app_name = AppName::new(new_app_name).expect("Valid app name");
    let mut builder = shared_config
        .to_builder()
        .app_name(app_name)
        .retry_config(make_retry_config(&input.retry));
    if !input.region.is_empty() {
        builder = builder.region(Region::new(input.region.clone()));
    }
    builder.build()
}
