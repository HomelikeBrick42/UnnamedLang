#include "Parsing.hpp"
#include "CompileError.hpp"

#include <sstream>
#include <assert.h>

namespace Langite {

    AstFile ParseFile(std::string_view filepath, std::string_view source) {
        Lexer lexer{ filepath, source };
        std::vector<Ast> expressions;
        while (lexer.PeekKind() != TokenKind::EndOfFile) {
            AllowMultipleNewlines(lexer);
            expressions.push_back(ParseExpression(lexer));
            ExpectNewline(lexer);
        }
        Token endOfFileToken = lexer.NextToken();
        assert(endOfFileToken.Kind == TokenKind::EndOfFile);
        return AstFile{
            .Expressions    = std::move(expressions),
            .EndOfFileToken = endOfFileToken,
        };
    }

    AstBlock ParseBlock(Lexer& lexer) {
        Token openBraceToken = ExpectToken(lexer, TokenKind::OpenBrace);
        std::vector<Ast> expressions;
        while (lexer.PeekKind() != TokenKind::CloseBrace) {
            AllowMultipleNewlines(lexer);
            expressions.push_back(ParseExpression(lexer));
            ExpectNewline(lexer);
        }
        Token closeBraceToken = ExpectToken(lexer, TokenKind::CloseBrace);
        return AstBlock{
            .OpenBraceToken  = openBraceToken,
            .Expressions     = std::move(expressions),
            .CloseBraceToken = closeBraceToken,
        };
    }

    AstDeclaration ParseDeclaration(Lexer& lexer) {
        Token nameToken  = ExpectToken(lexer, TokenKind::Name);
        Token colonToken = ExpectToken(lexer, TokenKind::Colon);
        Ast type         = ParseExpression(lexer);
        return AstDeclaration{
            .NameToken  = nameToken,
            .ColonToken = colonToken,
            .Type       = std::make_unique<Ast>(std::move(type)),
        };
    }

    AstIf ParseIf(Lexer& lexer) {
        Token ifToken      = ExpectToken(lexer, TokenKind::If);
        Ast condition      = ParseExpression(lexer);
        AstBlock thenBlock = ParseBlock(lexer);
        std::optional<Token> elseToken;
        std::optional<std::unique_ptr<Ast>> elseScope;
        if (lexer.PeekKind() == TokenKind::Else) {
            elseToken = lexer.NextToken();
            if (lexer.PeekKind() == TokenKind::If)
                elseScope = std::make_unique<Ast>(ParseIf(lexer));
            else
                elseScope = std::make_unique<Ast>(ParseBlock(lexer));
        }
        return AstIf{
            .IfToken   = ifToken,
            .Condition = std::make_unique<Ast>(std::move(condition)),
            .ThenBlock = std::move(thenBlock),
            .ElseToken = elseToken,
            .ElseScope = std::move(elseScope),
        };
    }

    Ast ParseExpression(Lexer& lexer) {
        return ParseBinaryExpression(lexer, 0);
    }

    Ast ParseLeastExpression(Lexer& lexer) {
        return ParseBinaryExpression(lexer, std::numeric_limits<size_t>::max());
    }

