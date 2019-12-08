#!/bin/sh
sh cargo.sh true
sh cargo.sh false
for WHICH in debug ndebug
do
    echo "-- $WHICH"
    time ./$WHICH ./acgt 0
done
