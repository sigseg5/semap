# semap

This is a dynamic dconf layout switcher for IBM Model M keyboards. Tested at Fedora 36/Pop!_OS 22.04.

## Dependencies

* systemd;
* GNOME DE;
* dconf
* cargo (install via `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`).

## Get your fingerprint data

* Run app with `cargo run -- --find`/`cargo run -- -f` command;
* You'l get something like that:

```text
You fingerprint is [
    (
        0x13BA,
        0x18,
    ),
]
```

* Put this values to 30 line in main.rs, after that rebuild and reinstall service (from `Installation` section).

## Configuration

You can configure layout by pass valid dconf string to 37 line in main.rs

## Installation

* Run `bash build.sh` and `sudo bash install.sh` to build app and install `systemd` service.

## Upgrading

* Run `sudo bash uninstall.sh`
* Continue normal installation

## Uninstall

* Run `sudo bash uninstall.sh`
