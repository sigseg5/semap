use std::thread::sleep;
use std::time::Duration;

fn main() {
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
