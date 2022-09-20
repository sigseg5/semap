use clap::{App, Arg};
use std::process::exit;
use std::thread;
use std::time::Duration;

mod cli;
mod dconf;

fn main() {
    cli::check_platform();

    let matches = App::new("semap")
        .version("0.2.0")
        .author("sigseg5")
        .about("Dynamic dconf layout switcher for IBM Model M keyboards.")
        .arg(
            Arg::new("find")
                .short('f')
                .long("find")
                .takes_value(false)
                .help("This option helps you determine your keyboard fingerprint.")
                .required(false),
        )
        .get_matches();

    if matches.is_present("find") {
        cli::find_device();
        exit(0);
    }

    // Pass your keyboard fingerprint here
    let kb_fingerprint: (u16, u16) = (0x13BA, 0x18);
    let default_settings = "['caps:ctrl_modifier', 'compose:ralt', 'lv3:menu_switch']";
    let xkb_opt = "/org/gnome/desktop/input-sources/xkb-options";
    let model_m_settings =
        "['compose:ralt', 'lv3:menu_switch', 'caps:ctrl_modifier', 'altwin:swap_alt_win']";

    loop {
        for i in cli::get_devices() {
            if i == kb_fingerprint {
                if dconf::get(xkb_opt).expect("Can't get xkb-options from dconf.")
                    == model_m_settings
                {
                    break;
                };
                dconf::set(xkb_opt, model_m_settings).expect("Can't set xkb-options for Model M.");
                break;
            } else {
                if dconf::get(xkb_opt).expect("Can't get xkb-options from dconf.")
                    == default_settings
                {
                    break;
                } else {
                    dconf::set(xkb_opt, default_settings)
                        .expect("Can't set xkb-options for standart keyboard.");
                    break;
                }
            }
        }
        thread::sleep(Duration::from_secs(5))
    }
}
