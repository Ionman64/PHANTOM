import matplotlib.pyplot as plt
import matplotlib.ticker as pltticker


def __style_line__(axes):
    axes.set_ylabel("frequency")


def __style_norm__(axes):
    axes.set_ylabel("norm.")


def __style_acc__(axes):
    axes.set_ylabel("acc.")


def __style_acc_norm__(axes):
    axes.set_ylabel("acc. norm.")


def __style_euclidean_axes__(axes, y_label='avg. distance'):
    axes.set_ylabel(y_label)
    axes.yaxis.tick_right()

def __style_max_peak_portion(axes):
    axes.set_ylabel("portion in %")
    axes.set_xlabel("project ID")

def __style_time_between_peaks(axes, peak_type):
    assert(peak_type in ['all', 'up', 'down'])
    axes.set_title("Time between %s peaks" % peak_type)
    axes.set_ylabel("days")
    axes.set_xlabel("peak number")
    axes.xaxis.set_major_locator(pltticker.MultipleLocator(base=10))
    axes.xaxis.set_minor_locator(pltticker.MultipleLocator(base=1))



def get_fig_and_ax_map(arg_time_unit, arg_rollingmean, arg_window):
    # figure map -------------------------------------------------------------------------------------------------------
    fig = {
        'date': plt.figure(figsize=(10, 10)),
        'left': plt.figure(figsize=(10, 10)),
        'right': plt.figure(figsize=(10, 10)),
        'max-peak': plt.figure(figsize=(10, 10)),
        'time-between-peaks': plt.figure(),
    }
    # axes map ---------------------------------------------------------------------------------------------------------
    date_grid = (4, 3)
    left_grid = (4, 3)
    right_grid = (4, 3)
    max_peak_grid = (5, 3)
    time_between_grid = (3, 3)
    ax = { # NOTE: The keys follow a naming convention for ease of use as well as for styling! See in below at 'axes styling'
        'date-line': plt.subplot2grid(date_grid, (0, 0), colspan=3, fig=fig['date']),
        'date-norm': plt.subplot2grid(date_grid, (1, 0), colspan=3, fig=fig['date']),
        'date-acc': plt.subplot2grid(date_grid, (2, 0), colspan=3, fig=fig['date']),
        'date-acc-norm': plt.subplot2grid(date_grid, (3, 0), colspan=3, fig=fig['date']),
        #
        'left-line': plt.subplot2grid(left_grid, (0, 0), colspan=2, fig=fig['left']),
        'left-line-euclidean': plt.subplot2grid(left_grid, (0, 2), colspan=1, fig=fig['left']),
        'left-norm': plt.subplot2grid(left_grid, (1, 0), colspan=2, fig=fig['left']),
        'left-norm-euclidean': plt.subplot2grid(left_grid, (1, 2), colspan=1, fig=fig['left']),
        'left-acc': plt.subplot2grid(left_grid, (2, 0), colspan=2, fig=fig['left']),
        'left-acc-euclidean': plt.subplot2grid(left_grid, (2, 2), colspan=1, fig=fig['left']),
        'left-acc-norm': plt.subplot2grid(left_grid, (3, 0), colspan=2, fig=fig['left']),
        'left-acc-norm-euclidean': plt.subplot2grid(left_grid, (3, 2), colspan=1, fig=fig['left']),
        #
        'right-line': plt.subplot2grid(right_grid, (0, 0), colspan=2, fig=fig['right']),
        'right-line-euclidean': plt.subplot2grid(right_grid, (0, 2), colspan=1, fig=fig['right']),
        'right-norm': plt.subplot2grid(right_grid, (1, 0), colspan=2, fig=fig['right']),
        'right-norm-euclidean': plt.subplot2grid(right_grid, (1, 2), colspan=1, fig=fig['right']),
        'right-acc': plt.subplot2grid(right_grid, (2, 0), colspan=2, fig=fig['right']),
        'right-acc-euclidean': plt.subplot2grid(right_grid, (2, 2), colspan=1, fig=fig['right']),
        'right-acc-norm': plt.subplot2grid(right_grid, (3, 0), colspan=2, fig=fig['right']),
        'right-acc-norm-euclidean': plt.subplot2grid(right_grid, (3, 2), colspan=1, fig=fig['right']),
        #
        'max-peak-line': plt.subplot2grid(max_peak_grid, (0, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-line-euclidean': plt.subplot2grid(max_peak_grid, (0, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-norm': plt.subplot2grid(max_peak_grid, (1, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-norm-euclidean': plt.subplot2grid(max_peak_grid, (1, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-le-ge-zero': plt.subplot2grid(max_peak_grid, (2, 0), colspan=3, fig=fig['max-peak']),
        'max-peak-acc': plt.subplot2grid(max_peak_grid, (3, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-euclidean': plt.subplot2grid(max_peak_grid, (3, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-acc-norm': plt.subplot2grid(max_peak_grid, (4, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-norm-euclidean': plt.subplot2grid(max_peak_grid, (4, 2), colspan=1, fig=fig['max-peak']),
        #
        'time-between-all-peaks': plt.subplot2grid(time_between_grid, (0, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-all-peaks-euclidean': plt.subplot2grid(time_between_grid, (0, 2), colspan=1, fig=fig['time-between-peaks']),
        'time-between-up-peaks': plt.subplot2grid(time_between_grid, (1, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-up-peaks-euclidean': plt.subplot2grid(time_between_grid, (1, 2), colspan=1, fig=fig['time-between-peaks']),
        'time-between-down-peaks': plt.subplot2grid(time_between_grid, (2, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-down-peaks-euclidean': plt.subplot2grid(time_between_grid, (2, 2), colspan=1, fig=fig['time-between-peaks']),
    }
    # axes styling -----------------------------------------------------------------------------------------------------
    for key in ax:
        if key.endswith('-line'):
            __style_line__(ax[key])
        elif key.endswith('-acc-norm'):
            __style_acc_norm__(ax[key])
        elif key.endswith('-norm'):
            __style_norm__(ax[key])
        elif key.endswith('-acc'):
            __style_acc__(ax[key])
        elif key.endswith('-euclidean'):
            __style_euclidean_axes__(ax[key])
        elif key == 'max-peak-le-ge-zero':
            __style_max_peak_portion(ax[key])
        elif key.startswith('time-between-') and key.endswith('-peaks'):
            peak_type = key.split('-')[2]
            __style_time_between_peaks(ax[key], peak_type)
        else:
            print "[WARNING] No axes style define for key %s" % key
    # figure titles ----------------------------------------------------------------------------------------------------
    grouped_by_text = {"D": "day", "W": "week", "M": "month", "Q": "quarter", "A": "year"}[arg_time_unit]
    rolling_mean_text = ("(rolling mean with window=%s) " % arg_window) if arg_rollingmean else ""
    fig['date'].suptitle("Commit frequency %sgrouped by %s" % (rolling_mean_text, grouped_by_text))
    fig['left'].suptitle("Commit frequency (left shifted) %sgrouped by %s" % (rolling_mean_text, grouped_by_text))
    fig['right'].suptitle("Commit frequency (right shifted) %sgrouped by %s" % (rolling_mean_text, grouped_by_text))
    fig['max-peak'].suptitle(
        "Commit frequency (max peak shifted) %sgrouped by %s" % (rolling_mean_text, grouped_by_text))
    fig['time-between-peaks'].suptitle(
        "Time between peaks (values %sgrouped by %s before peak analysis)" % (rolling_mean_text, grouped_by_text))
    # ------------------------------------------------------------------------------------------------------------------
    return fig, ax
