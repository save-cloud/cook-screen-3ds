#![allow(non_snake_case)]

use std::sync::{Mutex, OnceLock};

use dioxus::prelude::*;

use crate::{
    constant::{SCREEN_BOTTOM_WIDTH, SCREEN_HEIGHT, SCREEN_TOP_WIDTH},
    utils::get_frame_time,
};

static APP_EXIT: OnceLock<Mutex<bool>> = OnceLock::new();

#[derive(Clone)]
pub struct AppExit;

impl AppExit {
    pub fn get() -> &'static Mutex<bool> {
        APP_EXIT.get_or_init(|| Mutex::new(false))
    }

    pub fn set_exit() {
        *Self::get().lock().unwrap() = true;
    }

    pub fn is_exit() -> bool {
        Self::get().lock().is_ok_and(|exit| *exit)
    }
}

// animation frame duration
#[derive(Clone, Copy, PartialEq)]
pub struct AppFrameDuration(pub u64);

pub fn Main() -> Element {
    // app frame
    use_context_provider(|| AppFrameDuration(get_frame_time()));

    rsx! {
        div {
            "scale": 0.4,
            display: "block",
            width: SCREEN_TOP_WIDTH,
            height: SCREEN_HEIGHT,
            color: "main-text",
            z_index: 0,
            onkeypress: move |e: KeyboardEvent| {
                if e.data().code() == Code::Enter {
                    AppExit::set_exit();
                }
            },

            div {
                "screen": "top",
                "bg_reset": "white",
                position: "absolute",
                left: 0,
                top: 0,
                width: SCREEN_TOP_WIDTH,
                height: SCREEN_HEIGHT,
                background_color: "white",
            }

            div {
                "screen": "bottom",
                "bg_reset": "white",
                position: "absolute",
                left: 0,
                top: 0,
                width: SCREEN_BOTTOM_WIDTH,
                height: SCREEN_HEIGHT,
                background_color: "white",
            }
        }
    }
}
