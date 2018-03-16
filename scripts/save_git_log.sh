#!/bin/bash
path_to_git=$1
output_file=$2

git --git-dir $path_to_git log --numstat > $output_file