#!/bin/bash
export PGPASSWORD=$1
test_db_name=test_pa

dropdb --host localhost --username postgres $test_db_name --if-exists
createdb --host localhost --username postgres $test_db_name

pg_dump postgres://postgres:0000@localhost/project_analyser --schema-only > db_schema.sql

psql postgres://postgres:0000@localhost/$test_db_name < db_schema.sql

rm db_schema.sql