#!/bin/sh
set -eu
activity_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
implementation=${1:-reference}
case "$implementation" in reference|starter) ;; *) echo "Choose reference or starter" >&2; exit 2;; esac
test_dir=$(mktemp -d)
trap 'rm -f "$test_dir/test_logic"; rmdir "$test_dir"' EXIT HUP INT TERM
cc -std=c11 -Wall -Wextra -Werror -pedantic -I "$activity_dir/$implementation" "$activity_dir/$implementation/logic.c" "$activity_dir/tests/test_logic.c" -o "$test_dir/test_logic"
"$test_dir/test_logic"
