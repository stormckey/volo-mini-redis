#!   /bin/bash

echo "rm -rf *.log"
rm -rf *.log
echo "source begin.sh"
source begin.sh
sleep 1
echo "\nclinet -p 8100 get zju"
client -p 8100 get zju
echo "\nclient -p 8101 set zju 1"
client -p 8101 set zju 1
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get a"
client -p 8000 get a
echo "\nclient -p 8000 set a 2"
client -p 8000 set a 2
echo "\nclient -p 8000 set zju 1"
client -p 8000 set zju 1
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get a"
client -p 8000 get a
echo "\nclient -p 8000 del zju"
client -p 8000 del zju
echo "\nclient -p 8000 get zju"
client -p 8000 get zju
echo "\nclient -p 8000 get a"
client -p 8000 del a
echo "\nclient -p 8000 get a"
client -p 8000 get a


