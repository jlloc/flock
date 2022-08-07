#![allow(unused)]

use super::SensorHandle;
use esp_idf_hal::gpio;
use esp_idf_hal::gpio::Pin;
use esp_idf_sys::{esp, EspError};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub enum PixelFormat {
    RGB565,    // 2BPP/RGB565
    YUV422,    // 2BPP/YUV422
    YUV420,    // 1.5BPP/YUV420
    GRAYSCALE, // 1BPP/GRAYSCALE
    JPEG,      // JPEG/COMPRESSED
    RGB888,    // 3BPP/RGB888
    RAW,       // RAW
    RGB444,    // 3BP2P/RGB444
    RGB555,    // 3BP2P/RGB555
}

impl From<PixelFormat> for esp_idf_sys::camera::pixformat_t {
    fn from(pix_format: PixelFormat) -> Self {
        match pix_format {
            PixelFormat::RGB565 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB565,
            PixelFormat::YUV422 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_YUV422,
            PixelFormat::YUV420 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_YUV420,
            PixelFormat::GRAYSCALE => esp_idf_sys::camera::pixformat_t_PIXFORMAT_GRAYSCALE,
            PixelFormat::JPEG => esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG,
            PixelFormat::RGB888 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB888,
            PixelFormat::RAW => esp_idf_sys::camera::pixformat_t_PIXFORMAT_RAW,
            PixelFormat::RGB444 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB444,
            PixelFormat::RGB555 => esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB555,
        }
    }
}

