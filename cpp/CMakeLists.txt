cmake_minimum_required(VERSION 3.21)

project(rust-cpp-cmake-bindings-cmake)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(rust-cpp-cmake-bindings-cpp SHARED ${CMAKE_CURRENT_SOURCE_DIR}/src/mylibrary.cpp)

target_include_directories(rust-cpp-cmake-bindings-cpp PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

set(OPTIMIZER_BACKEND casadi)

set(CMAKE_SHARED_LINKER_FLAGS "-static-libgcc -static-libstdc++")
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++")

include(FetchContent)

FetchContent_Declare(
  TrajoptLib
  GIT_REPOSITORY https://github.com/SleipnirGroup/TrajoptLib.git
  GIT_TAG 70f70618669177ee94a71a8ad74fb56785980972
)

FetchContent_GetProperties(TrajoptLib)
if(NOT TrajoptLib_POPULATED)
  FetchContent_Populate(TrajoptLib)
  add_subdirectory(${trajoptlib_SOURCE_DIR} ${trajoptlib_BINARY_DIR})
endif()

target_link_libraries(rust-cpp-cmake-bindings-cpp PRIVATE TrajoptLib)

install(TARGETS rust-cpp-cmake-bindings-cpp
    RUNTIME DESTINATION "bin"
    LIBRARY DESTINATION "bin"
    )

# install(TARGETS TrajoptLib)

# Install CasADi libraries since FetchContent setting that up properly
install(FILES ${CASADI_INSTALL_LIBS} DESTINATION ${CASADI_INSTALL_DEST})

