#include "Parsing.hpp"
#include "CompileError.hpp"

#include <sstream>
#include <assert.h>

namespace Langite {

    std::unique_ptr<AstFile> ParseFile(std::string_view filepath, std::string_view source) {
        Lexer lexer{ filepath, source };
        std::vector<std::unique_ptr<Ast>> expressions;
        while (lexer.PeekKind() != TokenKind::EndOfFile) {
            AllowMultipleNewlines(lexer);
            expressions.push_back(ParseExpression(lexer));
            ExpectNewline(lexer);
        }
        Token endOfFileToken = lexer.NextToken();
        assert(endOfFileToken.Kind == TokenKind::EndOfFile);
        return std::make_unique<AstFile>(std::move(expressions), endOfFileToken);
    }

    std::unique_ptr<AstBlock> ParseBlock(Lexer& lexer) {
        Token openBraceToken = ExpectToken(lexer, TokenKind::OpenBrace);
        std::vector<std::unique_ptr<Ast>> expressions;
        while (lexer.PeekKind() != TokenKind::CloseBrace) {
            AllowMultipleNewlines(lexer);
            expressions.push_back(ParseExpression(lexer));
            ExpectNewline(lexer);
        }
        Token closeBraceToken = ExpectToken(lexer, TokenKind::CloseBrace);
        return std::make_unique<AstBlock>(openBraceToken, std::move(expressions), closeBraceToken);
    }

    std::unique_ptr<AstDeclaration> ParseDeclaration(Lexer& lexer) {
        Token nameToken           = ExpectToken(lexer, TokenKind::Name);
        Token colonToken          = ExpectToken(lexer, TokenKind::Colon);
        std::unique_ptr<Ast> type = ParseExpression(lexer);
        return std::make_unique<AstDeclaration>(nameToken, colonToken, std::move(type));
    }

    std::unique_ptr<AstIf> ParseIf(Lexer& lexer) {
        Token ifToken                       = ExpectToken(lexer, TokenKind::If);
        std::unique_ptr<Ast> condition      = ParseExpression(lexer);
        std::unique_ptr<AstBlock> thenBlock = ParseBlock(lexer);
        std::optional<Token> elseToken;
        std::optional<std::unique_ptr<Ast>> elseScope;
        if (lexer.PeekKind() == TokenKind::Else) {
            elseToken = lexer.NextToken();
            if (lexer.PeekKind() == TokenKind::If)
                elseScope = ParseIf(lexer);
            else
                elseScope = ParseBlock(lexer);
        }
        return std::make_unique<AstIf>(
            ifToken, std::move(condition), std::move(thenBlock), elseToken, std::move(elseScope));
    }

    std::unique_ptr<Ast> ParseExpression(Lexer& lexer) {
        return ParseBinaryExpression(lexer, 0);
    }

    std::unique_ptr<Ast> ParseLeastExpression(Lexer& lexer) {
        return ParseBinaryExpression(lexer, std::numeric_limits<size_t>::max());
    }

