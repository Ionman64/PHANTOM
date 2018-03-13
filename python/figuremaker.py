from sklearn.manifold import TSNE
from sklearn.decomposition import PCA
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
    return {
        'org': df_org,
        'util': df_util,
        'neg': df_neg
    }


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


# Assert command line args
assert len(sys.argv) > 1
arg1 = sys.argv[1]
assert os.path.isdir(arg1)
feature_vector_csv_dir = os.path.expanduser(arg1)
# Configure pandas options ----------------------------------------------------------
pd.set_option("display.max_rows", 500)
pd.set_option('display.expand_frame_repr', False)

# Load dataframes -------------------------------------------------------------------
frame_map = load_dataframes(binary_label=False, path=feature_vector_csv_dir)
frame = pd.concat(frame_map.values(), ignore_index=True)
labels = frame['label']
frame.drop('label', axis=1, inplace=True)
# df_all.fillna(0, inplace=True) TODO How to handle NaN values?


# Pre process frame
# frame.fillna(0, inplace=True) # TODO NaN values cannot be handled by t-SNE and PCA
# frame.drop('sum_y', axis=1, inplace=True)
# frame.drop('peak_down', axis=1, inplace=True)
# frame.drop('median_y', axis=1, inplace=True)
# frame.drop('min_amp', axis=1, inplace=True)
# frame.drop('max_amp', axis=1, inplace=True)

mmFrame = (frame - frame.min()) / (frame.max() - frame.min())
zFrame = (frame - frame.mean()) / frame.std()
# Plotting... -----------------------------------------------------------------------
# histograms(mmFrame, labels, "Commit Frequency")
# plt.savefig('/home/joshua/Documents/commit_frequency/hist.pdf')
# scatter_matrix(mmFrame, labels, "Commit Frequency")
# plt.savefig('/home/joshua/Documents/commit_frequency/scatter.pdf')


# tsne(frame, labels, "Commit Frequency", 2)
# pca(frame, labels, "Commit Frequency", 2)
# plt.show()

corr(mmFrame, "Commit Frequency")
corr(frame, "Commit Frequency")
plt.show()
