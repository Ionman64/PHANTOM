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


def populate_figure(group, ax_line, ax_norm, ax_acc, ax_acc_norm, style_line="-", style_rolling_mean='--',
                    window_size_rolling_mean=5, ):
    group.plot(label=key, style=style_line, legend=True, ax=ax_line)
    rolling_mean_for_series(group).plot(label=key, style=style_rolling_mean, color=last_color(ax_line), ax=ax_line)

    norm_group = normalise_series(group)
    norm_group.plot(label=key, style=style_line, legend=True, ax=ax_norm)
    rolling_mean_for_series(norm_group).plot(label=key, style=style_rolling_mean, color=last_color(ax_norm), ax=ax_norm)

    acc_group = accumulate_series(group)
    acc_group.plot(label=key, style=style_line, legend=True, ax=ax_acc)
    norm_acc_group = normalise_series(accumulate_series(group))
    norm_acc_group.plot(label=key, style=style_line, legend=True, ax=ax_acc_norm)


def last_color(axes):
    return axes.lines[-1].get_color()


def plot_line(series, key, ax, rolling_mean=True, window_size=5):
    series.plot(label=key, style='-', legend=True, ax=ax)


def rolling_mean_for_series(series, window_size=5):
    return series.rolling(window=window_size).mean()


def accumulate_series(series):
    return series.agg(np.add.accumulate)


def normalise_series(series):
    return (series - series.min()) / (series.max() - series.min())


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

    fig_dates = plt.figure(figsize=(10, 10))
    fig_leftshift = plt.figure(figsize=(10, 10))
    fig_rightshift = plt.figure(figsize=(10, 10))
    fig_max_peak = plt.figure(figsize=(10, 10))
    ax = {
        'line': plt.subplot2grid((3, 2), (0, 0), colspan=2, fig=fig_dates),
        'norm': plt.subplot2grid((3, 2), (1, 0), colspan=2, fig=fig_dates),
        'acc': plt.subplot2grid((3, 2), (2, 0), colspan=1, fig=fig_dates),
        'acc-norm': plt.subplot2grid((3, 2), (2, 1), colspan=1, fig=fig_dates),
        'left-line': plt.subplot2grid((3, 2), (0, 0), colspan=2, fig=fig_leftshift),
        'left-norm': plt.subplot2grid((3, 2), (1, 0), colspan=2, fig=fig_leftshift),
        'left-acc': plt.subplot2grid((3, 2), (2, 0), colspan=1, fig=fig_leftshift),
        'left-acc-norm': plt.subplot2grid((3, 2), (2, 1), colspan=1, fig=fig_leftshift),
        'right-line': plt.subplot2grid((3, 2), (0, 0), colspan=2, fig=fig_rightshift),
        'right-norm': plt.subplot2grid((3, 2), (1, 0), colspan=2, fig=fig_rightshift),
        'right-acc': plt.subplot2grid((3, 2), (2, 0), colspan=1, fig=fig_rightshift),
        'right-acc-norm': plt.subplot2grid((3, 2), (2, 1), colspan=1, fig=fig_rightshift),
        'max-peak-line': plt.subplot2grid((3, 2), (0, 0), colspan=2, fig=fig_max_peak),
        'max-peak-norm': plt.subplot2grid((3, 2), (1, 0), colspan=2, fig=fig_max_peak),
        'max-peak-acc': plt.subplot2grid((3, 2), (2, 0), colspan=1, fig=fig_max_peak),
        'max-peak-acc-norm': plt.subplot2grid((3, 2), (2, 1), colspan=1, fig=fig_max_peak),
    }
    ax['line'].set_ylabel("frequency")
    ax['norm'].set_ylabel("norm.")
    ax['acc'].set_ylabel("acc.")
    ax['acc-norm'].set_ylabel("acc. norm.")
    #
    ax['left-line'].set_ylabel("frequency")
    ax['left-norm'].set_ylabel("norm.")
    ax['left-acc'].set_ylabel("acc.")
    ax['left-acc-norm'].set_ylabel("acc.")
    #
    ax['right-line'].set_ylabel("frequency")
    ax['right-norm'].set_ylabel("norm.")
    ax['right-acc'].set_ylabel("acc.")
    ax['right-acc-norm'].set_ylabel("acc.")
    #
    ax['max-peak-line'].set_ylabel("frequency")
    ax['max-peak-norm'].set_ylabel("norm.")
    ax['max-peak-acc'].set_ylabel("acc.")
    ax['max-peak-acc-norm'].set_ylabel("acc. norm.")
    fig_dates.suptitle("Commit frequency")
    fig_leftshift.suptitle("Commit frequency (left shifted)")
    fig_rightshift.suptitle("Commit frequency (right shifted)")
    fig_max_peak.suptitle("Commit frequency (max peak shifted)")

    style_line = '-'
    style_rolling_mean = '--'
    for key, group in frame.groupby('repository_id'):
        group = group.frequency.resample(arg_time_unit).sum()
        populate_figure(group, ax_line=ax['line'], ax_norm=ax['norm'], ax_acc=ax['acc'], ax_acc_norm=ax['acc-norm'])

        leftshifted_x = [(group.index[idx] - group.index[0]).days for idx in range(len(group.index[:-1]))]
        leftshifted = pd.Series(data=group.values[:-1], index=leftshifted_x)
        populate_figure(leftshifted, ax_line=ax['left-line'], ax_norm=ax['left-norm'], ax_acc=ax['left-acc'], ax_acc_norm=ax['left-acc-norm'])

        rightshifted_x = [leftshifted_x[idx] - leftshifted_x[-1] for idx in range(len(leftshifted_x))]
        rightshifted = pd.Series(data=group.values[:-1], index=rightshifted_x)
        populate_figure(rightshifted, ax_line=ax['right-line'], ax_norm=ax['right-norm'], ax_acc=ax['right-acc'], ax_acc_norm=ax['right-acc-norm'])

        max_peak = np.argmax(group.values)
        max_peakshifted_x = [leftshifted_x[idx] - leftshifted_x[max_peak] for idx in range(len(leftshifted_x))]
        max_peakshifted = pd.Series(data=group.values[:-1], index=max_peakshifted_x)
        populate_figure(max_peakshifted, ax_line=ax['max-peak-line'], ax_norm=ax['max-peak-norm'], ax_acc=ax['max-peak-acc'], ax_acc_norm=ax['max-peak-acc-norm'])


    plt.show()
