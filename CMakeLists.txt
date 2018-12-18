cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./src/constants/RenderConstants.h
  ./src/constants/MapConstants.h
  ./src/constants/CameraConstants.h
  ./src/constants/SimulationConstants.h
  ./src/systems/ConfigSystem.h
  ./src/systems/RenderSystem.h
  ./src/systems/MapSystem.h
  ./src/systems/MapGenerator.h
  ./src/systems/CameraSystem.h
  ./src/systems/TimeSystem.h
  ./src/systems/InputSystem.h
  ./src/Game.h
)

set(
  SOURCES
  ./src/constants/RenderConstants.cpp
  ./src/constants/MapConstants.cpp
  ./src/constants/CameraConstants.cpp
  ./src/constants/SimulationConstants.cpp
  ./src/systems/ConfigSystem.cpp
  ./src/systems/RenderSystem.cpp
  ./src/systems/MapSystem.cpp
  ./src/systems/MapGenerator.cpp
  ./src/systems/CameraSystem.cpp
  ./src/systems/TimeSystem.cpp
  ./src/systems/InputSystem.cpp
  ./src/Game.cpp
  ./src/main.cpp
)

set(
  CMAKE_MODULE_PATH
  "${CMAKE_MODULE_PATH}"
  "${CMAKE_CURRENT_SOURCE_DIR}/dist/cmake/Modules"
)

set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/dist")

find_package(
  Boost 1.66.0 REQUIRED 
  COMPONENTS 
  filesystem iostreams serialization 
)
find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)
find_package(YamlCpp REQUIRED)

set(SDL2_COMPILE_FLAGS "-I/usr/local/include/SDL2 -D_THREAD_SAFE")
set(SDL2_LINK_FLAGS "-L/usr/local/lib -lSDL2 -lSDL2_image")

set(
  CMAKE_CXX_FLAGS
  "${CMAKE_CXX_FLAGS} ${SDL2_COMPILE_FLAGS}"
)
set(
  CMAKE_EXE_LINKER_FLAGS
  "${CMAKE_EXE_LINKER_FLAGS} ${SDL2_LINK_FLAGS}"
)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

include_directories(
  ${Boost_INCLUDE_DIRS}
  ${SDL2_INCLUDE_DIR}
  ${SDL2_IMAGE_INCLUDE_DIR}
  ${YAMLCPP_INCLUDE_DIR}
)

target_link_libraries(
  LastDitch 
  ${Boost_LIBRARIES}
  ${SDL2_LIBRARY}
  ${SDL2_IMAGE_LIBRARY}
  ${YAMLCPP_LIBRARY}
)
