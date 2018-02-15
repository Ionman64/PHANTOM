CREATE TABLE commit_frequency (
    repository_id BIGSERIAL references git_repository,
    commit_date date NOT NULL,
    frequency smallint NOT NULL CONSTRAINT positive_frequency CHECK (frequency > 0),
    PRIMARY KEY (repository_id, commit_date)
);