import pandas as pd
import numpy as np
import sqlalchemy
from sklearn import cluster
from utils.setup import get_db_connection_string
from plotter import peak_analysis
if __name__ == "__main__":
    engine = sqlalchemy.create_engine(get_db_connection_string())

    db_frame = pd.read_sql_query(
        "SELECT repository_id, commit_date::DATE as commit_date, COUNT(commit_date::DATE) as frequency FROM repository_commit GROUP BY commit_date::DATE, repository_id;",
        con=engine,        index_col='commit_date',
        parse_dates="commit_date").sort_index()

    id_frame = pd.DataFrame(index=db_frame.repository_id.unique()).sort_index()

    resample_time_unit = "w"
    resample_time_unit_days = 7
    for key, series in db_frame.groupby('repository_id')['frequency']:
        #print "*** ID == ", key
        series = series.resample(resample_time_unit).sum()
        peaks = peak_analysis(series)
        df = pd.DataFrame(data={'values': series.values, 'peaks': peaks.values}, index=series.index)

        # duration
        id_frame.at[key, 'duration'] = len(df)

        # max value position
        ymax = df['values'].max()
        id_frame.at[key, 'max y'] = ymax
        ymax_idx = df['values'].values.argmax()
        id_frame.at[key, 'max y pos'] = ymax_idx + 1

        # number of up peaks
        peak_counts = df.groupby('peaks').count()
        id_frame.at[key, 'peakup'] = peak_counts.loc[1][0] if 1 in peak_counts.index else 0

        # avg. time between peaks
        peak_up_times = df[df['peaks'] == 1].index
        time_between_ups = [(peak_up_times[idx] - peak_up_times[idx-1]).days for idx in range(1, len(peak_up_times))]
        avg_delta_ups = -100
        if len(time_between_ups) > 0:
            avg_delta_ups = np.average(time_between_ups)  / resample_time_unit_days
        id_frame.at[key, 'avg delta ups'] = avg_delta_ups
        # peak amplitudes
        up_idx = np.where(df.peaks.values == 1)[0]
        vals = df['values'].values
        amplitudes = []
        for idx in up_idx:
            prev_val = vals[idx-1]
            peak_val = vals[idx]

            diff = peak_val - prev_val
            amplitudes.append(np.true_divide(diff, ymax))

        min_amp = -100
        avg_amp = -100
        max_amp = -100
        if len(amplitudes) > 0:
            min_amp = np.min(amplitudes)
            avg_amp = np.average(amplitudes)
            max_amp = np.max(amplitudes)
        id_frame.at[key, 'min amplitude'] = min_amp
        id_frame.at[key, 'avg amplitude'] = avg_amp
        id_frame.at[key, 'max amplitude'] = max_amp




        # density calculation
        #print df['values'][ymax_idx:ymax_idx+10].count()

    #print "\n*\t--------------------\t*\n", id_frame, "\n\n", id_frame.describe()[1:]
    mat = id_frame.as_matrix()
    km = cluster.KMeans(n_clusters=10)
    km.fit(mat)
    labels = km.labels_

    pd.set_option("display.max_rows", 400)
    df_results = pd.DataFrame(data={'repository_id': id_frame.index, 'cluster': labels})
    print df_results