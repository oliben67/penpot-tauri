#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="$SCRIPT_DIR/docker-compose.yml"
PENPOT_URL="${PENPOT_URL:-http://localhost:9001}"
BACKEND_HOST="localhost"
BACKEND_PORT="9001"
WAIT_TIMEOUT=120

echo "==> Starting penpot stack via docker compose..."
docker compose -f "$COMPOSE_FILE" up -d

echo "==> Waiting for penpot to be ready at $PENPOT_URL (timeout: ${WAIT_TIMEOUT}s)..."
elapsed=0
until nc -z "$BACKEND_HOST" "$BACKEND_PORT" 2>/dev/null; do
  if [ "$elapsed" -ge "$WAIT_TIMEOUT" ]; then
    echo "ERROR: penpot did not become ready within ${WAIT_TIMEOUT}s." >&2
    exit 1
  fi
  printf "."
  sleep 2
  elapsed=$((elapsed + 2))
done
echo ""

# Give the backend a couple more seconds to finish initialising after the port opens
sleep 3

echo "==> Installing npm dependencies (if needed)..."
npm install --prefix "$SCRIPT_DIR" --silent

echo "==> Launching penpot-electron -> $PENPOT_URL"
PENPOT_URL="$PENPOT_URL" npm start --prefix "$SCRIPT_DIR"
