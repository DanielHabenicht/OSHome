use log::warn;
use oshome_core::{config_template, home_assistant::sensors::{Component, HASensor}, Message, Module};
use std::{collections::HashMap, future::Future, pin::Pin, str, time::Duration};
use tokio::sync::broadcast::{Receiver, Sender};
use serde::{Deserialize, Deserializer};

#[derive(Clone, Deserialize, Debug)]
pub struct BME280InternalConfig {
    pub name: Option<String>
}

#[derive(Clone, Deserialize, Debug)]
pub struct BME280SensorConfig {
    pub temperature: BME280InternalConfig,
    pub pressure: BME280InternalConfig,
    pub humidity: BME280InternalConfig,
}


#[derive(Clone, Deserialize, Debug)]
pub struct NoConfig {
    // pub bla: String
}


config_template!(bme280, Option<NoConfig>, NoConfig, NoConfig, BME280SensorConfig);


#[derive(Clone, Debug)]
pub struct Default{
    config: CoreConfig,

} 

impl Default {
    pub fn new(config_string: &String) -> Self {

        let core_config = serde_yaml::from_str::<CoreConfig>(config_string).unwrap();

        Default {
            config: core_config,
        }
    }
}

impl Module for Default {
    fn validate(&mut self) -> Result<(), String> {
        Ok(())
    }



    fn init(&mut self) -> Result<Vec<Component>, String> {
        let mut components: Vec<Component> = Vec::new();

        if let Some(sensors) = self.config.sensor.clone() {
            for (key, sensor) in sensors {
                match sensor.extra {
                    SensorKind::bme280(_) => {
                        let id = format!("{}_{}_{}", self.config.oshome.name, key.clone(), "temperature");
                        components.push(
                            Component::Sensor(
                                HASensor {
                                    platform: "sensor".to_string(),
                                    icon: sensor.icon.clone(),
                                    unique_id: id.clone(),
                                    device_class: sensor.device_class.clone(),
                                    unit_of_measurement: sensor.unit_of_measurement.clone(),
                                    name: sensor.name.clone(),
                                    object_id: id.clone(),
                                }
                            )
                        );
                        let id = format!("{}_{}_{}", self.config.oshome.name, key.clone(), "pressure");
                        components.push(
                            Component::Sensor(
                                HASensor {
                                    platform: "sensor".to_string(),
                                    icon: sensor.icon.clone(),
                                    unique_id: id.clone(),
                                    device_class: sensor.device_class.clone(),
                                    unit_of_measurement: sensor.unit_of_measurement.clone(),
                                    name: sensor.name.clone(),
                                    object_id: id.clone(),
                                }
                            )
                        );
                        let id = format!("{}_{}_{}", self.config.oshome.name, key.clone(), "pressure");
                        components.push(
                            Component::Sensor(
                                HASensor {
                                    platform: "sensor".to_string(),
                                    icon: sensor.icon.clone(),
                                    unique_id: id.clone(),
                                    device_class: sensor.device_class.clone(),
                                    unit_of_measurement: sensor.unit_of_measurement.clone(),
                                    name: sensor.name.clone(),
                                    object_id: id.clone(),
                                }
                            )
                        );
                    },
                    _ => {}
                }
            }
        }
        Ok(components)
    }

    fn run(
        &self,
        sender: Sender<Option<Message>>,
        _: Receiver<Option<Message>>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static>>
    {
        // let mqtt_config = self.mqtt_config.clone();
        // let config = config.clone();
        Box::pin(async move {
            #[cfg(any(target_os = "macos", target_os = "windows"))]
            {
                panic!("GPIO is not supported.");
            }
            #[cfg(target_os = "linux")]
            {
                use linux_embedded_hal::{Delay, I2cdev};
                use bme280::i2c::BME280;

                
                let result = I2cdev::new("/dev/i2c-1");
                match result {
                    Err(e) => {
                        warn!("Error initializing I2C: {}", e);
                        return Ok(())
                    }
                    _ => {}
                }

                // using Linux I2C Bus #1 in this example
                let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();

                // initialize the BME280 using the primary I2C address 0x76
                let mut bme280 = BME280::new_primary(i2c_bus);

                // or, initialize the BME280 using the secondary I2C address 0x77
                // let mut bme280 = BME280::new_secondary(i2c_bus, Delay);

                // or, initialize the BME280 using a custom I2C address
                // let bme280_i2c_addr = 0x88;
                // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);

                // initialize the sensor
                let mut delay = Delay;
                bme280.init(&mut delay).unwrap();

                // measure temperature, pressure, and humidity
                let measurements = bme280.measure(&mut delay).unwrap();

                println!("Relative Humidity = {}%", measurements.humidity);
                println!("Temperature = {} deg C", measurements.temperature);
                println!("Pressure = {} pascals", measurements.pressure);

                let msg = Some(Message::SensorValueChange {
                    key: "bme280.temperature".to_string(),
                    value: measurements.temperature.to_string(),
                });

                sender.send(msg).unwrap();


                // Handle Button Presses
                // let cloned_config = config.clone();
                // let cloned_shell_config = shell_config.clone();
            }



            Ok(())
        })
    }

} 
