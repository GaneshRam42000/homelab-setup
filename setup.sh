## updating the packages (comment out if using different distro other than debian derivative)
#!/bin/bash

apt-get update && sudo apt-get upgrade

echo "********Installing docker***********"
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
echo "************************************"
echo "********Installing PiHole***********"
pushd PiHole/
sudo docker compose up -d
popd
echo "************************************"
echo "******installing Jellyfin***********"
pushd Jellyfin/
sudo docker compose up -d
popd
echo "*************************************"
echo "********Installing NextCloud*********"
pushd Nextcloud/
sudo docker compose up -d
popd
echo "*************************************"
