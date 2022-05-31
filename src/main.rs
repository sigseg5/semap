use std::thread::sleep;
use std::time::Duration;

mod dconf;

// TODO: Parse kb_fingerprint from argv
// TODO: Read remap scheme from argv

fn main() {
    let xkb_opt = "/org/gnome/desktop/input-sources/xkb-options";
    let target_settings = "['caps:ctrl_modifier', 'compose:ralt', 'lv3:menu_switch']";
    let kb_fingerprint = (5050, 0024);

    dconf::set(xkb_opt, target_settings).unwrap();

    loop {
        for device in rusb::devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();
            if (device_desc.vendor_id(), device_desc.product_id()) == kb_fingerprint {
                println!("Model M detected")
            }
        }
        sleep(Duration::from_secs(5))
    }
}
