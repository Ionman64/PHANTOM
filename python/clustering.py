from sklearn.metrics import precision_recall_fscore_support, confusion_matrix
from sklearn import cluster
import pandas as pd
import numpy as np

def __convert_numeric_labels_to_string_label(numeric_labels, string_labels = ["P", "NP"]):
    assert len(np.unique(numeric_labels)) == 2
    assert len(string_labels) == 2

    converted_labels = pd.Series(numeric_labels)
    label_pos0 = converted_labels[0]
    other_label = abs(label_pos0 - 1)

    converted_labels.replace(to_replace=label_pos0, value=string_labels[0], inplace=True)
    converted_labels.replace(to_replace=other_label, value=string_labels[1], inplace=True)
    return converted_labels

def get_kmeans_model_and_labels(frame):
    km = cluster.KMeans(n_clusters=2)
    model = km.fit(frame.as_matrix())
    return model, __convert_numeric_labels_to_string_label(model.labels_)

def predict_and_get_labels(model, data):
    labels = model.predict(data.as_matrix())
    return __convert_numeric_labels_to_string_label(labels)

def print_results(true_labels, predicted_labels, cluster_names = ["P", "NP"]):
    p, r, f, s = precision_recall_fscore_support(y_true=true_labels, y_pred=predicted_labels, labels=cluster_names)
    print "Precision(P)  = %.2f" % p[0], "\t", "Recall(P)  = %.2f" % r[0], "\t", "F-Measure(P)  = %.2f" % f[0]
    print "Precision(NP) = %.2f" % p[1], "\t", "Recall(NP) = %.2f" % r[1], "\t", "F-Measure(NP) = %.2f" % f[1]

def get_tp_fp_tn_fn(true_labels, predicted_labels, cluster_names = ["P", "NP"]):
    type = []
    for (true, pred) in zip(true_labels, predicted_labels):
        if true == cluster_names[0] and pred == cluster_names[0]:
            type.append("true positive")
        elif true == cluster_names[0] and pred == cluster_names[1]:
            type.append("false negative")
        elif true == cluster_names[1] and pred == cluster_names[0]:
            type.append("false positive")
        elif true == cluster_names[1] and pred == cluster_names[1]:
            type.append("true negative")
    return np.array(type)