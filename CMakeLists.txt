cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./include/map/MapConstants.h
  ./include/render/RenderConstants.h
  ./include/render/CameraConstants.h
  ./include/time/TimeConstants.h
  ./include/utility/AssetManager.h
  ./include/utility/ConfigSystem.h
  ./include/map/MapSystem.h
  ./include/map/MapGenerator.h
  ./include/entity/EntitySystem.h
  ./include/render/RenderSystem.h
  ./include/render/CameraSystem.h
  ./include/time/TimeSystem.h
  ./include/interface/InputSystem.h
  ./include/ui/UIConstants.h
  ./include/ui/UISystem.h
  ./include/utility/FileSystem.h
  ./include/LastDitch.h
)

set(
  SOURCES
  ./src/map/MapConstants.cpp
  ./src/render/RenderConstants.cpp
  ./src/render/CameraConstants.cpp
  ./src/time/TimeConstants.cpp
  ./src/utility/AssetManager.cpp
  ./src/utility/ConfigSystem.cpp
  ./src/map/MapSystem.cpp
  ./src/map/MapGenerator.cpp
  ./src/entity/EntitySystem.cpp
  ./src/render/RenderSystem.cpp
  ./src/render/CameraSystem.cpp
  ./src/time/TimeSystem.cpp
  ./src/interface/InputSystem.cpp
  ./src/ui/UIConstants.cpp
  ./src/ui/UISystem.cpp
  ./src/utility/FileSystem.cpp
  ./src/LastDitch.cpp
)

set(
  CMAKE_MODULE_PATH
  "${CMAKE_MODULE_PATH}"
  "${CMAKE_CURRENT_SOURCE_DIR}/build/cmake/Modules"
)

set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_BINARY_DIR}/build")

find_package(
  Boost 1.66.0 REQUIRED 
  COMPONENTS 
  filesystem iostreams serialization 
)
find_package(SDL2 REQUIRED)
find_package(SDL2_image REQUIRED)
find_package(SDL2_ttf REQUIRED)
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
  ${SDL2_TTF_INCLUDE_DIRS}
  ${YAMLCPP_INCLUDE_DIR}
)

target_link_libraries(
  LastDitch 
  ${Boost_LIBRARIES}
  ${SDL2_LIBRARY}
  ${SDL2_IMAGE_LIBRARY}
  ${SDL2_TTF_LIBRARIES}
  ${YAMLCPP_LIBRARY}
)
