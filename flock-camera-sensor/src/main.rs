mod camera;

use anyhow::{bail, Context};
use camera::*;

use embedded_svc::ipv4;
use embedded_svc::mqtt::client::{utils, Message};
use embedded_svc::mqtt::client::{Client, Connection, Event, MessageImpl, Publish, QoS};
use embedded_svc::ping::Ping;
use embedded_svc::wifi::*;
use esp_idf_hal::mutex::Condvar;
use esp_idf_hal::prelude::*;
use esp_idf_svc::mqtt::client::{EspMqttClient, MqttClientConfiguration};
use esp_idf_svc::netif::EspNetifStack;
use esp_idf_svc::nvs::EspDefaultNvs;
use esp_idf_svc::ping::EspPing;
use esp_idf_svc::sysloop::EspSysLoopStack;
use esp_idf_svc::wifi::EspWifi;
use esp_idf_sys as _;
// If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::EspError;
use log::*;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;
use flock_api::CameraSensorConfig;

const SSID: &str = env!("FLOCK_WIFI_SSID");
const PASS: &str = env!("FLOCK_WIFI_PASS");
const MQTT_BROKER_ADDR: &str = env!("FLOCK_MQTT_BROKER_ADDR");
const CLIENT_ID: &str = env!("FLOCK_CLIENT_ID");
const CONTROLLER_TOPIC: &str = env!("FLOCK_CONTROLLER_TOPIC");

#[allow(unused)]
fn mqtt_client_id() -> String {
    format!("flock-client-{}", CLIENT_ID)
}


impl From<SensorStatus> for CameraSensorConfig {
    fn from(s: SensorStatus) -> Self {
        Self {
            brightness: s.brightness,
            contrast: s.contrast,
            saturation: s.saturation,
            sharpness: s.sharpness,
            de_noise: s.de_noise,
            special_effect: s.special_effect,
            wb_mode: s.wb_mode,
            awb: s.awb,
            awb_gain: s.awb_gain,
            gain_ceiling: s.gain_ceiling,
            lens_correction: s.lenc,
            horizontal_mirror: s.horizontal_mirror,
            vertical_flip: s.vertical_flip,
        }
    }
}

fn sensor_config(cam: &Camera) -> Result<(), EspError> {
    // BRIGHTNESS (-2 to 2)
    cam.sensor().set_brightness(2)?;
    // CONTRAST (-2 to 2)
    cam.sensor().set_contrast(0)?;
    // SATURATION (-2 to 2)
    cam.sensor().set_saturation(0)?;
    // SPECIAL EFFECTS (0 - No Effect, 1 - Negative, 2 - Grayscale, 3 - Red Tint, 4 - Green Tint, 5 - Blue Tint, 6 - Sepia)
    cam.sensor().set_special_effect(0)?;
    // WHITE BALANCE (false = Disable , true = Enable)
    cam.sensor().set_whitebal(true)?;
    // AWB GAIN (false = Disable , true = Enable)
    cam.sensor().set_awb_gain(true)?;
    // WB MODES (0 - Auto, 1 - Sunny, 2 - Cloudy, 3 - Office, 4 - Home)
    cam.sensor().set_wb_mode(0)?;
    // EXPOSURE CONTROLS (false = Disable , true = Enable)
    cam.sensor().set_exposure_ctrl(false)?;
    // AEC2 (false = Disable , true = Enable)
    cam.sensor().set_aec2(true)?;
    // AE LEVELS (-2 to 2)
    cam.sensor().set_ae_level(0)?;
    // AEC VALUES (0 to 1200)
    cam.sensor().set_aec_value(300)?;
    // GAIN CONTROLS (false = Disable , true = Enable)
    cam.sensor().set_gain_ctrl(true)?;
    // AGC GAIN (0 to 30)
    cam.sensor().set_agc_gain(0)?;
    // GAIN CEILING (0 to 6)
    cam.sensor().set_gain_ceiling(0)?;
    // BPC (false = Disable , true = Enable)
    cam.sensor().set_bpc(false)?;
    // WPC (false = Disable , true = Enable)
    cam.sensor().set_wpc(true)?;
    // RAW GMA (false = Disable , true = Enable)
    cam.sensor().set_raw_gma(true)?;
    // LENC (false = Disable , true = Enable)
    cam.sensor().set_lenc(true)?;
    // HORIZ MIRROR (false = Disable , true = Enable)
    cam.sensor().set_hmirror(false)?;
    // VERT FLIP (false = Disable , true = Enable)
    cam.sensor().set_vflip(false)?;
    // DCW (false = Disable , true = Enable)
    cam.sensor().set_dcw(true)?;
    // COLOR BAR PATTERN (false = Disable , true = Enable)
    cam.sensor().set_color_bar(false)?;
    Ok(())
}

