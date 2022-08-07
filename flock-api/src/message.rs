// use crate::SensorStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Payload {
    Connected,
    Disconnected,
    Instruction(Instruction),
    SensorReading(SensorData),
    SensorConfig(SensorConfig),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Instruction {
    ReadSensor,
    ReadSensorConfig,
    WriteSensorConfig(SensorConfig),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SensorData {
    Camera { frame_buffer: Vec<u8> },
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CameraSensorConfig {
    pub brightness: i8,
    pub contrast: i8,
    pub saturation: i8,
    pub sharpness: i8,
    pub de_noise: u8,
    pub special_effect: u8,
    pub wb_mode: u8,
    pub awb: bool,
    pub awb_gain: bool,
    pub gain_ceiling: u8,
    pub lens_correction: bool,
    pub horizontal_mirror: bool,
    pub vertical_flip: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum SensorConfig {
    Camera(CameraSensorConfig),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    // pub message_id: String,
    pub client_id: String,
    pub recipient: String,
    pub payload: Box<Payload>,
}

impl Message {
    pub fn new(client_id: String, recipient: String, payload: Payload) -> Self {
        Self {
            // message_id: uuid::Uuid::new_v4().to_string(),
            client_id,
            recipient,
            payload: Box::new(payload),
        }
    }

    pub fn new_err(client_id: String, recipient: String, message: String) -> Self {
        Self {
            client_id,
            recipient,
            payload: Box::new(Payload::Error(message)),
        }
    }

    pub fn new_connected(client_id: String, recipient: String) -> Self {
        Self::new(client_id, recipient, Payload::Connected)
    }
}
