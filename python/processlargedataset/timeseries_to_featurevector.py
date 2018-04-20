import pandas as pd
import numpy as np
import sys, os
from plotter import peak_analysis

# Pandas options for better command line output
pd.set_option("display.max_rows", 500)
pd.set_option('display.expand_frame_repr', False)

# Check command line input: Expects path to time-series csv file
assert len(sys.argv) == 3
path_to_timeseries = os.path.expanduser(sys.argv[1])
assert os.path.isfile(path_to_timeseries)
path_to_featuretable_output = os.path.expanduser(sys.argv[2])

# Read the time-series to a dataframe
frame = pd.read_csv(path_to_timeseries, index_col=[0, 1], parse_dates=[1], usecols=[0,1,4], dtype={'integrations': np.int})  # , usecols=[0, 14])

#with open(path_to_featuretable_output, 'w+') as out:
#   out.write("filename,duration,max_y,max_y_pos,mean_y,median_y,sum_y,peak_down,peak_none,peak_up,atbp_up,atbp_down,min_amp,avg_amp,max_amp,mpg,mng\n")


write_header=True
for filename, group in frame.groupby(level=0):
    print filename
    # group.reset_index("filename", drop=True, inplace=True)
    series = group.reset_index("filename", drop=True).integrations
    print series.dtypes

    peaks = peak_analysis(series, path_to_utils_binary="../../target/debug/utils")
    df = pd.DataFrame(data={'values': series.values, 'peaks': peaks.values}, index=series.index)

    feature_duration = len(df)
    feature_max_y = df['values'].max()
    feature_max_y_position = df['values'].values.argmax() + 1
    feature_mean_y = df['values'].values.mean()
    feature_median_y = df['values'].median()
    feature_sum_y = df['values'].sum()
    ### number of up peaks
    peak_counts = df.groupby(
        'peaks').count()  # this results in a data frame index with -1, 0 and 1 and a count in the values columns
    feature_peak_down = peak_counts.loc[-1][0] if -1 in peak_counts.index else 0
    feature_peak_none = peak_counts.loc[0][0] if 0 in peak_counts.index else 0
    feature_peak_up = peak_counts.loc[1][0] if 1 in peak_counts.index else 0
    ### avg. time between peaks
    peak_up_times = df[df['peaks'] == 1].index
    time_between_ups = [(peak_up_times[idx] - peak_up_times[idx - 1]).days for idx in range(1, len(peak_up_times))]
    avg_delta_ups = (np.average(time_between_ups) / 7) if len(time_between_ups) > 0 else np.NaN
    feature_atbp_up = avg_delta_ups

    peak_down_times = df[df['peaks'] == -1].index
    time_between_downs = [(peak_down_times[idx] - peak_down_times[idx - 1]).days for idx in
                          range(1, len(peak_down_times))]
    avg_delta_downs = (np.average(time_between_downs) / 7) if len(
        time_between_downs) > 0 else np.NaN
    feature_atbp_down = avg_delta_downs
    ### peak amplitudes
    up_idx = np.where(df.peaks.values == 1)[0]
    vals = df['values'].values
    amplitudes = []
    for idx in up_idx:
        prev_val = vals[idx - 1]
        peak_val = vals[idx]
        diff = peak_val - prev_val
        amplitudes.append(np.true_divide(diff, feature_max_y))
    min_amp, avg_amp, max_amp = (np.min(amplitudes), np.average(amplitudes), np.max(amplitudes)) if len(
        amplitudes) > 0 else (np.NaN, np.NaN, np.NaN)
    feature_min_amp = min_amp
    feature_avg_amp = avg_amp
    feature_max_amp = max_amp
    ### gradients

    gradients = []
    for idx in range(1, len(vals)):
        gradients.append(vals[idx] - vals[idx - 1])
    gradients = np.array(gradients)
    pos_gradients = gradients[np.where(gradients >= 0)]
    neg_gradients = gradients[np.where(gradients < 0)]
    feature_mpg = pos_gradients.mean() if len(pos_gradients) > 0 else np.NaN
    feature_mng = neg_gradients.mean() if len(neg_gradients) > 0 else np.NaN

    pd.DataFrame(data={
        "duration": feature_duration,
        "max_y": feature_max_y,
        "max_y_pos": feature_max_y_position,
        "mean_y": feature_mean_y,
        "median_y": feature_median_y,
        "sum_y": feature_sum_y,
        "peak_down": feature_peak_down,
        "peak_none": feature_peak_none,
        "peak_up": feature_peak_up,
        "atbp_up": feature_atbp_up,
        "atbp_down": feature_atbp_down,
        "min_amp": feature_min_amp,
        "avg_amp": feature_avg_amp,
        "max_amp": feature_max_amp,
        "mpg": feature_mpg,
        "mng": feature_mng
    }, index=[filename]).to_csv(path_to_featuretable_output, mode="a", header=write_header)
    write_header=False


