use esphome_native_api::parser;
use esphome_native_api::parser::ProtoMessage;
use esphome_native_api::proto;
use esphome_native_api::proto::BluetoothServiceData;
use esphome_native_api::proto::DeviceInfoResponse;
use esphome_native_api::proto::EntityCategory;
use esphome_native_api::proto::SensorLastResetType;
use esphome_native_api::proto::SensorStateClass;
use esphome_native_api::to_packet_from_ref;
use log::debug;
use log::error;
use log::info;
use log::trace;
use log::warn;
use serde::{Deserialize, Deserializer};
use ubihome_core::features::ip::get_ip_address;
use ubihome_core::features::ip::get_network_mac_address;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::num::ParseIntError;
use std::{future::Future, pin::Pin, str};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpSocket;
use tokio::sync::broadcast;
use tokio::sync::broadcast::Receiver;
use tokio::sync::broadcast::Sender;
use ubihome_core::internal::sensors::InternalComponent;
use ubihome_core::NoConfig;
use ubihome_core::{
    config_template, home_assistant::sensors::Component, ChangedMessage, Module, PublishedMessage,
};

#[derive(Clone, Deserialize, Debug)]
pub struct ApiConfig {}

fn mac_to_u64(mac: &str) -> Result<u64, ParseIntError> {
    let mac = mac.replace(":", "");
    u64::from_str_radix(&mac, 16)
}

config_template!(
    api,
    Option<ApiConfig>,
    NoConfig,
    NoConfig,
    NoConfig,
    NoConfig
);

#[derive(Clone, Debug)]
pub struct UbiHomeDefault {
    config: CoreConfig,
    components_by_key: HashMap<u32, ProtoMessage>,
    components_key_id: HashMap<String, u32>,
    device_info: DeviceInfoResponse,
}

impl Module for UbiHomeDefault {
    fn new(config_string: &String) -> Result<Self, String> {
        match serde_yaml::from_str::<CoreConfig>(config_string) {
            Ok(config) => {
                let ip = get_ip_address().unwrap();
                let mac = get_network_mac_address(ip).unwrap();


                let device_info = DeviceInfoResponse {
                    uses_password: false,
                    name: config.ubihome.name.clone(),
                    mac_address: mac,
                    esphome_version: "2025.4.0".to_owned(),
                    compilation_time: "".to_owned(),
                    model: whoami::devicename(),
                    has_deep_sleep: false,
                    project_name: "".to_owned(),
                    project_version: "".to_owned(),
                    webserver_port: 8080,
                    // See https://github.com/esphome/aioesphomeapi/blob/c1fee2f4eaff84d13ca71996bb272c28b82314fc/aioesphomeapi/model.py#L154
                    legacy_bluetooth_proxy_version: 1,
                    bluetooth_proxy_feature_flags: 1,
                    manufacturer: "Test".to_string(),
                    // format!(
                    //     "{} {} {}",
                    //     whoami::platform(),
                    //     whoami::distro(),
                    //     whoami::arch()
                    // ),
                    friendly_name: config
                        .ubihome
                        .friendly_name
                        .clone()
                        .unwrap_or(config.ubihome.name.clone()),
                    legacy_voice_assistant_version: 0,
                    voice_assistant_feature_flags: 0,
                    suggested_area: "".to_owned(),
                    bluetooth_mac_address: "18:65:71:EB:5A:FB".to_owned(),
                };

                Ok(UbiHomeDefault {
                    config: config,
                    components_by_key: HashMap::new(),
                    components_key_id: HashMap::new(),
                    device_info: device_info,
                })
            }
            Err(e) => {
                return Err(format!("Failed to parse API config: {:?}", e));
            }
        }
    }

    fn components(&mut self) -> Vec<InternalComponent> {
        Vec::new()
    }

