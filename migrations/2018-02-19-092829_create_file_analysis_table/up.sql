CREATE TABLE file_analysis (
  file_id bigint NOT NULL REFERENCES commit_file,
  commit_hash char(40) NOT NULL,
  loc int NOT NULL,
  PRIMARY KEY (file_id, commit_hash)
);
--Should be referencing repository_commit on hash