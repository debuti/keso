# keso
Real-time operating system for RP2040 written in Rust

# Environment setup
This chapter targets Ubuntu 20.04

## Build

### Toolchain
```
sudo apt install cmake libnewlib-arm-none-eabi build-essential curl libncurses5
```

```
GANE_VERSION=10-2020-q4-major
wget https://developer.arm.com/-/media/Files/downloads/gnu-rm/10-2020q4/gcc-arm-none-eabi-$GANE_VERSION-x86_64-linux.tar.bz2
sudo tar xjvf gcc-arm-none-eabi-$GANE_VERSION-x86_64-linux.tar.bz2 -C /usr/share && rm gcc-arm-none-eabi-$GANE_VERSION-x86_64-linux.tar.bz2
for n in gcc gdb size objdump readelf ar as addr2line ld nm strings objcopy strip; do
  sudo ln -s /usr/share/gcc-arm-none-eabi-$GANE_VERSION/bin/arm-none-eabi-$n /usr/bin/arm-none-eabi-$n
done
```


### Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
bash rustup.sh -y
rm rustup.sh
source $HOME/.cargo/env
# Add cross-compilation support for core Cortex-M0+ (ARMv6M)
rustup target add thumbv6m-none-eabi
cargo install cargo-binutils
```


## Dev

### Debugger bridge
```
git clone https://github.com/raspberrypi/openocd.git --branch picoprobe --depth=1 --no-single-branch
sudo apt install automake autoconf build-essential texinfo libtool libftdi-dev libusb-1.0-0-dev
cd openocd
./bootstrap
./configure --enable-picoprobe
make 
sudo make install
echo "#Picoprobe
ATTRS{idVendor}==\"2e8a\", ATTRS{idProduct}==\"0004\", MODE=\"664\", GROUP=\"plugdev\"
" >> /etc/udev/rules.d/99-openocd.rules
sudo udevadm control --reload
```

### VSCode as IDE
```
curl -L "https://code.visualstudio.com/sha/download?build=stable&os=linux-deb-x64" --output vscode.deb
sudo dpkg -i vscode.deb && rm vscode.deb
code --install-extension marus25.cortex-debug
code --install-extension ms-vscode.cmake-tools
code --install-extension ms-vscode.cpptools
```

# License
This software project follows the license (CC BY-NC-SA 4.0)[https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode]


# TODO
* Use timer to blink a led
* Use timers to create a primitive scheduler
* Create the concept of task
* Sync both cores with spinlocks/barriers, to sync the schedule tables
* Remove unsafe blocks!!