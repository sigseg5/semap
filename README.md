# semap

This is a dynamic dconf layout switcher for my IBM Model M. Tested at Fedora 36.

# Todo

I haven't any time for this, so, if you want to use `semap` recompile this app with your params.

* Parse keyboard vendor_id and priduct_id from argv;
* Parse dconf config string from argv.

# Dependencies

* systemd;
* GNOME DE;
* cargo.

# Installation

* Run `bash build.sh` and `sudo bash install.sh` to build app and install `systemd` service.

# Uninstall

* Run `sudo bash uninstall.sh`