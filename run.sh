#!/usr/bin/env bash

# @see https://intoli.com/blog/exit-on-errors-in-bash-scripts/
set -e

trap 'LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ $FAIL == 1 ]; then fail finished with error; else pass "finished all";fi' EXIT

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
DIRNAME="${SRCDIR##*/}"

main() {
    FAIL=0;
    process_arguments "$@"
}

function process_arguments() {
    # Help if no arguments
    if [[ $# -eq 0 ]] ; then help ; exit 0 ; fi

    ## Processing rest of arguments
    for arg in "$@"; do
    case "$arg" in

    check) check;;
    watch) watch;;
    wtest) wtest;;
    wcheck) wcheck;;
    doc) doc;;
    help) help;;
    esac
    done

    # TODO: should implement check whether there is unknown argument
    # does not work if [ ! -z $1 ]; then fail "Unknown argument: $1"; trap - EXIT; exit 1 ; fi
}


function check() {
    if !(cargo clippy --version); then
        fail 'Checks requires clippy; try `rustup component add clippy-preview`.'
        fail "If that doesn't fix things; try `rustup self update`."
        exit 1
    fi

    if !(cargo fmt --version); then
        fail 'Checks requires rustfmt; try `rustup component add rustfmt-preview`.'
        fail "If that doesn't fix things; try `rustup self update`."
        exit 1
    fi

    info 'Checking compiler warnings:'
    if ! (RUSTFLAGS="-D warnings" cargo check --all); then
        fail 'Cargo check produced errors. Exiting.'
        exit $?
    fi
    pass 'Check passed!'

    info 'Running rustfmt'

    if ! (cargo fmt --all -- --check); then
        fail 'Rustfmt failed. Please run rustfmt.'
        exit $?
    fi

    pass 'Rustfmt check passed'

    info 'Running clippy'

    if ! (cargo clippy --all -- -D warnings); then
        fail 'Clippy has nits. :('
        exit $?
    fi
    pass 'Clippy check passed'

    pass ${BLUE}'All checks passed'${NC}
}

function wtest() {
  cargo watch -i data -i cfg -x 'test --all -- --nocapture'
}

function watch() {
  cargo watch -i data -i cfg -s 'cargo test --all -- --nocapture;cargo build'
}

function wcheck() {
  cargo watch -i data -i cfg -s './run.sh check'
}


function doc() {
  cargo doc -p server --no-deps --open
}

function help() {
    info "Build script v1.0"
    echo "start     Starts server in background"
    echo "stop      Kills server running in background"
    echo "test      Tests 'python modules"
    echo "testssh   Tests connection to 'loremipsum'"
    echo "bench     Benchmark"
    echo "check     Checks if python and other stuff is installed"
    echo "serve     Start http server"
    echo "doc       Generate and open documentation"
}

# ################
# Output funcitons
# ################
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

function info () {
  echo -e "[${BLUE} $@ ${NC}]"
}

function pass() {
  echo -e "[${GREEN}PASS${NC}] $@"
}

function fail() {
  FAIL=1; echo -e "[${RED}FAIL${NC}] $@"
}

# Run main function
main "$@"
