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


def euclidean_distance(pid_series, series_was_shifted_to, norm=False, acc=False, round_to_decimals=2):
    pids = pid_series.keys()
    series = pid_series.values()
    if acc:
        series = [accumulate_series(s) for s in series]
    if norm:
        series = [normalise_series(s) for s in series]

    N = len(pids)
    dist_matrix = np.zeros((N, N))
    for i in range(N):
        for j in range(N):
            if i <= j:
                continue
            eucl = np.round(np.average(__get_euclidean__(series[i], series[j], series_was_shifted_to)),
                            decimals=round_to_decimals)
            dist_matrix[i, j] = eucl
            dist_matrix[j, i] = eucl
    df = pd.DataFrame(dist_matrix, index=pids, columns=pids)
    return df


def __get_euclidean__(series1, series2, series_was_shifted_to):
    """
    Calculates the euclidean distance between the values of the two series. Compares points in order, not by similarity
    on the x-axis (i.e. index). Than the values are normed by the max value of both series and finally the average is return.
    :param series1:
    :param series2:
    :return: The similarity in percent.
    """
    assert (series_was_shifted_to in ['left', 'right', 'max-peak'])
    if series_was_shifted_to == 'left':
        # values are overlayed correclty already, but they might have different lengths.
        shared_length = min(len(series1.values), len(series2.values))
        values1 = series1.values[:shared_length]
        values2 = series2.values[:shared_length]
    elif series_was_shifted_to == 'right':
        # values are not overlayed correctly, therefore take only last one and consider different lengths
        shared_length = min(len(series1.values), len(series2.values))
        values1 = series1.values[-shared_length:]
        values2 = series2.values[-shared_length:]
    elif series_was_shifted_to == 'max-peak':
        # values must be overlayed where the index is 0. To do this, we skip some elements of the vector with more
        # values before 0, which will align them. Then consider different lengths of the remaining values.
        pos0_1 = np.where(series1.index == 0)[0][0]
        pos0_2 = np.where(series2.index == 0)[0][0]
        start_idx = pos0_1 - pos0_2
        if start_idx == 0:
            skip1, skip2 = 0, 0
        elif start_idx > 0:
            skip1, skip2 = start_idx, 0
        elif start_idx < 0:
            skip1, skip2 = 0, start_idx
        shared_length = min(len(series1.index[skip1:]), len(series2.index[skip2:]))
        values1 = series1.values[skip1:][:shared_length]
        values2 = series2.values[skip2:][:shared_length]
    dist_vector = np.absolute(values1 - values2)
    return dist_vector


def populate_figure(group, ax_line, ax_norm, ax_acc, ax_acc_norm, style_line="-", style_rolling_mean='--',
                    window_size_rolling_mean=5, ):
    group.plot(label=key, style=style_line, legend=True, ax=ax_line)
    # rolling_mean_for_series(group).plot(label=key, style=style_rolling_mean, color=last_color(ax_line), ax=ax_line)

    norm_group = normalise_series(group)
    norm_group.plot(label=key, style=style_line, legend=True, ax=ax_norm)
    # rolling_mean_for_series(norm_group).plot(label=key, style=style_rolling_mean, color=last_color(ax_norm), ax=ax_norm)

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


def leftshift_series(series):
    leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    return pd.Series(data=series.values[:-1], index=leftshifted_x)


def rightshift_series(series):
    leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    rightshifted_x = [leftshifted_x[idx] - leftshifted_x[-1] for idx in range(len(leftshifted_x))]
    return pd.Series(data=group.values[:-1], index=rightshifted_x)


