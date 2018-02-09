import matplotlib.pyplot as pyplot
import numpy


def plot_line_graph(axes, data, labels, x_as_dates):
    assert (len(data) == len(labels))
    plot_fun = axes.plot
    if x_as_dates:
        plot_fun = axes.plot_date

    for (idx, row) in enumerate(data):
        plot_fun(row[0], row[1], '-', label=labels[idx])


def plot_ydist_graph(axes, data, labels, avg_distances):
    assert (len(data) == len(labels) == len(avg_distances))
    for (idx, row) in data:
        axes.plot(row[0], row[1], '-', label="%i (%.2f%%)" % (labels[idx], avg_distances[idx] * 100))


def plot_peaks(axes, data, peaks, x_as_dates, up_color='green', down_color='red'):
    assert (len(data) == len(peaks))
    plot_fun = axes.plot
    if x_as_dates:
        plot_fun = axes.plot_date

    for (idx, row) in enumerate(data):
        ups = numpy.where(numpy.array(peaks[idx][1]) == 1)[0]
        ups_data = (numpy.array(row[0])[ups], numpy.array(row[1])[ups])
        plot_fun(ups_data[0], ups_data[1], '^', label="Up" if idx == 0 else "", color=up_color)

        downs = numpy.where(numpy.array(peaks[idx][1]) == -1)[0]
        downs_data = (numpy.array(row[0])[downs], numpy.array(row[1])[downs])
        plot_fun(downs_data[0], downs_data[1], 'v', label="Down" if idx == 0 else "", color=down_color)
