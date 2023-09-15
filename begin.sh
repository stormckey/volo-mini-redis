#! /bin/bash
lsof -ti :8000 | xargs kill -9
lsof -ti :8100 | xargs kill -9
lsof -ti :8101 | xargs kill -9
lsof -ti :8102 | xargs kill -9
lsof -ti :8200 | xargs kill -9
lsof -ti :8201 | xargs kill -9
lsof -ti :8202 | xargs kill -9
lsof -ti :8300 | xargs kill -9
lsof -ti :8301 | xargs kill -9
lsof -ti :8302 | xargs kill -9
echo "server -p 8100 master &"
server -p 8100 master &
echo "server -p 8101 slave 8100 &"
server -p 8101 slave 8100 &
echo "server -p 8102 slave 8100 &"
server -p 8102 slave 8100 &
echo "server -p 8200 master &"
server -p 8200 master &
echo "server -p 8201 slave 8200 &"
server -p 8201 slave 8200 &
echo "server -p 8202 slave 8200 &"
server -p 8202 slave 8200 &
echo "server -p 8300 master &"
server -p 8300 master &
echo "server -p 8301 slave 8300 &"
server -p 8301 slave 8300 &
echo "server -p 8302 slave 8300 &"
server -p 8302 slave 8300 &
echo "server -p 8000 proxy &"
server -p 8000 proxy &
