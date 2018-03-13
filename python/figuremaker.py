from sklearn.manifold import TSNE
from sklearn.decomposition import PCA
from sklearn.metrics import precision_recall_fscore_support
from sklearn import cluster
from mpl_toolkits.mplot3d import Axes3D  # 3D plots
import matplotlib.pyplot as plt
import matplotlib.gridspec as gridspec
import seaborn as sns
import pandas as pd
import numpy as np
import sys
import os


def load_dataframes(binary_label, path):
    path_org = str(os.path.join(path, "organization.csv"))
    path_util = str(os.path.join(path, "utility.csv"))
    path_neg = str(os.path.join(path, "negative_instances.csv"))

    df_org = pd.read_csv(path_org, index_col=0)
    df_util = pd.read_csv(path_util, index_col=0)
    df_neg = pd.read_csv(path_neg, index_col=0)

    df_org['label'] = 'Org' if not binary_label else 'P'
    df_util['label'] = 'Util' if not binary_label else 'P'
    df_neg['label'] = 'Neg' if not binary_label else 'NP'

    map = {
        #'org': df_org,
        'util': df_util,
        'neg': df_neg
    }
    return map


def load_validation_dataframes(binary_label, path):
    path_val_p = str(os.path.join(path, "validation_p.csv"))
    path_val_np = str(os.path.join(path, "validation_np.csv"))

    df_val_p = pd.read_csv(path_val_p, index_col=0)
    df_val_np = pd.read_csv(path_val_np, index_col=0)

    df_val_p['label'] = 'VP' if not binary_label else 'P'
    df_val_np['label'] = 'VNP' if not binary_label else 'NP'

    map = {
        'VP': df_val_p,
        'VNP': df_val_np,
    }
    return map


def histograms(frame, labels, measure_name):
    fig = plt.figure(figsize=(20, 10))
    for num, col in enumerate(frame.columns):
        ax = plt.subplot(4, 5, num + 1)  # TODO calculate shape to fit the frame

        data = []
        for lbl in labels.unique():
            data.append(frame[labels == lbl][col].dropna())
        bins = 5
        ax.hist(data, bins=bins, label=labels.unique(), rwidth=0.2, histtype='step')
        plt.title(col)
    fig.legend(labels.unique(), loc='lower right', prop={'size': 12})
    plt.suptitle("Feature Vector %s Histogram" % measure_name)
    plt.tight_layout(rect=(0, 0, 1, 0.975))


def scatter_matrix(frame, labels, measure_name):
    fig = plt.figure(figsize=(20, 10))
    grid = gridspec.GridSpec(len(frame.columns), len(frame.columns), wspace=0, hspace=0)
    for i, col1 in enumerate(frame.columns):
        for j, col2 in enumerate(frame.columns):
            if i < j:
                continue

            ax = fig.add_subplot(grid[i, j])
            ax.xaxis.set_visible(False)
            ax.yaxis.set_visible(False)

            for lbl in labels.unique():
                x, y = frame[labels == lbl][col1], frame[labels == lbl][col2]
                ax.scatter(x, y, label=lbl, alpha=0.8, marker='x', s=30)

            if i - j == 0:
                ax.set_title(col2, fontsize=12)
            if j == 0:
                ax.set_yticklabels([])
                ax.set_yticks([])
                ax.yaxis.set_visible(True)
                ax.set_ylabel(col1, fontsize=12, rotation=0, labelpad=35)
    fig.legend(labels.unique(), loc='upper right', prop={'size': 12})
    plt.suptitle("Feature Vector %s Correlations" % measure_name)
    plt.tight_layout(pad=1.5, h_pad=0, w_pad=0)


def corr(frame, measure_name):
    mat = frame.corr()
    sns.heatmap(mat, xticklabels=mat.columns, yticklabels=mat.columns)
    plt.suptitle("Feature Vector %s Correlation Matrix" % measure_name)
    plt.tight_layout(pad=3)


def tsne(frame, labels, measure_name, n_components):
    assert n_components == 2 or 3
    model = TSNE(n_components=n_components, random_state=0)
    transformed = model.fit_transform(frame)

    fig = plt.figure()

    if n_components == 2:
        ax = fig.add_subplot(111)
    elif n_components == 3:
        ax = fig.add_subplot(111, projection='3d')

    for lbl in labels.unique():
        label_idx = np.where(labels == lbl)
        if n_components == 2:
            ax.scatter(transformed[label_idx, 0], transformed[label_idx, 1], marker='x', label=lbl)
        elif n_components == 3:
            ax.scatter(transformed[label_idx, 0], transformed[label_idx, 1], transformed[label_idx, 2], marker='x',
                       label=lbl)
    plt.legend(loc='best')
    plt.suptitle("Feature Vector %s t-SNE" % measure_name)
    plt.tight_layout(pad=3, h_pad=0, w_pad=0)


