CREATE TABLE git_repository (
    id BIGSERIAL NOT NULL PRIMARY KEY,
    url text NOT NULL UNIQUE
);

CREATE TABLE repository_commit (
    repository_id bigint NOT NULL REFERENCES git_repository,
    commit_hash char(40) NOT NULL,
    commit_date timestamp without time zone NOT NULL,
    PRIMARY KEY(repository_id, commit_hash)
);


CREATE TABLE commit_file (
    file_id BIGSERIAL NOT NULL PRIMARY KEY,
    commit_hash char(40) NOT NULL,
    repository_id bigint NOT NULL,
    file_path text NOT NULL,
    action char(1) NOT NULL,
    FOREIGN KEY (repository_id, commit_hash) REFERENCES repository_commit,
    UNIQUE (repository_id, commit_hash, file_path)
);
--CREATE TABLE file_analysis (
 --   file_id bigint NOT NULL,
   -- commit_id bigint NOT NULL
--);




