# semap

This is a dynamic dconf layout switcher for my IBM Model M. Tested at Fedora 36.

# Dependencies

* systemd;
* GNOME DE;
* cargo.

# Get your fingerprint data

* Run app with `cargo run -- --find-dev` command;
* You'l get something like that:
```
You fingerprint is [
    (
        0x13BA,
        0x18,
    ),
]
```

* Put this values to 50 line in main.rs, after that rebuild and reinstall service (from Installation section).

# Installation

* Run `bash build.sh` and `sudo bash install.sh` to build app and install `systemd` service.

# Uninstall

* Run `sudo bash uninstall.sh`