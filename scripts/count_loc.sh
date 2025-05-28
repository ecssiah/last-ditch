#!/bin/bash

DIR=${1}  
EXT=${2}

echo "Lines of code: $DIR $EXT"
find "./$DIR" -name "*.$EXT" | xargs wc -l
