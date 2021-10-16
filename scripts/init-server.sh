#! /bin/bash
PORT=${PORT:-'3000'}
HOST=${HOST:-'0.0.0.0'}

COMMAND="cargo watch -x run"

if [ ${DEBUG} = 'True' ]; then
    echo "Starting server in DEBUG mode and RELOAD"
    COMMAND="${COMMAND}"
fi

eval "PORT=${PORT} HOST=${HOST} ${COMMAND}"
