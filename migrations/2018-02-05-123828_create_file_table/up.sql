CREATE TABLE commit_file (
    file_id BIGSERIAL,
    commit_id BIGSERIAL references repository_commit,
    file_path TEXT NOT NULL,
    PRIMARY KEY(file_id, commit_id)
);