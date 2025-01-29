
set(ELOSD_SCANNER_PATH ${CMAKE_INSTALL_FULL_LIBDIR}/elos/scanner)
set(CARGO_TARGET_DIR ${CMAKE_BINARY_DIR}/cargo-target)
set(PLUGIN_NAME scanner_rust_dummy)


if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(SUBDIR debug)
    set(OPTION "")
else()
    set(SUBDIR release)
    set(OPTION --release)
endif()

add_custom_command(
    OUTPUT ${CARGO_TARGET_DIR}/${SUBDIR}/lib${PLUGIN_NAME}.so
    COMMAND cargo build ${OPTION} --target-dir ${CARGO_TARGET_DIR}
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building Rust executable in build directory"
)

add_custom_target(${PLUGIN_NAME} ALL
    DEPENDS ${CARGO_TARGET_DIR}/${SUBDIR}/lib${PLUGIN_NAME}.so
)

install(
    FILES ${CARGO_TARGET_DIR}/${SUBDIR}/lib${PLUGIN_NAME}.so
    DESTINATION ${ELOSD_SCANNER_PATH}
    RENAME ${PLUGIN_NAME}.so
)
