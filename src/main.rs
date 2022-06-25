use std::env;
use std::process::exit;
use std::time::Duration;
use std::{thread, time};

mod cli;
mod dconf;

fn main() {
    cli::check_platform();
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => (),
        _ => {
            if args[1] != "--find-dev" {
                println!(
                    "Can't recognize command, use --find-dev for detect your keyboard fingerprint."
                );
                exit(1)
            }
            println!("Disconnect your Model M now (you have 5 sec)…");
            let five_secs = time::Duration::from_secs(5);
            thread::sleep(five_secs);
            let pre_connect_devices = cli::get_devices();
            println!("Now connect your Model M (you have 5 sec)…");
            thread::sleep(five_secs);
            let after_connect_devices = cli::get_devices();
            let mut difference = vec![];
            after_connect_devices.iter().for_each(|i| {
                if !pre_connect_devices.contains(i) {
                    difference.push(&*i);
                }
            });
            println!(
                "You fingerprint is {:#X?}\n
                Put this data to 50 line in main.rs, after that rebuild and reinstall service.\n
                Data format: (0xYYYY, 0xYYYY), replace all y's with fingerprint.",
                difference
            );
            exit(0);
        }
    }

    let kb_fingerprint: (u16, u16) = (0x13BA, 0x18);
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
        thread::sleep(Duration::from_secs(5))
    }
}
