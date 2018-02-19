CREATE TABLE file_analysis (
  file_id bigint NOT NULL,
  commit_hash char(40) NOT NULL,
  loc int NOT NULL,
  PRIMARY KEY (file_id, commit_hash)
);