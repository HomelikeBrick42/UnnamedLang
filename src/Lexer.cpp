#define KEEP_TOKEN_KINDS
#include "Lexer.hpp"
#include "CompileError.hpp"

#include <sstream>
#include <cassert>

namespace Langite {

    Lexer::Lexer(std::string_view filepath, std::string_view source)
        : Location{ .Filepath = filepath, .Position = 0, .Line = 1, .Column = 1 }, Source(source) {}

    Token Lexer::NextToken() {
Start:
        SourceLocation startLocation = Location;
        switch (CurrentChar()) {
#define TOKEN_KIND(kind, string)
#define TOKEN_KIND_KEYWORD(kind, string)
#define TOKEN_KIND_SINGLE(kind, string, character)                  \
    case character:                                                 \
        NextChar();                                                 \
        return Token{                                               \
            .Kind     = TokenKind::kind,                            \
            .Location = startLocation,                              \
            .Length   = Location.Position - startLocation.Position, \
        };
#define TOKEN_KIND_DOUBLE(kind, firstCharacter, doubleKind, secondCharacter) \
    case firstCharacter:                                                     \
        NextChar();                                                          \
        if (CurrentChar() == secondCharacter) {                              \
            NextChar();                                                      \
            return Token{                                                    \
                .Kind     = TokenKind::doubleKind,                           \
                .Location = startLocation,                                   \
                .Length   = Location.Position - startLocation.Position,      \
            };                                                               \
        }                                                                    \
        return Token{                                                        \
            .Kind     = TokenKind::kind,                                     \
            .Location = startLocation,                                       \
            .Length   = Location.Position - startLocation.Position,          \
        };
#define TOKEN_KIND_TRIPLE(                                                        \
    kind, firstCharacter, doubleKind, secondCharacter, thirdKind, thirdCharacter) \
    case firstCharacter:                                                          \
        NextChar();                                                               \
        if (CurrentChar() == secondCharacter) {                                   \
            NextChar();                                                           \
            return Token{                                                         \
                .Kind     = TokenKind::doubleKind,                                \
                .Location = startLocation,                                        \
                .Length   = Location.Position - startLocation.Position,           \
            };                                                                    \
        } else if (CurrentChar() == thirdCharacter) {                             \
            NextChar();                                                           \
            return Token{                                                         \
                .Kind     = TokenKind::thirdKind,                                 \
                .Location = startLocation,                                        \
                .Length   = Location.Position - startLocation.Position,           \
            };                                                                    \
        }                                                                         \
        return Token{                                                             \
            .Kind     = TokenKind::kind,                                          \
            .Location = startLocation,                                            \
            .Length   = Location.Position - startLocation.Position,               \
        };
            TOKEN_KINDS
#undef TOKEN_KIND
#undef TOKEN_KIND_KEYWORD
#undef TOKEN_KIND_SINGLE
#undef TOKEN_KIND_DOUBLE
#undef TOKEN_KIND_TRIPLE

            case ' ':
            case '\t':
            case '\r':
                while (CurrentChar() == ' ' || CurrentChar() == '\t' || CurrentChar() == '\r')
                    NextChar();
                goto Start;

            case '"': {
                NextChar();
                std::vector<char> value{}; // TODO: Maybe dont leak this
                while (CurrentChar() != '"' && CurrentChar() != '\0') {
                    if (CurrentChar() == '\\') {
                        NextChar();
                        switch (CurrentChar()) {
                            case '"':
                                value.push_back('\"');
                                break;

                            case '\\':
                                value.push_back('\\');
                                break;

                            case '0':
                                value.push_back('\0');
                                break;

                            case 'n':
                                value.push_back('\n');
                                break;

                            case 'r':
                                value.push_back('\r');
                                break;

                            default: {
                                char chr = NextChar();
                                throw CompileError{
                                    .Location = startLocation,
                                    .Message  = (std::stringstream{} << "Unknown escape character '"
                                                                    << chr << '\'')
                                                   .str(),
                                };
                            }
                        }
                        NextChar();
                    } else {
                        value.push_back(NextChar());
                    }
                }
                if (CurrentChar() != '"') {
                    throw CompileError{
                        .Location = startLocation,
                        .Message  = "Unclosed string literal at end of file",
                    };
                }
                NextChar();
                return Token{
                    .Kind     = TokenKind::String,
                    .Location = startLocation,
                    .Length   = Location.Position - startLocation.Position,
                    .Data     = value,
                };
            };

            case '0':
            case '1':
            case '2':
            case '3':
            case '4':
            case '5':
            case '6':
            case '7':
            case '8':
            case '9': {
                size_t base = 10;
                if (CurrentChar() == '0') {
                    NextChar();
                    switch (CurrentChar()) {
                        case 'b':
                            NextChar();
                            base = 2;
                            break;

                        case 'o':
                            NextChar();
                            base = 8;
                            break;

                        case 'd':
                            NextChar();
                            base = 10;
                            break;

                        case 'x':
                            NextChar();
                            base = 16;
                            break;

                        default:
                            base = 10;
                            break;
                    }
                }

                size_t intValue = 0;
                while (std::isalnum(CurrentChar()) || CurrentChar() == '_') {
                    if (CurrentChar() == '_')
                        continue;

                    size_t value;
                    if (CurrentChar() >= 'A' && CurrentChar() <= 'Z') {
                        value = CurrentChar() - 'A' + 10;
                    } else if (CurrentChar() >= 'a' && CurrentChar() <= 'z') {
                        value = CurrentChar() - 'a' + 10;
                    } else {
                        value = CurrentChar() - '0';
                    }

                    if (value >= base) {
                        throw CompileError{
                            .Location = Location,
                            .Message =
                                (std::stringstream{} << "Digit '" << CurrentChar()
                                                     << "' is too big for base '" << base << '\'')
                                    .str(),
                        };
                    }

                    intValue *= base;
                    intValue += value;

                    NextChar();
                }

                if (CurrentChar() == '.') {
                    NextChar();

                    double floatValue  = static_cast<double>(intValue);
                    size_t denominator = 1;

                    while (std::isalnum(CurrentChar()) || CurrentChar() == '_') {
                        if (CurrentChar() == '_')
                            continue;

                        size_t value;
                        if (CurrentChar() >= 'A' && CurrentChar() <= 'Z') {
                            value = CurrentChar() - 'A' + 10;
                        } else if (CurrentChar() >= 'a' && CurrentChar() <= 'z') {
                            value = CurrentChar() - 'a' + 10;
                        } else {
                            value = CurrentChar() - '0';
                        }

                        if (value >= base) {
                            throw CompileError{
                                .Location = Location,
                                .Message  = (std::stringstream{} << "Digit '" << CurrentChar()
                                                                << "' is too big for base '" << base
                                                                << '\'')
                                               .str(),
                            };
                        }

                        denominator *= base;
                        floatValue += static_cast<double>(value) / static_cast<double>(denominator);

                        NextChar();
                    }

                    return Token{
                        .Kind     = TokenKind::Float,
                        .Location = startLocation,
                        .Length   = Location.Position - startLocation.Position,
                        .Data     = floatValue,
                    };
                }

                return Token{
                    .Kind     = TokenKind::Integer,
                    .Location = startLocation,
                    .Length   = Location.Position - startLocation.Position,
                    .Data     = intValue,
                };
            };

            case 'A':
            case 'B':
            case 'C':
            case 'D':
            case 'E':
            case 'F':
            case 'G':
            case 'H':
            case 'I':
            case 'J':
            case 'K':
            case 'L':
            case 'M':
            case 'N':
            case 'O':
            case 'P':
            case 'Q':
            case 'R':
            case 'S':
            case 'T':
            case 'U':
            case 'V':
            case 'W':
            case 'X':
            case 'Y':
            case 'Z':
            case 'a':
            case 'b':
            case 'c':
            case 'd':
            case 'e':
            case 'f':
            case 'g':
            case 'h':
            case 'i':
            case 'j':
            case 'k':
            case 'l':
            case 'm':
            case 'n':
            case 'o':
            case 'p':
            case 'q':
            case 'r':
            case 's':
            case 't':
            case 'u':
            case 'v':
            case 'w':
            case 'x':
            case 'y':
            case 'z':
            case '_': {
                while (std::isalnum(CurrentChar()) || CurrentChar() == '_') {
                    NextChar();
                }
                size_t length = Location.Position - startLocation.Position;
                std::string_view name{ &Source[startLocation.Position], length };
#define TOKEN_KIND(kind, string)
#define TOKEN_KIND_KEYWORD(kind, string) \
    if (name == string) {                \
        return Token{                    \
            .Kind     = TokenKind::kind, \
            .Location = startLocation,   \
            .Length   = length,          \
        };                               \
    } else
#define TOKEN_KIND_SINGLE(kind, string, character)
#define TOKEN_KIND_DOUBLE(kind, firstCharacter, doubleKind, secondCharacter)
#define TOKEN_KIND_TRIPLE( \
    kind, firstCharacter, doubleKind, secondCharacter, thirdKind, thirdCharacter)
                TOKEN_KINDS
#undef TOKEN_KIND
#undef TOKEN_KIND_KEYWORD
#undef TOKEN_KIND_SINGLE
#undef TOKEN_KIND_DOUBLE
#undef TOKEN_KIND_TRIPLE
                {
                    return Token{
                        .Kind     = TokenKind::Name,
                        .Location = startLocation,
                        .Length   = length,
                        .Data     = name,
                    };
                }
            }

            default: {
                char chr = NextChar();
                throw CompileError{
                    .Location = startLocation,
                    .Message  = (std::stringstream{} << "Unknown character '" << chr << '\'').str(),
                };
            }
        }
    }

    char Lexer::CurrentChar() {
        if (Location.Position < Source.length())
            return Source[Location.Position];
        return '\0';
    }

    char Lexer::NextChar() {
        char current = CurrentChar();
        if (current != '\0') {
            Location.Position++;
            Location.Column++;
            if (current == '\n') {
                Location.Line++;
                Location.Column = 1;
            }
        }
        return current;
    }
}
