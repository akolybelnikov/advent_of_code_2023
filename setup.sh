#!/bin/bash

# Specify the number of days
num_days=25

# Loop to create binaries
for (( day=1; day<=num_days; day++ ))
do
    directory="src/bin/day$day"
    mkdir -p $directory
    echo "fn main() {
    println!(\"Hello from day$day!\");
}" > $directory/main.rs

    # Writing to Cargo.toml
    {
        echo "[[bin]]"
        echo "name = \"day$day\""
        echo "path = \"$directory/main.rs\""
    } >> Cargo.toml
done
