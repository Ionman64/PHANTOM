"""
Usage:
    plotter_v2.py <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT> Resample the data as by day, week, month, quarter, year(annual)[values: D, W, M, Q, A]
Options:
    -h --help           Show this screen.
    --peak              Highlight peaks
    --rollingmean       Use a rolling mean instead of the actual values
    --window=<size>     Override the default window size for the rolling mean. [values: size > 0]
    --hide              Hide the figures
    -o --out=<file>     Path to output file. (e.g. fig.png, fig.pdf)

"""
from docopt import docopt
from utils import data_processor as processor, data_provider as provider, data_visualiser as visualiser
import numpy as np
import matplotlib.pyplot as plt
import pandas as pd
from sqlalchemy import create_engine
from subprocess import check_output


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


def peak_analysis(pid_series, path_to_utils_binary="../target/debug/utils", utils_binary_flags=["--findpeaks"], col_name_values='values', col_name_peaks='peaks'):
    pid_value_peak_frame = {}
    for key in pid_series: # TODO optimise by passing multiple series values at the same time
        series = pid_series[key]
        output = check_output([path_to_utils_binary] + utils_binary_flags + map(str, series.values))
        output_as_array = map(int, output[1:-1].split(','))
        df = pd.DataFrame(data={col_name_values: series.values, col_name_peaks:output_as_array}, index=series.index)
        pid_value_peak_frame[key] = df
        print df
    return pid_value_peak_frame

def populate_figure(series, ax_line, ax_norm, ax_acc, ax_acc_norm,
                    style_line="-", style_rolling_mean='--',
                    window_size_rolling_mean=5, ):
    ax = series.plot(label=key, style=style_line, ax=ax_line)
    # rolling_mean_for_series(group).plot(label=key, style=style_rolling_mean, color=last_color(ax_line), ax=ax_line)

    norm_group = normalise_series(series)
    norm_group.plot(label=key, style=style_line, ax=ax_norm)
    # rolling_mean_for_series(norm_group).plot(label=key, style=style_rolling_mean, color=last_color(ax_norm), ax=ax_norm)

    acc_group = accumulate_series(series)
    acc_group.plot(label=key, style=style_line, ax=ax_acc)

    norm_acc_group = normalise_series(accumulate_series(series))
    norm_acc_group.plot(label=key, style=style_line, ax=ax_acc_norm)

    # lines, labels = ax.get_legend_handles_labels()
    # ax.legend(loc="lower center")


def populate_figure_with_euclidean(pid_series, series_was_shifted_to,
                                   ax_line_euclidean, ax_norm_euclidean, ax_acc_euclidean, ax_acc_norm_euclidean):
    euclidean_distance(pid_series, series_was_shifted_to).plot(kind='bar', legend=False, ax=ax_line_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, norm=True).plot(kind='bar', legend=False,
                                                                          ax=ax_norm_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, acc=True).plot(kind='bar', legend=False, ax=ax_acc_euclidean)
    euclidean_distance(pid_series, series_was_shifted_to, acc=True, norm=True).plot(kind='bar', legend=False,
                                                                                    ax=ax_acc_norm_euclidean)


def last_color(axes):
    return axes.lines[-1].get_color()


def rolling_mean_for_series(series, window_size=5):
    return series.rolling(window=window_size).mean()[window_size - 1:]


def accumulate_series(series):
    return series.agg(np.add.accumulate)


def normalise_series(series):
    return (series - series.min()) / (series.max() - series.min())


def leftshift_series(series, return_shifted_x=False):
    leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    if not return_shifted_x:
        return pd.Series(data=series.values[:-1], index=leftshifted_x)
    else:
        return pd.Series(data=series.values[:-1], index=leftshifted_x), leftshifted_x


def rightshift_series(series, leftshifted_x=None):
    if leftshifted_x == None:
        leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    rightshifted_x = [leftshifted_x[idx] - leftshifted_x[-1] for idx in range(len(leftshifted_x))]
    return pd.Series(data=group.values[:-1], index=rightshifted_x)


def maxpeakshift_series(series, leftshifted_x=None):
    if leftshifted_x == None:
        leftshifted_x = [(series.index[idx] - series.index[0]).days for idx in range(len(series.index[:-1]))]
    max_peak = np.argmax(series.values)
    max_peakshifted_x = [leftshifted_x[idx] - leftshifted_x[max_peak] for idx in range(len(leftshifted_x))]
    return pd.Series(data=series.values[:-1], index=max_peakshifted_x)


