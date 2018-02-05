CREATE TABLE commit (
    commit_id BIGSERIAL PRIMARY KEY,
    repository_id BIGSERIAL references git_repository,
    commit_hash char(40) NOT NULL,
    commit_date date NOT NULL,
    UNIQUE (commit_id, repository_id)
);