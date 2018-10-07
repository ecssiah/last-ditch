cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./src/LastDitch.h
)

set(
  SOURCES
  ./src/LastDitch.cpp
)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

find_package(SDL2 REQUIRED)

include_directories(${SDL2_INCLUDE_DIRS})
target_link_libraries(LastDitch ${SDL2_LIBRARIES})
