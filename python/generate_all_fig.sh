#!/bin/bash

python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans000.png -h
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans010.png -h --norm
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans001.png -h --shiftleft
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans011.png -h --norm --shiftleft
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans100.png -h --acc
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans110.png -h --acc --norm
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans101.png -h --acc --shiftleft
python plotter.py in2/2.csv in2/3.csv in2/4.csv -o out/234_ans111.png -h --acc --norm --shiftleft

echo Done.