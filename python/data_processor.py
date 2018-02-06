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

    @data_array:    Data in the form [ ([dates], [values]), ([dates], [values]), ... ]

    @accumulate:    Accumulates the values of each tuple

    @normalise:     Normalises the values of each tuple

    @shift:         Shifts the dates of each tuple. [possible values: "left", "right"]
    """
    for (idx, data) in enumerate(data_array):
        x = data[0]
        y = data[1]
        if accumulate:
            y = accumulate_array(y)
        if normalise:
            y = normalise_array(y)
        shift_functions = {
            "left": shift_dates_left,
            "right": shift_dates_right,
        }
        if shift in shift_functions:
            x = shift_functions[shift](x)
        data_array[idx] = (x, y)
    return data_array


def accumulate_array(y):
    return numpy.add.accumulate(y)


def normalise_array(y):
    y_max = numpy.amax(y)
    return numpy.true_divide(y, y_max)


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
    ref = (data[0][0], data[0][1])
    distances = []
    for row in data[1:]:
        comp = numpy.array((row[0], row[1]))
        dist = numpy.absolute(numpy.copy(ref[:][1]))
        idx_same_x = numpy.where(ref[:][0] == comp[:][0])
        ref_y = ref[:][1][idx_same_x]
        comp_y = comp[:][1][idx_same_x]
        dist[idx_same_x] = numpy.absolute(numpy.subtract(ref_y, comp_y))
        distances.append(dist)
    return ref[:][0], distances

