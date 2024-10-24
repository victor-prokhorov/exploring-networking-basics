source .pirc

echo -e "\ntarget\t\t$TARGET"
echo -e "pi ip\t\t$PI_IP"
echo -e "pi secret\t$PI_SECRET"
echo -e "running as\t$(sshpass -p "$PI_SECRET" ssh pi@$PI_IP 'whoami')\n"
# control c only stop sshpass not the server
# with "" it execut on host with '' it does not interpret the $()
# sshpass -p "$PI_SECRET" ssh pi@$PI_IP 'sudo kill -9 $(sudo lsof -t -i :80) && sudo ./iteration1'
cargo watch --shell "cargo build --target $TARGET --release && \
    sshpass -p \"$PI_SECRET\" ssh pi@$PI_IP 'sudo lsof -t -i :80 | xargs -r sudo kill -9' && \
    sshpass -p \"$PI_SECRET\" ssh pi@$PI_IP 'sudo rm /home/pi/iteration1' && \
    sshpass -p \"$PI_SECRET\" scp -r ./target/$TARGET/release/iteration1 pi@$PI_IP:/home/pi && \
    sshpass -p \"$PI_SECRET\" ssh pi@$PI_IP 'cd /home/pi && sudo ./iteration1'"

