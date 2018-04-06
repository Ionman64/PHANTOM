import os, sys, csv
import numpy as np
from datetime import datetime as dt

def get_log_paths(path_to_log_direcotry):
    return [os.path.join(path_to_log_direcotry, log_name) for log_name in os.listdir(log_directory_path)]

def get_number_of_parents(value):
    if len(value) == 0:
        return 0
    return len(value.split(' '))

def get_date_from_timestamp(timestamp):
    return dt.fromtimestamp(float(timestamp)).strftime("%Y-%m-%d")

def transform_to_integration_frequency(commiter_dates):
    pass

def transform_to_commit_frequency(author_dates):
    pass

def transform_to_merge_frequency(author_dates, committer_date, merges):
    pass

def transform_to_author_frequency():
    pass

def transform_to_integrator_frequency():
    pass

if __name__ == "__main__":
    assert len(sys.argv) > 1
    arg1 = sys.argv[1]

    assert os.path.isdir(arg1)
    log_directory_path = os.path.expanduser(arg1)

    COL_HASH = 0
    COL_PARENTS = 1
    COL_AUTHOR = 2
    COL_AUTHOR_MAIL = 3
    COL_AUTHOR_DATE = 4
    COL_COMMITTER = 5
    COL_COMMITTER_MAIL = 6
    COL_COMMITTER_DATE = 7

    SKIP = 0
    LIMIT = 100
    for log_path in get_log_paths(log_directory_path):
        if SKIP > 0:
            SKIP = SKIP - 1
            continue
        # -----------------------------------------------------------------------
        with open(log_path, 'rb') as csvfile:
            extracted_data = []
            for idx, row in enumerate(csv.reader(csvfile, delimiter=',')):
                is_merge = 1 if get_number_of_parents(row[COL_PARENTS]) > 1 else 0
                author_date = get_date_from_timestamp(row[COL_AUTHOR_DATE])
                commiter_date = get_date_from_timestamp(row[COL_COMMITTER_DATE])
                data = [
                    row[COL_HASH],
                    is_merge,
                    author_date,
                    row[COL_AUTHOR_MAIL],
                    commiter_date,
                    row[COL_COMMITTER_MAIL]
                ]
                extracted_data.append(data)
            extracted_data = np.array(extracted_data)
            # TODO transform extracted_data to time-series using the transform_XXX functions
                # -----------------------------------------------------------------------
        if LIMIT == 1:
            break
        LIMIT = LIMIT - 1

    print "Done."