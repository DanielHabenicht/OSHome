use oshome_shell::start;

use clap::Parser;
use env_logger;
use env_logger::Env;
use log::{info, warn};
use oshome::Config;
use oshome_mqtt::start_mqtt_client;
use std::fs;
use tokio::signal;
use tokio::sync::broadcast;



#[derive(Parser)]
#[command(name = "OSHome")]
struct Cli {
    #[arg(
        short,
        long,
        help = "Optional configuration file. If not provided, the default configuration will be used."
    )]
    configuration_override_file: Option<String>,
}

// Embed the default configuration file at compile time
const DEFAULT_CONFIG: &str = include_str!("../config.yaml");

// Function to read the YAML configuration file
fn read_full_config(path: Option<&str>) -> Result<Config, serde_yaml::Error> {
    if let Some(path) = path {
        if let Ok(content) = fs::read_to_string(path) {
            return serde_yaml::from_str(&content);
        } else {
            warn!(
                "Failed to read the configuration file at '{}', falling back to default.",
                path
            );
        }
    }

    // Fallback to the embedded default configuration
    serde_yaml::from_str(DEFAULT_CONFIG)
}

fn read_base_config(path: Option<&str>) -> Result<oshome_core::CoreConfig, serde_yaml::Error> {
    if let Some(path) = path {
        if let Ok(content) = fs::read_to_string(path) {
            return serde_yaml::from_str(&content);
        } else {
            warn!(
                "Failed to read the configuration file at '{}', falling back to default.",
                path
            );
        }
    }

    // Fallback to the embedded default configuration
    serde_yaml::from_str(DEFAULT_CONFIG)
}

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let cli = Cli::parse();

    // Read the configuration file or use the embedded default
    let config: Config =
        read_full_config(cli.configuration_override_file.as_deref()).unwrap_or_else(|err| {
            warn!(
                "Failed to parse configuration: {}, using default configuration.",
                err
            );
            read_full_config(None).expect("Failed to load embedded default configuration")
        });

    let base_config: oshome_core::CoreConfig = read_base_config(cli.configuration_override_file.as_deref())
        .unwrap_or_else(|err| {
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

            // while let Ok(Some(cmd)) = rx.recv().await {
            //     use Message::*;

            //     match cmd {
            //         ButtonPress { key } => {
            //             if let Some(button) = &base_config.button.as_ref().and_then(|b| b.get(&key))
            //             {
            //                 debug!("Button pressed: {}", key);
            //                 debug!("Executing command: {}", button.command);
            //             } else {
            //                 debug!("Button pressed: {}", key);
            //                 warn!("No Action found?!");
            //             }
            //         },
            //         SensorValueChange { key, value } => {
            //             debug!("Sensor value changed: {} = {}", key, value);
            //             // Handle sensor value change
            //         }
            //     }
            // }
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

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutdown complete");
    std::process::exit(0);
}
