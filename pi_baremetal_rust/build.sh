cargo rustc -- -C link-arg=--script=./linker.ld && \
arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/pi_baremetal_rust ./kernel7.img && \
cp kernel7.img /media/victorprokhorov/C753-FCC9/ && \
umount /media/victorprokhorov/C753-FCC9/ && \
udisksctl power-off -b /dev/sdb && \
echo 'build, wrote and ejected'
