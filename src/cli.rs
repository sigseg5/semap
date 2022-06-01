pub fn help() {
    println!("Usage…")
}

pub fn get_devices() -> Vec<(u16, u16)> {
    let mut devices = Vec::new();
    for device in rusb::devices().unwrap().iter() {
        let dev = device.device_descriptor().unwrap();
        devices.push((dev.vendor_id(), dev.product_id()));
    }
    devices
}