// fn set_sensor_config(cam: &Camera, status: SensorStatus) -> Result<(), EspError> {
//     cam.sensor().set_frame_size(status.frame_size)?;
//     // cam.sensor().set_scale(status.scale)?;
//     // cam.sensor().set_binning(status.binning)?;
//     cam.sensor().set_quality(status.quality.into())?;
//     cam.sensor().set_brightness(status.brightness.into())?;
//     cam.sensor().set_contrast(status.contrast.into())?;
//     cam.sensor().set_saturation(status.saturation.into())?;
//     cam.sensor().set_sharpness(status.sharpness.into())?;
//     cam.sensor().set_denoise(status.de_noise.into())?;
//     cam.sensor()
//         .set_special_effect(status.special_effect.into())?;
//     cam.sensor().set_wb_mode(status.wb_mode.into())?;
//     // cam.sensor().set_awb(status.awb)?;
//     cam.sensor().set_awb_gain(status.awb_gain)?;
//     // cam.sensor().set_aec(status.aec)?;
//     cam.sensor().set_aec2(status.aec2)?;
//     cam.sensor().set_ae_level(status.ae_level.into())?;
//     cam.sensor().set_aec_value(status.aec_value.into())?;
//     // cam.sensor().set_agc(status.agc)?;
//     cam.sensor().set_agc_gain(status.agc_gain.into())?;
//     cam.sensor().set_gain_ceiling(status.gain_ceiling.into())?;
//     cam.sensor().set_bpc(status.bpc)?;
//     cam.sensor().set_wpc(status.wpc)?;
//     cam.sensor().set_raw_gma(status.raw_gma)?;
//     cam.sensor().set_lenc(status.lenc)?;
//     cam.sensor().set_hmirror(status.horizontal_mirror)?;
//     cam.sensor().set_vflip(status.vertical_flip)?;
//     cam.sensor().set_dcw(status.dcw)?;
//     cam.sensor().set_color_bar(status.color_bar)
// }


fn set_sensor_config(cam: &Camera, cfg: &CameraSensorConfig) -> Result<(), EspError> {
    let sensor = cam.sensor();
    sensor.set_brightness(cfg.brightness.into())?;
    sensor.set_contrast(cfg.contrast.into())?;
    sensor.set_saturation(cfg.saturation.into())?;
    sensor.set_sharpness(cfg.sharpness.into())?;
    sensor.set_denoise(cfg.de_noise.into())?;
    sensor.set_special_effect(cfg.special_effect.into())?;
    sensor.set_wb_mode(cfg.wb_mode.into())?;
    sensor.set_whitebal(cfg.awb)?;
    sensor.set_awb_gain(cfg.awb_gain)?;
    sensor.set_gain_ceiling(cfg.gain_ceiling.into())?;
    sensor.set_lenc(cfg.lens_correction)?;
    sensor.set_hmirror(cfg.horizontal_mirror)?;
    sensor.set_vflip(cfg.vertical_flip)?;
    Ok(())
}

fn ping(ip_settings: &ipv4::ClientSettings) -> anyhow::Result<()> {
    info!("About to do some pings for {:?}", ip_settings);

    let ping_summary = EspPing::default().ping(ip_settings.subnet.gateway, &Default::default())?;
    if ping_summary.transmitted != ping_summary.received {
        bail!(
            "Pinging gateway {} resulted in timeouts",
            ip_settings.subnet.gateway
        );
    }

    info!("Pinging done");

    Ok(())
}

fn wifi(
    netif_stack: Arc<EspNetifStack>,
    sys_loop_stack: Arc<EspSysLoopStack>,
    default_nvs: Arc<EspDefaultNvs>,
) -> anyhow::Result<Box<EspWifi>> {
    let mut wifi = Box::new(EspWifi::new(netif_stack, sys_loop_stack, default_nvs)?);

    info!("Wifi created, about to scan");

    let ap_infos = wifi.scan()?;

    let ours = ap_infos.into_iter().find(|a| a.ssid == SSID);

    let channel = if let Some(ours) = ours {
        info!(
            "Found configured access point {} on channel {}",
            SSID, ours.channel
        );
        Some(ours.channel)
    } else {
        info!(
            "Configured access point {} not found during scanning, will go with unknown channel",
            SSID
        );
        None
    };

    let config = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        password: PASS.into(),
        channel,
        ..Default::default()
    });
    wifi.set_configuration(&config)?;

    info!("Wifi configuration set, about to get status");

    wifi.wait_status_with_timeout(Duration::from_secs(20), |status| !status.is_transitional())
        .map_err(|e| anyhow::anyhow!("Unexpected Wifi status: {:?}", e))?;

    let status = wifi.get_status();

    if let Status(
        ClientStatus::Started(ClientConnectionStatus::Connected(ClientIpStatus::Done(ip_settings))),
        ApStatus::Stopped,
    ) = status
    {
        info!("Wifi connected");
        ping(&ip_settings)?;
    } else {
        bail!("Unexpected Wifi status: {:?}", status);
    }

    Ok(wifi)
}

