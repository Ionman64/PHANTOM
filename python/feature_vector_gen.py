import sys
import numpy as np
import pandas as pd
from plotter import peak_analysis, rolling_mean_for_series

path_to_csv_in = sys.argv[1]
path_to_csv_out = sys.argv[2]

df_in = pd.read_csv(path_to_csv_in, index_col=0, parse_dates=True)
df_feature_table = pd.DataFrame(index=df_in.repository_id.unique()).sort_index()

resample_time_unit = "w" # choose the time unit to group. 'w' = week
resample_time_unit_days = 7
for key, series in df_in.groupby('repository_id')['frequency']:
    ### print "*** ID == ", key
    series = series.resample(resample_time_unit).sum()

    #peaks = peak_analysis(rolling_mean_for_series(series, window_size=4))
    peaks = peak_analysis(series)
    df = pd.DataFrame(data={'values': series.values, 'peaks': peaks.values }, index=series.index)

    df_feature_table.at[key, 'commits'] = df['values'].sum()

    ### duration
    df_feature_table.at[key, 'duration'] = len(df)

    ### max value position
    ymax = df['values'].max()
    df_feature_table.at[key, 'max_y'] = ymax
    ymax_idx = df['values'].values.argmax()
    df_feature_table.at[key, 'max_y_pos'] = ymax_idx + 1

    ### y values
    df_feature_table.at[key, 'mean_y'] = df['values'].values.mean()
    df_feature_table.at[key, 'median_y'] = df['values'].median()
    df_feature_table.at[key, 'sum_y'] = df['values'].sum()

    ### number of up peaks
    peak_counts = df.groupby('peaks').count() # this results in a data frame index with -1, 0 and 1 and a count in the values columns
    df_feature_table.at[key, 'peak_down'] = peak_counts.loc[-1][0] if -1 in peak_counts.index else 0
    df_feature_table.at[key, 'peak_none'] = peak_counts.loc[0][0] if 0 in peak_counts.index else 0
    df_feature_table.at[key, 'peak_up'] = peak_counts.loc[1][0] if 1 in peak_counts.index else 0
    ### avg. time between peaks
    peak_up_times = df[df['peaks'] == 1].index
    time_between_ups = [(peak_up_times[idx] - peak_up_times[idx - 1]).days for idx in range(1, len(peak_up_times))]
    avg_delta_ups = (np.average(time_between_ups) / resample_time_unit_days) if len(time_between_ups) > 0 else np.NaN
    df_feature_table.at[key, 'ATBP_up'] = avg_delta_ups

    peak_down_times = df[df['peaks'] == -1].index
    time_between_downs = [(peak_down_times[idx] - peak_down_times[idx-1]).days for idx in range(1, len(peak_down_times))]
    avg_delta_downs = (np.average(time_between_downs) / resample_time_unit_days) if len(time_between_downs) > 0 else np.NaN
    df_feature_table.at[key, 'ATBP_down'] = avg_delta_downs
    ### peak amplitudes
    up_idx = np.where(df.peaks.values == 1)[0]
    vals = df['values'].values
    amplitudes = []
    for idx in up_idx:
        prev_val = vals[idx - 1]
        peak_val = vals[idx]
        diff = peak_val - prev_val
        amplitudes.append(np.true_divide(diff, ymax))
    min_amp, avg_amp, max_amp = (np.min(amplitudes), np.average(amplitudes), np.max(amplitudes)) if len(amplitudes) > 0 else (np.NaN, np.NaN, np.NaN)
    df_feature_table.at[key, 'min_amp'] = min_amp
    df_feature_table.at[key, 'avg_amp'] = avg_amp
    df_feature_table.at[key, 'max_amp'] = max_amp
    ### gradients

    gradients = []
    for idx in range(1, len(vals)):
        gradients.append(vals[idx] - vals[idx - 1])
    gradients = np.array(gradients)
    pos_gradients = gradients[np.where(gradients >= 0)]
    neg_gradients = gradients[np.where(gradients < 0)]
    df_feature_table.at[key, 'MPG'] = pos_gradients.mean() if len(pos_gradients) > 0 else np.NaN
    df_feature_table.at[key, 'MNG'] = neg_gradients.mean() if len(neg_gradients) > 0 else np.NaN

pd.set_option("display.max_rows", 300)
pd.set_option('display.expand_frame_repr', False)

df_feature_table.to_csv(path_to_csv_out)