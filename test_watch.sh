#! /bin/bash

echo_and_run() {
    echo "$@"
    eval "$@"
}
echo_and_run source begin.sh
sleep 0.5
echo_and_run client set thu 1
echo_and_run client set zju 3
echo_and_run client multi
echo_and_run client watch zju 0
echo_and_run client watch thu 0
echo_and_run client set zju 1
echo_and_run client set thu 3
echo_and_run client exec 0
echo_and_run client get zju
echo_and_run client get thu