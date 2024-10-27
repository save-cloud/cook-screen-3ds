use core::str;
use std::{
    ffi::{c_char, CStr},
    fmt::{Display, Formatter},
    ops::Deref,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{c2d::rgba, platform::is_new_3ds};

extern "C" {
    fn get_format_time() -> *mut c_char;
    fn free_c_str(data: *mut c_char);
}

pub struct CInputStr(*mut c_char);

impl Deref for CInputStr {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        unsafe {
            if self.0.is_null() {
                return "";
            }
            let c_str = CStr::from_ptr(self.0);
            c_str.to_str().unwrap()
        }
    }
}

impl Display for CInputStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl Drop for CInputStr {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { free_c_str(self.0) }
        }
    }
}

pub fn get_current_format_time() -> CInputStr {
    unsafe { CInputStr(get_format_time()) }
}

pub fn current_time() -> u128 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis()
}

pub fn str_to_c_null_term_bytes(data: &str) -> Vec<u8> {
    format!("{}\0", data).into_bytes()
}

pub fn ease_out_expo(elapsed: Duration, duration: Duration, start: f64, end: f64) -> f64 {
    if elapsed >= duration {
        return end;
    }
    start
        + (end - start)
            * (1.0 - 2.0_f64.powf(-10.0 * elapsed.as_millis() as f64 / duration.as_millis() as f64))
}

pub fn color_name_rgba(color: &str) -> u32 {
    match color {
        // if there is a color tag, translate it
        "red" => rgba(0xff, 0x00, 0x00, 0xff),
        "green" => rgba(0x00, 0xff, 0x00, 0xff),
        "blue" => rgba(0x00, 0x00, 0xff, 0xff),
        "dir" => rgba(0x00, 0xb4, 0xd8, 255),
        "white" => rgba(0xff, 0xff, 0xff, 0xff),
        "gray" => rgba(0xbb, 0xbb, 0xbb, 0xff),
        "black" => rgba(0x00, 0x00, 0x00, 0xff),
        "main-text" => rgba(0xee, 0xee, 0xee, 0xff),
        "main_bg" => rgba(0x22, 0x22, 0x22, 0xff),
        "selected_bg" => rgba(0x44, 0x44, 0x44, 0xff),
        "selected_bg_info" => rgba(0x33, 0x33, 0x33, 0xff),
        "selected_bg_dark" => rgba(0x28, 0x28, 0x28, 0xff),
        "selected_bg_light" => rgba(0x60, 0x60, 0x60, 0xff),
        "transparent" => rgba(0x0, 0x0, 0x0, 0x0),
        "tips" => rgba(0xaa, 0xaa, 0xaa, 0xff),
        "panel_bg" => rgba(0x26, 0x26, 0x26, 0xff),
        _ => rgba(0x00, 0x00, 0x00, 0xff),
    }
}

pub async fn sleep_micros(micros: u64) {
    tokio::time::sleep(Duration::from_micros(micros)).await;
}

pub async fn sleep_micros_for_ever(micros: u64) {
    loop {
        tokio::time::sleep(Duration::from_micros(micros)).await;
    }
}

pub fn get_frame_time() -> u64 {
    if is_new_3ds() {
        16666
    } else {
        33333
    }
}
