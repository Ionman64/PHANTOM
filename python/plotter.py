"""
Usage:
    plotter.py <files>... [options]

Arguments:
    files
    timeunit

Options:
    -t --time=<val>     Possible units are day, week, month, year [default: month]
    -s --show=<bool>    Show the diagram [default: true]
    -o --out=<file>     Path to output file. You can specify the file format by using the desired file extension (e.g. png, pdf)

"""
from docopt import docopt
#plot(args['files'], args['-t'], args['-s'], args['-o'])
import sys
import matplotlib.pyplot as plt
import matplotlib.dates as mpl_dates
from matplotlib.dates import YearLocator, MonthLocator, DayLocator, DateFormatter
from datetime import datetime, timedelta
import numpy as np
import csv
from schema import Regex, SchemaError


def convert_date_to_day(date):
    return date


def convert_date_to_week(date):
    return date - timedelta(days=date.weekday())


def convert_date_to_month(date):
    return date.replace(day=1)


def convert_date_to_year(date):
    return date.replace(month=1, day=1)


def read_csv(path, convert_date_fun):
    date_count = {}
    with open(path) as csvfile:
        plots = csv.reader(csvfile, delimiter=',')
        for row in plots:
            date = datetime.strptime(row[0], '%Y-%m-%d')
            date = convert_date_fun(date)
            val = int(row[1])
            date_count[date] = date_count.get(date, 0) + val

    x = date_count.keys()
    y = date_count.values()
    return sort_by_x(x, y)


def sort_by_x(x, y):
    order = np.argsort(x)
    sorted_x = np.array(x)[order]
    sorted_y = np.array(y)[order]
    return sorted_x, sorted_y


def plot_from_csv(handle, path, convert_date_fun, fmt='-', label=None):
    x, y = read_csv(path, convert_date_fun)
    handle.plot_date(x, y, fmt, label=label)


def plot(files, time_unit, show, output_file):
    years = YearLocator()
    months = MonthLocator()
    days = DayLocator()
    yearsFmt = mpl_dates.DateFormatter('%Y')

    fig, ax = plt.subplots()
    convert_date_fun_options = {
        'day': convert_date_to_day,
        'week': convert_date_to_week,
        'month': convert_date_to_month,
        'year': convert_date_to_year,
    }

    for file in files:
        plot_from_csv(
            handle=ax,
            path=file,
            convert_date_fun=convert_date_fun_options[time_unit],
            label=file)

    # format settings
    # tick label
    ax.xaxis.set_major_locator(years)
    ax.xaxis.set_major_formatter(yearsFmt)
    ax.xaxis.set_minor_locator(months)
    #ax.xaxis.set_minor_formatter(monthsFmt)

    ax.autoscale_view()

    plt.title('Number of commits over time')
    fig.autofmt_xdate()
    plt.legend(loc='upper left')

    if show:
        plt.show()
    if not output_file == None:
        plt.savefig(output_file)


if __name__ == '__main__':
    args = docopt(__doc__)
    try:
        Regex('day|week|month|year').validate(args['--time'])
        Regex('true|false|True|False|yes|no').validate(args['--show'])
    except SchemaError as e:
        print("Invalid argument:")
        exit(e)

    plot(args['<files>'], args['--time'],
         (args['--show'] in ('true', 'True', 'yes')), args['--out'])
