# EasyGPG

ALMOST DONE. I'll make a release soon.

EasyGPG is a command line tool to help simplify the process of encrypting and decrypting files or plains text with GPG.
<br>I made this because I always forget the GPG commands when I need them.. so now I don't have to remeber them at all :)

## Usage

```
easygpg [-e] [-d] [-s] [--help] [-h]
```

- `-e`: Triggers the encryption process.
- `-d`: Triggers the decryption process.
- `-s`: Lists the your current public keys available.
- `--help` or `-h`: Displays the help message.

## Setup

EasyGPG is dependant on [GnuPG](https://gnupg.org/)... since that's what it was built for. Follow these steps for set up (or figure it out yourself, I'm sure you're smart enough):

### 1. Installing GnuPG

Debian: `sudo apt-get install gnupg`

Fedora: `sudo dnf install gnupg`

Arch: `sudo pacman -S gnupg`

macOS / OSX: `brew isntall gnupg`

Windows: Download from [here](https://gpg4win.org/download.html).

### 2. Install EasyGPG

[...]

## Todo

- finish decryption
- finish readme docs
- add tests
- make package and release v0.1
