cmake_minimum_required(VERSION 3.20)
project(Langite)

set(CMAKE_CXX_STANDARD 23)

add_executable(
        Langite
        src/Ast.cpp
        src/Ast.hpp
        src/CompileError.hpp
        src/Lexer.cpp
        src/Lexer.hpp
        src/Main.cpp
        src/Parsing.cpp
        src/Parsing.hpp
        src/Resolvers.cpp
        src/Resolvers.hpp
        src/SourceLocation.hpp
        src/Token.cpp
        src/Token.hpp
        src/Types.hpp)
target_compile_options(Langite PRIVATE -Wall -Wextra -Werror)
