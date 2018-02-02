"""
Usage:
    plotter_v2.py (--peak|--line|--euclidean) <id>... --timeunit=<UNIT> [options]

Arguments:
    -t --timeunit=<UNIT>    Time unit of the x axis. Units: day, week, month, year
Options:
    -h --help           Show this screen.
    -h --hide           Hide the diagram / Don't show the diagram
    -o --out=<file>     Path to output file. You can specify the file format by using the desired file extension (e.g. png, pdf)
    --shiftleft         All projects start at x=0
    --norm              Normalises all y values to be between 0 and 1
    --acc               Accumulate y values
"""
from docopt import docopt
import database_handler as db_handler

if __name__ == '__main__':
    x, y = db_handler.get_commit_frequency_by_id(18)
    print(len(x))
    print()
    args = docopt(__doc__)

    timeunit = ["day", "week", "month", "year"]
    if not args['--timeunit'] in timeunit:
        print("Invalid timeunit. Use --help to get more information.")
        exit(1)

    if (args['--peak']):
        pass
    elif args['--line']:
        pass
    elif args['--euclidean']:
        pass