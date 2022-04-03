#pragma once

#include "SourceLocation.hpp"

#include <variant>

#define TOKEN_KINDS                                                      \
    TOKEN_KIND_SINGLE(EndOfFile, "EOF", '\0')                            \
    TOKEN_KIND(Name, "Name")                                             \
    TOKEN_KIND(Integer, "Integer")                                       \
    TOKEN_KIND(Float, "Float")                                           \
    TOKEN_KIND(String, "String")                                         \
    TOKEN_KIND_KEYWORD(Wildcard, "_")                                    \
    TOKEN_KIND_KEYWORD(Const, "const")                                   \
    TOKEN_KIND_KEYWORD(Func, "func")                                     \
    TOKEN_KIND_KEYWORD(Proc, "proc")                                     \
    TOKEN_KIND_KEYWORD(Return, "return")                                 \
    TOKEN_KIND_KEYWORD(If, "if")                                         \
    TOKEN_KIND_KEYWORD(Else, "else")                                     \
    TOKEN_KIND_SINGLE(Newline, "newline", '\n')                          \
    TOKEN_KIND_SINGLE(OpenParentesis, "(", '(')                          \
    TOKEN_KIND_SINGLE(CloseParentesis, ")", ')')                         \
    TOKEN_KIND_SINGLE(OpenBrace, "{", '{')                               \
    TOKEN_KIND_SINGLE(CloseBrace, "}", '}')                              \
    TOKEN_KIND_SINGLE(OpenSquareBracket, "[", '[')                       \
    TOKEN_KIND_SINGLE(CloseSquareBracket, "]", ']')                      \
    TOKEN_KIND_SINGLE(Colon, ":", ':')                                   \
    TOKEN_KIND_SINGLE(Comma, ",", ',')                                   \
    TOKEN_KIND_SINGLE(Period, ".", '.')                                  \
    TOKEN_KIND_SINGLE(At, "@", '@')                                      \
    TOKEN_KIND_SINGLE(Plus, "+", '+')                                    \
    TOKEN_KIND_DOUBLE(Minus, '-', RightArrow, '>')                       \
    TOKEN_KIND_SINGLE(Asterisk, "*", '*')                                \
    TOKEN_KIND_SINGLE(Slash, "/", '/')                                   \
    TOKEN_KIND_SINGLE(Modulus, "%", '%')                                 \
    TOKEN_KIND_TRIPLE(LessThan, '<', LessThanEqual, '=', LeftArrow, '-') \
    TOKEN_KIND_DOUBLE(GreaterThan, '>', GreaterThanEqual, '=')

namespace Langite {

    enum struct TokenKind {
#define TOKEN_KIND(kind, string)                                             kind,
#define TOKEN_KIND_KEYWORD(kind, string)                                     kind,
#define TOKEN_KIND_SINGLE(kind, string, character)                           kind,
#define TOKEN_KIND_DOUBLE(kind, firstCharacter, doubleKind, secondCharacter) kind, doubleKind,
#define TOKEN_KIND_TRIPLE(                                                        \
    kind, firstCharacter, doubleKind, secondCharacter, thirdKind, thirdCharacter) \
    kind, doubleKind, thirdKind,
        TOKEN_KINDS
#undef TOKEN_KIND
#undef TOKEN_KIND_KEYWORD
#undef TOKEN_KIND_SINGLE
#undef TOKEN_KIND_DOUBLE
#undef TOKEN_KIND_TRIPLE
    };

    std::string TokenKind_ToString(TokenKind kind);

    struct Token {
        TokenKind Kind;
        SourceLocation Location;
        size_t Length;
        std::variant<size_t, double, std::string_view, std::string> Data;
    };

}

#ifndef KEEP_TOKEN_KINDS
    #undef TOKEN_KINDS
#endif
