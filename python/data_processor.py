"""
This file provides methods to process data. The method 'process(...)' is a wrapper that invokes several other methods
to process the data in a safe order (e.g. normalisation of values has to happen last).
"""
import numpy
from datetime import datetime, timedelta


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
