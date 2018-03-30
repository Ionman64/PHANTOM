#!/bin/bash
path_to_git=$1
output_file=$2

git --git-dir $path_to_git/.git log --format='%H,%P,%an,%ae,%at,%cn,%ce,%ct' > $output_file