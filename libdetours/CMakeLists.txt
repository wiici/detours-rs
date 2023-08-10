cmake_minimum_required(VERSION 3.2)

project(libdetours)

include(FetchContent)
FetchContent_Declare(
  detours
  GIT_REPOSITORY https://github.com/microsoft/Detours.git
  GIT_TAG        4b8c659f549b0ab21cf649377c7a84eb708f5e68
)
FetchContent_MakeAvailable(detours)

set(DOTURS_SRC_DIR ${detours_SOURCE_DIR}/src)

add_library(${PROJECT_NAME} STATIC)
target_sources(
    ${PROJECT_NAME}
    PRIVATE
    ${DOTURS_SRC_DIR}/detours.h ${DOTURS_SRC_DIR}/detours.cpp
    ${DOTURS_SRC_DIR}/detver.h
    ${DOTURS_SRC_DIR}/creatwth.cpp
    ${DOTURS_SRC_DIR}/image.cpp
    ${DOTURS_SRC_DIR}/disasm.cpp
    ${DOTURS_SRC_DIR}/modules.cpp
)

install(
  TARGETS ${PROJECT_NAME}
  ARCHIVE DESTINATION lib
  LIBRARY DESTINATION lib
)

install(
  FILES
  ${DOTURS_SRC_DIR}/detours.h
  DESTINATION
  include
)
