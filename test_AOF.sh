#! /bin/bash

echo "server -p 8888 master &"
server -p 8888 master &
sleep 0.5
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3

echo "sleep 5 secs"
sleep 5
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3
echo "client -p 8888 set zju 3"
client -p 8888 set zju 3
echo "client -p 8888 del zju"
client -p 8888 del zju
echo "client -p 8888 set pku 1"
client -p 8888 set pku 1

kill -2 %2