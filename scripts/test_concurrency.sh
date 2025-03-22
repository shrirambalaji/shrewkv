#!/bin/bash

# Number of concurrent instances
CONCURRENT_INSTANCES=10

# Loop and execute the command in the background
for i in $(seq 1 $CONCURRENT_INSTANCES); do
   cargo run -- ping &
done

# Wait for all background jobs to finish
wait
echo "All instances completed."
