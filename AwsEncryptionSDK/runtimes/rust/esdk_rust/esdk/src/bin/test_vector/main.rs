use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Turn an Encrypt Manifest into a Decrypt Manifest
    Encrypt {
        #[arg(long, help = "where to find encrypt manifest.")]
        manifest_path: String,
        #[arg(long, help = "where to put the decrypt manifest.")]
        decrypt_manifest_path: String,
        #[arg(long, help = "id of the test to run.", default_value = "")]
        test_name: String,
    },
    /// Validate a Decrypt Manifest
    Decrypt {
        #[arg(long, help = "where to find plaintext and ciphertext directories.")]
        manifest_path: String,
        #[arg(long, help = "where to put the decrypt manifest.")]
        manifest_name: String,
        #[arg(long, help = "id of the test to run.", default_value = "")]
        test_name: String,
    },
}
 #[tokio::main]
 async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encrypt {
            manifest_path,
            decrypt_manifest_path,
            test_name,
        } => aws_esdk::test_vectors::run_tests::encrypt_test_vectors(manifest_path, decrypt_manifest_path, test_name).await,
        Commands::Decrypt {
            manifest_path,
            manifest_name,
            test_name,
        } => aws_esdk::test_vectors::run_tests::decrypt_test_vectors(manifest_path, manifest_name, test_name).await,
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}
