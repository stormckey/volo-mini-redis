#! /bin/bash

echo_and_run() {
    echo "$@"
    eval "$@"
}

echo_and_run server -p 8085 master &
echo_and_run server -p 8086 slave 8085 &
echo_and_run server -p 8087 slave 8085 &
sleep 0.5
echo_and_run client -p 8085 get zju
echo_and_run client -p 8086 get zju
echo_and_run client -p 8087 get zju
echo_and_run client -p 8086 set zju 1
echo_and_run client -p 8085 set zju 1
echo_and_run client -p 8086 get zju
echo_and_run client -p 8087 get zju
echo_and_run client -p 8085 set zju 114514