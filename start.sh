#! /bin/bash

filename="server.conf"

if [ ! -f "$filename" ]; then
  echo "file '$filename' dont exist。"
else 
    first_line=true
    while IFS= read -r line; do
    if $first_line; then
        echo "server --port $line -m"
        # server --port $line -m &
        first_line=false
    else
        echo "server --port $line -s"
        # server --port $line -s &
    fi
    done < "$filename"
fi



