cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./src/Game.h
)

set(
  SOURCES
  ./src/main.cpp
  ./src/Game.cpp
)

find_package(Eigen3 3.3 REQUIRED NO_MODULE)
find_package(SDL2 REQUIRED)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

include_directories(
  ${SDL2_INCLUDE_DIRS}
)

target_link_libraries(
  LastDitch 
  Eigen3::Eigen
  ${SDL2_LIBRARIES}
)
