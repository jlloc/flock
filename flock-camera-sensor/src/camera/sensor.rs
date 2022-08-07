#![allow(unused)]

use super::{FrameBuffer, FrameSize, PixelFormat};
use esp_idf_sys::{esp, EspError};
use log::*;
use serde::{Deserialize, Serialize};

pub struct SensorId {
    pub midh: u8,
    pub midl: u8,
    pub pid: u16,
    pub ver: u8,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensorStatus {
    pub frame_size: FrameSize,
    pub scale: bool,
    pub binning: bool,
    pub quality: u8,
    pub brightness: i8,
    pub contrast: i8,
    pub saturation: i8,
    pub sharpness: i8,
    pub de_noise: u8,
    pub special_effect: u8,
    pub wb_mode: u8,
    pub awb: bool,
    pub awb_gain: bool,
    pub aec: bool,
    pub aec2: bool,
    pub ae_level: i8,
    pub aec_value: u16,
    pub agc: bool,
    pub agc_gain: u8,
    pub gain_ceiling: u8,
    pub bpc: bool,
    pub wpc: bool,
    pub raw_gma: bool,
    pub lenc: bool,
    pub horizontal_mirror: bool,
    pub vertical_flip: bool,
    pub dcw: bool,
    pub color_bar: bool,
}

pub struct SensorHandle {
    s: *mut esp_idf_sys::camera::sensor_t,
}

impl SensorHandle {
    pub fn get() -> Result<Self, EspError> {
        let s = unsafe { esp_idf_sys::camera::esp_camera_sensor_get() };
        if s.is_null() {
            return Err(EspError::from(esp_idf_sys::camera::ESP_ERR_CAMERA_NOT_DETECTED).unwrap());
        }
        Ok(Self { s })
    }

    pub fn id(&self) -> SensorId {
        SensorId {
            midh: unsafe { (*self.s).id.MIDH },
            midl: unsafe { (*self.s).id.MIDL },
            pid: unsafe { (*self.s).id.PID },
            ver: unsafe { (*self.s).id.VER },
        }
    }

    pub fn slv_addr(&self) -> u8 {
        unsafe { (*self.s).slv_addr }
    }

    pub fn pix_format(&self) -> PixelFormat {
        unsafe { (*self.s).pixformat.into() }
    }

    pub fn status(&self) -> SensorStatus {
        SensorStatus {
            frame_size: unsafe { (*self.s).status.framesize.into() },
            scale: unsafe { (*self.s).status.scale },
            binning: unsafe { (*self.s).status.binning },
            quality: unsafe { (*self.s).status.quality },
            brightness: unsafe { (*self.s).status.brightness },
            contrast: unsafe { (*self.s).status.contrast },
            saturation: unsafe { (*self.s).status.saturation },
            sharpness: unsafe { (*self.s).status.sharpness },
            de_noise: unsafe { (*self.s).status.denoise },
            special_effect: unsafe { (*self.s).status.special_effect },
            wb_mode: unsafe { (*self.s).status.wb_mode },
            awb: unsafe { (*self.s).status.awb != 0 },
            awb_gain: unsafe { (*self.s).status.awb_gain != 0 },
            aec: unsafe { (*self.s).status.aec != 0 },
            aec2: unsafe { (*self.s).status.aec2 != 0 },
            ae_level: unsafe { (*self.s).status.ae_level },
            aec_value: unsafe { (*self.s).status.aec_value },
            agc: unsafe { (*self.s).status.agc != 0 },
            agc_gain: unsafe { (*self.s).status.agc_gain },
            gain_ceiling: unsafe { (*self.s).status.gainceiling },
            bpc: unsafe { (*self.s).status.bpc != 0 },
            wpc: unsafe { (*self.s).status.wpc != 0 },
            raw_gma: unsafe { (*self.s).status.raw_gma != 0 },
            lenc: unsafe { (*self.s).status.lenc != 0 },
            horizontal_mirror: unsafe { (*self.s).status.hmirror != 0 },
            vertical_flip: unsafe { (*self.s).status.vflip != 0 },
            dcw: unsafe { (*self.s).status.dcw != 0 },
            color_bar: unsafe { (*self.s).status.colorbar != 0 },
        }
    }

    pub fn xclk_freq_hz(&self) -> i32 {
        unsafe { (*self.s).xclk_freq_hz }
    }

