#!/bin/bash
# shellcheck disable=SC2317

CMDPATH=$(realpath "$(dirname "$0")")
BASE_DIR=$(realpath "${CMDPATH}/../..")

BUILD_TYPE="${BUILD_TYPE-Debug}"
ELOSD_PORT=54323

. "${CMDPATH}/smoketest_env.sh"

export NETSTAT=$(which netstat 2>/dev/null || which ss 2> /dev/null || echo "no ${NETSTAT} compliant tool found")
export SMOKETEST_ENABLE_COMPILE_TESTS="${SMOKETEST_ENABLE_COMPILE_TESTS-""}"

prepare_env() {
    test_name=${1?:"first parameter missing"}

    result_dir="${SMOKETEST_RESULT_DIR}/${test_name}"
    if [ -e "${result_dir}" ]
    then
        rm -rf "${result_dir}"
    fi
    mkdir -p "${result_dir}"

    if [ -e "${SMOKETEST_TMP_DIR}" ]
    then
        rm -rf "${SMOKETEST_TMP_DIR}"
    fi
    mkdir -p "${SMOKETEST_TMP_DIR}"

    export RESULT_DIR="${result_dir}"

    . "${CMDPATH}/smoketest_log.sh"
    setup_log

    ELOSD_PIDS=$(pgrep elosd || echo "")
    for ELOSD_PID in $ELOSD_PIDS; do
       find /proc/$$ -type d -name "${ELOSD_PID}" >/dev/null 2>&1
       if [ $? -ne 0 ]; then
           log "Found elosd from other process".
           continue
       fi
       log "Existing instance of elosd found ($ELOSD_PID), terminating..."
       kill -15 "${ELOSD_PID}"
       wait "${ELOSD_PID}" > /dev/null
       log "done ($?)"
       sleep 2s
    done

    export ELOS_STORAGE_BACKEND_JSONBACKEND_FILE="${result_dir}/elosd_event_%count%.log"
}

wait_for_file() {
    local i=0
    while [ ! -e "${1}" ]
    do
      i=$((i+1))
      sleep 0.1s
      if [ "${i}" -gt 100 ]; then
         log "Error: Waiting for file $1 timed out"
         exit 124
      fi
    done

}

wait_for_elosd_socket() {
    local i=0
    ${NETSTAT} -l | grep "${ELOSD_PORT}" | grep tcp > /dev/null 2>&1
    while [ $? -ne 0 ]
    do
      i=$((i+1))
      sleep 0.1s
      if [ "${i}" -gt 100 ]; then
         log "Error: Waiting for elosd socket timed out"
         exit 124
      fi
      ${NETSTAT} -l | grep 54323 | grep tcp > /dev/null 2>&1
    done
}

smoketest_client_dummy() {
    prepare_env "client_dummy"

    LOG_ELOSD="$RESULT_DIR/elosd.log"
    
    log "Starting elosd"
    elosd > $LOG_ELOSD 2>&1 &
    ELOSD_PID=$!

    wait_for_elosd_socket

    log "Stop elosd ($ELOSD_PID)"
    kill $ELOSD_PID > /dev/null 2>&1
    wait $ELOSD_PID > /dev/null 2>&1

    TEST_RESULT=0
    log "check if Dummy Client Plugin was loaded"
    grep -q 'Dummy Client Plugin .* has been loaded' "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't load Dummy Client Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Client Plugin was started"
    grep -q 'Dummy Client Plugin .* has been started' "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't start Dummy Client Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Client Plugin was stopped"
    grep -q 'Stopping Dummy Client Plugin' "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't stop Dummy Client Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Client Plugin was unloaded"
    grep -q 'Unloading Dummy Client Plugin' "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't unload Dummy Client Plugin"
        TEST_RESULT=1
    fi

    return $TEST_RESULT
}

smoketest_backend_dummy() {
    prepare_env "backend_dummy"

    LOG_ELOSD="$RESULT_DIR/elosd.log"

    log "Starting elosd"
    elosd > $LOG_ELOSD 2>&1 &
    ELOSD_PID=$!

    wait_for_elosd_socket

    log "Stop elosd ($ELOSD_PID)"
    kill $ELOSD_PID > /dev/null 2>&1
    wait $ELOSD_PID > /dev/null 2>&1

    TEST_RESULT=0
    log "check if Dummy Backend Plugin was loaded"
    grep -q "DummyBackend.* has been loaded" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't load DummyBackend Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Backend Plugin was started"
    grep -q "DummyBackend.* has been started" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't start Dummy Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Backend Plugin was stopped"
    grep -q "Stopping Plugin .*DummyBackend" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't stop Dummy Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if Dummy Client Plugin was unloaded"
    grep -q "Unloading Plugin.*DummyBackend" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't unload Dummy Backend Plugin"
        TEST_RESULT=1
    fi

    return $TEST_RESULT
}

