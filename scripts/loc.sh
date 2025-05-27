#!/bin/bash

# Default to ./src if no argument provided
DIR=${1:-./src}  

echo "Counting lines of Rust code in: $DIR"
find "$DIR" -name '*.rs' | xargs wc -l
