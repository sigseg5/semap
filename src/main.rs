use std::thread::sleep;
use std::time::Duration;

mod dconf;

// TODO: Parse kb_fingerprint from argv (may be compile time?)
// TODO: Read remap scheme from argv?
// TODO: Add systemd intergration?

fn main() {
    let xkb_opt = "/org/gnome/desktop/input-sources/xkb-options";
    let default_settings = "['caps:ctrl_modifier', 'compose:ralt', 'lv3:menu_switch']";
    let model_m_settings =
        "['compose:ralt', 'lv3:menu_switch', 'caps:ctrl_modifier', 'altwin:swap_alt_win']";
    let kb_fingerprint = (5050, 0024);

    loop {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if (device_desc.vendor_id(), device_desc.product_id()) == kb_fingerprint {
                if dconf::get(xkb_opt).unwrap() == model_m_settings {
                    break;
                };
                dconf::set(xkb_opt, model_m_settings).unwrap();
                break;
            }
            if dconf::get(xkb_opt).unwrap() == default_settings {
                break;
            };
            dconf::set(xkb_opt, default_settings).unwrap();
            break;
        }
        sleep(Duration::from_secs(5))
    }
}
