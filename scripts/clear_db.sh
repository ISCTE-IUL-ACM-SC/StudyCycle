#!/usr/bin/env bash

psql -U studycycle -c "DROP SCHEMA public CASCADE; CREATE SCHEMA public; DROP SCHEMA utils CASCADE;"
