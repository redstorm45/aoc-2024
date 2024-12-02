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


printf "\nTiming programs...\n"
for i in $(seq 1 $PROGCOUNT)
do
	p=$(printf "%02d" $i)
	dirname="aoc${p}"
	echo "Running [$p]"
	\time --format="[$p] %e" -o $outfile -a "./${dirname}/target/debug/${dirname}" "./${dirname}/input"
done

printf "\nTimes:\n"
cat $outfile

rm $outfile
