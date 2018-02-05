CREATE TABLE file_analysis (
    commit_id BIGSERIAL references repository_commit,
    file_id BIGSERIAL references commit_file,
    PRIMARY KEY (commit_id, file_id)
);