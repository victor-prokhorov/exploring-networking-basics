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
[Pi 2/3/4: AArch32 target with hard float (arm-none-linux-gnueabihf)](https://developer.arm.com/downloads/-/gnu-a)
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

