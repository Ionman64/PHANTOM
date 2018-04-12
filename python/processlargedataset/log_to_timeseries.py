import os, sys, csv
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt
from datetime import datetime as dt

RESAMPLE_KEY = 'W'

def get_log_paths(path_to_log_direcotry):
    return [(os.path.join(path_to_log_direcotry, log_name), log_name) for log_name in os.listdir(log_directory_path)]

def get_number_of_parents(value):
    if len(value) == 0:
        return 0
    return len(value.split(' '))

def get_date_from_timestamp(timestamp):
    return dt.fromtimestamp(float(timestamp)).strftime("%Y-%m-%d")

def transform_to_integration_frequency(commiter_dates):
    return pd.DataFrame(data={"Integrations": np.ones(len(commiter_dates))}, index=pd.DatetimeIndex(commiter_dates)).resample(RESAMPLE_KEY).sum()


def transform_to_commit_frequency(author_dates):
    return pd.DataFrame(data={"Commits": np.ones(len(author_dates))}, index=pd.DatetimeIndex(author_dates)).resample(RESAMPLE_KEY).sum()

def transform_to_merge_frequency(committer_date, merges):
    return pd.DataFrame(data={'Merges': merges}, index=pd.DatetimeIndex(committer_date)).resample(RESAMPLE_KEY).sum()

def transform_to_author_frequency(author_dates, author_names):
    frame = pd.DataFrame(data={'Authors': author_names}, index=pd.DatetimeIndex(author_dates))
    return frame.resample(RESAMPLE_KEY).agg({'Authors': pd.Series.nunique})

def transform_to_integrator_frequency(commiter_dates, commiter_names):
    frame = pd.DataFrame(data={'Integrators': commiter_names}, index=pd.DatetimeIndex(commiter_dates))
    return frame.resample(RESAMPLE_KEY).agg({'Integrators': pd.Series.nunique})

def transform(merges, author_dates, author_names, commiter_dates, commiter_names):
    mf_frame = transform_to_merge_frequency(author_dates, merges)
    cf_frame = transform_to_commit_frequency(author_dates)
    if_frame = transform_to_integration_frequency(commiter_dates)
    author_frame = transform_to_author_frequency(author_dates, author_names)
    integrator_frame = transform_to_integrator_frequency(commiter_dates, commiter_names)

    return pd.concat([mf_frame, cf_frame, if_frame, author_frame, integrator_frame], axis=1)

def plot_log():
    if len(frame) > 25:
        f, ax = plt.subplots(2, sharex=True, sharey=True, figsize=(15, 10))
        frame[['Commits', 'Authors', 'Merges']].plot(ax=ax[0])
        frame[['Integrations', 'Integrators', 'Merges']].plot(ax=ax[1])
        plt.suptitle(log_path)

        print ">> ", log_path
        print frame
        plt.show()


if __name__ == "__main__":
    pd.set_option("display.max_rows", 500)
    pd.set_option('display.expand_frame_repr', False)

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
    LIMIT = 10000
    poor_format = [] # stores the path to the logs that are poorly formatted
    log_timeseries_frames = [] # stores the data frame for each log
    log_timeseries_keys = [] # stores the keys for the timeseries data frames
    for log_path, log_name in get_log_paths(log_directory_path):
        if SKIP > 0:
            SKIP = SKIP - 1
            continue
        # -----------------------------------------------------------------------

        with open(log_path, 'rb') as csvfile:
            extracted_data = []
            for idx, row in enumerate(csv.reader(csvfile, delimiter=',')):
                if (len(row) != 8):
                    poor_format.append(log_path)
                    break

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
            extracted_data_frame = pd.DataFrame(data=extracted_data, columns=['hash', 'is_merge', 'author_date', 'author_mail', 'commiter_date', 'commiter_mail'])
            frame = transform(extracted_data_frame['is_merge'].values, extracted_data_frame['author_date'].values, extracted_data_frame['author_mail'].values, extracted_data_frame['commiter_date'].values, extracted_data_frame['commiter_mail'].values)
            del extracted_data
            del extracted_data_frame
            pd.concat([frame], keys=[log_name], names=["repo", "date"]).to_csv("/home/joshua/Desktop/timeseries.csv", mode='a', header=None)
            del frame
        # -----------------------------------------------------------------------
        if LIMIT == 1:
            break
        LIMIT = LIMIT - 1
    #assert (len(log_timeseries_frames) == len(log_timeseries_keys))
    #masterframe = pd.concat(log_timeseries_frames, keys=log_timeseries_keys, names=["project", "date"])
    print "Skipped due to poor format: ", len(poor_format)


