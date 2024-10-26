source .pirc
ERROR='commit seppuku by sunrise tomorrow'
echo "testing from: $(hostname -I)"
REF=$(curl -sS http://$PI_IP:80)
REF_PLUS_1=$(curl -sS http://$PI_IP:80)
if (( REF_PLUS_1 != REF + 1 ))
then
    echo "$ERROR"
    exit 1
fi
echo 'ok'
