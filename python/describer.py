import pandas as pd
import numpy as np
import sqlalchemy
from sklearn import cluster
from utils.setup import get_db_connection_string
from plotter import peak_analysis

def normalise_columns_with_standard_score(dataframe):
    for col in dataframe.columns:
        dataframe[col] = (dataframe[col] - dataframe[col].mean()) / dataframe[col].std()

if __name__ == "__main__":
    # get data ---------------------------------------------------------------------------------------------------------
    engine = sqlalchemy.create_engine(get_db_connection_string())
    db_frame = pd.read_sql_query(
        "SELECT repository_id, commit_date::DATE as commit_date, COUNT(commit_date::DATE) as frequency FROM repository_commit GROUP BY commit_date::DATE, repository_id;",
        con=engine,        index_col='commit_date',
        parse_dates="commit_date").sort_index()
    id_frame = pd.DataFrame(index=db_frame.repository_id.unique()).sort_index()
    # setup ------------------------------------------------------------------------------------------------------------
    resample_time_unit = "w" # choose the time unit to group. 'w' = week
    resample_time_unit_days = 7
    # iterate over IDs to populate dataframe ---------------------------------------------------------------------------
    for key, series in db_frame.groupby('repository_id')['frequency']:
        ### print "*** ID == ", key
        series = series.resample(resample_time_unit).sum()
        peaks = peak_analysis(series)
        df = pd.DataFrame(data={'values': series.values, 'peaks': peaks.values}, index=series.index)

        ### duration
        id_frame.at[key, 'duration'] = len(df)

        ### max value position
        ymax = df['values'].max()
        id_frame.at[key, 'max y'] = ymax
        ymax_idx = df['values'].values.argmax()
        id_frame.at[key, 'max y pos'] = ymax_idx + 1

        ### number of up peak s
        peak_counts = df.groupby('peaks').count()
        id_frame.at[key, 'peakup'] = peak_counts.loc[1][0] if 1 in peak_counts.index else 0

        ### avg. time between peaks
        peak_up_times = df[df['peaks'] == 1].index
        time_between_ups = [(peak_up_times[idx] - peak_up_times[idx-1]).days for idx in range(1, len(peak_up_times))]
        avg_delta_ups = 0
        if len(time_between_ups) > 0:
            avg_delta_ups = np.average(time_between_ups)  / resample_time_unit_days
        id_frame.at[key, 'avg delta ups'] = avg_delta_ups
        ### peak amplitudes
        up_idx = np.where(df.peaks.values == 1)[0]
        vals = df['values'].values
        amplitudes = []
        for idx in up_idx:
            prev_val = vals[idx-1]
            peak_val = vals[idx]

            diff = peak_val - prev_val
            amplitudes.append(np.true_divide(diff, ymax))

        min_amp = 0
        avg_amp = 0
        max_amp = 0
        if len(amplitudes) > 0:
            min_amp = np.min(amplitudes)
            avg_amp = np.average(amplitudes)
            max_amp = np.max(amplitudes)
        id_frame.at[key, 'min amplitude'] = min_amp
        id_frame.at[key, 'avg amplitude'] = avg_amp
        id_frame.at[key, 'max amplitude'] = max_amp

    pd.set_option("display.max_rows", 400)

    print id_frame, "\n\n", id_frame.describe(), "\n\n"

    normalise_columns_with_standard_score(id_frame)


    # setup kmeans with number of clusters
    km = cluster.KMeans(n_clusters=2)

    # run kmeans N times
    N = 10
    # accuracy for each run
    acc0 = np.zeros(N)
    acc1 = np.zeros(N)
    # misclassification for each run
    mis0 = np.zeros(N)
    mis1 = np.zeros(N)

    mat = id_frame[['duration', 'avg amplitude']].as_matrix()
    for idx in range(N):
        df_results = pd.DataFrame(data={'repository_id': id_frame.index, 'cluster': km.fit(mat).labels_})

        cut = 152 # the first 152 projects are projects according to the csv. So they should have the same label
        rep0 = df_results[df_results.repository_id <= cut]
        rep1 = df_results[df_results.repository_id > cut]

        lab0 = rep0.loc[0].cluster # label for first cluster is the label for the first project
        lab1 = np.mod(lab0+1, 2) # as there are two label, the other label is determined by modulo

        acc0[idx] = np.true_divide(rep0[rep0.cluster == lab0]['cluster'].count(), rep0['cluster'].count())
        mis0[idx] = np.true_divide(rep0[rep0.cluster == lab1]['cluster'].count(), rep0['cluster'].count())

        acc1[idx] = np.true_divide(rep1[rep1.cluster == lab1]['cluster'].count(), rep1['cluster'].count())
        mis1[idx] = np.true_divide(rep1[rep1.cluster == lab0]['cluster'].count(), rep1['cluster'].count())

    print "Acc(0): ", np.average(acc0)
    print "Mis(0): ", np.average(mis0)
    print "Acc(1): ", np.average(acc1)
    print "Mis(1): ", np.average(mis1)


