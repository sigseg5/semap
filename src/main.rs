use std::thread::sleep;
use std::time::Duration;

mod cli;
mod dconf;

fn main() {
    cli::check_platform();

    let kb_fingerprint: (u16, u16) = (5050, 0024);
    let default_settings = "['caps:ctrl_modifier', 'compose:ralt', 'lv3:menu_switch']";
    let xkb_opt = "/org/gnome/desktop/input-sources/xkb-options";
    let model_m_settings =
        "['compose:ralt', 'lv3:menu_switch', 'caps:ctrl_modifier', 'altwin:swap_alt_win']";

    loop {
        for i in cli::get_devices() {
            if i == kb_fingerprint {
                if dconf::get(xkb_opt).unwrap() == model_m_settings {
                    break;
                };
                dconf::set(xkb_opt, model_m_settings).unwrap();
                break;
            } else {
                if dconf::get(xkb_opt).unwrap() == default_settings {
                    break;
                } else {
                    dconf::set(xkb_opt, default_settings).unwrap();
                    break;
                }
            }
        }
        sleep(Duration::from_secs(5))
    }
}
