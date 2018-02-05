CREATE TABLE commit_file (
    file_id BIGSERIAL PRIMARY KEY,
    commit_id BIGSERIAL references repository_commit,
    file_path TEXT NOT NULL,
    UNIQUE(file_id, commit_id)
);