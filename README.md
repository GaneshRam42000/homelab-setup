# Homelab Setup for Raspberry pi4

This Homelab setup is for setting up these services in raspberry pi using docker.

- PiHole
- Jellyfin
- Nextcloud
- Transmission 

## Initial setup

- Make sure latest version of raspberry Pi OS is installed in the sd card.
- connect the raspberry pi to the internet.
- enable `ssh` and expand the volume of the os install using `raspi-config` .
- update all the packages using the command : `sudo apt-get update && sudo apt-get upgrade`.


## optional Install 

### for managing the Pi from the browser install cockpit 
` sudo apt-get install cockpit `
### After installing check if the servce is running or not using the command
`sudo systemctl status cockpit.sock`
### If not enable the service and reboot

```bash
sudo systemctl enable cockpit.sock 
sudo reboot
``` 

### access the service using the address

`https://<LOCAL_IP_ADDR>:9090/`

## Install Docker
The best way to install docker is by using the install script provided in the docker [documentation](https://docs.docker.com/engine/install/debian/#install-using-the-convenience-script)

```bash 
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
```
