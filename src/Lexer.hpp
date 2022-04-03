#pragma once

#include "Token.hpp"

namespace Langite {

    class Lexer {
    public:
        Lexer(std::string_view filepath, std::string_view source);
        Token NextToken();
    private:
        char CurrentChar();
        char NextChar();
    private:
        SourceLocation Location;
        std::string_view Source;
    };

}
