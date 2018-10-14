cmake_minimum_required(VERSION 3.7)

project(LastDitch)

set(
  HEADERS
  ./src/constants/RenderConstants.h
  ./src/constants/SimulationConstants.h
  ./src/systems/RenderSystem.h
  ./src/systems/CameraSystem.h
  ./src/systems/TimeSystem.h
  ./src/systems/InputSystem.h
  ./src/Game.h
)

set(
  SOURCES
  ./src/constants/RenderConstants.cpp
  ./src/constants/SimulationConstants.cpp
  ./src/systems/RenderSystem.cpp
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

find_package(OpenGL REQUIRED)
find_package(GLM REQUIRED)
find_package(GLEW REQUIRED)
find_package(GLFW3 REQUIRED)

set(
  LINKER_FLAGS 
  "-framework OpenGL -lglew -lglfw"
)

set(
  CMAKE_EXE_LINKER_FLAGS
  "${CMAKE_EXE_LINKER_FLAGS} ${LINKER_FLAGS}"
)

add_executable(LastDitch ${HEADERS} ${SOURCES})
target_compile_features(LastDitch PRIVATE cxx_range_for)

include_directories(
  ${OPENGL_INCLUDE_DIRS}
  ${GLM_INCLUDE_DIRS}
  ${GLEW_INCLUDE_DIRS}
)

target_link_libraries(
  LastDitch 
  ${OPENGL_LIBRARIES}
  ${GLM_LIBRARIES}
  ${GLEW_LIBRARIES}
  ${GLFW3_LIBRARIES}
)
