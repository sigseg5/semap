use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// TODO: Parse kb_fingerprint from argv
// TODO: Read remap scheme from argv
// TODO: Place support funcs to other file

fn set(key: &str, value: &str) -> Result<(), String> {
    let mut cmd = Command::new("dconf");
    cmd.args(&["write", key, value]);
    match cmd.output() {
        Ok(_) => Ok(()),
        Err(_) => Err("Unable to set key".to_string()),
    }
}

fn main() {
    let xkb_opt = "/org/gnome/desktop/input-sources/xkb-options";
    let target_settings = "['caps:ctrl_modifier', 'compose:ralt', 'lv3:menu_switch']";
    
    set(xkb_opt, target_settings).unwrap();

    let kb_fingerprint = (5050, 0024);
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
