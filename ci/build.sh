#!/bin/bash
set -e -u

CMD_PATH=$(realpath "$(dirname "$0")")
BASE_DIR=${CMD_PATH%/*}

CMAKE_PARAM=${CMAKE_PARAM:-""}
NINJA_PARAM=${NINJA_PARAM:-"-j$(nproc)"}

PARAM=""
OPTION_CLEAN=0
OPTION_VERBOSE=0
for element in "$@"; do
    case $element in
        --clean|-c)    OPTION_CLEAN=1 ;;
        --verbose|-v)  OPTION_VERBOSE=1 ;;
        -*)          echo "error: unknown option: $1"; exit 1 ;;
        *)           PARAM="$PARAM $element" ;;
    esac
done

set -- $PARAM
if [ $# -gt 1 ]; then
    echo "error: only one build-type allowed"
    exit 1
fi

BUILD_TYPE="${1:-Debug}"

. "$BASE_DIR/ci/common_names.sh"

CMAKE_BUILD_DIR=${BUILD_DIR}/cmake
export PREFIX_PATH="${DIST_DIR}/usr/local"
CMAKE_PARAM="${CMAKE_PARAM} -DCMAKE_PREFIX_PATH=${BASE_DIR}/build/deps"
CMAKE_PARAM="${CMAKE_PARAM} -DCMAKE_INSTALL_PREFIX:PATH=${PREFIX_PATH}"

if [ $OPTION_CLEAN -eq 1 ]; then
    if [ -e "$BUILD_DIR" ]; then
        echo "Removing ${BUILD_DIR} ..."
        rm -rf "$BUILD_DIR"
    fi
fi
if [ $OPTION_VERBOSE -eq 1 ]; then
    NINJA_PARAM="${NINJA_PARAM} -v"
fi

echo -e "\n#### Building ${PROJECT} (${BUILD_TYPE}) ####"
mkdir -p "${RESULT_DIR}" "${DIST_DIR}"
if [ ! -e "${CMAKE_BUILD_DIR}/build.ninja" ]; then
    cmake -B "${CMAKE_BUILD_DIR}" "${BASE_DIR}" "-DCMAKE_BUILD_TYPE=${BUILD_TYPE}" -G Ninja $CMAKE_PARAM
fi

ninja -C "${CMAKE_BUILD_DIR}" $NINJA_PARAM all install 2>&1 | tee "${RESULT_DIR}/build_log.txt"

re=${PIPESTATUS[0]}

"$BASE_DIR/ci/check_build_log.py" "$RESULT_DIR/build_log.txt"

exit "$re"
