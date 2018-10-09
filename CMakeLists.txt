cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./src/TextureManager.h
  ./src/GameObject.h
  ./src/Game.h
)

set(
  SOURCES
  ./src/TextureManager.cpp
  ./src/GameObject.cpp
  ./src/Game.cpp
  ./src/main.cpp
)

set(
  CMAKE_MODULE_PATH
  "${CMAKE_MODULE_PATH}"
  "${CMAKE_CURRENT_SOURCE_DIR}/dist/cmake/Modules")

set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/dist")

find_package(Eigen3 3.3 REQUIRED NO_MODULE)
find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)

set(SDL2_LINK_FLAGS "-lSDL2 -lSDL2_image")

set(
  CMAKE_EXE_LINKER_FLAGS
  "${CMAKE_EXE_LINKER_FLAGS} ${SDL2_LINK_FLAGS}"
)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

include_directories(
  ${SDL2_INCLUDE_DIRS}
  ${SDL2_IMAGE_INCLUDE_DIRS}
)

target_link_libraries(
  LastDitch 
  Eigen3::Eigen
  ${SDL2_LIBRARIES}
  ${SDL2_IMAGE_LIBRARY}
)
