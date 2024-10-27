# exploring networking basics

## iteration 1: reference

### basic setup catch trace on host machine

- https://www.raspberrypi.com/documentation/computers/remote-access.html#copy-files-to-your-raspberry-pi
```sh
source .pirc
scp iteration1/target/armv7-unknown-linux-gnueabihf/debug/iteration1 "pi@$PI_IP:"
```
- `curl --trace trace.txt http://localhost:80` save trace of reference implementation on host machine

### run it from the pi

- [cross compiling rust for pi](https://capnfabs.net/posts/cross-compiling-rust-apps-raspberry-pi/)
```sh
sudo apt install gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf
export CARGO_TARGET_ARMV7_UNKNOWN_LINUX_GNUEABIHF_LINKER=/usr/bin/arm-linux-gnueabihf-gcc
cargo build --target=armv7-unknown-linux-gnueabihf
```
- [installing arm cross compiling gnu toolchain](https://chacin.dev/blog/cross-compiling-rust-for-the-raspberry-pi/)
- [Pi 2/3/4: AArch32 target with hard float (arm-none-linux-gnueabihf)](https://developer.arm.com/downloads/-/gnu-a)
```
export PATH="$HOME/bin/gcc-arm-10.3-2021.07-x86_64-arm-none-linux-gnueabihf/bin:$PATH"
```
- [what actually worked, kudos to TheDutchMC76](https://www.reddit.com/r/rust/comments/vparsp/has_anyone_programmed_a_raspberry_pi_with_rust/)
```sh
# requires gcc compiler
sudo apt install -y gcc-aarch64-linux-gnu
#  build
rustup target add aarch64-unknown-linux-gnu
cargo build --target aarch64-unknown-linux-gnu
```
- .cargo/config.toml
```toml
[target.aarch64-unknown-linux-gnu]
linker = "/usr/bin/aarch64-linux-gnu-gcc"
```

### config router

- from `ifconfig` get mac address and ip
- DHCP address reservation, add new to make sure not accidentally operate on another machine
- allow accepting in `iptables`
```sh
pi@raspberrypi:~ $ sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT
pi@raspberrypi:~ $ sudo iptables -L
Chain INPUT (policy ACCEPT)
target     prot opt source               destination
ACCEPT     tcp  --  anywhere             anywhere             tcp dpt:http

Chain FORWARD (policy ACCEPT)
target     prot opt source               destination

Chain OUTPUT (policy ACCEPT)
target     prot opt source               destination
```
- https://canyouseeme.org/

## iteration 2: with linux system calls
- https://www.cs.cmu.edu/~prs/15-441-F10/lectures/r01-sockets.pdf

## iteratoin 3: preparing the os from scratch
- https://os.phil-opp.com/freestanding-rust-binary/#comments
- https://www.youtube.com/watch?v=rH5jnbJ3tL4&list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY&index=1
- [redox should work on a pi but it's much more complex](https://www.redox-os.org/)
- https://www.youtube.com/watch?app=desktop&v=IgC2HvBesms
- https://github.com/smoltcp-rs/smoltcp/tree/main
- https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials

### booting the thing

- preparing the sd card
```sh
lsblk
sudo umount /dev/sdb1
sudo umount /dev/sdb2
sudo lsof | grep /dev/sdb
sudo fdisk /dev/sdb
# `d` delete partition
# `n` create new partition
# `t` and pick FAT32
# `w` to write in memory change
# format partition
sudo mkfs.vfat /dev/sdb1
sudo mount /dev/sdb1 /mnt
```
- [super straightforward minimal example](https://harmonicss.co.uk/rust/rust-on-a-raspberry-pi-part-1/)
- i will derive this one and will try to blink activity led to start
- toolchain doublecheck
```sh
rustup target add armv7a-none-eabi
sudo apt install binutils-arm-none-eabi
```
- cargo build and inspect with
```s
arm-none-eabi-objdump -D ./target/armv7a-none-eabi/debug/pi_baremetal_rust | less
```
- use custom linked script
```sh
cargo rustc -- -C link-arg=--script=./linker.ld
```
- start need to at `00008000 <_start>:`
- https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
- linux will produce .elf files
- `arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/pi_baremetal_rust ./kernel7.img` 
- firemware boot, from https://github.com/raspberrypi/firmware.git
```sh
sudo curl -O https://raw.githubusercontent.com/raspberrypi/firmware/HEAD/boot/start.elf
sudo curl -O https://raw.githubusercontent.com/raspberrypi/firmware/HEAD/boot/bootcode.bin
sudo curl -O https://raw.githubusercontent.com/raspberrypi/firmware/HEAD/boot/fixup.dat
```
