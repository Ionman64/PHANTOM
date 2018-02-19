table! {
    commit_file (file_id) {
        file_id -> Int8,
        commit_hash -> Bpchar,
        repository_id -> Int8,
        file_path -> Text,
        action -> Bpchar,
    }
}

table! {
    file_analysis (file_id, commit_hash) {
        file_id -> Int8,
        commit_hash -> Bpchar,
        loc -> Int4,
    }
}

table! {
    git_repository (id) {
        id -> Int8,
        url -> Text,
    }
}

table! {
    repository_commit (repository_id, commit_hash) {
        repository_id -> Int8,
        commit_hash -> Bpchar,
        commit_date -> Timestamp,
    }
}

joinable!(repository_commit -> git_repository (repository_id));

allow_tables_to_appear_in_same_query!(
    commit_file,
    file_analysis,
    git_repository,
    repository_commit,
);
