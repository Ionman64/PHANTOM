import pandas as pd
import numpy as np
import matplotlib.pyplot as plt

from plotter import leftshift_series, maxpeakshift_series

def plot_timeseries(frame, key_range, file_suffix):
    figures = []

    fig = plt.figure(figsize=(5, 5))
    ax2 = fig.add_subplot(111)
    figures.append(fig)

    #fig = plt.figure()
    #ax3 = fig.add_subplot(111)
    #figures.append(fig)

    project_keys = []
    for key, series in frame.groupby('repository_id'):
        if not key in key_range:
            continue
        project_keys.append(key)
        series = series.frequency.resample('Q').sum()

        print "%s " % key, series.max()

        leftshifted = leftshift_series(series)
        maxpeakshifted = maxpeakshift_series(leftshifted, leftshifted.index)

        leftshifted.index = leftshifted.index / 7
        maxpeakshifted.index = maxpeakshifted.index / 7

        leftshifted.plot(ax=ax2, legend=False)
        #maxpeakshifted.plot(ax=ax3, legend=False)


    ax2.set_xlabel('week number after start')
    #ax3.set_xlabel('week number w.r.t highest peak')

    y_label = "number of commits"
    ax2.set_ylabel(y_label)
    #ax3.set_ylabel(y_label)

    ax2.legend(project_keys, loc='upper right')
    #ax3.legend(project_keys, loc='upper right')

    def set_xticks(ax):
        start, end = ax.get_xlim()
        ax.xaxis.set_ticks(np.arange(0, int(end), 50))
        ax.xaxis.set_ticks(np.arange(0, int(end), 10), minor=True)

    set_xticks(ax2)
    #set_yticks(ax2)
    #set_xticks(ax3)

    #for fig, name in zip(figures, ['date', 'left', 'peak']):
        #plt.figure(fig.number)
    plt.tight_layout()
    #plt.savefig("/home/joshua/Documents/commit_frequency/timeseries_plots/ts_%s_%s.png" % (file_suffix, "left"))
    #plt.close()
    plt.show()

if __name__ == "__main__":
    frame = pd.read_csv("/home/joshua/Documents/commit_frequency/csv/organization.csv", index_col=0, parse_dates=True)
    plt.rc('figure', figsize=(10, 4))

    plot_timeseries(frame, [3, 5], "3 and 6")
    #plot_timeseries(frame, [10], "10")
    #plot_timeseries(frame, [3, 5, 6, 2, 9], "3-5-6-2-9")
    #plot_timeseries(frame, [16, 7, 20], "16-7-20")
    #plot_timeseries(frame, [8, 4, 11, 15], "8-4-11-15")
    #plot_timeseries(frame, [14, 18, 19], "14-18-19")
    #plot_timeseries(frame, [1, 12, 17], "1-12-17")





