#!/bin/bash
echo -n "Preparing........ "
sleep $1
echo "[DONE]"
echo -n "Running system... "
sleep $1
echo "[DONE]"
exec 1>/dev/null
exec 0>/dev/null
sleep 0.1
exit $2
