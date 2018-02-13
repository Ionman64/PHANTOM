"""
Usage:
    plotter_v2.py <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT> Resample the data as by day, week, month, quarter, year(annual)[values: D, W, M, Q, A]
Options:
    -h --help           Show this screen.
    --mark-peaks         Highlight peaks
    --rollingmean       Use a rolling mean instead of the actual values
    --window=<size>     Override the default window size for the rolling mean. [values: size > 0]
    --hide              Hide the figures
    -o --out=<file>     Path to output file. (e.g. fig.png, fig.pdf)

"""
from utils.pyplot_styler import get_fig_and_ax_map, post_plot_figure_style, post_plot_axes_style
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from docopt import docopt
from sqlalchemy import create_engine
from subprocess import check_output


def euclidean_distance(pid_series, series_was_shifted_to, norm=False, acc=False):
    """
    Calculate the distance matrix between each pair of series in the specified map.

    :param pid_series: Map of project ids to pandas series
    :param series_was_shifted_to: Specify the way the series was shifted
    :param norm: If true, the series values will be normalised
    :param acc: If true, the series values will be accumulated
    :return: Pandas dataframe with index equal to pid_series keys and column names as well. Cells contain the average
        distance between two projects (intersect index and column name)
    """
    pids = pid_series.keys()
    pids.sort()
    series = pid_series.values()
    if acc:
        series = [accumulate_series(s) for s in series]
    if norm:
        series = [normalise_series(s) for s in series]

    N = len(pids)
    dist_matrix = np.zeros((N, N))
    # max_distance = 0
    for i in range(N):
        for j in range(N):
            if i <= j:
                continue
            dist_vector = __get_euclidean__(series[i], series[j], series_was_shifted_to)
            # max_distance = max(max_distance, np.max(dist_vector))
            eucl = np.average(dist_vector)
            dist_matrix[i, j] = eucl
            dist_matrix[j, i] = eucl

    # dist_matrix = np.round(np.true_divide(dist_matrix, max_distance), decimals=round_to_decimals)
    df = pd.DataFrame(dist_matrix, index=pids, columns=pids)
    return df


