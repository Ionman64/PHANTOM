table! {
    commit_frequency (repository_id, commit_date) {
        repository_id -> Int8,
        commit_date -> Timestamp,
        frequency -> Int2,
    }
}

table! {
    git_repository (id) {
        id -> Int8,
        url -> Text,
    }
}

joinable!(commit_frequency -> git_repository (repository_id));

allow_tables_to_appear_in_same_query!(
    commit_frequency,
    git_repository,
);
