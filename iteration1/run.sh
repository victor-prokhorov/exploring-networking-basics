source .pirc

echo -e "\ntarget\t\t$TARGET"
echo -e "pi ip\t\t$PI_IP"
echo -e "pi secret\t$PI_SECRET\n"

cargo build --target $TARGET --release
sshpass -p "$PI_SECRET" scp -r ./target/$TARGET/release/iteration1 pi@$PI_IP:/home/pi
sshpass -p "$PI_SECRET" ssh pi@$PI_IP './iteration1'
