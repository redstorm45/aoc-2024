#!/bin/bash

lastprog=$(ls | grep aoc | tail -n 1)
PROGCOUNT=${lastprog: -2}
outfile=$(mktemp)


echo "Build all..."
for i in $(seq 1 $PROGCOUNT)
do
	dirname="aoc$(printf "%02d" $i)"
	(cd $dirname && cargo build)
done


echo "\nTiming programs..."
for i in $(seq 1 $PROGCOUNT)
do
	p=$(printf "%02d" $i)
	dirname="aoc${p}"
	echo "Running [$p]"
	\time --format="[$p] %E" -o $outfile -a "./${dirname}/target/debug/${dirname}" "./${dirname}/input"
done

echo "\nTimes:"
cat $outfile

rm $outfile
