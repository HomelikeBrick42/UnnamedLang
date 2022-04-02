cmake_minimum_required(VERSION 3.20)
project(UnnamedLanguage)

set(CMAKE_CXX_STANDARD 23)

add_executable(
        UnnamedLanguage
        src/Main.cpp)
target_compile_options(UnnamedLanguage PRIVATE -Wall -Wextra -Werror)
