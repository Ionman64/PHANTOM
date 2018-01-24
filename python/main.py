import matplotlib.pyplot as plt
from datetime import *
import math

#Private Imports
from DataHandler import *
from utils import convert_date_to_time


FILE_PATH = 'C:\\Users\\Hazel\\Desktop\\Chalmers\\'
INPUT_DATE_FORMAT = "dd-mmm-yyyy hh:mm:ss"
OUTPUT_DATE_FORMAT = "dd-mmm-yyyy hh:mm:ss"

Issues = []


def get_bug_count_for_weeks(target_date):
    count = 0
    with open(FILE_PATH + "Bugs.csv", "r") as file:
        for line in file:
            modified_line = line.replace("\n", "").replace("\"", "")
            second_column = modified_line.split(",")[1]
            if (second_column.split("|")[0].startswith(target_date)):
                count += 1
    return count


def get_month(month_str):
    try:
        months = ["JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC"]
        month_int = int(float(month_str))
        return months[month_int - 1]
    except IndexError:
        print("Failed to write month string")
        return month_str


def get_number_of_seconds(unit):
    s = 1
    if unit == "minute":
        s = 60
    if unit == "hour":
        s = get_number_of_seconds("minute")*60
    if unit == "day":
        s = get_number_of_seconds("hour")*24
    if unit == "week":
        s = get_number_of_seconds("day")*7
    return s


def create_graph(title, x_label, y_label, x_axis, y_axis):
    plt.figure()
    plt.xlabel(x_label)
    plt.ylabel(y_label)
    plt.title(title)
    plt.plot(x_axis, y_axis)
    plt.savefig(FILE_PATH + "Bugs_output.png")


def export(datahandler, filename):
    separator_char = ","
    quotation_marks = True
    with open(filename, "w") as file:
        i = 1
        for header in datahandler.headers:
            if i == len(datahandler.headers):
                separator_char = ""
            if quotation_marks:
                file.write("\"%s\"%s" % (header, separator_char))
            else:
                file.write("%s%s" % (header, separator_char))
            i += 1
        file.write("\n")
        for row in datahandler.find_all_issues():
            i = 0
            for cell in row.get_all_cells():
                separator_char = ","
                if i == len(datahandler.headers):
                    separator_char = ""
                if quotation_marks:
                    file.write("\"%s\"%s" % (cell.get_value(), separator_char))
                else:
                    file.write("%s%s" % (cell.get_value(), separator_char))
                i += 1
            file.write("\n")




if __name__ == "__main__":
    DATE_FORMAT = "%Y-%m-%d"
    INPUT_DATE_FORMAT = "%Y-%m-%d|%H:%M:%S"
    time_unit = "week"
    start_date = "2000-01-24"
    end_date = "2000-02-1"
    issue_handler = DataHandler(FILE_PATH + "Bugs.csv", INPUT_DATE_FORMAT)
    export(issue_handler, FILE_PATH + "Bugs_Output_2.csv")
    print ("num issues %i" % issue_handler.num_issues())
    start_time = convert_date_to_time(start_date, DATE_FORMAT)
    end_time = convert_date_to_time(end_date, DATE_FORMAT)
    gap_time = get_number_of_seconds(time_unit)
    current_time = start_time
    x_axis = []
    y_axis = []
    title = "Showing %i %s(s) Between (%s - %s)" % (((end_time - start_time) / gap_time), time_unit,  start_date, end_date)
    i = 0
    while current_time < end_time:
        i += 1
        total_for_the_week = len(issue_handler.find_issues(current_time, current_time+gap_time))
        print ("Week %i - %i" % (i, total_for_the_week))
        y_axis.append(total_for_the_week)
        x_axis.append(i)
        current_time += gap_time
    create_graph(title, "Week_Num", "Number of Defects", x_axis, y_axis)


#get_days("2000-09-01", "2001-09-01", INPUT_DATE_FORMAT)
#create_graph("2000-09-01", "2000-10-01", [SUM(x), "SUM(CREATED-RESOLVED)"])