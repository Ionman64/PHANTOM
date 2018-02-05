"""
This file provides methods to obtain data from CSV files and the database. While getting data, some methods allow
to convert data into a specific format. When working with datetime object consider using the DateUtil class in this file.
"""

import csv
import numpy
from datetime import datetime, timedelta
import database_handler as db_handler


def get_from_csvs(files, convert_first=str, convert_second=int, sort_by_first=True):
    """ Iterates over the specified files and parses the contents as CSV.
    Returns the file contents as an array where the index corresponds the the index of the file specifed when calling the
    function

    @file:               Path to csv file.

    @convert_first_col:  Function that is applied to each element in the first column. (e.g. convert from string to date)

    @convert_seconf_col: Function that is appied to each element in the second column. The returned value is required to be
    compatible with the + operator. (e.g. parse to int or double)

    @sort_by_first: Sort the rows by the first column.
    """
    contents = []
    for file in files:
        #contents.append(get_from_csv(file, convert_first, convert_second, sort_by_first))
        map = {}
        with open(file) as csvfile:
            rows = csv.reader(csvfile, delimiter=',')
            for row in rows:
                x = convert_first(row[0])
                map[x] = map.get(x, convert_second(0)) + convert_second(row[1])
        if sort_by_first:
            contents.append(sort_by_x(map.keys(), map.values()))
        else:
            contents.append((map.keys(), map.values()))
    return contents


def get_commit_frequencies(repository_ids, convert_date_fun, sort_by_date=True):
    """ Takes an array of ids and queries the database for the commit frequency of each project.
        Returns an array of queried data, where the position corresponds
        to the indicies of the repository_ids array.

        @repository_id: Id of the repository.

        @convert_date_fun: Function to convert the date. Must take a datetime and returns a datetime.

        @sort_by_date: Sorts the rows by the date column.
        """
    queries = []
    for id in repository_ids:
        commit_frequencies = db_handler.get_commit_frequency_by_id(id) # TODO Don't open a new connection each time
        map = {}
        for key in commit_frequencies:
            date = convert_date_fun(key)
            map[date] = map.get(date, 0) + int(commit_frequencies[key])
        if sort_by_date:
            queries.append(sort_by_x(map.keys(), map.values()))
        else:
            queries.append((map.keys(), map.values()))
    return queries


def sort_by_x(x, y):
    """ Sorts two arrays in the same way, by sorting x and then sorting y based on the order of sorting x"""
    order = numpy.argsort(x)
    return numpy.array(x)[order], numpy.array(y)[order]


class DateUtil:
    """ Provides utility methods for datetime object. """

    @staticmethod
    def date_to_day(date):
        """ Takes a datetime and returns it as it is."""
        return date

    @staticmethod
    def date_to_week(date):
        """ Takes a datetime and sets it to the last Monday. If it is a Monday already, the date stays the same"""
        return date - timedelta(days=date.weekday())

    @staticmethod
    def date_to_month(date):
        """ Takes a datetime and sets the day to 1."""
        return date.replace(day=1)

    @staticmethod
    def date_to_year(date):
        """ Takes a datetime and set month and date to 1. """
        return date.replace(month=1, day=1)

    @staticmethod
    def str_to_date(date_as_string, fmt='%Y-%m-%d'):
        return datetime.strptime(date_as_string, fmt)
