//! # SBMUMC CLI Interface
//!
//! Command-line tool for SBMUMC system interaction

use crate::core::config::SbmumcConfig;
use crate::omnidev_integration::OmniDevIntegration;
use crate::core::SbmumcError;
use std::path::PathBuf;
use std::io::{self, Write};

#[derive(Debug, Clone)]
pub struct CliConfig {
    pub interactive: bool,
    pub verbose: bool,
    pub config_path: Option<PathBuf>,
    pub log_file: Option<PathBuf>,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            interactive: true,
            verbose: false,
            config_path: None,
            log_file: None,
        }
    }
}

pub struct SbmumcCli {
    config: SbmumcConfig,
    cli_config: CliConfig,
    omnidev: Option<OmniDevIntegration>,
}

impl SbmumcCli {
    pub fn new(cli_config: CliConfig, system_config: SbmumcConfig) -> Self {
        Self {
            config: system_config,
            cli_config,
            omnidev: None,
        }
    }

    pub fn init(&mut self) -> Result<(), CliError> {
        if self.cli_config.verbose {
            println!("Initializing SBMUMC CLI...");
            println!("Mode: {:?}", self.config.system.mode);
        }

        // Initialize OmniDev if enabled
        if self.config.omnidev.enabled {
            self.omnidev = Some(OmniDevIntegration::new());
            if self.cli_config.verbose {
                println!("OmniDev AGI initialized");
            }
        }

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), CliError> {
        if self.cli_config.interactive {
            self.run_repl()
        } else {
            self.run_command()
        }
    }

    fn run_repl(&mut self) -> Result<(), CliError> {
        println!("\n╔════════════════════════════════════════╗");
        println!("║     SBMUMC Interactive Terminal      ║");
        println!("╚════════════════════════════════════════╝");
        println!("Type 'help' for available commands, 'exit' to quit\n");

        loop {
            print!("sbmumc> ");
            io::stdout().flush().map_err(|e| CliError::IoError(e.to_string()))?;

            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .map_err(|e| CliError::IoError(e.to_string()))?;

            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            match self.process_input(input) {
                Ok(Some(response)) => println!("{}", response),
                Ok(None) => {}
                Err(e) => println!("Error: {}", e),
            }

            if input == "exit" || input == "quit" || input == "q" {
                println!("Shutting down SBMUMC...");
                break;
            }
        }

        Ok(())
    }

