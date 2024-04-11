<p align="center">
    <img src="logo.png" alt="logo" width="200">
    <h1 align="center">EasyGPG</h1>
</p>

[![Build](https://github.com/jwt2706/EasyGPG/actions/workflows/build.yml/badge.svg?branch=main)](https://github.com/jwt2706/EasyGPG/actions/workflows/build.yml)

EasyGPG is a command line tool to help simplify the process of encrypting and decrypting with GPG.

## Why

I kept forgetting the GPG commands and got fed up with looking them up.

:)

## Usage

```
easygpg [-e] [-d] [-s] [--help] [-h]
```

- `-e`: Triggers the encryption process.
- `-d`: Triggers the decryption process.
- `-s`: Lists the your current public keys available.
- `--help` or `-h`: Displays the help message.

## Shell Installers

To install EasyGPG using the provided shell installer, follow these steps:

#### Linux and macOS

1. Download the installer script from the [installer folder](https://github.com/jwt2706/EasyGPG/tree/v0.1.0/installer).
2. Make the installer executable: `chmod +x install.sh`.
3. Run the installer: `./install.sh`.

#### Windows

1. Download and install [Git for Windows](https://gitforwindows.org/).
2. Download the installer from the [installer folder](https://github.com/jwt2706/EasyGPG/tree/v0.1.0/installer).
3. Open Git Bash and navigate to the directory where you downloaded the installer.
4. Run the installer: `bash install.sh`.

For uninstalling, just run the `uninstall.sh` shell script.

## Manual Installation

EasyGPG is dependant on [GnuPG](https://gnupg.org/)... since that's what it was built for. Follow these steps for set up (or figure it out yourself, I'm sure you're smart enough):

### 1. Installing GnuPG

Debian: `sudo apt-get install gnupg`<br />
Fedora: `sudo dnf install gnupg`<br />
Arch: `sudo pacman -S gnupg`<br />
macOS / OSX: `brew install gnupg`<br />
Windows: Download from [here](https://gpg4win.org/download.html).

### 2. Install EasyGPG

#### Linux

1. Download the latest EasyGPG release from the [releases page](https://github.com/jwt2706/easygpg/releases).
2. Make the binary executable: `chmod +x easygpg`
3. Move the binary to a directory in your PATH, e.g., `/usr/local/bin`: `sudo mv easygpg /usr/local/bin/`

#### macOS

1. Download the latest EasyGPG release from the [releases page](https://github.com/jwt2706/easygpg/releases).
2. Make the binary executable: `chmod +x easygpg`
3. Move the binary to a directory in your PATH, e.g., `/usr/local/bin`: `mv easygpg /usr/local/bin/`

#### Windows

1. Download the latest EasyGPG release from the [releases page](https://github.com/jwt2706/easygpg/releases).
2. Rename the downloaded file to `easygpg.exe`.
3. Move the `easygpg.exe` file to a directory in your PATH. You can add a directory to your PATH by following [these instructions](https://www.architectryan.com/2018/03/17/add-to-the-path-on-windows-10/).

<br />
<b>Disclaimer:</b> I did not get to test the installation process on all operating systems. If you are running into an problem, you can open an [issue](https://github.com/jwt2706/EasyGPG/issues).

## Todo

- Add decryption for direct ascii-armored inputs
