#!/usr/bin/env bash
# IMPORTANT NOTE: this script does not use the normal STUDYCYCLE_DATABASE_URL format
#   it is expected that this script is called by run-federation-test.sh script.
set -e

if [ -z "$STUDYCYCLE_LOG_LEVEL" ]; then
  STUDYCYCLE_LOG_LEVEL=info
fi

export RUST_BACKTRACE=1
export RUST_LOG="warn,studycycle_server=$STUDYCYCLE_LOG_LEVEL,studycycle_federate=$STUDYCYCLE_LOG_LEVEL,studycycle_api=$STUDYCYCLE_LOG_LEVEL,studycycle_api_common=$STUDYCYCLE_LOG_LEVEL,studycycle_api_crud=$STUDYCYCLE_LOG_LEVEL,studycycle_apub=$STUDYCYCLE_LOG_LEVEL,studycycle_db_schema=$STUDYCYCLE_LOG_LEVEL,studycycle_db_views=$STUDYCYCLE_LOG_LEVEL,studycycle_routes=$STUDYCYCLE_LOG_LEVEL,studycycle_utils=$STUDYCYCLE_LOG_LEVEL,studycycle_websocket=$STUDYCYCLE_LOG_LEVEL"

export STUDYCYCLE_TEST_FAST_FEDERATION=1 # by default, the persistent federation queue has delays in the scale of 30s-5min

PICTRS_PATH="api_tests/pict-rs"
PICTRS_EXPECTED_HASH="7f7ac2a45ef9b13403ee139b7512135be6b060ff2f6460e0c800e18e1b49d2fd  api_tests/pict-rs"

# Pictrs setup. Download file with hash check and up to 3 retries.
if [ ! -f "$PICTRS_PATH" ]; then
  count=0
  while [ ! -f "$PICTRS_PATH" ] && [ "$count" -lt 3 ]; do
    # This one sometimes goes down
    curl "https://git.asonix.dog/asonix/pict-rs/releases/download/v0.5.17-pre.9/pict-rs-linux-amd64" -o "$PICTRS_PATH"
    # curl "https://codeberg.org/asonix/pict-rs/releases/download/v0.5.5/pict-rs-linux-amd64" -o "$PICTRS_PATH"
    PICTRS_HASH=$(sha256sum "$PICTRS_PATH")
    if [[ "$PICTRS_HASH" != "$PICTRS_EXPECTED_HASH" ]]; then
      echo "Pictrs binary hash mismatch, was $PICTRS_HASH but expected $PICTRS_EXPECTED_HASH"
      rm "$PICTRS_PATH"
      let count=count+1
    fi
  done
  chmod +x "$PICTRS_PATH"
fi

./api_tests/pict-rs \
  run -a 0.0.0.0:8080 \
  --danger-dummy-mode \
  --api-key "my-pictrs-key" \
  filesystem -p /tmp/pictrs/files \
  sled -p /tmp/pictrs/sled-repo 2>&1 &

for INSTANCE in studycycle_alpha studycycle_beta studycycle_gamma studycycle_delta studycycle_epsilon; do
  echo "DB URL: ${STUDYCYCLE_DATABASE_URL} INSTANCE: $INSTANCE"
  psql "${STUDYCYCLE_DATABASE_URL}/studycycle" -c "DROP DATABASE IF EXISTS $INSTANCE"
  echo "create database"
  psql "${STUDYCYCLE_DATABASE_URL}/studycycle" -c "CREATE DATABASE $INSTANCE"
done

if [ -z "$DO_WRITE_HOSTS_FILE" ]; then
  if ! grep -q studycycle-alpha /etc/hosts; then
    echo "Please add the following to your /etc/hosts file, then press enter:

      127.0.0.1       studycycle-alpha
      127.0.0.1       studycycle-beta
      127.0.0.1       studycycle-gamma
      127.0.0.1       studycycle-delta
      127.0.0.1       studycycle-epsilon"
    read -p ""
  fi
else
  for INSTANCE in studycycle-alpha studycycle-beta studycycle-gamma studycycle-delta studycycle-epsilon; do
    echo "127.0.0.1 $INSTANCE" >>/etc/hosts
  done
fi

echo "$PWD"

LOG_DIR=target/log
mkdir -p $LOG_DIR

echo "start alpha"
STUDYCYCLE_CONFIG_LOCATION=./docker/federation/studycycle_alpha.hjson \
  STUDYCYCLE_DATABASE_URL="${STUDYCYCLE_DATABASE_URL}/studycycle_alpha" \
  target/studycycle_server >$LOG_DIR/studycycle_alpha.out 2>&1 &

echo "start beta"
STUDYCYCLE_CONFIG_LOCATION=./docker/federation/studycycle_beta.hjson \
  STUDYCYCLE_DATABASE_URL="${STUDYCYCLE_DATABASE_URL}/studycycle_beta" \
  target/studycycle_server >$LOG_DIR/studycycle_beta.out 2>&1 &

echo "start gamma"
STUDYCYCLE_CONFIG_LOCATION=./docker/federation/studycycle_gamma.hjson \
  STUDYCYCLE_DATABASE_URL="${STUDYCYCLE_DATABASE_URL}/studycycle_gamma" \
  target/studycycle_server >$LOG_DIR/studycycle_gamma.out 2>&1 &

echo "start delta"
STUDYCYCLE_CONFIG_LOCATION=./docker/federation/studycycle_delta.hjson \
  STUDYCYCLE_DATABASE_URL="${STUDYCYCLE_DATABASE_URL}/studycycle_delta" \
  target/studycycle_server >$LOG_DIR/studycycle_delta.out 2>&1 &

echo "start epsilon"
STUDYCYCLE_CONFIG_LOCATION=./docker/federation/studycycle_epsilon.hjson \
  STUDYCYCLE_PLUGIN_PATH=api_tests/plugins \
  STUDYCYCLE_DATABASE_URL="${STUDYCYCLE_DATABASE_URL}/studycycle_epsilon" \
  target/studycycle_server >$LOG_DIR/studycycle_epsilon.out 2>&1 &

echo "wait for all instances to start"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'studycycle-alpha:8541/api/v4/site')" != "200" ]]; do sleep 1; done
echo "alpha started"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'studycycle-beta:8551/api/v4/site')" != "200" ]]; do sleep 1; done
echo "beta started"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'studycycle-gamma:8561/api/v4/site')" != "200" ]]; do sleep 1; done
echo "gamma started"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'studycycle-delta:8571/api/v4/site')" != "200" ]]; do sleep 1; done
echo "delta started"
while [[ "$(curl -s -o /dev/null -w '%{http_code}' 'studycycle-epsilon:8581/api/v4/site')" != "200" ]]; do sleep 1; done
echo "epsilon started. All started"
