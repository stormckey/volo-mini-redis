#! /bin/bash

echo_and_run() {
    echo "$@"
    eval "$@"
}

echo_and_run source begin.sh
sleep 0.5
echo_and_run client del thu
echo_and_run client del zju
echo_and_run client multi
echo_and_run client set zju 3 -t 0
echo_and_run client set thu 1 -t 0
echo_and_run client get zju
echo_and_run client get thu
echo_and_run client exec 0
echo_and_run client get zju
echo_and_run client get thu
