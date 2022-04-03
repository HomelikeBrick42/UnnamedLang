#pragma once

#include "Token.hpp"

#include <variant>
#include <optional>
#include <memory>

namespace Langite {

    struct AstFile;
    struct AstBlock;
    struct AstUnary;
    struct AstBinary;
    struct AstFieldAccess;
    struct AstIndex;
    struct AstCall;
    struct AstGenericInstantiation;
    struct AstParenthesisedExpression;
    struct AstConstDeclaration;
    struct AstDeclaration;
    struct AstName;
    struct AstWildcard;
    struct AstInteger;
    struct AstFloat;
    struct AstString;
    struct AstFunction;
    struct AstProcedure;
    struct AstReturn;
    struct AstIf;

    using Ast = std::variant<AstFile,
                             AstBlock,
                             AstUnary,
                             AstBinary,
                             AstFieldAccess,
                             AstIndex,
                             AstCall,
                             AstGenericInstantiation,
                             AstParenthesisedExpression,
                             AstConstDeclaration,
                             AstDeclaration,
                             AstName,
                             AstWildcard,
                             AstInteger,
                             AstFloat,
                             AstString,
                             AstFunction,
                             AstProcedure,
                             AstReturn,
                             AstIf>;

    struct AstFile {
        std::vector<Ast> Expressions;
        Token EndOfFileToken;
    };

    struct AstBlock {
        Token OpenBraceToken;
        std::vector<Ast> Expressions;
        Token CloseBraceToken;
    };

    struct AstUnary {
        Token OperatorToken;
        std::unique_ptr<Ast> Operand;
    };

    struct AstBinary {
        std::unique_ptr<Ast> Left;
        Token OperatorToken;
        std::unique_ptr<Ast> Right;
    };

    struct AstFieldAccess {
        std::unique_ptr<Ast> Operand;
        Token PeriodToken;
        Token FieldNameToken;
    };

    struct AstIndex {
        std::unique_ptr<Ast> Operand;
        Token AtToken;
        std::unique_ptr<Ast> Indexer;
    };

    struct AstCall {
        std::unique_ptr<Ast> Operand;
        Token OpenParenthesisToken;
        std::vector<Ast> Arguments;
        Token CloseParenthesisToken;
    };

    struct AstGenericInstantiation {
        std::unique_ptr<Ast> Operand;
        Token OpenSquareBracketToken;
        std::vector<Ast> GenericArguments;
        Token CloseSquareBracketToken;
    };

    struct AstParenthesisedExpression {
        Token OpenParenthesisToken;
        std::unique_ptr<Ast> Expression;
        Token CloseParenthesisToken;
    };

    struct AstConstDeclaration {
        Token ConstToken;
        Token NameToken;
        std::optional<Token> OpenSquareBracketToken;
        std::optional<std::vector<AstDeclaration>> GenericParameters;
        std::optional<Token> CloseSquareBracketToken;
        std::optional<Token> ColonToken;
        std::optional<std::unique_ptr<Ast>> Type;
        Token EqualToken;
        std::unique_ptr<Ast> Value;
    };

    struct AstDeclaration {
        Token NameToken;
        Token ColonToken;
        std::unique_ptr<Ast> Type;
    };

    struct AstName {
        Token NameToken;
    };

    struct AstWildcard {
        Token WildcardToken;
    };

    struct AstInteger {
        Token IntegerToken;
    };

    struct AstFloat {
        Token FloatToken;
    };

    struct AstString {
        Token StringToken;
    };

    struct AstFunction {
        Token FuncToken;
        Token OpenParenthesisToken;
        std::vector<AstDeclaration> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<AstBlock> Body;
    };

    struct AstProcedure {
        Token ProcToken;
        Token OpenParenthesisToken;
        std::vector<AstDeclaration> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<AstBlock> Body;
    };

    struct AstReturn {
        Token ReturnToken;
        std::optional<std::unique_ptr<Ast>> Value;
    };

    struct AstIf {
        Token IfToken;
        std::unique_ptr<Ast> Condition;
        AstBlock ThenBlock;
        std::optional<Token> ElseToken;
        std::optional<std::unique_ptr<Ast>> ElseScope;
    };

}
