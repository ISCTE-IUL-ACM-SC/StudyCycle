#!/usr/bin/env bash
set -e

export STUDYCYCLE_DATABASE_URL=postgres://studycycle:password@localhost:5432
pushd ..
cargo build
rm target/studycycle_server || true
cp target/debug/studycycle_server target/studycycle_server
killall -s1 studycycle_server || true
./api_tests/prepare-drone-federation-test.sh
popd

pnpm i
pnpm api-test || true

killall -s1 studycycle_server || true
killall -s1 pict-rs || true
for INSTANCE in studycycle_alpha studycycle_beta studycycle_gamma studycycle_delta studycycle_epsilon; do
  psql "$STUDYCYCLE_DATABASE_URL" -c "DROP DATABASE $INSTANCE"
done
rm -r /tmp/pictrs
