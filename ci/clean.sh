#!/bin/sh -eu
set -e -u

CMD_PATH="$(realpath "$(dirname "$0")")"
BASE_DIR="$(realpath "$CMD_PATH/..")"

OPTION_DEPS=0
for element in "$@"; do
    case $element in
        --all|-a|--deps)  OPTION_DEPS=1 ;;
	--no-deps)        OPTION_DEPS=0 ;;
        *)                echo "error: unknown option: $1"; exit 1 ;;
    esac
done

echo "remove build directories"
if [ $OPTION_DEPS -eq 1 ]; then
  rm -rf "$BASE_DIR/build"
else
  rm -rf \
    "$BASE_DIR/build/Debug" \
    "$BASE_DIR/build/Release"
fi
