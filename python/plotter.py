"""
Usage:
    plotter_v2.py <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT>    Time unit of the x axis. Units: day, week, month, year
Options:
    -h --help           Show this screen.
    --hide              Hide the diagram / Don't show the diagram
    -o --out=<file>     Path to output file. You can specify the file format by using the desired file extension (e.g. png, pdf)
    --shift=<direction> Shift the dates of projects [values: left, right]
    --norm              Normalises all y values to be between 0 and 1
    --acc               Accumulate y values
    --peak              Highlight peaks
    --ydist             Calculate the distance on the y-axis between the first project and all other projects
"""
from docopt import docopt
from utils import data_processor as processor, data_provider as provider, data_visualiser as visualiser
import matplotlib.pyplot as pyplot
import numpy

if __name__ == '__main__':
    args = docopt(__doc__)
    # Validate command line arguments ----------------------------------------------------------------------------------
    valid_timeunits = ["day", "week", "month", "year"]
    if not args['--timeunit'] in valid_timeunits:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)

    valid_shift_values = ["left", "right", "peak", None]
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
    arg_ydist = args['--ydist']
    arg_peak = args['--peak']

    if arg_ydist and not arg_shift == "left":
        print "Y distance calculation only works with left-shifted data. Please set '--shift left'"
        exit(1)

    # Get the data -----------------------------------------------------------------------------------------------------
    data = provider.get_commit_frequencies(arg_ids, convert_date_functions[arg_time_unit])

    # Process the data -------------------------------------------------------------------------------------------------
    data = processor.process(data, accumulate=arg_acc, normalise=arg_norm, shift=arg_shift)

    # Plot the specified graph -----------------------------------------------------------------------------------------

    ## Line graph ##
    if not arg_ydist:
        fig, line_handle = pyplot.subplots()
    else:
        fig, (line_handle, ydist_handle) = pyplot.subplots(2, 1, sharey=True, sharex=True)
    visualiser.plot_line_graph(line_handle, data, arg_ids, arg_shift is None)
    ## Peak markers ##
    if (arg_peak):
        peaks = processor.find_peaks(data)
        visualiser.plot_peaks(line_handle, data, peaks, arg_shift is None)

    pyplot.subplot(line_handle)
    pyplot.title('Number of commits over time (' + arg_time_unit + 's)')
    pyplot.legend(loc='upper right')

    ## Euclidean distances/"--ydist" graph ##
    if arg_ydist:
        distances, avg_distances = processor.get_euclidean(data)
        assert (len(arg_ids) == len(avg_distances))
        labels = ["%s (%.0f%%)" % (arg_ids[idx], avg_distances[idx]*100) for idx in range(len(avg_distances))]
        visualiser.plot_line_graph(ydist_handle, data, labels, x_as_dates=False)

        pyplot.subplot(ydist_handle)
        pyplot.title('Distance graph')
        pyplot.legend(loc='upper right')

    # Display and save figure ------------------------------------------------------------------------------------------
    pyplot.tight_layout()
    if not arg_out_file is None:
        print "Save figure to ", arg_out_file
        pyplot.savefig(arg_out_file)
    if not arg_hide:
        pyplot.show()
