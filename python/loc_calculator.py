import pandas as pd
import numpy as np
import matplotlib.pyplot as plt
from sqlalchemy import create_engine
from utils.setup import get_db_connection_string

pd.set_option("display.max_rows", 2000)
pd.set_option('display.expand_frame_repr', False)

engine = create_engine(get_db_connection_string())
frame = pd.read_sql_query(
    "SELECT repository_commit.repository_id, repository_commit.commit_date::DATE, file_analysis.commit_hash, commit_file.action, commit_file.file_path, file_analysis.loc \
    FROM file_analysis \
    INNER JOIN commit_file ON file_analysis.file_id = commit_file.file_id \
    INNER JOIN repository_commit ON commit_file.repository_id = repository_commit.repository_id \
        AND repository_commit.commit_hash = commit_file.commit_hash;",
    con=engine,
    index_col='commit_date',
    parse_dates="commit_date").sort_index(ascending=True)


ext_filter = frame['file_path'].str.endswith('.java')
frame = frame[ext_filter]

for key, group in frame.groupby('repository_id'):
    dates = group.index
    hashes = group['commit_hash'].values
    actions = group['action'].values
    paths = group['file_path'].values
    loc_values = group['loc'].values

    assert (len(dates) == len(hashes) == len(actions) == len(loc_values))

    loc_for_path = {}
    loc_for_date_map = {}

    num_rows = len(dates)
    for idx in range(0, num_rows):
        if actions[idx] == 'A' or actions[idx] == 'M':
            loc_for_path[paths[idx]] = loc_values[idx]
        elif actions[idx] == 'R':
            loc_for_path.pop(paths[idx], None)
        else:
            print "Abort due to unknown action: '%s'" % actions[idx]

        if idx == num_rows-1 or dates[idx] == dates[idx+1]:
            loc_for_date_map[dates[idx]] = np.sum(np.array(loc_for_path.values()))

    date_loc_frame = pd.DataFrame(data=loc_for_date_map.values(), index=loc_for_date_map.keys())
    ax = date_loc_frame.plot(style='.')
    date_loc_frame.plot(style='--', ax=ax)

plt.show()


