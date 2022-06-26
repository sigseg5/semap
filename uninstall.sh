#! /bin/bash

# check sudo
if [ "$EUID" -ne 0 ]
	then echo "Please run as root"
	exit
fi

sudo systemctl stop semap
sudo systemctl disable semap
sudo rm -f /lib/systemd/system/semap.service
sudo rm -f /usr/bin/semap

echo "Done."