    std::unique_ptr<Ast> ParsePrimaryExpression(Lexer& lexer) {
        switch (lexer.PeekKind()) {
            case TokenKind::OpenParenthesis: {
                Token openParenthesisToken      = lexer.NextToken();
                std::unique_ptr<Ast> expression = ParseExpression(lexer);
                Token closeParenthesisToken     = ExpectToken(lexer, TokenKind::CloseParenthesis);
                return std::make_unique<AstParenthesisedExpression>(
                    openParenthesisToken, std::move(expression), closeParenthesisToken);
            }

            case TokenKind::Const: {
                Token constToken = lexer.NextToken();
                Token nameToken;
                if (lexer.PeekKind() == TokenKind::Wildcard)
                    nameToken = lexer.NextToken();
                else
                    nameToken = ExpectToken(lexer, TokenKind::Name);
                std::optional<Token> openSquareBracketToken;
                std::optional<std::vector<std::unique_ptr<AstDeclaration>>> genericParameters;
                std::optional<Token> closeSquareBracketToken;
                if (lexer.PeekKind() == TokenKind::OpenSquareBracket) {
                    openSquareBracketToken = lexer.NextToken();
                    genericParameters      = std::vector<std::unique_ptr<AstDeclaration>>{};
                    while (lexer.PeekKind() != TokenKind::CloseSquareBracket) {
                        genericParameters->push_back(ParseDeclaration(lexer));
                        ExpectCommaOrNewline(lexer);
                    }
                    closeSquareBracketToken = ExpectToken(lexer, TokenKind::CloseSquareBracket);
                }
                std::optional<Token> colonToken;
                std::optional<std::unique_ptr<Ast>> type;
                if (lexer.PeekKind() == TokenKind::Colon) {
                    colonToken = lexer.NextToken();
                    type       = ParseExpression(lexer);
                }
                Token equalToken           = ExpectToken(lexer, TokenKind::Equal);
                std::unique_ptr<Ast> value = ParseExpression(lexer);
                return std::make_unique<AstConstDeclaration>(constToken,
                                                             nameToken,
                                                             openSquareBracketToken,
                                                             std::move(genericParameters),
                                                             closeSquareBracketToken,
                                                             colonToken,
                                                             std::move(type),
                                                             equalToken,
                                                             std::move(value));
            }

            case TokenKind::Name: {
                Token nameToken = lexer.NextToken();
                if (lexer.PeekKind() == TokenKind::Colon) {
                    Token colonToken          = lexer.NextToken();
                    std::unique_ptr<Ast> type = ParseLeastExpression(lexer);
                    return std::make_unique<AstDeclaration>(nameToken, colonToken, std::move(type));
                } else {
                    return std::make_unique<AstName>(nameToken);
                }
            }

            case TokenKind::Wildcard:
                return std::make_unique<AstWildcard>(lexer.NextToken());

            case TokenKind::Integer:
                return std::make_unique<AstInteger>(lexer.NextToken());

            case TokenKind::Float:
                return std::make_unique<AstFloat>(lexer.NextToken());

            case TokenKind::String:
                return std::make_unique<AstString>(lexer.NextToken());

            case TokenKind::Func: {
                Token funcToken            = lexer.NextToken();
                Token openParenthesisToken = ExpectToken(lexer, TokenKind::OpenParenthesis);
                std::vector<std::unique_ptr<AstDeclaration>> parameters;
                while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                    parameters.push_back(ParseDeclaration(lexer));
                    ExpectCommaOrNewline(lexer);
                }
                Token closeParenthesisToken     = ExpectToken(lexer, TokenKind::CloseParenthesis);
                Token colonToken                = ExpectToken(lexer, TokenKind::Colon);
                std::unique_ptr<Ast> returnType = ParseLeastExpression(lexer);
                std::optional<std::unique_ptr<AstBlock>> body;
                if (lexer.PeekKind() == TokenKind::OpenBrace)
                    body = ParseBlock(lexer);
                return std::make_unique<AstFunction>(funcToken,
                                                     openParenthesisToken,
                                                     std::move(parameters),
                                                     closeParenthesisToken,
                                                     colonToken,
                                                     std::move(returnType),
                                                     std::move(body));
            }

            case TokenKind::Proc: {
                Token procToken            = lexer.NextToken();
                Token openParenthesisToken = ExpectToken(lexer, TokenKind::OpenParenthesis);
                std::vector<std::unique_ptr<AstDeclaration>> parameters;
                while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                    parameters.push_back(ParseDeclaration(lexer));
                    ExpectCommaOrNewline(lexer);
                }
                Token closeParenthesisToken     = ExpectToken(lexer, TokenKind::CloseParenthesis);
                Token colonToken                = ExpectToken(lexer, TokenKind::Colon);
                std::unique_ptr<Ast> returnType = ParseLeastExpression(lexer);
                std::optional<std::unique_ptr<AstBlock>> body;
                if (lexer.PeekKind() == TokenKind::OpenBrace)
                    body = ParseBlock(lexer);
                return std::make_unique<AstProcedure>(procToken,
                                                      openParenthesisToken,
                                                      std::move(parameters),
                                                      closeParenthesisToken,
                                                      colonToken,
                                                      std::move(returnType),
                                                      std::move(body));
            }

            case TokenKind::Return: {
                Token returnToken = lexer.NextToken();
                std::optional<std::unique_ptr<Ast>> value;
                if (lexer.PeekKind() != TokenKind::EndOfFile &&
                    lexer.PeekKind() != TokenKind::CloseParenthesis &&
                    lexer.PeekKind() != TokenKind::CloseSquareBracket &&
                    lexer.PeekKind() != TokenKind::Newline)
                    value = ParseExpression(lexer);
                return std::make_unique<AstReturn>(returnToken, std::move(value));
            }

            case TokenKind::If:
                return ParseIf(lexer);

            case TokenKind::OpenBrace:
                return ParseBlock(lexer);