    Ast ParsePrimaryExpression(Lexer& lexer) {
        switch (lexer.PeekKind()) {
            case TokenKind::OpenParenthesis: {
                Token openParenthesisToken  = lexer.NextToken();
                Ast expression              = ParseExpression(lexer);
                Token closeParenthesisToken = ExpectToken(lexer, TokenKind::CloseParenthesis);
                return AstParenthesisedExpression{
                    .OpenParenthesisToken  = openParenthesisToken,
                    .Expression            = std::make_unique<Ast>(std::move(expression)),
                    .CloseParenthesisToken = closeParenthesisToken,
                };
            }

            case TokenKind::Const: {
                Token constToken = lexer.NextToken();
                Token nameToken;
                if (lexer.PeekKind() == TokenKind::Wildcard)
                    nameToken = lexer.NextToken();
                else
                    nameToken = ExpectToken(lexer, TokenKind::Name);
                std::optional<Token> openSquareBracketToken;
                std::optional<std::vector<AstDeclaration>> genericParameters;
                std::optional<Token> closeSquareBracketToken;
                if (lexer.PeekKind() == TokenKind::OpenSquareBracket) {
                    openSquareBracketToken = lexer.NextToken();
                    genericParameters      = std::vector<AstDeclaration>{};
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
                    type       = std::make_unique<Ast>(ParseExpression(lexer));
                }
                Token equalToken = ExpectToken(lexer, TokenKind::Equal);
                Ast value        = ParseExpression(lexer);
                return AstConstDeclaration{
                    .ConstToken              = constToken,
                    .NameToken               = nameToken,
                    .OpenSquareBracketToken  = openSquareBracketToken,
                    .GenericParameters       = std::move(genericParameters),
                    .CloseSquareBracketToken = closeSquareBracketToken,
                    .ColonToken              = colonToken,
                    .Type                    = std::move(type),
                    .EqualToken              = equalToken,
                    .Value                   = std::make_unique<Ast>(std::move(value)),
                };
            }

            case TokenKind::Name: {
                Token nameToken = lexer.NextToken();
                if (lexer.PeekKind() == TokenKind::Colon) {
                    Token colonToken = lexer.NextToken();
                    Ast type         = ParseExpression(lexer);
                    return AstDeclaration{
                        .NameToken  = nameToken,
                        .ColonToken = colonToken,
                        .Type       = std::make_unique<Ast>(std::move(type)),
                    };
                } else {
                    return AstName{
                        .NameToken = nameToken,
                    };
                }
            }

            case TokenKind::Wildcard:
                return AstWildcard{
                    .WildcardToken = lexer.NextToken(),
                };

            case TokenKind::Integer:
                return AstInteger{
                    .IntegerToken = lexer.NextToken(),
                };

            case TokenKind::Float:
                return AstFloat{
                    .FloatToken = lexer.NextToken(),
                };

            case TokenKind::String:
                return AstString{
                    .StringToken = lexer.NextToken(),
                };

            case TokenKind::Func: {
                Token funcToken            = lexer.NextToken();
                Token openParenthesisToken = ExpectToken(lexer, TokenKind::OpenParenthesis);
                std::vector<AstDeclaration> parameters;
                while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                    parameters.push_back(ParseDeclaration(lexer));
                    ExpectCommaOrNewline(lexer);
                }
                Token closeParenthesisToken = ExpectToken(lexer, TokenKind::CloseParenthesis);
                Token colonToken            = ExpectToken(lexer, TokenKind::Colon);
                Ast returnType              = ParseLeastExpression(lexer);
                std::optional<AstBlock> body;
                if (lexer.PeekKind() == TokenKind::OpenBrace)
                    body = ParseBlock(lexer);
                return AstFunction{
                    .FuncToken             = funcToken,
                    .OpenParenthesisToken  = openParenthesisToken,
                    .Parameters            = std::move(parameters),
                    .CloseParenthesisToken = closeParenthesisToken,
                    .ColonToken            = colonToken,
                    .ReturnType            = std::make_unique<Ast>(std::move(returnType)),
                    .Body                  = std::move(body),
                };
            }

            case TokenKind::Proc: {
                Token procToken            = lexer.NextToken();
                Token openParenthesisToken = ExpectToken(lexer, TokenKind::OpenParenthesis);
                std::vector<AstDeclaration> parameters;
                while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                    parameters.push_back(ParseDeclaration(lexer));
                    ExpectCommaOrNewline(lexer);
                }
                Token closeParenthesisToken = ExpectToken(lexer, TokenKind::CloseParenthesis);
                Token colonToken            = ExpectToken(lexer, TokenKind::Colon);
                Ast returnType              = ParseLeastExpression(lexer);
                std::optional<AstBlock> body;
                if (lexer.PeekKind() == TokenKind::OpenBrace)
                    body = ParseBlock(lexer);
                return AstProcedure{
                    .ProcToken             = procToken,
                    .OpenParenthesisToken  = openParenthesisToken,
                    .Parameters            = std::move(parameters),
                    .CloseParenthesisToken = closeParenthesisToken,
                    .ColonToken            = colonToken,
                    .ReturnType            = std::make_unique<Ast>(std::move(returnType)),
                    .Body                  = std::move(body),
                };
            }