def __get_euclidean__(series1, series2, series_was_shifted_to):
    """
    Calculates the euclidean distance between the values of the two series. Compares points in order, not by similarity
    on the x-axis (i.e. index).

    :param series1: Pandas series
    :param series2: Pandas series
    :param series_was_shifted_to: Determines how to overlay two series, with different lengths and different points of
        interest
    :return: Vector of distances between points
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


def peak_analysis(series, path_to_utils_binary="../target/debug/utils", utils_binary_flags=["--findpeaks"]):
    """
    Calculates the peek positions of a series.

    :param series: Pandas series
    :param path_to_utils_binary: Path to the rust program to calculate peaks
    :param utils_binary_flags: Flags to specify when calling the rust program
    :return: Pandas series indexed with range(0...length) and values are in [-1, 0, 1] indicating
        [peak down, no peak, peak up] respectively
    """
    # TODO optimise by passing multiple series values at the same time
    output = check_output([path_to_utils_binary] + utils_binary_flags + map(str, series.values))
    output_as_array = map(int, output[1:-1].split(','))
    peak_series = pd.Series(data=output_as_array)
    return peak_series


def populate_figure_with_standard_plots(series, ax_line, ax_norm, ax_acc, ax_acc_norm,
                                        style_line="-", peak_series=None, style_peak_up='^g', style_peak_down='vr'):
    """
    Plots the series in several formats on the specified axes.

    :param series: Pandas series
    :param ax_line: Pyplot axes to plot the series
    :param ax_norm: Pyplot axes to plot the normalised series
    :param ax_acc: Pyplot axes to plot the accumulated series
    :param ax_acc_norm: Pyplot axes to plot the accumulated, normalised series
    :param style_line: Style for plotting in the specified axes
    :param peak_series: Pandas series with peak positions of parameter 'series'
    :param style_peak_up: Style used for plotting upward peaks
    :param style_peak_down: Style used for plotting downward peaks
    """
    # line
    ax = series.plot(label=key, style=style_line, ax=ax_line)
    # norm
    norm_series = normalise_series(series)
    norm_series.plot(label=key, style=style_line, ax=ax_norm)
    # acc
    acc_series = accumulate_series(series)
    acc_series.plot(label=key, style=style_line, ax=ax_acc)
    # acc norm
    acc_norm_series = normalise_series(accumulate_series(series))
    acc_norm_series.plot(label=key, style=style_line, ax=ax_acc_norm)
    # peak highlighting
    if not type(peak_series) == type(None):
        peak_up_idx = peak_series.values == 1
        peak_down_idx = peak_series.values == -1
        for (shifted_series, shifted_ax) in [(series, ax_line), (norm_series, ax_norm)]:
            shifted_series[peak_up_idx].plot(ax=shifted_ax, style=style_peak_up)
            shifted_series[peak_down_idx].plot(ax=shifted_ax, style=style_peak_down, )


def populate_figure_with_euclidean(pid_series, series_was_shifted_to, ax_line_euclidean, ax_norm_euclidean,
                                   ax_acc_euclidean, ax_acc_norm_euclidean):
    """
    Plots the euclidean distances of different formats on the specified axes.

    :param pid_series: Map of project ids to pandas series
    :param series_was_shifted_to: Specify the way the series was shifted
    :param ax_line_euclidean: Pyplot axes for distance on series
    :param ax_norm_euclidean: Pyplot axes for distance on normalised series
    :param ax_acc_euclidean: Pyplot axes for distance on accumulated series
    :param ax_acc_norm_euclidean:  Pyplot axes for distance on accumulated, normalised series
    """
    euclidean_distance(pid_series, series_was_shifted_to).plot(kind='bar', legend=False, ax=ax_line_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, norm=True).plot(kind='bar', legend=False,
                                                                          ax=ax_norm_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, acc=True).plot(kind='bar', legend=False, ax=ax_acc_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, acc=True, norm=True).plot(kind='bar', legend=False,
                                                                                    ax=ax_acc_norm_euclidean)


def populate_axes_with_euclidean(pid_series, series_was_shifted_to, axes):
    """
    Plots the distance of the series on the specified axes.

    :param pid_series: Map of project ids to pandas series
    :param series_was_shifted_to: Specify the way the series was shifted
    :param axes Pyplot axes to plot the distance
    """
    euclidean_distance(pid_series, series_was_shifted_to).plot(kind='bar', legend=False, ax=axes)


def last_color(axes):
    """
    Determines the last used color of the axes.

    :param axes: Pyplot axes
    :return: Most recent line color
    """
    return axes.lines[-1].get_color()


def rolling_mean_for_series(series, window_size=5):
    return series.rolling(window=window_size).mean()[window_size - 1:]


def accumulate_series(series):
    return series.agg(np.add.accumulate)


def normalise_series(series):
    return (series - series.min()) / (series.max() - series.min())


def leftshift_series(series):
    """
    Transforms the series with a datetime index to a series with an integer index. The index represents the number of
    days that passed since the first day.

    :param series: Pandas series with a datetime index
    :return: Pandas series with the same values but leftshifted index
    """
    leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index))]
    return pd.Series(data=series.values, index=leftshifted_x)


def rightshift_series(series, leftshifted_idx):
    """
    Transforms the series with a datetime index to a series with an integer index. The index represents the number of
    days that passed since the last day. Therefore, all indices but the last will be negative.
    :param series: Pandas series
    :param leftshifted_idx: Numpy array of the leftshifted index of the specified series (note: take the index from leftshift_series())
    :return: Pandas series with the same values but rightshifted index
    """
    rightshifted_x = [leftshifted_idx[idx] - leftshifted_idx[-1] for idx in range(len(leftshifted_idx))]
    return pd.Series(data=series.values, index=rightshifted_x)


def maxpeakshift_series(series, leftshifted_idx):
    """
    Transforms the series with a datetime index to a series with an integer index. The index represents the number of
    days that passed since the maximum value. Therefore, there will be an index equal to 0 (max peak) and the remaining
    indices are either negative (pre-peak) or positive (post-peak).

    :param series: Pandas series
    :param leftshifted_idx: Numpy array of the leftshifted index of the specified series (note: take the index from leftshift_series())
    :return: Pandas series with the same values but max-peak-shifted index
    """
    max_peak = np.argmax(series.values)
    max_peakshifted_x = [leftshifted_idx[idx] - leftshifted_idx[max_peak] for idx in range(len(leftshifted_idx))]
    return pd.Series(data=series.values, index=max_peakshifted_x)


if __name__ == '__main__':
    # setup command line arguments -------------------------------------------------------------------------------------
    args = docopt(__doc__)
    arg_ids = args['<id>']
    arg_time_unit = args['--timeunit'].upper()
    arg_out_file = args['--out']
    arg_hide = args['--hide']
    arg_mark_peaks = args['--mark-peaks']
    arg_rollingmean = args['--rollingmean']
    arg_window = -1 if args['--window'] == None else int(args['--window'])
    # Validate command line arguments ----------------------------------------------------------------------------------
    if arg_rollingmean and arg_window == -1:
        arg_window = {"D": 14, "W": 4, "M": 3, "Q": 4, "A": 2}[arg_time_unit]
        print "No window for rolling mean specified. Default window size for time unit is %s." % arg_window
    valid_timeunits = ["D", "W", "M", "Q", "A"]
    if not arg_time_unit in valid_timeunits:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)
    if arg_rollingmean and arg_window <= 0:
        print "Invalid window size. Use --help to get more information."
        exit(1)
    # get data from database -------------------------------------------------------------------------------------------
    engine = create_engine("postgres://postgres:new@localhost/project_analyser")
    frame = pd.read_sql_query(
        "SELECT repository_id, commit_date::DATE as commit_date, COUNT(commit_date::DATE) as frequency FROM repository_commit WHERE repository_id in (%s) GROUP BY commit_date::DATE, repository_id;" % str(arg_ids)[1:-1],
        con=engine,
        index_col='commit_date',
        parse_dates="commit_date")
    print (frame)
    # setup figures and axes -------------------------------------------------------------------------------------------
    fig, ax = get_fig_and_ax_map(arg_time_unit, arg_rollingmean, arg_window)
    # setup data structures to store information -----------------------------------------------------------------------
    ### pid_frame
    # is indexed by the repository id. Add columns to it to save repository-based information
    pid_frame = pd.DataFrame(index=frame['repository_id'].unique()).sort_index()
    ### shifted_pid_series
    # stores multiple formats for project id series
    # example: shifted_pid_series['right'][2] stores the series for project 2 in a right-shifted format
    shifted_pid_series = {
        'date': {},
        'left': {},
        'right': {},
        'max-peak': {}
    }
    ### peak_pid_series
    # stores the time between peak analysis for projects
    # example: peak_pid_series['time-between-all-peaks'][2] stores the series of the analyis for project 2 of time between all peaks
    peak_pid_series = {
        'time-between-all-peaks': {},
        'time-between-up-peaks': {},
        'time-between-down-peaks': {},
    }
    # plot everything --------------------------------------------------------------------------------------------------
    for key, series in frame.groupby('repository_id'):
        ### transform and store series in different shifted formats ----------------------------------------------------
        # depending on the selected time unit the data are resampled. Data in the same resampled bin are summed up.
        series = series.frequency.resample(arg_time_unit).sum()
        if arg_rollingmean:
            series = rolling_mean_for_series(series, arg_window)

        leftshifted = leftshift_series(series)
        rightshifted = rightshift_series(series, leftshifted.index)
        max_peakshifted = maxpeakshift_series(series, leftshifted.index)

        shifted_pid_series['date'][key] = series
        shifted_pid_series['left'][key] = leftshifted
        shifted_pid_series['right'][key] = rightshifted
        shifted_pid_series['max-peak'][key] = max_peakshifted
        ### plotting of standard figure for each format ----------------------------------------------------------------
        peaks = peak_analysis(series)
        populate_figure_with_standard_plots(series, peak_series=peaks if arg_mark_peaks else None,
                                            ax_line=ax['date-line'], ax_norm=ax['date-norm'],
                                            ax_acc=ax['date-acc'], ax_acc_norm=ax['date-acc-norm'], )
        populate_figure_with_standard_plots(leftshifted, peak_series=peaks if arg_mark_peaks else None,
                                            ax_line=ax['left-line'], ax_norm=ax['left-norm'],
                                            ax_acc=ax['left-acc'], ax_acc_norm=ax['left-acc-norm'])
        populate_figure_with_standard_plots(rightshifted, peak_series=peaks if arg_mark_peaks else None,
                                            ax_line=ax['right-line'], ax_norm=ax['right-norm'],
                                            ax_acc=ax['right-acc'], ax_acc_norm=ax['right-acc-norm'])
        populate_figure_with_standard_plots(max_peakshifted, peak_series=peaks if arg_mark_peaks else None,
                                            ax_line=ax['max-peak-line'], ax_norm=ax['max-peak-norm'],
                                            ax_acc=ax['max-peak-acc'], ax_acc_norm=ax['max-peak-acc-norm'])
        ### time between peaks -----------------------------------------------------------------------------------------
        for (peak_condition, tbp_axes_key) in [
            (peaks.values == 1, 'time-between-up-peaks'),
            (peaks.values == -1, 'time-between-down-peaks'),
            (peaks.values != 0, 'time-between-all-peaks'),
        ]:
            peak_times = leftshifted[peak_condition].index
            time_between_peaks = [peak_times[0]] + [peak_times[idx] - peak_times[idx - 1] for idx in
                                                    range(1, len(peak_times))]
            peak_series = pd.Series(data=time_between_peaks)
            peak_series.plot(ax=ax[tbp_axes_key])
            peak_pid_series[tbp_axes_key][key] = peak_series

        ### percentage before and after max peak -----------------------------------------------------------------------
        len_mps = len(max_peakshifted.index)
        get_portion = lambda length: np.multiply(np.true_divide(length, len_mps), 100)
        pid_frame.at[key, 'pre-peak-portion'] = get_portion(len(np.where(max_peakshifted.index < 0)[0]))
        pid_frame.at[key, 'post-peak-portion'] = get_portion(len(np.where(max_peakshifted.index > 0)[0]))

    ### portion before and after the maximum peak ----------------------------------------------------------------------
    portion_ax = pid_frame[['pre-peak-portion', 'post-peak-portion']].plot(kind='bar', legend=False,
                                                                           ax=ax['max-peak-le-ge-zero'])

    ### euclidean distance ---------------------------------------------------------------------------------------------
    # for normal plots
    for prefix in ['left', 'right', 'max-peak']:
        populate_figure_with_euclidean(pid_series=shifted_pid_series[prefix],
                                       series_was_shifted_to=prefix,
                                       ax_line_euclidean=ax['%s-line-euclidean' % prefix],
                                       ax_norm_euclidean=ax['%s-norm-euclidean' % prefix],
                                       ax_acc_euclidean=ax['%s-acc-euclidean' % prefix],
                                       ax_acc_norm_euclidean=ax['%s-acc-norm-euclidean' % prefix])
    # for time between peak plots
    for key in peak_pid_series:
        pid_series = peak_pid_series[key]
        axes = ax["%s-euclidean" % key]
        populate_axes_with_euclidean(pid_series, 'left', axes)

    # ------------------------------------------------------------------------------------------------------------------
    post_plot_figure_style(fig, arg_ids)
    post_plot_axes_style(ax, arg_ids, silent=True)

    if not arg_hide:
        plt.show()
