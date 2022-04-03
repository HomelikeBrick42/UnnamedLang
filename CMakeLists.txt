cmake_minimum_required(VERSION 3.20)
project(Langite)

set(CMAKE_CXX_STANDARD 23)

add_executable(
        Langite
        src/CompileError.hpp
        src/Lexer.cpp
        src/Lexer.hpp
        src/Main.cpp
        src/SourceLocation.hpp
        src/Token.cpp
        src/Token.hpp)
target_compile_options(Langite PRIVATE -Wall -Wextra -Werror)
