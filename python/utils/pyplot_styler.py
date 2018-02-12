import matplotlib.pyplot as plt
import matplotlib.ticker as pltticker

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
    ax = {
        'date-line': plt.subplot2grid((4, 3), (0, 0), colspan=3, fig=fig['date']),
        'date-norm': plt.subplot2grid((4, 3), (1, 0), colspan=3, fig=fig['date']),
        'date-acc': plt.subplot2grid((4, 3), (2, 0), colspan=3, fig=fig['date']),
        'date-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=3, fig=fig['date']),
        #
        'left-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig['left']),
        'left-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig['left']),
        'left-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig['left']),
        'left-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig['left']),
        'left-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig['left']),
        'left-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig['left']),
        'left-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig['left']),
        'left-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig['left']),
        #
        'right-line': plt.subplot2grid((4, 3), (0, 0), colspan=2, fig=fig['right']),
        'right-line-euclidean': plt.subplot2grid((4, 3), (0, 2), colspan=1, fig=fig['right']),
        'right-norm': plt.subplot2grid((4, 3), (1, 0), colspan=2, fig=fig['right']),
        'right-norm-euclidean': plt.subplot2grid((4, 3), (1, 2), colspan=1, fig=fig['right']),
        'right-acc': plt.subplot2grid((4, 3), (2, 0), colspan=2, fig=fig['right']),
        'right-acc-euclidean': plt.subplot2grid((4, 3), (2, 2), colspan=1, fig=fig['right']),
        'right-acc-norm': plt.subplot2grid((4, 3), (3, 0), colspan=2, fig=fig['right']),
        'right-acc-norm-euclidean': plt.subplot2grid((4, 3), (3, 2), colspan=1, fig=fig['right']),
        #
        'max-peak-line': plt.subplot2grid((5, 3), (0, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-line-euclidean': plt.subplot2grid((5, 3), (0, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-norm': plt.subplot2grid((5, 3), (1, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-norm-euclidean': plt.subplot2grid((5, 3), (1, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-le-ge-zero': plt.subplot2grid((5, 3), (2, 0), colspan=3, fig=fig['max-peak']),
        'max-peak-acc': plt.subplot2grid((5, 3), (3, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-euclidean': plt.subplot2grid((5, 3), (3, 2), colspan=1, fig=fig['max-peak']),
        'max-peak-acc-norm': plt.subplot2grid((5, 3), (4, 0), colspan=2, fig=fig['max-peak']),
        'max-peak-acc-norm-euclidean': plt.subplot2grid((5, 3), (4, 2), colspan=1, fig=fig['max-peak']),
        #
        'time-between-all-peaks': plt.subplot2grid((3, 3), (0, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-all-peaks-euclidean': plt.subplot2grid((3, 3), (0, 2), colspan=1, fig=fig['time-between-peaks']),
        'time-between-up-peaks': plt.subplot2grid((3, 3), (1, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-up-peaks-euclidean': plt.subplot2grid((3, 3), (1, 2), colspan=1, fig=fig['time-between-peaks']),
        'time-between-down-peaks': plt.subplot2grid((3, 3), (2, 0), colspan=2, fig=fig['time-between-peaks']),
        'time-between-down-peaks-euclidean': plt.subplot2grid((3, 3), (2, 2), colspan=1, fig=fig['time-between-peaks']),
    }
    # axes styling -----------------------------------------------------------------------------------------------------
    ax['date-line'].set_ylabel("frequency")
    ax['date-norm'].set_ylabel("norm.")
    ax['date-acc'].set_ylabel("acc.")
    ax['date-acc-norm'].set_ylabel("acc. norm.")
    #
    ax['left-line'].set_ylabel("frequency")
    ax['left-norm'].set_ylabel("norm.")
    ax['left-acc'].set_ylabel("acc.")
    ax['left-acc-norm'].set_ylabel("acc. norm.")
    ax['left-line-euclidean'].set_ylabel("avg. distance")
    ax['left-line-euclidean'].yaxis.tick_right()
    ax['left-norm-euclidean'].set_ylabel("avg. distance")
    ax['left-norm-euclidean'].yaxis.tick_right()
    ax['left-acc-euclidean'].set_ylabel("avg. distance")
    ax['left-acc-euclidean'].yaxis.tick_right()
    ax['left-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['left-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['right-line'].set_ylabel("frequency")
    ax['right-norm'].set_ylabel("norm.")
    ax['right-acc'].set_ylabel("acc.")
    ax['right-acc-norm'].set_ylabel("acc. norm.")
    ax['right-line-euclidean'].set_ylabel("avg. distance")
    ax['right-line-euclidean'].yaxis.tick_right()
    ax['right-norm-euclidean'].set_ylabel("avg. distance")
    ax['right-norm-euclidean'].yaxis.tick_right()
    ax['right-acc-euclidean'].set_ylabel("avg. distance")
    ax['right-acc-euclidean'].yaxis.tick_right()
    ax['right-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['right-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['max-peak-line'].set_ylabel("frequency")
    ax['max-peak-norm'].set_ylabel("norm.")
    ax['max-peak-acc'].set_ylabel("acc.")
    ax['max-peak-acc-norm'].set_ylabel("acc. norm.")
    ax['max-peak-line-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-line-euclidean'].yaxis.tick_right()
    ax['max-peak-norm-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-norm-euclidean'].yaxis.tick_right()
    ax['max-peak-acc-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-acc-euclidean'].yaxis.tick_right()
    ax['max-peak-acc-norm-euclidean'].set_ylabel("avg. distance")
    ax['max-peak-acc-norm-euclidean'].yaxis.tick_right()
    #
    ax['max-peak-le-ge-zero'].set_ylabel("portion in %")
    ax['max-peak-le-ge-zero'].set_xlabel("project ID")
    #
    for tbp_axes_key in ['all', 'up', 'down']:
        tbp_ax = ax['time-between-%s-peaks' % tbp_axes_key]
        tbp_ax.set_title("Time between %s peaks" % tbp_axes_key)
        tbp_ax.set_ylabel("days")
        tbp_ax.set_xlabel("peak number")
        tbp_ax.xaxis.set_major_locator(pltticker.MultipleLocator(base=10))
        tbp_ax.xaxis.set_minor_locator(pltticker.MultipleLocator(base=1))

        tbp_ax = ax['time-between-%s-peaks-euclidean' % tbp_axes_key]
        tbp_ax.set_ylabel("avg. distance")
        tbp_ax.yaxis.tick_right()
    plt.figure(fig['time-between-peaks'].number)
    plt.tight_layout()
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