fn handle_flock_message(cam: &Camera, msg: flock_api::Message) -> Option<flock_api::Payload> {
    if let flock_api::Payload::Instruction(instruction) = *msg.payload {
        return match instruction {
            flock_api::Instruction::ReadSensor => {
                cam.fb_get()
                    .map(|fb| flock_api::Payload::SensorReading(flock_api::SensorData::Camera {
                        frame_buffer: Vec::from(fb.data())
                    }))
            }
            flock_api::Instruction::ReadSensorConfig => {
                let status = cam.sensor().status();
                Some(flock_api::Payload::SensorConfig(flock_api::SensorConfig::Camera(status.into())))
            }
            flock_api::Instruction::WriteSensorConfig(cfg) => {
                let flock_api::SensorConfig::Camera(cam_cfg) = cfg;
                if let Err(err) = set_sensor_config(cam, &cam_cfg) {
                    Some(flock_api::Payload::Error(format!("{:?}", err)))
                } else {
                    let status = cam.sensor().status();
                    Some(flock_api::Payload::SensorConfig(flock_api::SensorConfig::Camera(status.into())))
                }
            }
        };
    }
    return None;
}

fn handle_mqtt_message(evt: Event<MessageImpl>, cam: &Camera) -> Option<flock_api::Message> {
    let payload = match evt {
        Event::Connected(_) => Some(flock_api::Payload::Connected),
        Event::Received(m) => {
            match serde_json::from_slice::<flock_api::Message>(m.data()) {
                Ok(msg) => handle_flock_message(cam, msg),
                Err(err) => {
                    let message = format!("{:?}", err);
                    Some(flock_api::Payload::Error(message))
                }
            }
        }
        _ => None,
    };
    payload.map(|p| flock_api::Message::new(
        mqtt_client_id(),
        CONTROLLER_TOPIC.into(),
        p,
    ))
}


fn spawn_mqtt_receiver(
    mut connection: utils::Connection<Condvar, MessageImpl, EspError>,
    tx: Sender<flock_api::Message>,
) -> thread::JoinHandle<()> {
    info!("Spawning MQTT watcher thread");
    thread::spawn(move || {
        info!("Initializing camera");
        let cam = Camera::init(CameraConfig::default())
            .with_context(|| "Error initializing camera")
            .unwrap();
        info!("Configuring camera sensor");
        sensor_config(&cam)
            .with_context(|| "Error configuring camera sensor")
            .unwrap();

        while let Some(msg) = connection.next() {
            match msg {
                Ok(evt) => {
                    info!("MQTT Message received: {:?}", evt);
                    if let Some(msg) = handle_mqtt_message(evt, &cam) {
                        tx.send(msg).unwrap();
                    }
                }
                Err(err) => {
                    error!("MQTT Error : {:?}", err);
                }
            }
        }
        panic!("Connection watcher closed unexpectedly")
    })
}

#[allow(unused)]
fn spawn_mqtt_publisher(
    mut client: EspMqttClient<utils::ConnState<MessageImpl, EspError>>,
    rx: Receiver<flock_api::Message>,
) -> thread::JoinHandle<()> {
    let client_id = mqtt_client_id();
    info!("Subscribing to topic {}", &client_id);
    client.subscribe(&client_id, QoS::AtMostOnce).unwrap();

    thread::spawn(move || {
        info!("Waiting for messages to publish");
        for msg in rx {
            info!("Sending message: ({:?})", &msg);
            let payload = serde_json::to_vec(&msg).unwrap();
            if let Err(err) = client.publish(
                msg.recipient.as_str(),
                QoS::AtMostOnce,
                false,
                payload.as_slice(),
            ) {
                error!("Error publishing MQTT message: (err={})", err);
            }
        }
    })
}

fn main() -> anyhow::Result<()> {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let netif_stack = Arc::new(EspNetifStack::new()?);
    let sysloop_stack = Arc::new(EspSysLoopStack::new()?);
    let default_nvs = Arc::new(EspDefaultNvs::new()?);
    let _peripherals = Peripherals::take().unwrap();
    let _wifi = wifi(
        netif_stack.clone(),
        sysloop_stack.clone(),
        default_nvs.clone(),
    )?;

    let client_id = mqtt_client_id();
    let mqtt_config = MqttClientConfiguration {
        client_id: Some(client_id.as_str()),
        crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),
        ..Default::default()
    };

    let (client, connection) = EspMqttClient::new_with_conn(MQTT_BROKER_ADDR, &mqtt_config)?;

    let (tx, rx) = mpsc::channel::<flock_api::Message>();

    let mut handles = vec![];

    info!("Spawning MQTT receiver thread");
    handles.push(spawn_mqtt_receiver(connection, tx));

    info!("Spawning MQTT publisher thread");
    handles.push(spawn_mqtt_publisher(client, rx));

    for h in handles.into_iter() {
        h.join().unwrap()
    }

    Ok(())
}
