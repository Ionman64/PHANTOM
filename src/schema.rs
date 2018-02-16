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

joinable!(commit_file -> git_repository (repository_id));
joinable!(repository_commit -> git_repository (repository_id));

allow_tables_to_appear_in_same_query!(
    commit_file,
    git_repository,
    repository_commit,
);
