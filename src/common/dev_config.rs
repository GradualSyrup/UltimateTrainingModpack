use std::fs;

use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use toml;

use crate::common::input::*;
use crate::consts::DEV_TOML_PATH;
use crate::logging::info;

/// Hot-reloadable configs for quicker development
///
/// In game, press L+R+A at any point to reread these configs from
/// the file in DEV_TOML_PATH on the SD card
///
/// Example usage:
///
/// In this file:
/// ```rust
/// pub struct DevConfig {
///     pub quit_menu_title: String,
///     pub quit_menu_pos_y: i32,
/// }
/// ```
///
/// In another file such as `ui2d/menu.rs`:
/// ```rust
/// let dev_config = crate::dev_config::config();
/// quit_menu_button.pos_y = dev_config.quit_menu_pos_y;
/// quit_menu_text.as_textbox().set_text_string(&dev_config.quit_menu_title);
/// ```
#[derive(Deserialize, Default)]
pub struct DevConfig {}

pub unsafe fn config() -> &'static DevConfig {
    &*DEV_CONFIG.data_ptr()
}

lazy_static! {
    pub static ref DEV_CONFIG: Mutex<DevConfig> = Mutex::new(DevConfig::load_from_toml());
}

impl DevConfig {
    fn load_from_toml() -> DevConfig {
        let dev_path = DEV_TOML_PATH;
        if fs::metadata(dev_path).is_ok() {
            info!("Loading dev.toml configs...");
            let dev_config_str = fs::read_to_string(dev_path)
                .unwrap_or_else(|_| panic!("Could not read {}", dev_path));
            return toml::from_str(&dev_config_str).expect("Could not parse dev config");
        }

        DevConfig::default()
    }
}

pub fn handle_final_input_mapping(player_idx: i32, controller_struct: &SomeControllerStruct) {
    let current_buttons = controller_struct.controller.current_buttons;
    if player_idx == 0 && current_buttons.l() && current_buttons.r() && current_buttons.a() {
        let mut dev_config = DEV_CONFIG.lock();
        *dev_config = DevConfig::load_from_toml();
    }
}