if __name__ == '__main__':
    args = docopt(__doc__)

    arg_ids = args['<id>']
    arg_time_unit = args['--timeunit'].upper()
    # arg_acc = args['--acc']
    # arg_norm = args['--norm']
    # arg_shift = args['--shift']
    arg_out_file = args['--out']
    arg_hide = args['--hide']
    # arg_ydist = args['--ydist']
    # arg_peak = args['--peak']
    arg_rollingmean = args['--rollingmean']
    arg_window = -1 if args['--window'] == None else int(args['--window'])

    if arg_rollingmean and arg_window == -1:
        arg_window = {"D": 14, "W": 4, "M": 3, "Q": 4, "A": 2}[arg_time_unit]
        print "No window for rolling mean specified. Default window size for time unit is %s." % arg_window
    # Validate command line arguments ----------------------------------------------------------------------------------
    valid_timeunits = ["D", "W", "M", "Q", "A"]
    if not arg_time_unit in valid_timeunits:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)
    if arg_rollingmean and arg_window <= 0:
        print "Invalid window size. Use --help to get more information."
        exit(1)
    # get data ---------------------------------------------------------------------------------------------------------
    engine = create_engine("postgres://postgres:0000@localhost/project_analyser")
    frame = pd.read_sql_query(
        'SELECT * FROM commit_frequency WHERE repository_id IN (%s) ORDER BY commit_date' % str(arg_ids)[1:-1],
        con=engine,
        index_col='commit_date')

    # setup figures and axes -------------------------------------------------------------------------------------------
    fig = {
        'date': plt.figure(figsize=(10, 10)),
        'left': plt.figure(figsize=(10, 10)),
        'right': plt.figure(figsize=(10, 10)),
        'max-peak': plt.figure(figsize=(10, 10)),
        'other': plt.figure(figsize=(5, 5)),
    }
    # axes map
    ax = {
        'date-line': plt.subplot2grid((4, 3), (0, 0), colspan=3, fig=fig['date']),
        'date-norm': plt.subplot2grid((4, 3), (1, 0), colspan=3, fig=fig['date']),
        'date-acc': plt.subplot2grid((4, 3), (2, 0), colspan=3, fig=fig['date']),
        'date-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=3, fig=fig['date']),
        #
        'left-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig['left']),
        'left-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig['left']),
        'left-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig['left']),
        'left-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig['left']),
        'left-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig['left']),
        'left-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig['left']),
        'left-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig['left']),
        'left-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig['left']),
        #
        'right-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig['right']),
        'right-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig['right']),
        'right-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig['right']),
        'right-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig['right']),
        'right-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig['right']),
        'right-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig['right']),
        'right-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig['right']),
        'right-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig['right']),
        #
        'max-peak-line': plt.subplot2grid((5, 3), (0, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-line-euclidean': plt.subplot2grid((5, 3), (0, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-norm': plt.subplot2grid((5, 3), (1, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-norm-euclidean': plt.subplot2grid((5, 3), (1, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-le-ge-zero': plt.subplot2grid((5, 3), (2, 0), colspan=3, fig=fig['max-peak']),
        'max-peak-acc': plt.subplot2grid((5, 3), (3, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-euclidean': plt.subplot2grid((5, 3), (3, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-acc-norm': plt.subplot2grid((5, 3), (4, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-norm-euclidean': plt.subplot2grid((5, 3), (4, 2), colspan=1, fig=fig['max-peak']),
        #
        'descriptions': plt.subplot2grid((1, 1), (0, 0), fig=fig['other']),
    }
    # axes styling
    ax['date-line'].set_ylabel("frequency")
    ax['date-norm'].set_ylabel("norm.")
    ax['date-acc'].set_ylabel("acc.")
    ax['date-acc-norm'].set_ylabel("acc. norm.")
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
    ax['max-peak-le-ge-zero'].set_xlabel("project ID")
    #
    # figure titles
    grouped_by = {"D": "day", "W": "week", "M": "month", "Q": "quarter", "A": "year"}[arg_time_unit]

    fig['date'].suptitle("Commit frequency grouped by %s" % grouped_by)
    fig['left'].suptitle("Commit frequency (left shifted) grouped by %s" % grouped_by)
    fig['right'].suptitle("Commit frequency (right shifted) grouped by %s" % grouped_by)
    fig['max-peak'].suptitle("Commit frequency (max peak shifted) grouped by %s" % grouped_by)
    fig['other'].suptitle("Stat. description of commit frequency grouped by %s" % grouped_by)
    # plot everything --------------------------------------------------------------------------------------------------
    pid_frame = pd.DataFrame(
        index=frame['repository_id'].unique()).sort_index()  # This frame stores information about projects
    shifted_pid_series = {
        # map to store multiple formats for project id series
        # example: shifted_pid_series['right'][2] stores the series for project 2 in a right-shifted format
        'date': {},
        'left': {},
        'right': {},
        'max-peak': {}
    }
    for key, group in frame.groupby('repository_id'):
        ### transform and store series in different shifted formats
        group = group.frequency.resample(arg_time_unit).sum()
        if arg_rollingmean:
            group = rolling_mean_for_series(group, arg_window)

        leftshifted, leftshifted_x = leftshift_series(group, return_shifted_x=True)
        rightshifted = rightshift_series(group, leftshifted_x)
        max_peakshifted = maxpeakshift_series(group, leftshifted_x)

        shifted_pid_series['date'][key] = group
        shifted_pid_series['left'][key] = leftshifted
        shifted_pid_series['right'][key] = rightshifted
        shifted_pid_series['max-peak'][key] = max_peakshifted
        ### plotting of figure for each format
        populate_figure(group, ax_line=ax['date-line'], ax_norm=ax['date-norm'], ax_acc=ax['date-acc'], ax_acc_norm=ax['date-acc-norm'])
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

    ### portion before and after the maximum peak
    portion_ax = pid_frame[['pre-peak-portion', 'post-peak-portion']].plot(kind='bar', legend=False,
                                                                           ax=ax['max-peak-le-ge-zero'])

    for prefix in ['date', 'left', 'right', 'max-peak']:
        pid_value_peak_frame = peak_analysis(shifted_pid_series['date'])
        for key in pid_value_peak_frame:
            df = pid_value_peak_frame[key]
            up_series = df[df['peaks'] == 1]['values']
            down_series = df[df['peaks'] == -1]['values']

            up_series.plot(ax=ax['date-line'], style='^g')
            down_series.plot(ax=ax['date-line'], style='vr',)


    # annotate bars with value
    # for patch in portion_ax.patches:
    #    ax['max-peak-le-ge-zero'].annotate("%2.f" % np.round(patch.get_height(), decimals=2), (patch.get_x() + patch.get_width()/2, patch.get_height() - 10), rotation=90)
    ### statistical description
    pid_frame[['count', 'mean', 'std', 'min', '25%', '50%', '75%', 'max']].plot(kind='bar', legend=False,
                                                                                ax=ax['descriptions'])
    ### euclidean distance
    for prefix in ['left', 'right', 'max-peak']:
        populate_figure_with_euclidean(pid_series=shifted_pid_series[prefix],
                                       series_was_shifted_to=prefix,
                                       ax_line_euclidean=ax['%s-line-euclidean' % prefix],
                                       ax_norm_euclidean=ax['%s-norm-euclidean' % prefix],
                                       ax_acc_euclidean=ax['%s-acc-euclidean' % prefix],
                                       ax_acc_norm_euclidean=ax['%s-acc-norm-euclidean' % prefix])

    # as every figure has multiple axes with the same lines (i.e. the projects/ids) a one legend is drawed manually,
    # instead of having multiple legends with the same content in each axes
    for key in fig.keys():
        if key == 'other':
            fig[key].legend(bbox_to_anchor=[0, 0], loc='lower left', ncol=4)
        elif key in ['date', 'left', 'right', 'max-peak']:
            lines = fig[key].axes[0].lines
            fig[key].legend(lines, arg_ids, bbox_to_anchor=[0, 0], loc='lower left', ncol=min(len(arg_ids), 8))
        else:
            print "[Warning] No custom legend for figure key '%s'." % key

    patches, labels = ax['max-peak-le-ge-zero'].get_legend_handles_labels()
    ax['max-peak-le-ge-zero'].legend(patches, ['before', 'after'], ncol=2, loc='best', title="Commit portion max peak")

    ax['descriptions'].set_xticklabels(arg_ids, rotation=0)

    if not arg_hide:
        plt.show()
