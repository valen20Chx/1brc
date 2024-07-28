#!/bin/bash

if [ ! -d "./1brc-base" ]; then
	git clone https://github.com/gunnarmorling/1brc.git 1brc-base
fi

# Create the measurements
if [ ! -f "./measurements_e9.txt" ]; then
	# Build the project
	if [ ! -f "./1brc-base/data/measurements.txt" ]; then
		(
			cd ./1brc-base/src/main/python/;
			python3 ./create_measurements.py 1000000000;
		);
	fi
	mv ./1brc-base/data/measurements.txt ./measurements_e9.txt;
fi

if [ ! -f "./measurements_e3.txt" ]; then
	head -n 1000 ./measurements_e9.txt > measurements_e3.txt
fi

if [ ! -f "./measurements_e6.txt" ]; then
	head -n 1000000 ./measurements_e9.txt > measurements_e6.txt
fi

echo "Done!"
