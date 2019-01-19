cmake_minimum_required(VERSION 3.7)

set(LAST_DITCH_MAJOR_VERSION 0)
set(LAST_DITCH_MINOR_VERSION 1)
set(LAST_DITCH_PATCH_VERSION 0)
string(
  CONCAT LAST_DITCH_VERSION 
  "${LAST_DITCH_MAJOR_VERSION}."
  "${LAST_DITCH_MINOR_VERSION}."
  "${LAST_DITCH_PATCH_VERSION}"
)

project(LastDitch)

message(STATUS "Last Ditch version: ${LAST_DITCH_VERSION}")

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED YES)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")

find_package(
  Boost 1.66.0 REQUIRED 
  COMPONENTS 
  filesystem iostreams serialization 
)
find_package(YamlCpp REQUIRED)
find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)
find_package(SDL2_ttf REQUIRED)

set(SDL2_COMPILE_FLAGS "-I/usr/local/include/SDL2 -D_THREAD_SAFE")
set(SDL2_LINK_FLAGS "-L/usr/local/lib -lSDL2")

set(
  CMAKE_CXX_FLAGS 
  "${CMAKE_CXX_FLAGS} ${SDL2_COMPILE_FLAGS}"
)
set(
  CMAKE_EXE_LINKER_FLAGS 
  "${CMAKE_EXE_LINKER_FLAGS} ${SDL2_LINK_FLAGS}"
)

set(
  HEADERS
  ./src/utility/AssetManager.h
  ./src/utility/ConfigSystem.h
  ./src/map/MapSystem.h
  ./src/entity/EntitySystem.h
  ./src/render/RenderSystem.h
  ./src/render/CameraSystem.h
  ./src/time/TimeSystem.h
  ./src/interface/InputSystem.h
  ./src/ui/UISystem.h
  ./src/utility/FileSystem.h
  ./src/events/EventSystem.h
  ./src/LastDitch.h
)

set(
  SOURCES
  ./src/utility/AssetManager.cpp
  ./src/utility/ConfigSystem.cpp
  ./src/map/MapSystem.cpp
  ./src/entity/EntitySystem.cpp
  ./src/render/RenderSystem.cpp
  ./src/render/CameraSystem.cpp
  ./src/time/TimeSystem.cpp
  ./src/interface/InputSystem.cpp
  ./src/ui/UISystem.cpp
  ./src/utility/FileSystem.cpp
  ./src/events/EventSystem.cpp
  ./src/LastDitch.cpp
)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

include_directories(
  ${Boost_INCLUDE_DIRS}
  ${YAMLCPP_INCLUDE_DIR}
  ${SDL2_INCLUDE_DIR}
  ${SDL2_IMAGE_INCLUDE_DIR}
  ${SDL2_TTF_INCLUDE_DIRS}
)

target_link_libraries(
  LastDitch
  ${Boost_LIBRARIES}
  ${YAMLCPP_LIBRARY}
  ${SDL2_LIBRARY}
  ${SDL2_IMAGE_LIBRARY}
  ${SDL2_TTF_LIBRARIES}
)

set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}")

file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/data)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/saves)
file(MAKE_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR}/maps)

install(
  DIRECTORY ${CMAKE_SOURCE_DIR}/data
  DESTINATION . 
)
