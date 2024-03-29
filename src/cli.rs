use std::process::exit;
use std::{thread, time};

#[doc = "This function returns the `vendor_id` and `product_id` of the each usb device."]
pub(crate) fn get_devices() -> Vec<(u16, u16)> {
    let mut devices = Vec::with_capacity(2);
    rusb::devices().unwrap().iter().for_each(|device| {
        devices.push((
            device.device_descriptor().unwrap().vendor_id(),
            device.device_descriptor().unwrap().product_id(),
        ));
    });
    devices
}

#[doc = "This function exit with `1` status code if OS is not supported. Only GNU/Linux systems are supported now."]
pub fn check_platform() {
    if !cfg!(target_os = "linux") {
        println!("Only GNU/Linux systems are supported now.");
        exit(1);
    }
}

#[doc = "This function helps to dynamically determine keyboard's fingerprint."]
pub fn find_device() {
    println!("Disconnect your Model M now (you have 5 sec)…");
    let timeout = time::Duration::from_secs(5);
    thread::sleep(timeout);
    let pre_connect_devices = get_devices();
    println!("Now connect your Model M (you have 5 sec)…");
    thread::sleep(timeout);
    let after_connect_devices = get_devices();
    let mut difference = vec![];
    after_connect_devices.iter().for_each(|i| {
        if !pre_connect_devices.contains(i) {
            difference.push(&*i);
        }
    });
    if difference.is_empty() {
        println!(
            "The devices are not connected. Make sure you connect your Model M following by tips."
        );
    } else {
        println!(
            "You fingerprint is {:#X?}\n
                Put this data to 30 line in main.rs, after that rebuild and reinstall service.\n
                Data format: (0xYYYY, 0xYYYY), replace all y's with fingerprint.",
            difference
        );
    }
}
