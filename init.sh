#!/bin/bash

if [ ! -d "./1brc" ]; then
	git clone https://github.com/gunnarmorling/1brc.git
fi

# Create the measurements
if [ ! -f "./measurements_e9.txt" ]; then
	# Build the project
	if [ ! -f "./1brc/measurements.txt" ]; then
		eval "(cd ./1brc; ./mvnw clean verify)"
		eval "(cd ./1brc; ./create_measurements.sh 1000000000)"
	fi
	mv ./1brc/measurements.txt ./measurements_e9.txt
fi

if [ ! -f "./1brc/measurements_e3.txt" ]; then
	head -n 1000 ./measurements.txt >measurements_e3.txt
fi

if [ ! -f "./1brc/measurements_e6.txt" ]; then
	head -n 1000000 ./measurements.txt >measurements_e6.txt
fi

echo "Done!"
