#!/usr/bin/env bash

# root_dir is expected to have two directories named 'csv' and 'feature_table'.
# directory csv: contains the commit frequency csv files
# directory feature_table: is used for outputting the generated feature table for each file in directory csv
root_dir=$1

for file in $root_dir/csv/*.csv;
do
	filename=${file##*/}
	python feature_vector_gen.py $file $root_dir/feature_table/$filename

done