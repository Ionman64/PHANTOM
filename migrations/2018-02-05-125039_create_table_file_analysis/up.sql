CREATE TABLE file_analysis (
    file_id BIGINT,
    commit_id BIGINT,
    FOREIGN KEY (file_id, commit_id) REFERENCES commit_file(file_id, commit_id)
);