#!/usr/bin/env bash

cargo build >/dev/null

for i in 1 2 4 8; do
	./target/debug/process measurements_e3.txt $i | sort >temp$i.txt
done

for i in 2 4 8; do
	echo "Diff between 1 and $i threads :"
	diff -y --suppress-common-lines temp1.txt temp$i.txt | tail
	echo "[Total : $(diff -y --suppress-common-lines temp1.txt temp$i.txt | wc -l)]"

	echo ""
done
