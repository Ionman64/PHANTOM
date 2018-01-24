import os
import matplotlib.pyplot as plt
from DataHandler import *
import time
import datetime
import numpy as np
import matplotlib.dates as mdates
import matplotlib.cbook as cbook

def convert_date_to_time(date_string, format_string):
    return int(time.mktime(datetime.datetime.strptime(date_string, format_string).timetuple()))

def read_file():
    DATE_FORMAT = "%Y-%m-%d"
    gap_time = DataHandler.Units.WEEK
    start_date = "2018-01-24"
    end_date = "2018-04-08"
    start_time = convert_date_to_time(start_date, DATE_FORMAT)
    end_time = convert_date_to_time(end_date, DATE_FORMAT)
    data_handler = DataHandler("example_output_2.csv", [DataHandler.DATETIME, DataHandler.NUMBER], False)
    x_axis = []
    y_axis = []
    x_axis_label = "Date"
    y_axis_label = "# of Commits"
    working_time = start_time
    count = 0
    while working_time < end_time:
        total = 0
        count = count + 1
        for row in data_handler.get_rows():
            cell_time = convert_date_to_time(row.get_cell(0).get_value(), DATE_FORMAT)
            if cell_time > working_time and cell_time < working_time+data_handler.get_number_of_seconds(gap_time):
                total = total + int(row.get_cell(1).get_value())
        x_axis.append(datetime.datetime.fromtimestamp(working_time).strftime('%Y-%m-%d'))
        y_axis.append(total)
        working_time = working_time + data_handler.get_number_of_seconds(gap_time)

    title = "Showing %i %s(s) between (%s - %s)" % (((end_time - start_time) / data_handler.get_number_of_seconds(gap_time)), data_handler.unit_to_string(gap_time),  start_date, end_date)
    #title = "Commits per week"
    create_graph(title, x_axis_label, y_axis_label, x_axis, y_axis)

def create_graph(title, x_label, y_label, x_axis, y_axis):
    fig, ax = plt.subplots()
    ax.plot(x_axis, y_axis)

    # format the ticks
    #ax.xaxis.set_major_locator(years)
    #ax.xaxis.set_major_formatter(yearsFmt)
    #ax.xaxis.set_minor_locator(months)

    #datemin = datetime.date(r.date.min().year, 1, 1)
    #datemax = datetime.date(r.date.max().year + 1, 1, 1)
    #ax.set_xlim(datemin, datemax)

    def intergise(x):
        return int(x)
    ax.format_xdata = mdates.DateFormatter('%Y-%m-%d')
    ax.format_ydata = intergise
    ax.grid(True)

    # rotates and right aligns the x labels, and moves the bottom of the
    # axes up to make room for them
    plt.savefig("output_graph.png")
    fig.autofmt_xdate()

    #plt.show()
    #plt.figure()

    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title(title)
    #plt.plot(x_axis, y_axis)
    plt.savefig("output_graph.png")

read_file()

