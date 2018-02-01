#!/bin/bash
conn_url=$1
test_db=$2
production_db=project_analyser

echo "DROP DATABASE IF EXISTS $test_db; CREATE DATABASE $test_db;" | psql $conn_url

pg_dump $conn_url/$production_db --schema-only > db_schema.sql
psql $conn_url/$test_db < db_schema.sql
rm db_schema.sql