cmake_minimum_required(VERSION 3.15)
project(RayTracing)

set(CMAKE_CXX_STANDARD 14)

include_directories(lib)
include_directories(serialize)
include_directories(misc)
add_executable(ThinningAlgo main.cpp lib/matrix.h misc/exception.h lib/hawkes_poisson.h misc/utilities.h serialize/serialization.cpp serialize/serialization.h)