            case TokenKind::Return: {
                Token returnToken = lexer.NextToken();
                std::optional<std::unique_ptr<Ast>> value;
                if (lexer.PeekKind() != TokenKind::EndOfFile &&
                    lexer.PeekKind() != TokenKind::CloseParenthesis &&
                    lexer.PeekKind() != TokenKind::CloseSquareBracket &&
                    lexer.PeekKind() != TokenKind::Newline)
                    value = std::make_unique<Ast>(ParseExpression(lexer));
                return AstReturn{
                    .ReturnToken = returnToken,
                    .Value       = std::move(value),
                };
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

    Ast ParseBinaryExpression(Lexer& lexer, size_t parentPrecedence) {
        std::optional<Ast> left_; // this is to delay construction of the std::variant

        size_t unaryPrecedence = GetUnaryPrecedence(lexer.PeekKind());
        if (unaryPrecedence > 0) {
            Token operatorToken = lexer.NextToken();
            Ast operand         = ParseBinaryExpression(lexer, unaryPrecedence);

            left_ = AstUnary{
                .OperatorToken = operatorToken,
                .Operand       = std::make_unique<Ast>(std::move(operand)),
            };
        } else {
            left_ = ParsePrimaryExpression(lexer);
        }

        Ast left = std::move(*left_); // move it to an actual variable

        while (true) {
            switch (lexer.PeekKind()) {
                case TokenKind::Period: {
                    Token periodToken    = lexer.NextToken();
                    Token fieldNameToken = ExpectToken(lexer, TokenKind::Name);

                    left = AstFieldAccess{
                        .Operand        = std::make_unique<Ast>(std::move(left)),
                        .PeriodToken    = periodToken,
                        .FieldNameToken = fieldNameToken,
                    };
                } break;

                case TokenKind::At: {
                    Token atToken = lexer.NextToken();
                    Ast indexer   = ParseLeastExpression(lexer);

                    left = AstIndex{
                        .Operand = std::make_unique<Ast>(std::move(left)),
                        .AtToken = atToken,
                        .Indexer = std::make_unique<Ast>(std::move(indexer)),
                    };
                } break;

                case TokenKind::OpenParenthesis: {
                    Token openParenthesisToken = lexer.NextToken();
                    std::vector<Ast> arguments;
                    while (lexer.PeekKind() != TokenKind::CloseParenthesis) {
                        arguments.push_back(ParseExpression(lexer));
                        ExpectCommaOrNewline(lexer);
                    }
                    Token closeParenthesisToken = ExpectToken(lexer, TokenKind::CloseParenthesis);

                    left = AstCall{
                        .Operand               = std::make_unique<Ast>(std::move(left)),
                        .OpenParenthesisToken  = openParenthesisToken,
                        .Arguments             = std::move(arguments),
                        .CloseParenthesisToken = closeParenthesisToken,
                    };
                } break;

                case TokenKind::OpenSquareBracket: {
                    Token openSquareBracketToken = lexer.NextToken();
                    std::vector<Ast> genericArguments;
                    while (lexer.PeekKind() != TokenKind::CloseSquareBracket) {
                        genericArguments.push_back(ParseExpression(lexer));
                        ExpectCommaOrNewline(lexer);
                    }
                    Token closeSquareBracketToken =
                        ExpectToken(lexer, TokenKind::CloseSquareBracket);

                    left = AstGenericInstantiation{
                        .Operand                 = std::make_unique<Ast>(std::move(left)),
                        .OpenSquareBracketToken  = openSquareBracketToken,
                        .GenericArguments        = std::move(genericArguments),
                        .CloseSquareBracketToken = closeSquareBracketToken,
                    };
                } break;

                default: {
                    size_t binaryPrecedence = GetBinaryPrecedence(lexer.PeekKind());
                    if (binaryPrecedence <= parentPrecedence)
                        goto End;

                    Token operatorToken = lexer.NextToken();
                    Ast right           = ParseBinaryExpression(lexer, binaryPrecedence);

                    left = AstBinary{
                        .Left          = std::make_unique<Ast>(std::move(left)),
                        .OperatorToken = operatorToken,
                        .Right         = std::make_unique<Ast>(std::move(right)),
                    };
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
