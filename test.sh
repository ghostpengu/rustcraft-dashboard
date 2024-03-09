#!/bin/bash

# Check if the number of arguments is correct
if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <cpu_number>"
    exit 1
fi

# Extract the CPU number from the command line argument
cpu_number=$1

# Run the taskset command with the specified CPU number and start the Java server
taskset -c "$cpu_number" java -Xmx1024M -Xms1024M -jar server.jar nogui