# SPDX-License-Identifier: MIT
cmake_minimum_required(VERSION 3.21)

set(DUMMY_CLIENT_VERSION_MAJOR 0)
set(DUMMY_CLIENT_VERSION_MINOR 1)
set(DUMMY_CLIENT_VERSION_MICRO 0)
set(
  DUMMY_CLIENT_VERSION
  ${DUMMY_CLIENT_VERSION_MAJOR}.${DUMMY_CLIENT_VERSION_MINOR}.${DUMMY_CLIENT_VERSION_MICRO}
)

project(
  dummy-client
  DESCRIPTION "a dummy client plugin examples for elos"
  VERSION ${DUMMY_CLIENT_VERSION}
  LANGUAGES C
)

set(ELOSD_CLIENT_PATH ${CMAKE_INSTALL_FULL_LIBDIR}/elos/client)

find_package(libelosplugin 0.58.4 REQUIRED)
find_package(safu 0.54.0 REQUIRED)
find_package(samconf 0.53.1 REQUIRED)

add_library(
  client_dummy SHARED
  private/dummy.c
)

set_target_properties(client_dummy PROPERTIES PREFIX "")
set_target_properties(
  client_dummy
  PROPERTIES
  SOVERSION ${DUMMY_CLIENT_VERSION_MAJOR}
    VERSION ${DUMMY_CLIENT_VERSION}
)

target_link_libraries(
  client_dummy
  PUBLIC
    elos::libelosplugin
    samconf::samconf
    safu::safu
)

target_include_directories(
  client_dummy
  PUBLIC
    ${CMAKE_CURRENT_SOURCE_DIR}
)

install(TARGETS client_dummy DESTINATION ${ELOSD_CLIENT_PATH})
