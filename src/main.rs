mod constants;
mod service;

use directories::BaseDirs;
use flexi_logger::{detailed_format, Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use inquire::Text;
use oshome_shell::start;

use clap::{Arg, ArgAction, Command};
use log::{debug, info, warn, error};
use oshome::Config;
use oshome_mqtt::start_mqtt_client;
use service::{install, uninstall};
use std::path::Path;
use std::sync::mpsc::{self, Receiver};
use std::time::Duration;
use std::{env, fs};
use tokio::sync::broadcast;
use tokio::{runtime::Runtime, signal};

#[cfg(target_os = "windows")]
use windows_service::{
    define_windows_service,
    service::{
        ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
        ServiceType,
    },
    service_control_handler::{self, ServiceControlHandlerResult},
};
#[cfg(target_os = "windows")]
define_windows_service!(ffi_service_main, windows_service_main);

#[cfg(target_os = "windows")]
fn windows_service_main(_arguments: Vec<std::ffi::OsString>) {
    use std::time::Duration;
    
    use constants::SERVICE_NAME;
    info!("Starting Windows service...");

    let (shutdown_tx, shutdown_rx) = std::sync::mpsc::channel();
    let status_handle =
        service_control_handler::register(SERVICE_NAME, move |control_event| match control_event {
            ServiceControl::Stop => {
                shutdown_tx.send(()).unwrap();
                ServiceControlHandlerResult::NoError
            }
            _ => ServiceControlHandlerResult::NotImplemented,
        })
        .unwrap();

    status_handle
        .set_service_status(ServiceStatus {
            process_id: None,
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Running,
            controls_accepted: ServiceControlAccept::STOP,
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        })
        .unwrap();

    let mut dir = env::current_exe().unwrap();
    dir.pop();
    dir.push("config.yaml");
    let config = dir.to_string_lossy().to_string();
    info!("Config file path: {}", config);
    if let Err(e) = run(Some(&config), Some(shutdown_rx)) {
        error!("{}", e)
    }

    info!("Service is stopping...");
    status_handle
        .set_service_status(ServiceStatus {
            process_id: None,
            service_type: ServiceType::OWN_PROCESS,
            current_state: ServiceState::Stopped,
            controls_accepted: ServiceControlAccept::empty(),
            exit_code: ServiceExitCode::Win32(0),
            checkpoint: 0,
            wait_hint: Duration::from_secs(0),
        })
        .unwrap();
}

// use oshome::
const VERSION: &str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
const CARGO_PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

fn cli() -> Command {
    Command::new("oshome")
        .about(format!("OSHome - {}\n\n{}\n{}" ,VERSION, DESCRIPTION, CARGO_PKG_HOMEPAGE))
        .version(VERSION)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .args([
            Arg::new("configuration_file")
                .short('c')
                .long("configuration")
                .help("Optional configuration file. If not provided, the default configuration will be used.")
                .default_value("config.yaml")
                // .group("tests")
        ])
        .subcommand(
            Command::new("install")
                    .about("Install OSHome")
                    .arg(
                        Arg::new("location")
                        .help( "The location to install OSHome.")
                    )
        )
        .subcommand(
            Command::new("uninstall")
                    .about("Uninstall OSHome")
                    .arg(
                        Arg::new("location")
                        .help( "The location where OSHome is installed.")
                    )
        )
        .subcommand(
            Command::new("run")
                .about("Run OSHome manually.")
                .args([
                    Arg::new("as-windows-service")
                        .long("as-windows-service")
                        .help("Flag to identify if run as windows service.")
                        .hide(true)
                        .action(ArgAction::SetTrue)
                        .num_args(0)
                        // .group("tests")
                ])
        )
}

// Embed the default configuration file at compile time
// const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

// Function to read the YAML configuration file
fn read_full_config(path: Option<&String>) -> Result<Config, serde_yaml::Error> {
    if let Some(path) = path {
        let config_file_path = fs::canonicalize(path).unwrap();
        println!("Config file path: {}", config_file_path.display());
        if let Ok(content) = fs::read_to_string(config_file_path) {
            return serde_yaml::from_str(&content);
        } else {
            warn!(
                "Failed to read the configuration file at '{}', falling back to default.",
                path
            );
        }
    }

    // Fallback to the embedded default configuration
    // serde_yaml::from_str(DEFAULT_CONFIG)
    panic!("oh no!");
}

fn read_base_config(path: Option<&String>) -> Result<oshome_core::CoreConfig, serde_yaml::Error> {
    if let Some(path) = path {
        let config_file_path = fs::canonicalize(path).unwrap();
        if let Ok(content) = fs::read_to_string(config_file_path) {
            return serde_yaml::from_str(&content);
        } else {
            warn!(
                "Failed to read the configuration file at '{}', falling back to default.",
                path
            );
        }
    }

    // Fallback to the embedded default configuration
    // serde_yaml::from_str(DEFAULT_CONFIG)
    panic!("oh no!");
}

fn main() {
    println!("Starting OSHome - {}", VERSION);

    
    #[cfg(not(debug_assertions))]
    let base_dirs = BaseDirs::new().expect("Failed to get base directories");
    #[cfg(not(debug_assertions))]
    let log_directory = base_dirs.data_local_dir();

    #[cfg(debug_assertions)]
    let log_directory = Path::new("./");
    
    #[cfg(not(debug_assertions))]
    let log_level = "info";

    #[cfg(debug_assertions)]
    let log_level = "debug";

    let mut logger_builder = Logger::try_with_env_or_str(log_level).unwrap();

    logger_builder = logger_builder.format_for_files(detailed_format)
        .log_to_file(FileSpec::default().directory(&log_directory)) // write logs to file
        // .write_mode(WriteMode::BufferAndFlush)
        .append()
        .rotate(
            Criterion::AgeOrSize(Age::Day, 10 * 1024 * 1024),
            Naming::Timestamps,
            Cleanup::KeepLogFiles(7),
        );
    
    if cfg!(debug_assertions) {
        logger_builder = logger_builder.duplicate_to_stdout(Duplicate::Debug);
    }

    let mut logger = logger_builder
        .start()
        .unwrap();

    // TODO: Implement logger entry
    // logger.parse_and_push_temp_spec("info, critical_mod = trace");
    // logger.pop_temp_spec();
    println!("LogDirectory: {}", log_directory.display());

    let matches = cli().get_matches();
    let config_file = matches.try_get_one::<String>("configuration_file").unwrap();
    #[cfg(any(target_os = "linux", target_os = "macos"))]
    let default_installation_path = "/usr/bin/oshome";
    #[cfg(target_os = "windows")]
    let default_installation_path = "C:\\Program Files\\oshome";

    match matches.subcommand() {
        Some(("install", sub_matches)) => {
            let location = sub_matches.try_get_one::<String>("location").unwrap();
            if let Some(location) = location {
                // Perform installation logic here
                install(location);
            } else {
                let location = Text::new("Where do you want to install OSHome?")
                    .with_default(default_installation_path)
                    .prompt();
                install(location.unwrap().as_str());
            }
        }
        Some(("uninstall", sub_matches)) => {
            let location = sub_matches.try_get_one::<String>("location").unwrap();
            if let Some(location) = location {
                // Perform installation logic here
                uninstall(location);
            } else {
                let location = Text::new("OS Home is not installed under the default location. Where should the uninstall script run?").with_default(default_installation_path).prompt();
                uninstall(location.unwrap().as_str());
            }
        }
        Some(("run", sub_matches)) => {
            let is_windows_service = sub_matches.get_one::<bool>("as-windows-service").unwrap();
            if *is_windows_service {
                #[cfg(target_os = "linux")]
                // Run as a Windows service
                info!("Running as Windows service");
                #[cfg(target_os = "windows")]
                use windows_service::service_dispatcher;
                #[cfg(target_os = "windows")]
                service_dispatcher::start(constants::SERVICE_NAME, ffi_service_main).unwrap();
                panic!("Running as a Windows service is not supported on Linux.");
            } else {
                // Run normally
                run(config_file.clone(), None).unwrap();
            }
        }
        _ => {
            println!("No subcommand was used");
        }
    }
    std::process::exit(0);
}

fn run(
    config_file: Option<&String>,
    shutdown_signal: Option<Receiver<()>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let rt = Runtime::new().unwrap();

    // Spawn the root task
    rt.block_on(async {
        // let mut listener = TcpListener::bind("127.0.0.1:8080").await?;

        // let cli = Args::parse();
        // Read the configuration file or use the embedded default
        let config: Config = read_full_config(config_file).unwrap_or_else(|err| {
            warn!(
                "Failed to parse configuration: {}, using default configuration.",
                err
            );
            read_full_config(None).expect("Failed to load embedded default configuration")
        });

        let base_config: oshome_core::CoreConfig =
            read_base_config(config_file).unwrap_or_else(|err| {
                warn!(
                    "Failed to parse configuration: {}, using default configuration.",
                    err
                );
                read_base_config(None).expect("Failed to load embedded default configuration")
            });

        let (tx, mut _rx) = broadcast::channel(16);

        let mqtt_base_config = base_config.clone();
        // Start MQTT client in a separate task
        if let Some(mqtt_config) = config.mqtt {
            let tx2 = tx.clone();
            tokio::spawn(async move {
                let rx2 = tx2.subscribe();
                start_mqtt_client(tx2, rx2, &mqtt_base_config, &mqtt_config).await;
            });
        }

        if let Some(shell_config) = config.shell {
            let tx2 = tx.clone();
            let shell_base_config = base_config.clone();

            tokio::spawn(async move {
                let rx2 = tx2.subscribe();
                start(tx2, rx2, &shell_base_config, &shell_config).await;
            });
        };

        
        if let Some(gpio_config) = config.gpio {
            let tx2 = tx.clone();
            let shell_base_config = base_config.clone();

            tokio::spawn(async move {
                let rx2 = tx2.subscribe();
                oshome_gpio::start(tx2, rx2, &shell_base_config, &gpio_config).await;
            });
        };

        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        if let Some(some_shutdown_signal) = shutdown_signal {
            let shutdown_event = async {
                loop {
                    // Poll shutdown event.
                    match some_shutdown_signal.recv_timeout(Duration::from_secs(1)) {
                        // Break the loop either upon stop or channel disconnect
                        Ok(_) | Err(mpsc::RecvTimeoutError::Disconnected) => break,

                        // Continue work if no events were received within the timeout
                        Err(mpsc::RecvTimeoutError::Timeout) => ()
                    };
                }
            };
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
                _ = shutdown_event => {},
            }
        } else {
            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
        }
    });
    info!("Shutdown complete");
    Ok(())
}

