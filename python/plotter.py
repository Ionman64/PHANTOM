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
    fig1, ax1 = plt.subplots(3, sharex=True)
    fig2, ax2 = plt.subplots(3, sharex=True)
    ax = { 'line': ax1[0], 'norm': ax1[1], 'acc': ax1[2], 'left-line': ax2[0], 'left-norm': ax2[1], 'left-acc': ax2[2]}
    ax['line'].set_ylabel("frequency")
    ax['norm'].set_ylabel("norm. frequency")
    ax['acc'].set_ylabel("acc. frequency")
    ax['left-line'].set_ylabel("frequency")
    ax['left-norm'].set_ylabel("norm. frequency")
    ax['left-acc'].set_ylabel("acc. frequency")
    fig1.suptitle("Commit frequency over time")
    fig2.suptitle("Commit frequency over time delta")
    for key, group in frame.groupby('repository_id'):
        group = group.frequency.resample(arg_time_unit).sum()
        ## DATES
        # line
        group.plot(y='frequency', label=key, style='-', legend=True, ax=ax['line'])
        group.rolling(window=5).mean().plot(y='frequency', color=ax['line'].lines[-1].get_color(), style='--', ax=ax['line'],)
        # normalised
        norm_group = (group - group.min()) / (group.max() - group.min())
        norm_group.plot(y='frequency', label=key, legend=True, ax=ax['norm'])
        norm_group.rolling(window=5).mean().plot(y='frequency',color=ax['norm'].lines[-1].get_color(), style='--',  ax=ax['norm'])
        # accumulated
        acc_group = group.agg(np.add.accumulate)
        acc_group.plot(y='frequency', label=key, legend=True, ax=ax['acc'])
        ## TIME DELTA
        # line
        leftshifted_x = [(group.index[idx] - group.index[0]).days for idx in range(len(group.index[:-1]))]
        leftshifted = pd.Series(data=group.values[:-1], index=leftshifted_x)
        leftshifted.plot(label=key, legend=True, ax=ax['left-line'])
        leftshifted.rolling(window=5).mean().plot(label=key, legend=False, color=ax['left-line'].lines[-1].get_color(), ax=ax['left-line'])
        # normalised
        norm_leftshifted = (leftshifted - leftshifted.min()) / (leftshifted.max() - leftshifted.min())
        norm_leftshifted.plot(legend=True, label=key, ax=ax['left-norm'], )
        # accumulated
        leftshifted.agg(np.add.accumulate).plot(legend=True, label=key, ax=ax['left-acc'])

    plt.show()


