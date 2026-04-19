//! SBMUMC Binary Entry Point
//!
//! This is the main binary entry point for the Samuel Benwellonedge Mukandara
//! Universal Meta-Compiler system.

use sbmumc::{init_tracing, Sbmumc, SbmumcConfig};
use std::path::PathBuf;
use structopt::StructOpt;
use tracing::{info, error};

/// Command line arguments for SBMUMC
#[derive(Debug, StructOpt)]
#[structopt(name = "sbmumc", about = "Universal Meta-Compiler and AGI System")]
struct CliArgs {
    /// Configuration file path
    #[structopt(short, long, default_value = "sbmumc.toml")]
    config: PathBuf,

    /// Enable verbose logging
    #[structopt(short, long)]
    verbose: bool,

    /// Run in headless mode (no admin interface)
    #[structopt(long)]
    headless: bool,

    /// Initialize new instance
    #[structopt(long)]
    init: bool,

    /// Compile a file or project
    #[structopt(long)]
    compile: Option<PathBuf>,

    /// Target architecture for compilation
    #[structopt(long, default_value = "universal")]
    target: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    init_tracing();

    // Parse command line arguments
    let args = CliArgs::from_args();

    info!(
        version = sbmumc::VERSION,
        system = sbmumc::SYSTEM_NAME,
        "Starting SBMUMC"
    );

    // Load or create configuration
    let config = if args.config.exists() {
        SbmumcConfig::load_from_file(&args.config)?
    } else {
        SbmumcConfig::default()
    };

    // Handle initialization mode
    if args.init {
        info!("Initializing new SBMUMC instance");
        let config_path = args.config.clone();
        config.save_to_file(&config_path)?;
        info!("Configuration saved to {:?}", config_path);
        return Ok(());
    }

    // Handle compilation mode
    if let Some(path) = args.compile {
        info!("Compiling: {:?}", path);
        let mut system = Sbmumc::new(config).await?;
        let result = system.compile_file(&path, &args.target).await?;
        println!("Compilation result: {:?}", result);
        return Ok(());
    }

    // Initialize the main system
    let mut system = match Sbmumc::new(config).await {
        Ok(s) => s,
        Err(e) => {
            error!("Failed to initialize SBMUMC: {}", e);
            return Err(Box::new(e));
        }
    };

    // Start the system
    if let Err(e) = system.start().await {
        error!("System error: {}", e);
        return Err(Box::new(e));
    }

    info!("SBMUMC started successfully");

    // If not headless, start the admin interface
    if !args.headless {
        info!("Starting admin interface");
        if let Err(e) = system.start_admin_interface().await {
            error!("Admin interface error: {}", e);
        }
    }

    // Keep the system running
    system.wait_for_shutdown().await?;

    info!("SBMUMC shutting down");
    Ok(())
}
