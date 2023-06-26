cmake_minimum_required(VERSION 3.21)

project(rust-cpp-cmake-bindings-cmake)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(rust-cpp-cmake-bindings-cpp SHARED ${CMAKE_CURRENT_SOURCE_DIR}/src/mylibrary.cpp)

target_include_directories(rust-cpp-cmake-bindings-cpp PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

install(TARGETS rust-cpp-cmake-bindings-cpp
    RUNTIME DESTINATION "."
    LIBRARY DESTINATION "."
    )
