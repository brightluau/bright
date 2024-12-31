use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Bright, a scriptable tool for transforming and transpiling Luau code
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the current folder with a Bright setup
    Init,

    /// Runs the configured transformers over source code
    Run {
        #[arg(default_value="src/")]
        /// The source folder to run the transformers on
        source: PathBuf,
    },

    /// Runs a singular transformer for testing purposes
    Test {
        /// The transformer to test
        rule: String,

        /// The source file to run the transformer on
        source: PathBuf,

        /// The output file to write the transformed code to
        output: PathBuf,
    },
}

fn main() {
    let log_level = match std::env::var("RUST_LOG") {
        Ok(value) => value,
        Err(_) => "info".to_string(),
    };

    std::env::set_var("RUST_LOG", log_level);
    pretty_env_logger::init();

    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            todo!()
        },
        Commands::Run { source: source_folder } => {
            log::info!("folder: {}", source_folder.display());
            todo!()
        },
        Commands::Test { .. } => todo!()
    }
}
