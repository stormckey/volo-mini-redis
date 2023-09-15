#! /bin/bash

filename="server.conf_"

if [ ! -f "$filename" ]; then
  echo "file '$filename' dont exist。"
else 
    first_line=true
    while IFS= read -r line; do
    lsof -ti :$line | xargs kill -9
    if $first_line; then
        echo "server --port $line master"
        master=$line
        server --port $line master &
        first_line=false
    else
        echo "server --port $line slave $master"
        server --port $line slave $master &
    fi
    done < "$filename"
fi