impl From<esp_idf_sys::camera::pixformat_t> for PixelFormat {
    fn from(v: esp_idf_sys::camera::pixformat_t) -> Self {
        match v {
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB565 => Self::RGB565,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_YUV422 => Self::YUV422,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_YUV420 => Self::YUV420,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_GRAYSCALE => Self::GRAYSCALE,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_JPEG => Self::JPEG,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB888 => Self::RGB888,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_RAW => Self::RAW,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB444 => Self::RGB444,
            esp_idf_sys::camera::pixformat_t_PIXFORMAT_RGB555 => Self::RGB555,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FrameSize {
    FrameSize96X96,   // 96x96
    FrameSizeQQVGA,   // 160x120
    FrameSizeQCIF,    // 176x144
    FrameSizeHQVGA,   // 240x176
    FrameSize240X240, // 240x240
    FrameSizeQVGA,    // 320x240
    FrameSizeCIF,     // 400x296
    FrameSizeHVGA,    // 480x320
    FrameSizeVGA,     // 640x480
    FrameSizeSVGA,    // 800x600
    FrameSizeXGA,     // 1024x768
    FrameSizeHD,      // 1280x720
    FrameSizeSXGA,    // 1280x1024
    FrameSizeUXGA,    // 1600x1200
    // 3MP Sensors
    FrameSizeFHD,  // 1920x1080
    FrameSizePHD,  //  720x1280
    FrameSizeP3MP, //  864x1536
    FrameSizeQXGA, // 2048x1536
    // 5MP Sensors
    FrameSizeQHD,   // 2560x1440
    FrameSizeWQXGA, // 2560x1600
    FrameSizePFHD,  // 1080x1920
    FrameSizeQSXGA, // 2560x1920
    FrameSizeINVALID,
}

impl From<FrameSize> for esp_idf_sys::camera::framesize_t {
    fn from(frame_size: FrameSize) -> Self {
        match frame_size {
            FrameSize::FrameSize96X96 => esp_idf_sys::camera::framesize_t_FRAMESIZE_96X96,
            FrameSize::FrameSizeQQVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_QQVGA,
            FrameSize::FrameSizeQCIF => esp_idf_sys::camera::framesize_t_FRAMESIZE_QCIF,
            FrameSize::FrameSizeHQVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_HQVGA,
            FrameSize::FrameSize240X240 => esp_idf_sys::camera::framesize_t_FRAMESIZE_240X240,
            FrameSize::FrameSizeQVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_QVGA,
            FrameSize::FrameSizeCIF => esp_idf_sys::camera::framesize_t_FRAMESIZE_CIF,
            FrameSize::FrameSizeHVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_HVGA,
            FrameSize::FrameSizeVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_VGA,
            FrameSize::FrameSizeSVGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_SVGA,
            FrameSize::FrameSizeXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_XGA,
            FrameSize::FrameSizeHD => esp_idf_sys::camera::framesize_t_FRAMESIZE_HD,
            FrameSize::FrameSizeSXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_SXGA,
            FrameSize::FrameSizeUXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA,
            FrameSize::FrameSizeFHD => esp_idf_sys::camera::framesize_t_FRAMESIZE_FHD,
            FrameSize::FrameSizePHD => esp_idf_sys::camera::framesize_t_FRAMESIZE_P_HD,
            FrameSize::FrameSizeP3MP => esp_idf_sys::camera::framesize_t_FRAMESIZE_P_3MP,
            FrameSize::FrameSizeQXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_QXGA,
            FrameSize::FrameSizeQHD => esp_idf_sys::camera::framesize_t_FRAMESIZE_QHD,
            FrameSize::FrameSizeWQXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_WQXGA,
            FrameSize::FrameSizePFHD => esp_idf_sys::camera::framesize_t_FRAMESIZE_P_FHD,
            FrameSize::FrameSizeQSXGA => esp_idf_sys::camera::framesize_t_FRAMESIZE_QSXGA,
            FrameSize::FrameSizeINVALID => esp_idf_sys::camera::framesize_t_FRAMESIZE_INVALID,
        }
    }
}

impl From<esp_idf_sys::camera::framesize_t> for FrameSize {
    fn from(frame_size: esp_idf_sys::camera::framesize_t) -> Self {
        match frame_size {
            esp_idf_sys::camera::framesize_t_FRAMESIZE_96X96 => FrameSize::FrameSize96X96,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QQVGA => FrameSize::FrameSizeQQVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QCIF => FrameSize::FrameSizeQCIF,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_HQVGA => FrameSize::FrameSizeHQVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_240X240 => FrameSize::FrameSize240X240,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QVGA => FrameSize::FrameSizeQVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_CIF => FrameSize::FrameSizeCIF,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_HVGA => FrameSize::FrameSizeHVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_VGA => FrameSize::FrameSizeVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_SVGA => FrameSize::FrameSizeSVGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_XGA => FrameSize::FrameSizeXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_HD => FrameSize::FrameSizeHD,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_SXGA => FrameSize::FrameSizeSXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_UXGA => FrameSize::FrameSizeUXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_FHD => FrameSize::FrameSizeFHD,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_P_HD => FrameSize::FrameSizePHD,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_P_3MP => FrameSize::FrameSizeP3MP,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QXGA => FrameSize::FrameSizeQXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QHD => FrameSize::FrameSizeQHD,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_WQXGA => FrameSize::FrameSizeWQXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_P_FHD => FrameSize::FrameSizePFHD,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_QSXGA => FrameSize::FrameSizeQSXGA,
            esp_idf_sys::camera::framesize_t_FRAMESIZE_INVALID => FrameSize::FrameSizeINVALID,
            _ => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum CameraFbLocation {
    PSRAM,
    DRAM,
}

impl From<CameraFbLocation> for esp_idf_sys::camera::camera_fb_location_t {
    fn from(fb_location: CameraFbLocation) -> Self {
        match fb_location {
            CameraFbLocation::PSRAM => esp_idf_sys::camera::camera_fb_location_t_CAMERA_FB_IN_PSRAM,
            CameraFbLocation::DRAM => esp_idf_sys::camera::camera_fb_location_t_CAMERA_FB_IN_DRAM,
        }
    }
}

#[derive(Clone, Debug)]
pub enum CameraGrabMode {
    WhenEmpty, // Fills buffers when they are empty. Less resources but first 'fb_count' frames might be old
    Latest, // Except when 1 frame buffer is used, queue will always contain the last 'fb_count' frames
}

impl From<CameraGrabMode> for esp_idf_sys::camera::camera_grab_mode_t {
    fn from(grab_mode: CameraGrabMode) -> Self {
        match grab_mode {
            CameraGrabMode::WhenEmpty => {
                esp_idf_sys::camera::camera_grab_mode_t_CAMERA_GRAB_WHEN_EMPTY
            }
            CameraGrabMode::Latest => esp_idf_sys::camera::camera_grab_mode_t_CAMERA_GRAB_LATEST,
        }
    }
}

pub struct CameraConfig {
    pub pin_pwdn: i32,
    pub pin_reset: i32,
    pub pin_xclk: i32,
    pub pin_sda: i32,
    pub pin_scl: i32,
    pub pin_d7: i32,
    pub pin_d6: i32,
    pub pin_d5: i32,
    pub pin_d4: i32,
    pub pin_d3: i32,
    pub pin_d2: i32,
    pub pin_d1: i32,
    pub pin_d0: i32,
    pub pin_vsync: i32,
    pub pin_href: i32,
    pub pin_pclk: i32,
    pub xclk_freq_hz: i32,
    pub ledc_timer: u32,
    pub ledc_channel: u32,
    pub pixel_format: PixelFormat,
    pub frame_size: FrameSize,
    pub jpeg_quality: i32,
    pub fb_count: usize,
    pub fb_location: CameraFbLocation,
    pub grab_mode: CameraGrabMode,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            pin_pwdn: 32,
            pin_reset: -1,
            pin_xclk: 0,
            pin_sda: 26,
            pin_scl: 27,
            pin_d7: 35,
            pin_d6: 34,
            pin_d5: 39,
            pin_d4: 36,
            pin_d3: 21,
            pin_d2: 19,
            pin_d1: 18,
            pin_d0: 5,
            pin_vsync: 25,
            pin_href: 23,
            pin_pclk: 22,
            xclk_freq_hz: 20000000,
            ledc_timer: 0,
            ledc_channel: 0,
            pixel_format: PixelFormat::RGB565,
            frame_size: FrameSize::FrameSizeQVGA,
            jpeg_quality: 12,
            fb_count: 1,
            fb_location: CameraFbLocation::PSRAM,
            grab_mode: CameraGrabMode::WhenEmpty,
        }
    }
}

impl From<CameraConfig> for esp_idf_sys::camera::camera_config_t {
    fn from(config: CameraConfig) -> Self {
        Self {
            pin_pwdn: config.pin_pwdn,
            pin_reset: config.pin_reset, //.map_or(-1, |p| p.pin()),
            pin_xclk: config.pin_xclk,
            pin_sscb_sda: config.pin_sda,
            pin_sscb_scl: config.pin_scl,
            pin_d7: config.pin_d7,
            pin_d6: config.pin_d6,
            pin_d5: config.pin_d5,
            pin_d4: config.pin_d4,
            pin_d3: config.pin_d3,
            pin_d2: config.pin_d2,
            pin_d1: config.pin_d1,
            pin_d0: config.pin_d0,
            pin_vsync: config.pin_vsync,
            pin_href: config.pin_href,
            pin_pclk: config.pin_pclk,
            xclk_freq_hz: config.xclk_freq_hz.into(),
            ledc_timer: config.ledc_timer.into(),
            ledc_channel: config.ledc_channel.into(),
            pixel_format: config.pixel_format.into(),
            frame_size: config.frame_size.into(),
            jpeg_quality: config.jpeg_quality.into(),
            fb_count: config.fb_count.try_into().unwrap(),
            fb_location: config.fb_location.into(),
            grab_mode: config.grab_mode.into(),
        }
    }
}

pub struct Camera {
    // pub config: CameraConfig,
    initialized: bool,
    sensor: SensorHandle,
}

impl Camera {
    pub fn init(config: CameraConfig) -> Result<Self, EspError> {
        let cfg: esp_idf_sys::camera::camera_config_t = config.into();
        esp!(unsafe { esp_idf_sys::camera::esp_camera_init(&cfg) })?;
        Ok(Self {
            // config,
            initialized: true,
            sensor: SensorHandle::get()?,
        })
    }

    fn de_init(&self) -> Result<(), EspError> {
        if self.initialized {
            return esp!(unsafe { esp_idf_sys::camera::esp_camera_deinit() });
        }
        Ok(())
    }

    pub fn fb_get<'fb>(&self) -> Option<FrameBuffer<'fb>> {
        unsafe {
            let fb = esp_idf_sys::camera::esp_camera_fb_get();
            if fb.is_null() {
                return None;
            }
            Some(fb.into())
        }
    }

    pub fn sensor(&self) -> &SensorHandle {
        &self.sensor
    }
}

impl Drop for Camera {
    fn drop(&mut self) {
        self.de_init().expect("error de-initializing camera driver");
    }
}

pub struct FrameBuffer<'fb> {
    fb: *mut esp_idf_sys::camera::camera_fb_t,
    _phantom: PhantomData<&'fb esp_idf_sys::camera::camera_fb_t>,
}

impl<'fb> FrameBuffer<'fb> {
    pub fn data(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts::<'fb>((*self.fb).buf, (*self.fb).len.try_into().unwrap())
        }
    }

    pub fn len(&self) -> u32 {
        unsafe { (*self.fb).len }
    }

    pub fn width(&self) -> u32 {
        unsafe { (*self.fb).width }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.fb).height }
    }

    pub fn format(&self) -> PixelFormat {
        unsafe { (*self.fb).format.into() }
    }
}

impl<'fb> From<*mut esp_idf_sys::camera::camera_fb_t> for FrameBuffer<'fb> {
    fn from(fb: *mut esp_idf_sys::camera::camera_fb_t) -> Self {
        Self {
            fb,
            _phantom: PhantomData,
        }
    }
}

impl<'fb> Drop for FrameBuffer<'fb> {
    fn drop(&mut self) {
        unsafe { esp_idf_sys::camera::esp_camera_fb_return(self.fb) }
    }
}
