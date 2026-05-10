#!/usr/bin/env bash

psql -U studycycle -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public;"
cat docker/lemmy_dump_2021-01-29_16_13_40.sqldump | psql -U studycycle
psql -U studycycle -c "alter user studycycle with password 'password'"
