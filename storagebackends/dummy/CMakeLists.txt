# SPDX-License-Identifier: MIT
cmake_minimum_required(VERSION 3.21)

set(DUMMY_BACKEND_VERSION_MAJOR 0)
set(DUMMY_BACKEND_VERSION_MINOR 1)
set(DUMMY_BACKEND_VERSION_MICRO 0)
set(DUMMY_BACKEND_VERSION
  ${DUMMY_BACKEND_VERSION_MAJOR}.${DUMMY_BACKEND_VERSION_MINOR}.${DUMMY_BACKEND_VERSION_MICRO})

project(
  dummy-backend
  DESCRIPTION "a dummy backend plugin examples for elos"
  VERSION ${DUMMY_BACKEND_VERSION}
  LANGUAGES C)

set(ELOSD_BACKEND_PATH ${CMAKE_INSTALL_FULL_LIBDIR}/elos/backend)

find_package(safu 0.54.0 REQUIRED)
find_package(samconf 0.53.1 REQUIRED)
find_package(libelosplugin 0.58.4 REQUIRED)

add_library(backend_dummy SHARED private/dummyBackend.c)

set_target_properties(backend_dummy PROPERTIES PREFIX "")
set_target_properties(backend_dummy
    PROPERTIES SOVERSION ${DUMMY_BACKEND_VERSION_MAJOR}
               VERSION ${DUMMY_BACKEND_VERSION})

target_link_libraries(backend_dummy
    PUBLIC elos::libelosplugin samconf::samconf safu::safu)

target_include_directories(backend_dummy PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

install(TARGETS backend_dummy DESTINATION ${ELOSD_BACKEND_PATH})