def pca(frame, labels, measure_name, n_components):
    assert n_components == 2 or 3
    model = PCA(n_components=n_components)
    transformed = model.fit_transform(frame)

    fig = plt.figure()

    if n_components == 2:
        ax = fig.add_subplot(111)
    elif n_components == 3:
        ax = fig.add_subplot(111, projection='3d')

    for lbl in labels.unique():
        label_idx = np.where(labels == lbl)
        if n_components == 2:
            ax.scatter(transformed[label_idx, 0], transformed[label_idx, 1], marker='x', label=lbl)
        elif n_components == 3:
            ax.scatter(transformed[label_idx, 0], transformed[label_idx, 1], transformed[label_idx, 2], marker='x',
                       label=lbl)
    plt.legend(loc='best')
    plt.suptitle("Feature Vector %s PCA" % measure_name)
    plt.tight_layout(pad=3, h_pad=0, w_pad=0)


def kmeans(frame, labels, measure_name):
    # setup
    n_clusters = 2
    km = cluster.KMeans(n_clusters=n_clusters)
    # input
    mat = frame.as_matrix()
    fit = km.fit(mat)
    # get labels
    fitted_labels = pd.Series(fit.labels_)
    cluster_p = fitted_labels[0]
    cluster_np = abs(cluster_p - 1)
    fitted_labels.replace(to_replace=cluster_p, value='P', inplace=True)
    fitted_labels.replace(to_replace=cluster_np, value='NP', inplace=True)
    # compare to ground truth
    p, r, f, s = precision_recall_fscore_support(y_true=labels, y_pred=fitted_labels, labels=['P', 'NP'])
    print "Measured against true labels of training data"
    print "Precision(P)  = %.2f" % p[0]
    print "Precision(NP) = %.2f" % p[1]
    print
    print "Recall(P)     = %.2f" % r[0]
    print "Recall(NP)    = %.2f" % r[1]
    print
    print "F-Measure(P)  = %.2f" % f[0]
    print "F-Measure(NP) = %.2f" % f[1]
    # compare to validation
    val_frames = load_validation_dataframes(binary_label=True, path=os.path.expanduser(sys.argv[1]))
    val_frames = pd.concat(val_frames.values(), ignore_index=True)
    labels = val_frames['label']
    val_frames.fillna(0, inplace=True)
    fitted_labels = pd.Series(fit.predict(val_frames.drop('label', axis=1)))
    fitted_labels.replace(to_replace=cluster_p, value='P', inplace=True)
    fitted_labels.replace(to_replace=cluster_np, value='NP', inplace=True)
    p, r, f, s = precision_recall_fscore_support(y_true=labels, y_pred=fitted_labels, labels=['P', 'NP'])
    print "Measured against true labels of validation data"
    print "Precision(P)  = %.2f" % p[0]
    print "Precision(NP) = %.2f" % p[1]
    print
    print "Recall(P)     = %.2f" % r[0]
    print "Recall(NP)    = %.2f" % r[1]
    print
    print "F-Measure(P)  = %.2f" % f[0]
    print "F-Measure(NP) = %.2f" % f[1]


# Assert command line args
assert len(sys.argv) > 1
arg1 = sys.argv[1]
assert os.path.isdir(arg1)
feature_vector_csv_dir = os.path.expanduser(arg1)
# Configure pandas options ----------------------------------------------------------
pd.set_option("display.max_rows", 500)
pd.set_option('display.expand_frame_repr', False)

# Load dataframes -------------------------------------------------------------------
frame_map = load_dataframes(binary_label=True, path=feature_vector_csv_dir)
frame = pd.concat(frame_map.values(), ignore_index=True)
labels = frame['label']
frame.drop('label', axis=1, inplace=True)
# df_all.fillna(0, inplace=True) TODO How to handle NaN values?


# Pre process frame
frame.fillna(0, inplace=True)  # TODO NaN values cannot be handled by t-SNE and PCA

mmFrame = (frame - frame.min()) / (frame.max() - frame.min())
zFrame = (frame - frame.mean()) / frame.std()
# Plotting... -----------------------------------------------------------------------
# histograms(frame, labels, "Commit Frequency")
# plt.savefig('/home/joshua/Documents/commit_frequency/hist.pdf')
# scatter_matrix(mmFrame, labels, "Commit Frequency")
# plt.savefig('/home/joshua/Documents/commit_frequency/scatter.pdf')


# tsne(frame, labels, "Commit Frequency", 3)#
# pca(frame, labels, "Commit Frequency", 3)
# plt.show()

# corr(mmFrame, "Commit Frequency")
# corr(frame, "Commit Frequency")
kmeans(mmFrame, labels, "Commit Frequency")
# plt.show()
