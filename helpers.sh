REPO_ROOT=$(git rev-parse --show-toplevel)
COMPOSE_PROJECT_NAME="bdkct"

bitcoin_cli() {
  docker exec "${COMPOSE_PROJECT_NAME}-bitcoind-1" bitcoin-cli \
    -rpcuser="bitcoin" \
    -rpcpassword="bitcoin" \
    -rpcconnect="127.0.0.1" \
    -rpcport="18443" \
    "$@"
}
