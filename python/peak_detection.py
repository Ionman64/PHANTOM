from subprocess import check_output
import sys
try:
    # for Python 2.x
    from StringIO import StringIO
except ImportError:
    # for Python 3.x
    from io import StringIO
import csv

#Returns
def find_peaks(project_ids):
    return_dict = {}
    x = []
    y = []
    try:
        retstr = check_output(["./utils", "--findpeaks"] + project_ids)
    except OSError as e:
        print("Execution failed:", e, sys.stderr)
    for row in csv.reader(StringIO(retstr), delimiter=','):
        return_dict[row[1]].x.append(row[0])
        y.append(row[1])
    return x,y

if __name__ == "__main__":
    print (find_peaks(["1","2","3"]))