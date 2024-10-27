use std::error::Error;

use ctru::{
    applets::swkbd::{Button, ButtonConfig, CallbackResult, Kind, SoftwareKeyboard},
    services::{self, romfs::RomFS, soc::Soc},
};

use crate::{constant::INVALID_CHARS, resource::Resource};

extern "C" {
    /**
     * @brief Returns the Wifi signal strength.
     *
     * Valid values are 0-3:
     * - 0 means the singal strength is terrible or the 3DS is disconnected from
     *   all networks.
     * - 1 means the signal strength is bad.
     * - 2 means the signal strength is decent.
     * - 3 means the signal strength is good.
     *
     * Values outside the range of 0-3 should never be returned.
     *
     * These values correspond with the number of wifi bars displayed by Home Menu.
     *
     * @return the Wifi signal strength
     */
    // fn pl_get_wifi_strength() -> c_uchar;
    fn pl_is_n3ds() -> bool;
    // os function
    fn osSetSpeedupEnable(enable: bool);
}

pub fn setup_log_redirect(soc: &mut Soc) -> Option<()> {
    // if !cfg!(debug_assertions) {
    //     return None;
    // }
    // Set the output to be redirected to the `3dslink` server.
    soc.redirect_to_3dslink(true, true).ok()
}

pub fn setup_romfs() -> Result<RomFS, Box<dyn Error>> {
    services::romfs::RomFS::new().map_err(|e| e.into())
}

pub fn enable_hight_performance_for_new_3ds() {
    unsafe { osSetSpeedupEnable(true) }
}

// pub fn get_wifi_strength() -> u8 {
//     unsafe { pl_get_wifi_strength() }
// }

pub fn is_new_3ds() -> bool {
    unsafe { pl_is_n3ds() }
}

pub fn pl_show_swkbd(kind: Kind, resource: &Resource, initial_text: &str) -> Option<String> {
    // Prepares a software keyboard with two buttons: one to cancel input and one
    // to accept it. You can also use `SoftwareKeyboard::new()` to launch the keyboard
    // with different configurations.
    let mut keyboard = SoftwareKeyboard::new(kind, ButtonConfig::LeftRight);

    // Custom filter callback to handle the given input.
    // Using this callback it's possible to integrate the applet
    // with custom error messages when the input is incorrect.
    keyboard.set_filter_callback(Some(Box::new(move |str| {
        for c in INVALID_CHARS.iter() {
            if str.contains(*c) {
                return (
                    CallbackResult::Retry,
                    Some(r#"不能包含此类字符: \ /:*?"'<>|"#.into()),
                );
            }
        }

        (CallbackResult::Ok, None)
    })));

    keyboard.set_initial_text(Some(initial_text));

    // Launch the software keyboard. You can perform different actions depending on which
    // software button the user pressed.
    match keyboard.launch(&resource.apt, &resource.c2d.gfx) {
        Ok((text, Button::Right)) => {
            if text.is_empty() {
                None
            } else {
                Some(text)
            }
        }
        Ok((_, Button::Left)) => None,
        Ok((_, Button::Middle)) => None,
        Err(_) => None,
    }
}
