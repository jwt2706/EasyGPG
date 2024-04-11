#!/bin/bash

# Detect the operating system
OS="$(uname)"
ID_LIKE="$(source /etc/os-release; echo $ID_LIKE)"

# Remove EasyGPG binary
if [ "$OS" == "Linux" ]; then
    sudo rm /usr/local/bin/easygpg
elif [ "$OS" == "Darwin" ]; then
    rm /usr/local/bin/easygpg
else
    echo "Unsupported operating system. Please see README for manual uninstallation instructions."
    exit 1
fi

# Ask the user whether they want to uninstall GnuPG
read -p "Do you also want to uninstall GnuPG? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]
then
    # Uninstall GnuPG
    if [ "$OS" == "Linux" ]; then
        if [ "$ID_LIKE" == "debian" ]; then
            sudo apt-get remove -y gnupg
        elif [ "$ID_LIKE" == "rhel fedora" ]; then
            sudo dnf remove -y gnupg
        elif [ "$ID_LIKE" == "arch" ]; then
            sudo pacman -Rns gnupg
        else
            echo "Unsupported Linux distribution. Please see README for manual uninstallation instructions."
            exit 1
        fi
    elif [ "$OS" == "Darwin" ]; then
        brew uninstall gnupg
    else
        echo "Unsupported operating system. Please see README for manual uninstallation instructions."
        exit 1
    fi
fi