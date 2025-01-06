#!/bin/bash

lastprog=$(ls data | grep aoc | tail -n 1)
PROGCOUNT=${lastprog: -2}
outfile=$(mktemp)


echo "Build all..."
cargo build


printf "\nTiming programs...\n"
for i in $(seq 1 $PROGCOUNT)
do
	p=$(printf "%02d" $i)
	dirname="aoc${p}"
	echo "Running [$p]"
	\time --format="[$p] %e" -o $outfile -a "./target/debug/${dirname}" "./data/${dirname}/input"
done

printf "\nTimes:\n"
cat $outfile

rm $outfile
