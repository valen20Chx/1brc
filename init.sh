#!/bin/bash

if [ ! -d "./1brc" ]; then
	git clone https://github.com/gunnarmorling/1brc.git
fi

# Build the project
if [ ! -f "./1brc/mesurements.txt" ]; then
	eval "(cd ./1brc; ./mvnw clean verify)"

	# Create the measurements
	eval "(cd ./1brc; ./create_measurements.sh 1000000000)"
	mv ./1brc/mesurements.txt ./mesurements.txt
fi

if [ ! -f "./1brc/mesurements_e3.txt" ]; then
	head -n 1000 ./mesurements.txt >mesurements_e3.txt
fi

if [ ! -f "./1brc/mesurements_e6.txt" ]; then
	head -n 1000000 ./mesurements.txt >mesurements_e6.txt
fi
