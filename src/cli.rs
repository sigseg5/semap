use std::process::exit;

pub fn get_devices() -> Vec<(u16, u16)> {
    let mut devices = Vec::with_capacity(2);
    rusb::devices().unwrap().iter().for_each(|device| {
        devices.push((
            device.device_descriptor().unwrap().vendor_id(),
            device.device_descriptor().unwrap().product_id(),
        ));
    });
    devices
}

pub fn check_platform() {
    if cfg!(windows) {
        println!("Windows is not supported right now. And it won't be.");
        exit(1);
    } else if cfg!(target_os = "macos") {
        println!("macOS is not yet supported.");
        exit(1);
    }
}
