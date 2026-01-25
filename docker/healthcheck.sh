#!/bin/sh
# healthcheck.sh - Docker healthcheck script

set -e

wget --no-verbose --tries=1 --spider http://localhost:3000/api/health || exit 1
