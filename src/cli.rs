use std::process::exit;

pub fn get_devices() -> Vec<(u16, u16)> {
    let mut devices = Vec::new();
    for device in rusb::devices().unwrap().iter() {
        let dev = device.device_descriptor().unwrap();
        devices.push((dev.vendor_id(), dev.product_id()));
    }
    devices
}

pub fn check_platform() {
    if cfg!(windows) {
        println!("Windows is not supported right now. And it won't be.");
        exit(1);
    } else if cfg!(macos) {
        println!("macOS is not yet supported.");
        exit(1);
    }
}
