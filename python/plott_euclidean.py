"""
Usage:
    plotter_euclidean.py <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT>    Time unit of the x axis. Units: day, week, month, year
Options:
    -h --help           Show this screen.
    --hide           Hide the diagram / Don't show the diagram
    -o --out=<file>     Path to output file. You can specify the file format by using the desired file extension (e.g. png, pdf)
    --shift=<direction> Shift the dates of projects [values: left, right]
    --norm              Normalises all y values to be between 0 and 1
    --acc               Accumulate y values
"""
from docopt import docopt
import data_provider as provider
import data_processor as processor
import matplotlib.pyplot as pyplot
import numpy as numpy

if __name__ == '__main__':
    args = docopt(__doc__)
    # Validate command line arguments ----------------------------------------------------------------------------------
    valid_timeunits = ["day", "week", "month", "year"]
    if not args['--timeunit'] in valid_timeunits:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)

    valid_shift_values = ["left", "right", None]
    if not args['--shift'] in valid_shift_values:
        print("Invalid shift value. Use --help to get more information.")
        exit(1)

    # Process command line arguments -----------------------------------------------------------------------------------
    convert_date_functions = {
        "day": provider.DateUtil.date_to_day,
        "week": provider.DateUtil.date_to_week,
        "month": provider.DateUtil.date_to_month,
        "year": provider.DateUtil.date_to_year,
    }

    arg_ids = args['<id>']
    arg_time_unit = args['--timeunit']
    arg_acc = args['--acc']
    arg_norm = args['--norm']
    arg_shift = args['--shift']
    arg_out_file = args['--out']
    arg_hide = args['--hide']

    # Get the data -----------------------------------------------------------------------------------------------------
    data = provider.get_commit_frequencies(arg_ids, convert_date_functions[arg_time_unit])

    # Process the data -------------------------------------------------------------------------------------------------
    data = processor.process(data, accumulate=arg_acc, normalise=arg_norm, shift=arg_shift)

    fig, (line, eucl) = pyplot.subplots(1, 2, sharex=True)
    for id, row in enumerate(data):
        line.plot(row[0], row[1], '-', label=arg_ids[id])
    pyplot.subplot(line)
    pyplot.title("Line plot")
    pyplot.legend(loc='upper right')

    ref = numpy.column_stack(data[0])
    distances = []
    for row in data[1:]:
        comp = numpy.column_stack(row)
        dist = []
        m = numpy.minimum(len(ref), len(comp))
        for idx in range(0, m):
            dist.append(abs(comp[idx][1] - ref[idx][1]))
        distances.append(dist)

    eucl.plot(data[0][0], numpy.zeros(len(data[0][0])), label=arg_ids[0])
    for id, distance in enumerate(distances):
        m = numpy.minimum(len(data[0][0]), len(distance))
        eucl.plot(data[0][0][0:m], distance[0:m], label=arg_ids[id + 1])
    pyplot.subplot(eucl)
    pyplot.legend(loc='upper right')
    pyplot.title("Euclidean distance")
    pyplot.show()

