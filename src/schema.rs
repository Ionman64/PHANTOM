table! {
    commit_file (file_id, commit_id) {
        file_id -> Int8,
        commit_id -> Int8,
        file_path -> Text,
    }
}

table! {
    commit_frequency (repository_id, commit_date) {
        repository_id -> Int8,
        commit_date -> Timestamp,
        frequency -> Int2,
    }
}

table! {
    file_analysis (file_id, commit_id) {
        file_id -> Int8,
        commit_id -> Int8,
    }
}

table! {
    git_repository (id) {
        id -> Int8,
        url -> Text,
    }
}

table! {
    repository_commit (commit_id) {
        commit_id -> Int8,
        repository_id -> Int8,
        commit_hash -> Bpchar,
        commit_date -> Timestamp,
    }
}

joinable!(commit_file -> repository_commit (commit_id));
joinable!(commit_frequency -> git_repository (repository_id));
joinable!(repository_commit -> git_repository (repository_id));

allow_tables_to_appear_in_same_query!(
    commit_file,
    commit_frequency,
    file_analysis,
    git_repository,
    repository_commit,
);