    pub fn init_status(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.s).init_status.unwrap()(self.s) })
    }

    pub fn reset(&self) -> Result<(), EspError> {
        esp!(unsafe { (*self.s).reset.unwrap()(self.s) })
    }

    pub fn set_pix_format(&self, pix_format: PixelFormat) -> Result<(), EspError> {
        info!("setting sensor pixel format: {:?}", pix_format);
        esp!(unsafe { (*self.s).set_pixformat.unwrap()(self.s, pix_format.into()) })
    }

    pub fn set_frame_size(&self, frame_size: FrameSize) -> Result<(), EspError> {
        info!("setting sensor frame size: {:?}", frame_size);
        esp!(unsafe { (*self.s).set_framesize.unwrap()(self.s, frame_size.into()) })
    }

    pub fn set_contrast(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor contrast: {}", level);
        esp!(unsafe { (*self.s).set_contrast.unwrap()(self.s, level) })
    }

    pub fn set_brightness(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor brightness: {}", level);
        esp!(unsafe { (*self.s).set_brightness.unwrap()(self.s, level) })
    }

    pub fn set_saturation(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor saturation: {}", level);
        esp!(unsafe { (*self.s).set_saturation.unwrap()(self.s, level) })
    }

    pub fn set_sharpness(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor sharpness: {}", level);
        esp!(unsafe { (*self.s).set_sharpness.unwrap()(self.s, level) })
    }

    pub fn set_denoise(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor denoise: {}", level);
        esp!(unsafe { (*self.s).set_denoise.unwrap()(self.s, level) })
    }

    pub fn set_gain_ceiling(&self, gain_ceiling: u32) -> Result<(), EspError> {
        info!("setting sensor gain ceiling: {}", gain_ceiling);
        esp!(unsafe { (*self.s).set_gainceiling.unwrap()(self.s, gain_ceiling) })
    }

    pub fn set_quality(&self, quality: i32) -> Result<(), EspError> {
        info!("setting sensor quality: {}", quality);
        esp!(unsafe { (*self.s).set_quality.unwrap()(self.s, quality) })
    }

    pub fn set_color_bar(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor color bar: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_colorbar.unwrap()(self.s, v) })
    }

    pub fn set_whitebal(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor white-balance: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_whitebal.unwrap()(self.s, v) })
    }

    pub fn set_gain_ctrl(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor gain control: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_gain_ctrl.unwrap()(self.s, v) })
    }

    pub fn set_exposure_ctrl(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor exposure control: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_exposure_ctrl.unwrap()(self.s, v) })
    }

    pub fn set_hmirror(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor hmirror: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_hmirror.unwrap()(self.s, v) })
    }

    pub fn set_vflip(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor vflip: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_vflip.unwrap()(self.s, v) })
    }

    pub fn set_aec2(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor aec2: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_aec2.unwrap()(self.s, v) })
    }

    pub fn set_awb_gain(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor awb gain: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_awb_gain.unwrap()(self.s, v) })
    }

    pub fn set_agc_gain(&self, gain: i32) -> Result<(), EspError> {
        info!("setting sensor agc gain: {}", gain);
        esp!(unsafe { (*self.s).set_agc_gain.unwrap()(self.s, gain) })
    }

    pub fn set_aec_value(&self, gain: i32) -> Result<(), EspError> {
        info!("setting sensor aec value: {}", gain);
        esp!(unsafe { (*self.s).set_aec_value.unwrap()(self.s, gain) })
    }

    pub fn set_special_effect(&self, effect: i32) -> Result<(), EspError> {
        info!("setting sensor special effect: {}", effect);
        esp!(unsafe { (*self.s).set_special_effect.unwrap()(self.s, effect) })
    }

    pub fn set_wb_mode(&self, mode: i32) -> Result<(), EspError> {
        info!("setting sensor white-balance mode: {}", mode);
        esp!(unsafe { (*self.s).set_wb_mode.unwrap()(self.s, mode) })
    }

    pub fn set_ae_level(&self, level: i32) -> Result<(), EspError> {
        info!("setting sensor ae level: {}", level);
        esp!(unsafe { (*self.s).set_ae_level.unwrap()(self.s, level) })
    }

    pub fn set_dcw(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor dcw: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_dcw.unwrap()(self.s, v) })
    }

    pub fn set_bpc(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor bpc: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_bpc.unwrap()(self.s, v) })
    }

    pub fn set_wpc(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor wpc: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_wpc.unwrap()(self.s, v) })
    }

    pub fn set_raw_gma(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor raw gma: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_raw_gma.unwrap()(self.s, v) })
    }

    pub fn set_lenc(&self, enable: bool) -> Result<(), EspError> {
        info!("setting sensor lenc: {}", enable);
        let v = if enable { 1 } else { 0 };
        esp!(unsafe { (*self.s).set_lenc.unwrap()(self.s, v) })
    }

    pub fn get_reg(&self, reg: i32, mask: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.s).get_reg.unwrap()(self.s, reg, mask) })
    }

    pub fn set_reg(&self, reg: i32, mask: i32, value: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.s).set_reg.unwrap()(self.s, reg, mask, value) })
    }

    pub fn set_res_raw(
        &self,
        start_x: i32,
        start_y: i32,
        end_x: i32,
        end_y: i32,
        offset_x: i32,
        offset_y: i32,
        total_x: i32,
        total_y: i32,
        output_x: i32,
        output_y: i32,
        scale: bool,
        binning: bool,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.s).set_res_raw.unwrap()(
                self.s, start_x, start_y, end_x, end_y, offset_x, offset_y, total_x, total_y,
                output_x, output_y, scale, binning,
            )
        })
    }

    pub fn set_pll(
        &self,
        bypass: i32,
        mul: i32,
        sys: i32,
        root: i32,
        pre: i32,
        seld5: i32,
        pclken: i32,
        pclk: i32,
    ) -> Result<(), EspError> {
        esp!(unsafe {
            (*self.s).set_pll.unwrap()(self.s, bypass, mul, sys, root, pre, seld5, pclken, pclk)
        })
    }

    pub fn set_xclk(&self, timer: i32, xclk: i32) -> Result<(), EspError> {
        esp!(unsafe { (*self.s).set_xclk.unwrap()(self.s, timer, xclk) })
    }
}
