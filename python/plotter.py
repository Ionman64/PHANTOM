"""
Usage:
    plotter_v2.py <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT> Resample the data as by day, week, month, quarter, year(annual)[values: D, W, M, Q, A]
Options:
    -h --help           Show this screen.
    --hide              Hide the diagram / Don't show the diagram
    -o --out=<file>     Path to output file. You can specify the file format by using the desired file extension (e.g. png, pdf)
    --shift=<direction> Shift the dates of projects [values: left, right]
    --norm              Normalises all y values to be between 0 and 1
    --acc               Accumulate y values
    --peak              Highlight peaks
    --ydist             Calculate the distance on the y-axis between the first project and all other projects
"""
from docopt import docopt
from utils import data_processor as processor, data_provider as provider, data_visualiser as visualiser
import numpy as np

import matplotlib.pyplot as plt
import pandas as pd
from sqlalchemy import create_engine

if __name__ == '__main__':
    args = docopt(__doc__)

    arg_ids = args['<id>']
    arg_time_unit = args['--timeunit'].upper()
    arg_acc = args['--acc']
    arg_norm = args['--norm']
    arg_shift = args['--shift']
    arg_out_file = args['--out']
    arg_hide = args['--hide']
    arg_ydist = args['--ydist']
    arg_peak = args['--peak']

    # Validate command line arguments ----------------------------------------------------------------------------------
    valid_timeunits = ["D", "W", "M", "Q", "A"]
    if not arg_time_unit in valid_timeunits:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)

    valid_shift_values = ["left", "right", "peak", None]
    if not args['--shift'] in valid_shift_values:
        print("Invalid shift value. Use --help to get more information.")
        exit(1)

    if arg_ydist and not arg_shift == "left":
        print "Y distance calculation only works with left-shifted data. Please set '--shift left'"
        exit(1)

    # get data
    engine = create_engine("postgres://postgres:0000@localhost/project_analyser")
    frame = pd.read_sql_query(
        'SELECT * FROM commit_frequency WHERE repository_id IN (%s) ORDER BY commit_date' % str(arg_ids)[1:-1],
        con=engine,
        index_col='commit_date')
    fig, ax = plt.subplots(2, 2)
    ax = { 'line': ax[0][0], 'norm': ax[1][0], 'acc': ax[0][1], 'left': ax[1][1]}
    ax['line'].set_ylabel("frequency")
    ax['norm'].set_ylabel("normalised frequency")
    ax['acc'].set_ylabel("accumulated frequency")
    ax['left'].set_ylabel("frequency")

    for key, group in frame.groupby('repository_id'):
        group = group.frequency.resample(arg_time_unit).sum()
        # line + rolling mean
        group.plot(y='frequency', label=key, style='-', legend=True, ax=ax['line'])
        group.rolling(window=5).mean().plot(y='frequency', color=ax['line'].lines[-1].get_color(), style='--', ax=ax['line'],)
        # normalised
        norm_group = (group - group.min()) / (group.max() - group.min())
        norm_group.plot(y='frequency', label=key, legend=True, ax=ax['norm'])
        norm_group.rolling(window=5).mean().plot(y='frequency',color=ax['norm'].lines[-1].get_color(), style='--',  ax=ax['norm'])
        # accumulated
        acc_group = group.agg(np.add.accumulate)
        acc_group.plot(y='frequency', label=key, legend=True, ax=ax['acc'])
        # time delta
        leftshifted_x = [(group.index[idx] - group.index[0]).days for idx in range(len(group.index[:-1]))]
        series_leftshifted = pd.Series(data=group.values[:-1], index=leftshifted_x)
        series_leftshifted.plot(label=key, legend=True, ax=ax['left'])


    plt.tight_layout()
    plt.show()


