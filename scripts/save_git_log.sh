#!/bin/bash
path_to_git=$1
output_file=$2

git --git-dir $path_to_git log --format='%cd' --date=format:'%Y-%m-%d' > $output_file