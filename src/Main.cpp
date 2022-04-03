#include "SourceLocation.hpp"
#include "Token.hpp"
#include "Lexer.hpp"
#include "CompileError.hpp"

#include <iostream>

using namespace Langite;

int main(int, char**) {
    Lexer lexer{ "test.lang", "const <- hello 1 + 2 837.32\n0x43274 \"hello\" * <= >" };
    try {
        while (true) {
            Token token = lexer.NextToken();
            std::cout << TokenKind_ToString(token.Kind) << std::endl;
            if (token.Kind == TokenKind::EndOfFile)
                break;
        }
    } catch (CompileError e) {
        std::cerr << e.Location.Filepath << ':' << e.Location.Line << ':' << e.Location.Column
                  << ": " << e.Message << std::endl;
    }
    return 0;
}
