# SPDX-License-Identifier: MIT

macro(project_set_environment)
  set(CMAKE_C_STANDARD 17)
  set(CMAKE_C_STANDARD_REQUIRED ON)
  set(CMAKE_C_EXTENSIONS OFF)
  set(CMAKE_C_FLAGS_DEBUG "")
  set(CMAKE_C_FLAGS_RELEASE "")
  set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
  include(GNUInstallDirs)

  add_compile_options(
    -Wshadow -Wall -Wextra -pedantic -D_DEFAULT_SOURCE
  )

  if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE "Release")
  endif()

  if(CMAKE_BUILD_TYPE STREQUAL "Debug")
    add_compile_options(-Og -g3 -DDEBUG -fno-omit-frame-pointer)
  elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
    add_compile_options(-O3 -DNDEBUG -g -Werror)
  endif()
endmacro()
