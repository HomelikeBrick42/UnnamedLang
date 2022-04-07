#pragma once

#include "Ast.hpp"
#include "Lexer.hpp"

namespace Langite {

    std::unique_ptr<AstFile> ParseFile(std::string_view filepath, std::string_view source);
    std::unique_ptr<AstBlock> ParseBlock(Lexer& lexer);
    std::unique_ptr<AstDeclaration> ParseDeclaration(Lexer& lexer);
    std::unique_ptr<AstIf> ParseIf(Lexer& lexer);
    std::unique_ptr<Ast> ParseExpression(Lexer& lexer);
    std::unique_ptr<Ast> ParseLeastExpression(Lexer& lexer);
    std::unique_ptr<Ast> ParsePrimaryExpression(Lexer& lexer);

    std::unique_ptr<Ast> ParseUnaryExpression(Lexer& lexer);
    std::unique_ptr<Ast> ParseBinaryExpression(Lexer& lexer, size_t parentPrecedence);
    size_t GetUnaryPrecedence(TokenKind kind);
    size_t GetBinaryPrecedence(TokenKind kind);

    void AllowNewline(Lexer& lexer);
    void AllowMultipleNewlines(Lexer& lexer);
    void ExpectNewline(Lexer& lexer);
    void ExpectCommaOrNewline(Lexer& lexer);
    Token ExpectToken(Lexer& lexer, TokenKind kind);

}
