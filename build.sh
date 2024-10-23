source .pirc
cargo build --target $TARGET --release
sshpass -p 'pi1234' scp -r ./target/$TARGET/release/iteration1 pi@$PI_IP:/home/pi
sshpass -p 'pi1234' ssh pi@$PI_IP './iteration1'