def maxpeakshift_series(series):
    leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    max_peak = np.argmax(series.values)
    max_peakshifted_x = [leftshifted_x[idx] - leftshifted_x[max_peak] for idx in range(len(leftshifted_x))]
    return pd.Series(data=series.values[:-1], index=max_peakshifted_x)


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

    # get data ---------------------------------------------------------------------------------------------------------
    engine = create_engine("postgres://postgres:0000@localhost/project_analyser")
    frame = pd.read_sql_query(
        'SELECT * FROM commit_frequency WHERE repository_id IN (%s) ORDER BY commit_date' % str(arg_ids)[1:-1],
        con=engine,
        index_col='commit_date')

    # setup figures and axes -------------------------------------------------------------------------------------------
    fig_dates = plt.figure(figsize=(10, 10))
    fig_leftshift = plt.figure(figsize=(10, 10))
    fig_rightshift = plt.figure(figsize=(10, 10))
    fig_max_peak = plt.figure(figsize=(10, 10))
    fig_other = plt.figure(figsize=(5, 5))
    # axes map
    ax = {
        'line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig_dates),
        'norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig_dates),
        'acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig_dates),
        'acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig_dates),
        #
        'left-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig_leftshift),
        'left-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig_leftshift),
        'left-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig_leftshift),
        'left-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig_leftshift),
        'left-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig_leftshift),
        'left-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig_leftshift),
        'left-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig_leftshift),
        'left-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig_leftshift),
        #
        'right-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig_rightshift),
        'right-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig_rightshift),
        'right-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig_rightshift),
        'right-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig_rightshift),
        'right-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig_rightshift),
        'right-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig_rightshift),
        'right-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig_rightshift),
        'right-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig_rightshift),
        #
        'max-peak-line': plt.subplot2grid((5, 3), (0, 0), colspan=2, fig=fig_max_peak),
        'max-peak-line-euclidean': plt.subplot2grid((5, 3), (0, 2), colspan=1, fig=fig_max_peak),
        'max-peak-norm': plt.subplot2grid((5, 3), (1, 0), colspan=2, fig=fig_max_peak),
        'max-peak-norm-euclidean': plt.subplot2grid((5, 3), (1, 2), colspan=1, fig=fig_max_peak),
        'max-peak-le-ge-zero': plt.subplot2grid((5, 3), (2, 0), colspan=3, fig=fig_max_peak),
        'max-peak-acc': plt.subplot2grid((5, 3), (3, 0), colspan=2, fig=fig_max_peak),
        'max-peak-acc-euclidean': plt.subplot2grid((5, 3), (3, 2), colspan=1, fig=fig_max_peak),
        'max-peak-acc-norm': plt.subplot2grid((5, 3), (4, 0), colspan=2, fig=fig_max_peak),
        'max-peak-acc-norm-euclidean': plt.subplot2grid((5, 3), (4, 2), colspan=1, fig=fig_max_peak),
        #
        'descriptions': plt.subplot2grid((2, 1), (1, 0), fig=fig_other),
    }
    # axes styling
    ax['line'].set_ylabel("frequency")
    ax['norm'].set_ylabel("norm.")
    ax['acc'].set_ylabel("acc.")
    ax['acc-norm'].set_ylabel("acc. norm.")

    #ax['line'].legend(loc=3, ncol=2, mode="expand")
    #
    ax['left-line'].set_ylabel("frequency")
    ax['left-norm'].set_ylabel("norm.")
    ax['left-acc'].set_ylabel("acc.")
    ax['left-acc-norm'].set_ylabel("acc. norm.")
    ax['left-line-euclidean'].set_ylabel("avg. distance")
    ax['left-line-euclidean'].yaxis.tick_right()
    ax['left-norm-euclidean'].set_ylabel("avg. distance")
    ax['left-norm-euclidean'].yaxis.tick_right()
    ax['left-acc-euclidean'].set_ylabel("avg. distance")
    ax['left-acc-euclidean'].yaxis.tick_right()
    ax['left-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['left-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['right-line'].set_ylabel("frequency")
    ax['right-norm'].set_ylabel("norm.")
    ax['right-acc'].set_ylabel("acc.")
    ax['right-acc-norm'].set_ylabel("acc. norm.")

    ax['right-line-euclidean'].set_ylabel("avg. distance")
    ax['right-line-euclidean'].yaxis.tick_right()
    ax['right-norm-euclidean'].set_ylabel("avg. distance")
    ax['right-norm-euclidean'].yaxis.tick_right()
    ax['right-acc-euclidean'].set_ylabel("avg. distance")
    ax['right-acc-euclidean'].yaxis.tick_right()
    ax['right-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['right-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['max-peak-line'].set_ylabel("frequency")
    ax['max-peak-norm'].set_ylabel("norm.")
    ax['max-peak-acc'].set_ylabel("acc.")
    ax['max-peak-acc-norm'].set_ylabel("acc. norm.")

    ax['max-peak-line-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-line-euclidean'].yaxis.tick_right()
    ax['max-peak-norm-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-norm-euclidean'].yaxis.tick_right()
    ax['max-peak-acc-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-acc-euclidean'].yaxis.tick_right()
    ax['max-peak-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['max-peak-le-ge-zero'].set_ylabel("portion in %")
    ax['max-peak-le-ge-zero'].set_title("Pre and post max. peak proportion of number of entries")
    #
    ax['descriptions'].set_title("Statistical description of commit frequency")
    # figure titles
    fig_dates.suptitle("Commit frequency")
    fig_leftshift.suptitle("Commit frequency (left shifted)")
    fig_rightshift.suptitle("Commit frequency (right shifted)")
    fig_max_peak.suptitle("Commit frequency (max peak shifted)")
    # plot everything --------------------------------------------------------------------------------------------------
    pid_frame = pd.DataFrame(index=frame['repository_id'].unique()).sort_index() # pid = project ID
    shifted_pid_series = {
        'date': {},
        'left': {},
        'right': {},
        'max-peak': {}
    }
    for key, group in frame.groupby('repository_id'):
        ### series in different shifted formats
        group = group.frequency.resample(arg_time_unit).sum()
        leftshifted = leftshift_series(group)
        rightshifted = rightshift_series(group)
        max_peakshifted = maxpeakshift_series(group)

        shifted_pid_series['date'][key] = group
        shifted_pid_series['left'][key] = leftshifted
        shifted_pid_series['right'][key] = rightshifted
        shifted_pid_series['max-peak'][key] = max_peakshifted
        ### plotting of figure for each format
        populate_figure(group, ax_line=ax['line'], ax_norm=ax['norm'], ax_acc=ax['acc'], ax_acc_norm=ax['acc-norm'])
        populate_figure(leftshifted, ax_line=ax['left-line'], ax_norm=ax['left-norm'], ax_acc=ax['left-acc'],
                        ax_acc_norm=ax['left-acc-norm'])
        populate_figure(rightshifted, ax_line=ax['right-line'], ax_norm=ax['right-norm'], ax_acc=ax['right-acc'],
                        ax_acc_norm=ax['right-acc-norm'])
        populate_figure(max_peakshifted, ax_line=ax['max-peak-line'], ax_norm=ax['max-peak-norm'],
                        ax_acc=ax['max-peak-acc'], ax_acc_norm=ax['max-peak-acc-norm'])

        ### percentage before and after max peak
        pid_frame.at[key, 'pre-peak-portion'] = np.multiply(
            np.true_divide(len(np.where(max_peakshifted.index < 0)[0]), len(max_peakshifted.index)), 100)
        pid_frame.at[key, 'post-peak-portion'] = np.multiply(
            np.true_divide(len(np.where(max_peakshifted.index > 0)[0]), len(max_peakshifted.index)), 100)

        ### statistical description
        des_label = ['count', 'mean', 'std', 'min', '25%', '50%', '75%', 'max']
        for idx, des in enumerate(group.describe()):
            pid_frame.loc[key, des_label[idx]] = des

    pid_frame[['pre-peak-portion', 'post-peak-portion']].plot(kind='bar', legend=True, ax=ax['max-peak-le-ge-zero'])
    pid_frame[['count', 'mean', 'std', 'min', '25%', '50%', '75%', 'max']].plot(kind='bar', legend=True,
                                                                                ax=ax['descriptions'])

    euclidean_distance(shifted_pid_series['left'], 'left').plot(kind='bar', legend=False, ax=ax['left-line-euclidean'])
    euclidean_distance(shifted_pid_series['left'], 'left', norm=True).plot(kind='bar', legend=False,
                                                                           ax=ax['left-norm-euclidean'])
    euclidean_distance(shifted_pid_series['left'], 'left', acc=True).plot(kind='bar', legend=False,
                                                                          ax=ax['left-acc-euclidean'])
    euclidean_distance(shifted_pid_series['left'], 'left', acc=True, norm=True).plot(kind='bar', legend=False,
                                                                                     ax=ax['left-acc-norm-euclidean'])

    euclidean_distance(shifted_pid_series['right'], 'right').plot(kind='bar', legend=False,
                                                                  ax=ax['right-line-euclidean'])
    euclidean_distance(shifted_pid_series['right'], 'right', norm=True).plot(kind='bar', legend=False,
                                                                             ax=ax['right-norm-euclidean'])
    euclidean_distance(shifted_pid_series['right'], 'right', acc=True).plot(kind='bar', legend=False,
                                                                            ax=ax['right-acc-euclidean'])
    euclidean_distance(shifted_pid_series['right'], 'right', acc=True, norm=True).plot(kind='bar', legend=False, ax=ax[
        'right-acc-norm-euclidean'])

    euclidean_distance(shifted_pid_series['max-peak'], 'max-peak').plot(kind='bar', legend=False,
                                                                        ax=ax['max-peak-line-euclidean'])
    euclidean_distance(shifted_pid_series['max-peak'], 'max-peak', norm=True).plot(kind='bar', legend=False,
                                                                                   ax=ax['max-peak-norm-euclidean'])
    euclidean_distance(shifted_pid_series['max-peak'], 'max-peak', acc=True).plot(kind='bar', legend=False,
                                                                                  ax=ax['max-peak-acc-euclidean'])
    euclidean_distance(shifted_pid_series['max-peak'], 'max-peak', acc=True, norm=True).plot(kind='bar', legend=False,
                                                                                             ax=ax[
                                                                                                 'max-peak-acc-norm-euclidean'])

    if not arg_hide:
        plt.show()