    fn process_input(&mut self, input: &str) -> Result<Option<String>, CliError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            return Ok(None);
        }

        match parts[0] {
            "help" => Ok(Some(self.show_help())),
            "status" => self.show_status(),
            "omnidev" => self.handle_omnidev_command(&parts[1..]),
            "config" => self.handle_config_command(&parts[1..]),
            "graph" => self.handle_graph_command(&parts[1..]),
            "test" => self.handle_test_command(&parts[1..]),
            "clear" => {
                print!("\x1B[2J\x1B[1H");
                Ok(None)
            },
            _ => Ok(Some(format!("Unknown command: {}\nType 'help' for available commands.", parts[0]))),
        }
    }

    fn show_help(&self) -> String {
        r#"Available Commands:
  help              Show this help message
  status            Show system status
  clear             Clear the screen
  
  OmniDev Commands:
    omnidev init     Initialize OmniDev AGI
    omnidev status   Show OmniDev status
    omnidev process  Process a refactor request
  
  Graph Commands:
    graph search     Search semantic graph
    graph stats      Show graph statistics
  
  Config Commands:
    config show      Show current configuration
    config validate  Validate configuration
  
  Test Commands:
    test run         Run test suite
    test coverage    Show coverage report

  System Commands:
    exit/quit        Exit the CLI
"#.to_string()
    }

    fn show_status(&self) -> Result<Option<String>, CliError> {
        let mut status = String::new();
        status.push_str("═══════════════════════════════════════\n");
        status.push_str("           SBMUMC SYSTEM STATUS          \n");
        status.push_str("═══════════════════════════════════════\n\n");

        status.push_str(&format!("System Mode:    {:?}\n", self.config.system.mode));
        status.push_str(&format!("Network Mode:   {:?}\n", self.config.network.mode));
        status.push_str(&format!("Threads:        {}\n", self.config.system.threads));
        status.push_str(&format!("Max Memory:     {} MB\n", self.config.system.max_memory_mb));
        status.push_str(&format!("OmniDev:        {}\n", if self.config.omnidev.enabled { "Enabled" } else { "Disabled" }));
        status.push_str(&format!("Latency Target: {} ms\n", self.config.omnidev.latency_target_ms));
        status.push_str(&format!("Encryption:     {}\n", if self.config.security.encryption_enabled { "Enabled" } else { "Disabled" }));
        status.push_str(&format!("Audit Trail:    {}\n", if self.config.security.audit_trail_immutable { "Immutable" } else { "Mutable" }));

        if let Some(ref omnidev) = self.omnidev {
            let system_status = omnidev.get_system_status();
            status.push_str("\n─── OmniDev Status ───\n");
            status.push_str(&format!("Integration ID:  {}\n", system_status.integration_id));
            status.push_str(&format!("State:          {}\n", system_status.state));
            status.push_str(&format!("Nodes Indexed:  {}\n", system_status.nodes_indexed));
            status.push_str(&format!("Latency P99:    {} ms\n", system_status.latency_p99_ms));
            status.push_str(&format!("Audit Entries:  {}\n", system_status.audit_entries));
        }

        status.push_str("\n═══════════════════════════════════════\n");

        Ok(Some(status))
    }

    fn handle_omnidev_command(&mut self, args: &[&str]) -> Result<Option<String>, CliError> {
        match args.first().copied() {
            Some("init") => {
                if self.omnidev.is_none() {
                    self.omnidev = Some(OmniDevIntegration::new());
                    Ok(Some("OmniDev AGI initialized successfully".to_string()))
                } else {
                    Ok(Some("OmniDev is already initialized".to_string()))
                }
            },
            Some("status") => {
                if let Some(ref omnidev) = self.omnidev {
                    let status = omnidev.get_system_status();
                    Ok(Some(format!(
                        "OmniDev Status:\n  Integration ID: {}\n  State: {}\n  Nodes: {}\n  Latency P99: {}ms\n  Audit Entries: {}",
                        status.integration_id,
                        status.state,
                        status.nodes_indexed,
                        status.latency_p99_ms,
                        status.audit_entries
                    )))
                } else {
                    Ok(Some("OmniDev not initialized. Run 'omnidev init' first.".to_string()))
                }
            },
            Some("process") => {
                if let Some(ref mut omnidev) = self.omnidev {
                    let request = args.get(1).map(|s| s.to_string()).unwrap_or_else(|| "process_code".to_string());
                    match omnidev.process_refactor_request(&request) {
                        Ok(result) => Ok(Some(format!(
                            "Refactor Result:\n  Request ID: {}\n  Context Nodes: {}\n  Architecture Layers: {}\n  Transaction Committed: {}\n  Tests Generated: {}\n  Merge Allowed: {}\n  Total Time: {}ms",
                            result.request_id,
                            result.context_nodes,
                            result.architecture_layers,
                            result.transaction_committed,
                            result.tests_generated,
                            result.merge_allowed,
                            result.total_time_ms
                        ))),
                        Err(e) => Ok(Some(format!("Error: {:?}", e))),
                    }
                } else {
                    Ok(Some("OmniDev not initialized".to_string()))
                }
            },
            _ => Ok(Some("Usage: omnidev [init|status|process <request>]".to_string())),
        }
    }

    fn handle_config_command(&self, args: &[&str]) -> Result<Option<String>, CliError> {
        match args.first().copied() {
            Some("show") => {
                Ok(Some(format!("{:?}", self.config)))
            },
            Some("validate") => {
                let errors = self.config.validate();
                if errors.is_empty() {
                    Ok(Some("Configuration is valid".to_string()))
                } else {
                    Ok(Some(format!("Validation errors:\n{}",
                        errors.iter()
                            .map(|e| format!("  - {}: {}", e.field, e.message))
                            .collect::<Vec<_>>()
                            .join("\n")
                    )))
                }
            },
            _ => Ok(Some("Usage: config [show|validate]".to_string())),
        }
    }

    fn handle_graph_command(&self, args: &[&str]) -> Result<Option<String>, CliError> {
        match args.first().copied() {
            Some("stats") => {
                Ok(Some("Graph Statistics:\n  Total Nodes: 0\n  Total Edges: 0\n  Index Size: 0 MB".to_string()))
            },
            Some("search") => {
                Ok(Some("Graph search initiated (requires query)".to_string()))
            },
            _ => Ok(Some("Usage: graph [stats|search]".to_string())),
        }
    }

    fn handle_test_command(&self, args: &[&str]) -> Result<Option<String>, CliError> {
        match args.first().copied() {
            Some("run") => {
                Ok(Some("Running test suite...\n[Currently no tests configured]".to_string()))
            },
            Some("coverage") => {
                Ok(Some("Coverage Report:\n  Lines: 0%\n  Functions: 0%\n  Branches: 0%".to_string()))
            },
            _ => Ok(Some("Usage: test [run|coverage]".to_string())),
        }
    }

    fn run_command(&self) -> Result<(), CliError> {
        println!("Single command mode - use --help for usage");
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum CliError {
    IoError(String),
    ConfigError(String),
    CommandError(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::IoError(msg) => write!(f, "IO Error: {}", msg),
            CliError::ConfigError(msg) => write!(f, "Config Error: {}", msg),
            CliError::CommandError(msg) => write!(f, "Command Error: {}", msg),
        }
    }
}

impl std::error::Error for CliError {}

pub fn run_cli() -> Result<(), CliError> {
    let cli_config = CliConfig {
        interactive: true,
        verbose: true,
        config_path: None,
        log_file: None,
    };

    let system_config = if let Some(path) = &cli_config.config_path {
        SbmumcConfig::load_from_file(path).map_err(|e| CliError::ConfigError(format!("{:?}", e)))?
    } else {
        SbmumcConfig::default()
    };

    let mut cli = SbmumcCli::new(cli_config, system_config);
    cli.init()?;
    cli.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_creation() {
        let cli_config = CliConfig::default();
        let system_config = SbmumcConfig::default();
        let cli = SbmumcCli::new(cli_config, system_config);
        assert!(cli.omnidev.is_none());
    }

    #[test]
    fn test_help_message() {
        let cli_config = CliConfig::default();
        let system_config = SbmumcConfig::default();
        let cli = SbmumcCli::new(cli_config, system_config);
        let help = cli.show_help();
        assert!(help.contains("omnidev"));
        assert!(help.contains("config"));
    }
}
