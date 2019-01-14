# Locate YamlCpp library
#
# This module defines:
#
#  YamlCpp_FOUND - System has YamlCpp
#  YamlCpp_INCLUDE_DIRS - The YamlCpp include directories
#  YamlCpp_LIBRARIES - The libraries needed to use YamlCpp
#

find_package(PkgConfig)
pkg_check_modules(PC_YAMLCPP yaml-cpp)
# message(STATUS "PKG_CONFIG_FOUND: ${PKG_CONFIG_FOUND}")
# message(STATUS "PKG_CONFIG_EXECUTABLE: ${PKG_CONFIG_EXECUTABLE}")
# message(STATUS "PKG_CONFIG_VERSION_STRING: ${PKG_CONFIG_VERSION_STRING}")
# message(STATUS "PC_YAMLCPP_FOUND: ${PC_YAMLCPP_FOUND}")
# message(STATUS "PC_YamlCpp_INCLUDE_DIRS: ${PC_YamlCpp_INCLUDE_DIRS}")
# message(STATUS "PC_YAMLCPP_LIBRARY_DIRS: ${PC_YAMLCPP_LIBRARY_DIRS}")

find_path(
  YAMLCPP_INCLUDE_DIR yaml-cpp/yaml.h
  HINTS ${PC_YamlCpp_INCLUDE_DIRS}
  PATH_SUFFIXES include
)

find_library(
  YAMLCPP_LIBRARY NAMES yaml-cpp
  HINTS ${PC_YAMLCPP_LIBRARY_DIRS}
)

set(YamlCpp_LIBRARIES ${YAMLCPP_LIBRARY})
set(YamlCpp_INCLUDE_DIRS ${YAMLCPP_INCLUDE_DIR})
# message(STATUS "YAMLCPP_LIBRARY: ${YAMLCPP_LIBRARY}")
# message(STATUS "YAMLCPP_INCLUDE_DIR: ${YAMLCPP_INCLUDE_DIR}")

include(FindPackageHandleStandardArgs)
# handle the QUIETLY and REQUIRED arguments and set YAMLCPP_FOUND to TRUE
# if all listed variables are TRUE
find_package_handle_standard_args(
  YamlCpp DEFAULT_MSG
  YamlCpp_LIBRARIES YamlCpp_INCLUDE_DIRS
)

mark_as_advanced(YamlCpp_INCLUDE_DIRS YamlCpp_LIBRARIES)

# ensure that they are cached
set(
  YamlCpp_INCLUDE_DIRS 
  ${YamlCpp_INCLUDE_DIRS} 
  CACHE INTERNAL "The yaml-cpp include path"
)
set(
  YamlCpp_LIBRARIES 
  ${YamlCpp_LIBRARIES} 
  CACHE INTERNAL "The libraries needed to use yaml-cpp library"
)
