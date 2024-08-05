
. "${BASE_DIR}/ci/common_names.sh"

export PREFIX_PATH="${DIST_DIR}/usr/local"
export LD_LIBRARY_PATH="${LD_LIBRARY_PATH-""}:${PREFIX_PATH}/lib:${BASE_DIR}/build/deps/lib"
export PATH="${PATH}:${PREFIX_PATH}/bin:${DEPENDENCY_DIR}/bin"

export TEST_SOURCE_DIR="${BASE_DIR}/test/smoketest"
export SMOKETEST_DIR=${SMOKETEST_DIR-$TEST_SOURCE_DIR}
export SMOKETEST_RESULT_DIR=${SMOKETEST_RESULT_DIR-"$BUILD_DIR/result/smoketest_results"}
export SMOKETEST_TMP_DIR="${SMOKETEST_TMP_DIR-"/tmp/elosd"}"

export ELOS_CLIENT_PATH=${ELOS_CLIENT_PATH-"${PREFIX_PATH}/lib/elos/client"}
export ELOS_BACKEND_PATH=${ELOS_BACKEND_PATH-"${PREFIX_PATH}/lib/elos/backend"}
export ELOS_SCANNER_PATH=${ELOS_SCANNER_PATH-"${PREFIX_PATH}/lib/elos/scanner"}
export ELOS_LOG_LEVEL=DEBUG

export ELOS_CONFIG_PATH=${ELOS_CONFIG_PATH-"$SMOKETEST_DIR/config.json"}

# to make sure the legacy scanner manager doesn't crash elosd
mkdir -p "${ELOS_SCANNER_PATH}"
cp "${DEPENDENCY_DIR}/lib/elos/scanner/scanner_shmem.so" "${ELOS_SCANNER_PATH}"
# to make sure the smoketest finds a tcp port and knows elosd is running
cp "${DEPENDENCY_DIR}/lib/elos/client/client_tcp.so" "${ELOS_CLIENT_PATH}"
