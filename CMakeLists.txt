cmake_minimum_required(VERSION 3.20)
project(Langite)

set(CMAKE_CXX_STANDARD 23)

add_executable(
        Langite
        src/Main.cpp
        src/SourceLocation.hpp)
target_compile_options(Langite PRIVATE -Wall -Wextra -Werror)
