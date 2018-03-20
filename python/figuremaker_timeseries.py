import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

from plotter import leftshift_series, maxpeakshift_series

def plot_timeseries(frame, key_range, file_suffix):
    figures = []
    fig = plt.figure()
    ax1 = fig.add_subplot(111)
    figures.append(fig)

    fig = plt.figure()
    ax2 = fig.add_subplot(111)
    figures.append(fig)

    fig = plt.figure()
    ax3 = fig.add_subplot(111)
    figures.append(fig)

    project_keys = []
    for key, series in frame.groupby('repository_id'):
        if not key in key_range:
            continue
        project_keys.append(key)
        series = series.frequency.resample('W').sum()

        leftshifted = leftshift_series(series)
        maxpeakshifted = maxpeakshift_series(leftshifted, leftshifted.index)

        leftshifted.index = leftshifted.index / 7
        maxpeakshifted.index = maxpeakshifted.index / 7

        series.plot(ax=ax1, legend=False)
        leftshifted.plot(ax=ax2, legend=False)
        maxpeakshifted.plot(ax=ax3, legend=False)

    ax1.set_xlabel('week as date')
    ax2.set_xlabel('week number after start')
    ax3.set_xlabel('week number w.r.t highest peak')

    y_label = "number of commits"
    ax1.set_ylabel(y_label)
    ax2.set_ylabel(y_label)
    ax3.set_ylabel(y_label)

    ax1.legend(project_keys, loc='upper right')
    ax2.legend(project_keys, loc='upper right')
    ax3.legend(project_keys, loc='upper right')

    for fig, name in zip(figures, ['date', 'left', 'peak']):

        plt.figure(fig.number)
        plt.tight_layout()
        plt.savefig("/home/joshua/Documents/commit_frequency/timeseries_plots/ts_%s_%s.png" % (file_suffix, name))
        plt.close()


if __name__ == "__main__":
    frame = pd.read_csv("/home/joshua/Documents/commit_frequency/csv/organization.csv", index_col=0, parse_dates=True)
    plt.rc('figure', figsize=(10, 4))

    plot_timeseries(frame, range(1, 6), "1to5")
    plot_timeseries(frame, range(6, 11), "6to10")
    plot_timeseries(frame, range(11, 16), "11to15")
    plot_timeseries(frame, range(16, 21), "16to20")





