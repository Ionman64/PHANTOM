"""
This file provides methods to process data. The method 'process(...)' is a wrapper that invokes several other methods
to process the data in a safe order (e.g. normalisation of values has to happen last).
"""
import numpy
from datetime import datetime, timedelta
from subprocess import check_output


def process(data_array, accumulate=False, normalise=False, shift=None):
    """ Takes an array of (datetime, value)-tuple-arrays and iterates over each tuple-array. Then it converts the data
    according to the specified function arguments in a safe order.

    :param data_array:    Data in the form [ ([dates], [values]), ([dates], [values]), ... ]
    :param accumulate:    Accumulates the values of each tuple
    :param normalise:     Normalises the values of each tuple
    :param shift:         Shifts the dates of each tuple. [possible values: "left", "right"]
    :returns: The processed data array as an array of tuples [(x[], y[]), (x[], y[]), ...]
    """
    for (idx, data) in enumerate(data_array):
        x = data[0]
        y = data[1]
        if accumulate:
            y = accumulate_array(y)
        if normalise:
            y = normalise_array(y)
        shift_functions = {
            "left": lambda x, y: shift_dates_left(x),
            "right": lambda x, y: shift_dates_right(x),
            "peak": lambda x, y: shift_dates_peak(shift_dates_left(x), y),
        }
        if shift in shift_functions:
            x = shift_functions[shift](x, y)
        data_array[idx] = (x, y)
    return data_array


def accumulate_array(y):
    return numpy.add.accumulate(y)


def normalise_array(y):
    y_max = numpy.amax(y)
    return numpy.true_divide(y, y_max)


def shift_dates_peak(x, y):
    y_max_idx = numpy.argmax(y)
    x_val_at_y_max = x[y_max_idx]
    return [(val - x_val_at_y_max) for val in x]


def shift_dates_left(x):
    """ Takes an array of sorted datetimes and returns an array of integers. The integers represent the number of
    days that have passed since the first """
    return [(val - x[0]).days for val in x]


def shift_dates_right(x):
    """
    Takes an array of sorted datetimes and returns an array of integers. The integers represent the number of
    days that have passed since the last date. That means, the returned array ranges from negative integers to 0."""
    return [(val - x[-1]).days for val in x]


def find_peaks(data):
    peaks = []
    for row in data:
        output = check_output(["../target/debug/utils", "--findpeaks"] + map(str, row[1]))
        # TODO Use path seperators to ensure compatibility
        output = map(int, output[1:-1].split(','))
        peaks.append((row[0], output))
    return peaks


def get_euclidean(data):
    """
    Calculates the euclidean distance between the first element and every other element. The data points are
    analysed in the order they appear, not by their actual x value.

    :param data: The first element in data will be the reference for the comparison of euclidean distance
    :return: Return a array of distance-arrays with the same length is the input. The first element will be the distance
    to the reference point, so always be only 0.
    """
    ref = numpy.column_stack(data[0])
    x_values = data[0][0]
    distances = [(x_values, numpy.zeros(len(ref)))]
    for row in data[1:]:
        comp = numpy.column_stack(row)
        m = numpy.minimum(len(ref), len(comp))
        dist = numpy.zeros(m)
        for idx in range(0, m):
            dist[idx] = (abs(comp[idx][1] - ref[idx][1]))
        distances.append((x_values[0:m], numpy.array(dist)))
    return numpy.array(distances)
