cmake_minimum_required(VERSION 3.21)

project(rust-cpp-cmake-bindings-cmake)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

add_library(rust-cpp-cmake-bindings-cpp SHARED ${CMAKE_CURRENT_SOURCE_DIR}/src/mylibrary.cpp)

target_include_directories(rust-cpp-cmake-bindings-cpp PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)

cmake_POLICY(SET CMP0135 NEW)
set(CASADI_LIBDIR ${CMAKE_BINARY_DIR}/_deps/casadi-src/casadi)
set(CASADI_INCLUDEDIR ${CMAKE_BINARY_DIR}/_deps/casadi-src/casadi/include)
if (${CMAKE_SYSTEM_NAME} MATCHES "MINGW" OR ${CMAKE_SYSTEM_NAME} MATCHES "MSYS" OR WIN32)
  message(STATUS "Building for Windows")
  set(CASADI_URL https://github.com/casadi/casadi/releases/download/3.6.3/casadi-3.6.3-windows64-py311.zip)
elseif (APPLE)
  if (CMAKE_APPLE_SILICON_PROCESSOR MATCHES "arm64")
    set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_RPATH};@loader_path/../lib;@loader_path")
    set(CASADI_URL https://github.com/casadi/casadi/releases/download/3.6.3/casadi-3.6.3-osx_arm64-py311.zip)
    set(CASADI_INSTALL_LIBS
        ${CASADI_LIBDIR}/libcasadi.3.7.dylib
        ${CASADI_LIBDIR}/libc++.1.0.dylib
        ${CASADI_LIBDIR}/libcasadi_nlpsol_ipopt.dylib
        ${CASADI_LIBDIR}/libipopt.3.dylib
        ${CASADI_LIBDIR}/libcoinmumps.3.dylib
        ${CASADI_LIBDIR}/libcoinmetis.2.dylib
        ${CASADI_LIBDIR}/libgfortran.5.dylib
        ${CASADI_LIBDIR}/libquadmath.0.dylib
        ${CASADI_LIBDIR}/libgcc_s.1.1.dylib)
  elseif(CMAKE_APPLE_SILICON_PROCESSOR MATCHES "x86_64")
    set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_RPATH};@loader_path/../lib;@loader_path")
    set(CASADI_URL https://github.com/casadi/casadi/releases/download/3.6.3/casadi-3.6.3-osx64-py311.zip)
  endif()
  set(CASADI_INSTALL_DEST "lib")
elseif (UNIX)
  if (${CMAKE_SYSTEM_PROCESSOR} MATCHES "ARM64")
    message(STATUS "Building for Linux arm64")
    set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_RPATH};$ORIGIN/../lib;$ORIGIN")
    set(CASADI_URL https://github.com/casadi/casadi/releases/download/3.6.3/casadi-3.6.3-linux-aarch64-py311.zip)
  else()
    message(STATUS "Building for Linux x64")
    set(CMAKE_INSTALL_RPATH "${CMAKE_INSTALL_RPATH};$ORIGIN/../lib;$ORIGIN")
    set(CASADI_URL https://github.com/casadi/casadi/releases/download/3.6.3/casadi-3.6.3-linux64-py311.zip)
  endif()
  set(CASADI_INSTALL_DEST "lib")
endif()
message(STATUS "Downloading CasADi from ${CASADI_URL}")

include(FetchContent)

FetchContent_Declare(
  casadi
  URL ${CASADI_URL}
)

FetchContent_MakeAvailable(casadi)

target_include_directories(rust-cpp-cmake-bindings-cpp SYSTEM PRIVATE ${CASADI_INCLUDEDIR})
target_link_directories(rust-cpp-cmake-bindings-cpp PRIVATE ${CASADI_LIBDIR})
target_link_libraries(rust-cpp-cmake-bindings-cpp PRIVATE casadi)

install(TARGETS rust-cpp-cmake-bindings-cpp
    RUNTIME DESTINATION "lib"
    LIBRARY DESTINATION "lib"
    )

# Install CasADi libraries since FetchContent setting that up properly
install(FILES ${CASADI_INSTALL_LIBS} DESTINATION ${CASADI_INSTALL_DEST})
