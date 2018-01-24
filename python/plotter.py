import matplotlib.pyplot as plt
from matplotlib.dates import YearLocator, MonthLocator, DayLocator, DateFormatter
from datetime import datetime
import csv
import numpy as np

years = YearLocator()
months = MonthLocator()
days = DayLocator()
yearsFmt = DateFormatter('%Y-%m')
monthsFmt = DateFormatter('%m')

def read_csv(path):
    x = []
    y = []

    with open(path) as csvfile:
        plots = csv.reader(csvfile, delimiter=',')
        for row in plots:
            x.append(datetime.strptime(row[0], '%Y-%m-%d'))
            y.append(int(row[1]))

    order = np.argsort(x)
    sorted_x = np.array(x)[order]
    sorted_y = np.array(y)[order]
    return sorted_x, sorted_y

fig, ax = plt.subplots()

x1, y1 = read_csv('6.csv')
ax.plot_date(x1, y1, '-', label='6')

x2, y2 = read_csv('7.csv')
ax.plot_date(x2, y2, '-', label='7')

x3, y3 = read_csv('8.csv')
ax.plot_date(x3, y3, '-', label='8')

x4, y4 = read_csv('9.csv')
ax.plot_date(x4, y4, '-', label='9')

x5, y5 = read_csv('10.csv')
ax.plot_date(x5, y5, '-', label='10')

# tick formats
ax.xaxis.set_major_locator(years)
ax.xaxis.set_major_formatter(yearsFmt)
ax.xaxis.set_minor_locator(months)
ax.xaxis.set_minor_formatter(monthsFmt)
ax.autoscale_view()

#plt.title('Number of commits over time')
fig.autofmt_xdate()
plt.legend(loc='upper left')
plt.show()
