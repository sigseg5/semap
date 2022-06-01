#! /bin/bash

# check sudo
if [ "$EUID" -ne 0 ]
	then echo "Please run as root"
	exit
fi

# check for semap bin in `target/release/`
DIR=target/release

if [ $(ls -A "$DIR" | grep semap | wc -l) -gt 0 ]; then
	# copy bin and create systemd service
	sudo cp $DIR/semap /usr/bin/semap
	sudo cp semap.service /etc/systemd/system/semap.service
	sudo chmod 644 /etc/systemd/system/semap.service
	sudo systemctl start semap
	sudo systemctl enable semap
else
   echo "Can't find semap bin in ${DIR}."
   exit
fi

echo "Done."
