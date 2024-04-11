#!/bin/bash

# Detect the operating system
OS="$(uname)"
ID_LIKE="$(source /etc/os-release; echo $ID_LIKE)"

# Install GnuPG
if [ "$OS" == "Linux" ]; then
    if [ "$ID_LIKE" == "debian" ]; then
        sudo apt-get install -y gnupg
    elif [ "$ID_LIKE" == "rhel fedora" ]; then
        sudo dnf install -y gnupg
    elif [ "$ID_LIKE" == "arch" ]; then
        sudo pacman -Sy gnupg
    else
        echo "Unsupported Linux distribution. Please see README for manual installation instructions."
        exit 1
    fi
elif [ "$OS" == "Darwin" ]; then
    brew install gnupg
else
    echo "Unsupported operating system. Please see README for manual installation instructions."
    exit 1
fi

# Download the latest EasyGPG release
curl -s https://api.github.com/repos/jwt2706/easygpg/releases/latest \
| grep "browser_download_url.*easygpg" \
| cut -d : -f 2,3 \
| tr -d \" \
| wget -qi -

# Make the binary executable
chmod +x easygpg

# Move the binary to a directory in the PATH
if [ "$OS" == "Linux" ]; then
    sudo mv easygpg /usr/local/bin/
elif [ "$OS" == "Darwin" ]; then
    mv easygpg /usr/local/bin/
else
    echo "Unsupported operating system. Please see README for manual installation instructions."
    exit 1
fi