smoketest_cpp_in_memory_backend() {
    prepare_env "cpp_in_memory_backend"

    LOG_ELOSD="$RESULT_DIR/elosd.log"

    log "Starting elosd"
    elosd > $LOG_ELOSD 2>&1 &
    ELOSD_PID=$!

    wait_for_elosd_socket

    {
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1001, "severity": 4, "payload": "1. Event"}'
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1001, "severity": 4, "payload": "2. Event"}'
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1001, "severity": 4, "payload": "3. Event"}'
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1001, "severity": 4, "payload": "4. Event"}'
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1002, "severity": 4, "payload": "5. Event"}'
        elosc -P "${ELOSD_PORT}" -p '{"messageCode": 1001, "severity": 4, "payload": "6. Event"}'
    } >> "${RESULT_DIR}/event.log" 2>&1

    ALL_FETCHED="${RESULT_DIR}/fetch_all.reply"
    elosc -P "${ELOSD_PORT}" -f "1 1 EQ" >> "${ALL_FETCHED}" 2>&1
    FILTERED_FETCH="${RESULT_DIR}/fetch_filtered.reply"
    elosc -P "${ELOSD_PORT}" -f ".event.messageCode 1002 EQ" >> "${FILTERED_FETCH}" 2>&1


    log "Stop elosd ($ELOSD_PID)"
    kill $ELOSD_PID > /dev/null 2>&1
    wait $ELOSD_PID > /dev/null 2>&1
    TEST_RESULT=0

    log "check if C++ Backend Plugin was loaded"
    grep -q "CPlusPlusBackend.* has been loaded" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't load C++ Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if C++ Backend Plugin was started"
    grep -q "CPlusPlusBackend.* has been started" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't start C++ Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if C++ Backend Plugin was stopped"
    grep -q "Stopping Plugin .*CPlusPlusBackend" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't stop C++ Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if C++ Backend Plugin was unloaded"
    grep -q "Unloading Plugin.*CPlusPlusBackend" "${LOG_ELOSD}"
    if [ $? -ne 0 ]; then
        log_err "couldn't unload C++ Backend Plugin"
        TEST_RESULT=1
    fi

    log "check if messages got returned"
    payloads=('"payload":"1. Event"' '"payload":"2. Event"'
        '"payload":"3. Event"' '"payload":"4. Event"'
        '"payload":"5. Event"' '"payload":"6. Event"')
    match_count=0
    for payload in "${payloads[@]}"; do
        if grep -q "${payload}" "${ALL_FETCHED}"; then
            ((match_count=match_count+1))
        fi
    done
    ELOSBACKEND_RINGBUFFER_SIZE=5 # is not read from the environment by the plugin needs to be the same as config
    if [[ ${ELOSBACKEND_RINGBUFFER_SIZE} -ne ${match_count} ]]; then
        log_err "Wrong number of events fetched ${match_count}/${ELOSBACKEND_RINGBUFFER_SIZE}!"
        TEST_RESULT=1
    fi

    log "Test if fetch filter work"
    if ! grep -q '"messageCode":1002' "${FILTERED_FETCH}"; then
        log_err "couldn't find messageCode that was filtered for"
        TEST_RESULT=1
    fi
    if grep -q '"messageCode":1001' "${FILTERED_FETCH}"; then
        log_err "messageCode that wasn't filtered for was found"
        TEST_RESULT=1
    fi

    return $TEST_RESULT
}
# $1 - test name
# $2 - (optional) test function - valid options are [test_expect_success|test_expect_failure|test_expect_unstable]
call_test() {
    test_name=$1
    test_method=${2-"test_expect_success"}

    local result=1
    local skipped="false"

    echo -n "${test_name} ... "

    if [ "$ENABLED_TESTS" = "" ]; then
        echo $DISABLED_TESTS | grep -q "$test_name\b"
        if [ $? -ne 0 ]; then
            smoketest_${test_name}
            result=$?
        else
            skipped="true"
        fi
    else
        echo $ENABLED_TESTS | grep -q  "$test_name\b"
        if [ $? -eq 0 ]; then
                smoketest_${test_name}
                result=$?
        else
            skipped="true"
        fi
    fi

    if [ "${skipped}" = "true" ]; then
        echo "SKIPPED"
	result=0
    else
        if [ ${result} -eq 0 ]; then
            echo "OK"
        else
            echo "FAILED"
        fi
    fi

    return ${result}
}

FAILED_TESTS=0
call_test "client_dummy" || FAILED_TESTS=$((FAILED_TESTS+1))
call_test "backend_dummy" || FAILED_TESTS=$((FAILED_TESTS+1))
call_test "cpp_in_memory_backend" || FAILED_TESTS=$((FAILED_TESTS+1))

exit ${FAILED_TESTS}
