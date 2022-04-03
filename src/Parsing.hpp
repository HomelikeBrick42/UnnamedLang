#pragma once

#include "Ast.hpp"
#include "Lexer.hpp"

namespace Langite {

    AstFile ParseFile(std::string_view filepath, std::string_view source);
    AstBlock ParseBlock(Lexer& lexer);
    AstDeclaration ParseDeclaration(Lexer& lexer);
    AstIf ParseIf(Lexer& lexer);
    Ast ParseExpression(Lexer& lexer);
    Ast ParseLeastExpression(Lexer& lexer);
    Ast ParsePrimaryExpression(Lexer& lexer);

    Ast ParseUnaryExpression(Lexer& lexer);
    Ast ParseBinaryExpression(Lexer& lexer, size_t parentPrecedence);
    size_t GetUnaryPrecedence(TokenKind kind);
    size_t GetBinaryPrecedence(TokenKind kind);

    void AllowNewline(Lexer& lexer);
    void AllowMultipleNewlines(Lexer& lexer);
    void ExpectNewline(Lexer& lexer);
    void ExpectCommaOrNewline(Lexer& lexer);
    Token ExpectToken(Lexer& lexer, TokenKind kind);

}
