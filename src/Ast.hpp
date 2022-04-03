#pragma once

#include "Token.hpp"

#include <variant>
#include <optional>
#include <memory>

namespace Langite {

    struct Ast {};

    struct AstFile: Ast {
        std::vector<std::unique_ptr<Ast>> Expressions;
        Token EndOfFileToken;
    };

    struct AstBlock: Ast {
        Token OpenBraceToken;
        std::vector<std::unique_ptr<Ast>> Expressions;
        Token CloseBraceToken;
    };

    struct AstUnary: Ast {
        Token OperatorToken;
        std::unique_ptr<Ast> Operand;
    };

    struct AstBinary: Ast {
        std::unique_ptr<Ast> Left;
        Token OperatorToken;
        std::unique_ptr<Ast> Right;
    };

    struct AstFieldAccess: Ast {
        std::unique_ptr<Ast> Operand;
        Token PeriodToken;
        Token FieldNameToken;
    };

    struct AstIndex: Ast {
        std::unique_ptr<Ast> Operand;
        Token AtToken;
        std::unique_ptr<Ast> Indexer;
    };

    struct AstCall: Ast {
        std::unique_ptr<Ast> Operand;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<Ast>> Arguments;
        Token CloseParenthesisToken;
    };

    struct AstGenericInstantiation: Ast {
        std::unique_ptr<Ast> Operand;
        Token OpenSquareBracketToken;
        std::vector<std::unique_ptr<Ast>> GenericArguments;
        Token CloseSquareBracketToken;
    };

    struct AstParenthesisedExpression: Ast {
        Token OpenParenthesisToken;
        std::unique_ptr<Ast> Expression;
        Token CloseParenthesisToken;
    };

    struct AstDeclaration: Ast {
        Token NameToken;
        Token ColonToken;
        std::unique_ptr<Ast> Type;
    };

    struct AstConstDeclaration: Ast {
        Token ConstToken;
        Token NameToken;
        std::optional<Token> OpenSquareBracketToken;
        std::optional<std::vector<std::unique_ptr<AstDeclaration>>> GenericParameters;
        std::optional<Token> CloseSquareBracketToken;
        std::optional<Token> ColonToken;
        std::optional<std::unique_ptr<Ast>> Type;
        Token EqualToken;
        std::unique_ptr<Ast> Value;
    };

    struct AstName: Ast {
        Token NameToken;
    };

    struct AstWildcard: Ast {
        Token WildcardToken;
    };

    struct AstInteger: Ast {
        Token IntegerToken;
    };

    struct AstFloat: Ast {
        Token FloatToken;
    };

    struct AstString: Ast {
        Token StringToken;
    };

    struct AstFunction: Ast {
        Token FuncToken;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<AstDeclaration>> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<std::unique_ptr<AstBlock>> Body;
    };

    struct AstProcedure: Ast {
        Token ProcToken;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<AstDeclaration>> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<std::unique_ptr<AstBlock>> Body;
    };

    struct AstReturn: Ast {
        Token ReturnToken;
        std::optional<std::unique_ptr<Ast>> Value;
    };

    struct AstIf: Ast {
        Token IfToken;
        std::unique_ptr<Ast> Condition;
        std::unique_ptr<AstBlock> ThenBlock;
        std::optional<Token> ElseToken;
        std::optional<std::unique_ptr<Ast>> ElseScope;
    };

}