    fn run(
        &self,
        sender: Sender<ChangedMessage>,
        mut receiver: Receiver<PublishedMessage>,
    ) -> Pin<Box<dyn Future<Output = Result<(), Box<dyn std::error::Error>>> + Send + 'static>>
    {
        let core_config = self.config.clone();
        // let mut api_components = self.components.();
        let device_info = self.device_info.clone();
        let mut api_components_by_key = self.components_by_key.clone();
        let mut api_components_key_id = self.components_key_id.clone();
        info!("Starting API with config: {:?}", core_config.api);
        Box::pin(async move {
            while let Ok(cmd) = receiver.recv().await {
                match cmd {
                    PublishedMessage::Components { components } => {
                        for (index, component) in components.iter().enumerate() {
                            match component.clone() {
                                Component::Switch(switch_entity) => {
                                    let component_switch_entity =
                                        ProtoMessage::ListEntitiesSwitchResponse(
                                            proto::ListEntitiesSwitchResponse {
                                                object_id: switch_entity.id.clone(),
                                                key: index.try_into().unwrap(),
                                                name: switch_entity.name,
                                                unique_id: switch_entity.id.clone(),
                                                icon: switch_entity.icon.unwrap_or_default(),
                                                device_class: switch_entity
                                                    .device_class
                                                    .unwrap_or_default(),
                                                disabled_by_default: false,
                                                entity_category: EntityCategory::None as i32,
                                                assumed_state: switch_entity.assumed_state,
                                            },
                                        );
                                    api_components_by_key
                                        .insert(index.try_into().unwrap(), component_switch_entity);
                                    api_components_key_id.insert(
                                        switch_entity.id.clone(),
                                        index.try_into().unwrap(),
                                    );
                                }
                                Component::Button(button) => {
                                    let component_button = ProtoMessage::ListEntitiesButtonResponse(
                                        proto::ListEntitiesButtonResponse {
                                            object_id: button.id.clone(),
                                            key: index.try_into().unwrap(),
                                            name: button.name,
                                            unique_id: button.id.clone(),
                                            icon: "".to_string(),
                                            device_class: "".to_string(), //button.device_class,
                                            disabled_by_default: false,
                                            entity_category: EntityCategory::None as i32,
                                        },
                                    );
                                    api_components_by_key
                                        .insert(index.try_into().unwrap(), component_button);
                                    api_components_key_id
                                        .insert(button.id.clone(), index.try_into().unwrap());
                                }
                                Component::Sensor(sensor) => {
                                    let component_sensor = ProtoMessage::ListEntitiesSensorResponse(
                                        proto::ListEntitiesSensorResponse {
                                            object_id: sensor.id.clone(),
                                            key: index.try_into().unwrap(),
                                            name: sensor.name,
                                            unique_id: sensor.id.clone(),
                                            icon: "".to_string(),
                                            unit_of_measurement: sensor
                                                .unit_of_measurement
                                                .unwrap_or("".to_string()),
                                            accuracy_decimals: 2, //sensor.accuracy_decimals,
                                            force_update: false,
                                            device_class: sensor
                                                .device_class
                                                .unwrap_or("".to_string()), //sensor.device_class,
                                            state_class: SensorStateClass::StateClassMeasurement
                                                as i32,
                                            last_reset_type: SensorLastResetType::LastResetNone
                                                as i32,
                                            disabled_by_default: false,
                                            entity_category: EntityCategory::None as i32,
                                        },
                                    );
                                    api_components_by_key
                                        .insert(index.try_into().unwrap(), component_sensor);
                                    api_components_key_id
                                        .insert(sensor.id.clone(), index.try_into().unwrap());
                                }
                                Component::BinarySensor(binary_sensor) => {
                                    let component_binary_sensor =
                                        ProtoMessage::ListEntitiesBinarySensorResponse(
                                            proto::ListEntitiesBinarySensorResponse {
                                                object_id: binary_sensor.id.clone(),
                                                key: index.try_into().unwrap(),
                                                name: binary_sensor.name,
                                                unique_id: binary_sensor.id.clone(),
                                                icon: "".to_string(),
                                                device_class: binary_sensor
                                                    .device_class
                                                    .unwrap_or("".to_string()), //binary_sensor.device_class,
                                                is_status_binary_sensor: false,
                                                disabled_by_default: false,
                                                entity_category: EntityCategory::None as i32,
                                            },
                                        );
                                    api_components_by_key
                                        .insert(index.try_into().unwrap(), component_binary_sensor);
                                    api_components_key_id.insert(
                                        binary_sensor.id.clone(),
                                        index.try_into().unwrap(),
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                }
                break;
            }
            let (answer_messages_tx, answer_messages_rx) = broadcast::channel::<ProtoMessage>(16);
            let (messages_tx, messages_rx) = broadcast::channel::<ProtoMessage>(16);
            let api_components_key_id_clone = api_components_key_id.clone();

            tokio::spawn(async move {
                while let Ok(cmd) = receiver.recv().await {
                    match cmd {
                        PublishedMessage::SensorValueChanged { key, value } => {
                            let key = api_components_key_id_clone.get(&key).unwrap();
                            debug!("SensorValueChanged: {:?}", &value);

                            messages_tx
                                .send(ProtoMessage::SensorStateResponse(
                                    proto::SensorStateResponse {
                                        key: key.clone(),
                                        state: value,
                                        missing_state: false,
                                    },
                                ))
                                .unwrap();
                        }
                        PublishedMessage::BinarySensorValueChanged { key, value } => {
                            let key = api_components_key_id_clone.get(&key).unwrap();
                            debug!("SensorValueChanged: {:?}", &value);

                            messages_tx
                                .send(ProtoMessage::BinarySensorStateResponse(
                                    proto::BinarySensorStateResponse {
                                        key: key.clone(),
                                        state: value,
                                        missing_state: false,
                                    },
                                ))
                                .unwrap();
                        }
                        PublishedMessage::SwitchStateChange { key, state } => {
                            let key = api_components_key_id_clone.get(&key).unwrap();
                            debug!("SensorValueChanged: {:?}", &state);

                            messages_tx
                                .send(ProtoMessage::SwitchStateResponse(
                                    proto::SwitchStateResponse {
                                        key: key.clone(),
                                        state: state,
                                    },
                                ))
                                .unwrap();
                        }
                        PublishedMessage::BluetoothProxyMessage(msg) => {
                            debug!("BluetoothProxyMessage: {:?}", &msg);
                            let service_data: Vec<BluetoothServiceData> = msg
                                .service_data
                                .iter()
                                .map(|(k, v)| BluetoothServiceData {
                                    uuid: k.to_string(),
                                    data: v.clone(),
                                    legacy_data: Vec::new(),
                                })
                                .collect();
                            let manufacturer_data = msg
                                .manufacturer_data
                                .iter()
                                .map(|(k, v)| BluetoothServiceData {
                                    uuid: k.to_string(),
                                    data: v.clone(),
                                    legacy_data: Vec::new(),
                                })
                                .collect();
                            let test = proto::BluetoothLeAdvertisementResponse {
                                address: mac_to_u64(&msg.mac).unwrap(),
                                rssi: msg.rssi.try_into().unwrap(),
                                address_type: 1,
                                name: msg.name.as_bytes().to_vec(),
                                service_uuids: msg.service_uuids,
                                service_data: service_data,
                                manufacturer_data: manufacturer_data,
                            };

                            messages_tx
                                .send(ProtoMessage::BluetoothLeAdvertisementResponse(test))
                                .unwrap();
                        }
                        _ => {}
                    }
                }
            });

            let addr: SocketAddr = "0.0.0.0:6053".parse().unwrap();
            let socket = TcpSocket::new_v4().unwrap();
            socket.set_reuseaddr(true).unwrap();

            socket.bind(addr).unwrap();
            let listener = socket.listen(128).unwrap();

            // let listener = TcpListener::bind(&addr).await?;
            debug!("Listening on: {}", addr);

            loop {
                // Asynchronously wait for an inbound socket.
                let (socket, _) = listener.accept().await?;
                debug!("Accepted request from {}", socket.peer_addr().unwrap());
                let (mut read, mut write) = tokio::io::split(socket);

                let device_info_clone = device_info.clone();
                let api_components_clone = api_components_by_key.clone();
                // Read Loop
                let answer_messages_tx_clone = answer_messages_tx.clone();
                let cloned_sender = sender.clone();
                tokio::spawn(async move {
                    let mut buf = vec![0; 1024];

                    loop {
                        let n = read
                            .read(&mut buf)
                            .await
                            .expect("failed to read data from socket");

                        if n == 0 {
                            return;
                        }

                        trace!("TCP: {:02X?}", &buf[0..n]);

                        let mut cursor = 0;

                        while cursor < n {
                            // Ignore first byte
                            // Get Length of packet

                            let len = buf[cursor + 1] as usize;
                            let message_type = buf[cursor + 2];
                            let packet_content = &buf[cursor + 3..cursor + 3 + len];

                            debug!("Message type: {}", message_type);
                            debug!("Message: {:?}", packet_content);

                            // TODO: Parse Frames
                            let message =
                                parser::parse_proto_message(message_type, packet_content).unwrap();

                            match message {
                                ProtoMessage::HelloRequest(hello_request) => {
                                    debug!("HelloRequest: {:?}", hello_request);
                                    let response_message = proto::HelloResponse {
                                        api_version_major: 1,
                                        api_version_minor: 10,
                                        server_info: "Rust: esphome-native-api".to_string(),
                                        name: device_info_clone.name.clone(),
                                    };
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::HelloResponse(response_message))
                                        .unwrap();
                                }
                                ProtoMessage::DeviceInfoRequest(device_info_request) => {
                                    debug!("DeviceInfoRequest: {:?}", device_info_request);
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::DeviceInfoResponse(
                                            device_info_clone.clone(),
                                        ))
                                        .unwrap();
                                }
                                ProtoMessage::ConnectRequest(connect_request) => {
                                    debug!("ConnectRequest: {:?}", connect_request);
                                    let response_message = proto::ConnectResponse {
                                        invalid_password: false,
                                    };
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::ConnectResponse(response_message))
                                        .unwrap();
                                }

                                ProtoMessage::DisconnectRequest(disconnect_request) => {
                                    debug!("DisconnectRequest: {:?}", disconnect_request);
                                    let response_message = proto::DisconnectResponse {};
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::DisconnectResponse(response_message))
                                        .unwrap();
                                }
                                ProtoMessage::ListEntitiesRequest(list_entities_request) => {
                                    debug!("ListEntitiesRequest: {:?}", list_entities_request);

                                    for (key, sensor) in &api_components_clone {
                                        answer_messages_tx_clone.send(sensor.clone()).unwrap();
                                    }
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::ListEntitiesDoneResponse(
                                            proto::ListEntitiesDoneResponse {},
                                        ))
                                        .unwrap();
                                }
                                ProtoMessage::PingRequest(ping_request) => {
                                    debug!("PingRequest: {:?}", ping_request);
                                    let response_message = proto::PingResponse {};
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::PingResponse(response_message))
                                        .unwrap();
                                }
                                ProtoMessage::SubscribeLogsRequest(request) => {
                                    debug!("SubscribeLogsRequest: {:?}", request);
                                    let response_message = proto::SubscribeLogsResponse {
                                        level: 0,
                                        message: "Test log".to_string().into_bytes(),
                                        send_failed: false,
                                    };
                                    answer_messages_tx_clone
                                        .send(ProtoMessage::SubscribeLogsResponse(response_message))
                                        .unwrap();
                                }
                                ProtoMessage::SubscribeBluetoothLeAdvertisementsRequest(
                                    request,
                                ) => {
                                    debug!(
                                        "SubscribeBluetoothLeAdvertisementsRequest: {:?}",
                                        request
                                    );
                                    // let response_message = proto::BluetoothLeAdvertisementResponse {
                                    //     address: u64::from_str_radix("000000000000", 16).unwrap(),
                                    //     rssi: -100,
                                    //     address_type: 0,
                                    //     // data: vec![0, 1, 2, 3, 4, 5],
                                    // };
                                    // answer_buf = [
                                    //     answer_buf,
                                    //     to_packet(ProtoMessage::BluetoothLeAdvertisementResponse(
                                    //         response_message,
                                    //     ))
                                    //     .unwrap(),
                                    // ]
                                    // .concat();
                                }
                                ProtoMessage::UnsubscribeBluetoothLeAdvertisementsRequest(
                                    request,
                                ) => {
                                    debug!(
                                        "UnsubscribeBluetoothLeAdvertisementsRequest: {:?}",
                                        request
                                    );
                                    // let response_message = proto::BluetoothLeAdvertisementResponse {
                                    //     address: u64::from_str_radix("000000000000", 16).unwrap(),
                                    //     rssi: -100,
                                    //     address_type: 0,
                                    //     // data: vec![0, 1, 2, 3, 4, 5],
                                    // };
                                    // answer_buf = [
                                    //     answer_buf,
                                    //     to_packet(ProtoMessage::BluetoothLeAdvertisementResponse(
                                    //         response_message,
                                    //     ))
                                    //     .unwrap(),
                                    // ]
                                    // .concat();
                                }
                                ProtoMessage::SubscribeStatesRequest(subscribe_states_request) => {
                                    debug!(
                                        "SubscribeStatesRequest: {:?}",
                                        subscribe_states_request
                                    );
                                }
                                ProtoMessage::SubscribeHomeassistantServicesRequest(request) => {
                                    debug!("SubscribeHomeassistantServicesRequest: {:?}", request);
                                }
                                ProtoMessage::SubscribeHomeAssistantStatesRequest(
                                    subscribe_homeassistant_services_request,
                                ) => {
                                    debug!(
                                        "SubscribeHomeAssistantStatesRequest: {:?}",
                                        subscribe_homeassistant_services_request
                                    );
                                    let response_message =
                                        proto::SubscribeHomeAssistantStateResponse {
                                            entity_id: "test".to_string(),
                                            attribute: "test".to_string(),
                                            once: true,
                                        };
                                }
                                ProtoMessage::ButtonCommandRequest(button_command_request) => {
                                    debug!("ButtonCommandRequest: {:?}", button_command_request);
                                    let button = api_components_clone
                                        .get(&button_command_request.key)
                                        .unwrap();
                                    match button {
                                        ProtoMessage::ListEntitiesButtonResponse(button) => {
                                            debug!("ButtonCommandRequest: {:?}", button);
                                            let msg = ChangedMessage::ButtonPress {
                                                key: button.unique_id.clone(),
                                            };

                                            cloned_sender.send(msg).unwrap();
                                        }
                                        _ => {}
                                    }
                                }
                                ProtoMessage::SwitchCommandRequest(switch_command_request) => {
                                    debug!("SwitchCommandRequest: {:?}", switch_command_request);
                                    let switch_entity = api_components_clone
                                        .get(&switch_command_request.key)
                                        .unwrap();
                                    match switch_entity {
                                        ProtoMessage::ListEntitiesSwitchResponse(switch_entity) => {
                                            debug!(
                                                "switch_entityCommandRequest: {:?}",
                                                switch_entity
                                            );
                                            let msg = ChangedMessage::SwitchStateCommand {
                                                key: switch_entity.unique_id.clone(),
                                                state: switch_command_request.state,
                                            };

                                            cloned_sender.send(msg).unwrap();
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {
                                    debug!("Ignore message type: {:?}", message);
                                    return;
                                }
                            }

                            cursor += 3 + len;
                        }
                    }
                });

                // Write Loop
                let mut answer_messages_rx_clone = answer_messages_rx.resubscribe();
                let mut messages_rx_clone = messages_rx.resubscribe();
                tokio::spawn(async move {
                    let mut disconnect = false;
                    loop {
                        let mut answer_buf: Vec<u8> = vec![];

                        let answer_messages = answer_messages_rx_clone.recv();
                        let normal_messages = messages_rx_clone.recv();
                        let answer_message: ProtoMessage;
                        // Wait for any new message
                        tokio::select! {
                            message = answer_messages => {
                                answer_message = message.unwrap();
                            }
                            message = normal_messages => {
                                answer_message = message.unwrap();
                            }
                        };

                        debug!("Answer message: {:?}", answer_message);
                        answer_buf =
                            [answer_buf, to_packet_from_ref(&answer_message).unwrap()].concat();
                        match answer_message {
                            ProtoMessage::DisconnectResponse(_) => {
                                disconnect = true;
                            }
                            _ => {}
                        }

                        loop {
                            // let message = messages_rx_clone.recv().await.unwrap();
                            let answer_message = answer_messages_rx_clone.try_recv();
                            match answer_message {
                                Ok(answer_message) => {
                                    debug!("Answer message: {:?}", answer_message);
                                    answer_buf =
                                        [answer_buf, to_packet_from_ref(&answer_message).unwrap()]
                                            .concat();

                                    match answer_message {
                                        ProtoMessage::DisconnectResponse(_) => {
                                            disconnect = true;
                                        }
                                        _ => {}
                                    }
                                }
                                Err(_) => break,
                            }
                        }

                        trace!("Send response: {:?}", answer_buf);
                        write
                            .write_all(&answer_buf)
                            .await
                            .expect("failed to write data to socket");

                        if disconnect {
                            // Close the socket
                            debug!("Disconnecting");
                            write.shutdown().await.expect("failed to shutdown socket");
                            break;
                        }
                    }
                });
            }
        })
    }
}
