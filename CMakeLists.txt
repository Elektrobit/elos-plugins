# SPDX-License-Identifier: MIT
cmake_minimum_required(VERSION 3.21)

add_compile_definitions(SAFU_LOG=1)
include(cmake/project.cmake)
project(
  dummy-plugins
  DESCRIPTION "a collection of dummy plugin examples for elos"
  LANGUAGES C
)
project_set_environment()

add_subdirectory(clients)
add_subdirectory(storagebackends)
add_subdirectory(scanners)