            default: {
                Token token = lexer.NextToken();
                throw CompileError{
                    .Location = token.Location,
                    .Message  = (std::stringstream{} << "Expected an expression, but got '"
                                                    << TokenKind_ToString(token.Kind) << '\'')
                                   .str(),
                };
            }
        }
    }

    std::unique_ptr<Ast> ParseBinaryExpression(Lexer& lexer, size_t parentPrecedence) {
        std::unique_ptr<Ast> left; // this is to delay construction of the std::variant

        size_t unaryPrecedence = GetUnaryPrecedence(lexer.PeekKind());
        if (unaryPrecedence > 0) {
            Token operatorToken          = lexer.NextToken();
            std::unique_ptr<Ast> operand = ParseBinaryExpression(lexer, unaryPrecedence);

            left = std::make_unique<AstUnary>(operatorToken, std::move(operand));
        } else {
            left = ParsePrimaryExpression(lexer);
        }

        while (true) {
            switch (lexer.PeekKind()) {
                case TokenKind::Period: {
                    Token periodToken    = lexer.NextToken();
                    Token fieldNameToken = ExpectToken(lexer, TokenKind::Name);

                    left = std::make_unique<AstFieldAccess>(
                        std::move(left), periodToken, fieldNameToken);
                } break;

                case TokenKind::At: {
                    Token atToken                = lexer.NextToken();
                    std::unique_ptr<Ast> indexer = ParseLeastExpression(lexer);

                    left = std::make_unique<AstIndex>(std::move(left), atToken, std::move(indexer));
                } break;

                case TokenKind::OpenParenthesis: {
                    Token openParenthesisToken = lexer.NextToken();
                    std::vector<std::unique_ptr<Ast>> arguments;
                    while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                        arguments.push_back(ParseExpression(lexer));
                        ExpectCommaOrNewline(lexer);
                    }
                    Token closeParenthesisToken = ExpectToken(lexer, TokenKind::CloseParenthesis);

                    left = std::make_unique<AstCall>(std::move(left),
                                                     openParenthesisToken,
                                                     std::move(arguments),
                                                     closeParenthesisToken);
                } break;

                case TokenKind::OpenSquareBracket: {
                    Token openSquareBracketToken = lexer.NextToken();
                    std::vector<std::unique_ptr<Ast>> genericArguments;
                    while (lexer.PeekKind() != TokenKind::CloseSquareBracket) {
                        genericArguments.push_back(ParseExpression(lexer));
                        ExpectCommaOrNewline(lexer);
                    }
                    Token closeSquareBracketToken =
                        ExpectToken(lexer, TokenKind::CloseSquareBracket);

                    left = std::make_unique<AstGenericInstantiation>(std::move(left),
                                                                     openSquareBracketToken,
                                                                     std::move(genericArguments),
                                                                     closeSquareBracketToken);
                } break;

                default: {
                    size_t binaryPrecedence = GetBinaryPrecedence(lexer.PeekKind());
                    if (binaryPrecedence <= parentPrecedence)
                        goto End;

                    Token operatorToken        = lexer.NextToken();
                    std::unique_ptr<Ast> right = ParseBinaryExpression(lexer, binaryPrecedence);

                    left = std::make_unique<AstBinary>(
                        std::move(left), operatorToken, std::move(right));
                } break;
            }
        }
End:

        return left;
    }

    size_t GetUnaryPrecedence(TokenKind kind) {
        switch (kind) {
            case TokenKind::Plus:
            case TokenKind::Minus:
            case TokenKind::ExclamationMark:
                return 5;

            default:
                return 0;
        }
    }

    size_t GetBinaryPrecedence(TokenKind kind) {
        switch (kind) {
            case TokenKind::Asterisk:
            case TokenKind::Slash:
            case TokenKind::Modulus:
                return 4;

            case TokenKind::Plus:
            case TokenKind::Minus:
                return 3;

            case TokenKind::EqualEqual:
            case TokenKind::ExclamationMarkEqual:
                return 2;

            case TokenKind::LeftArrow:
            case TokenKind::RightArrow:
                return 1;

            default:
                return 0;
        }
    }

    void AllowNewline(Lexer& lexer) {
        if (lexer.PeekKind() == TokenKind::Newline)
            lexer.NextToken();
    }

    void AllowMultipleNewlines(Lexer& lexer) {
        while (lexer.PeekKind() == TokenKind::Newline)
            lexer.NextToken();
    }

    void ExpectNewline(Lexer& lexer) {
        TokenKind kind = lexer.PeekKind();
        if (kind != TokenKind::EndOfFile && kind != TokenKind::CloseParenthesis &&
            kind != TokenKind::CloseBrace && kind != TokenKind::CloseSquareBracket)
            ExpectToken(lexer, TokenKind::Newline);
    }

    void ExpectCommaOrNewline(Lexer& lexer) {
        TokenKind kind = lexer.PeekKind();
        if (kind != TokenKind::EndOfFile && kind != TokenKind::CloseParenthesis &&
            kind != TokenKind::CloseSquareBracket && kind != TokenKind::Newline)
            ExpectToken(lexer, TokenKind::Comma);
        AllowNewline(lexer);
    }

    Token ExpectToken(Lexer& lexer, TokenKind kind) {
        Token token = lexer.NextToken();
        if (token.Kind == kind)
            return token;
        throw CompileError{
            .Location = token.Location,
            .Message =
                (std::stringstream{} << "Expected '" << TokenKind_ToString(kind) << "', but got '"
                                     << TokenKind_ToString(token.Kind) << '\'')
                    .str(),
        };
    }
}
