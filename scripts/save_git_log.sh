#!/bin/bash
path_to_git=$1
output_file=$2

git --git-dir $path_to_git/.git log --format='%cd' --date=format:'%Y-%m-%d' > $output_file

rm -rf $path_to_git