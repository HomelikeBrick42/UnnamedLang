#pragma once

#include "Token.hpp"
#include "Types.hpp"

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
    struct AstDeclaration;
    struct AstConstDeclaration;
    struct AstName;
    struct AstWildcard;
    struct AstInteger;
    struct AstFloat;
    struct AstString;
    struct AstFunction;
    struct AstProcedure;
    struct AstReturn;
    struct AstIf;
    struct AstBuiltin;

    class AstVisitor {
    public:
        virtual ~AstVisitor()                               = default;
        virtual void Visit(AstFile& ast)                    = 0;
        virtual void Visit(AstBlock& ast)                   = 0;
        virtual void Visit(AstUnary& ast)                   = 0;
        virtual void Visit(AstBinary& ast)                  = 0;
        virtual void Visit(AstFieldAccess& ast)             = 0;
        virtual void Visit(AstIndex& ast)                   = 0;
        virtual void Visit(AstCall& ast)                    = 0;
        virtual void Visit(AstGenericInstantiation& ast)    = 0;
        virtual void Visit(AstParenthesisedExpression& ast) = 0;
        virtual void Visit(AstDeclaration& ast)             = 0;
        virtual void Visit(AstConstDeclaration& ast)        = 0;
        virtual void Visit(AstName& ast)                    = 0;
        virtual void Visit(AstWildcard& ast)                = 0;
        virtual void Visit(AstInteger& ast)                 = 0;
        virtual void Visit(AstFloat& ast)                   = 0;
        virtual void Visit(AstString& ast)                  = 0;
        virtual void Visit(AstFunction& ast)                = 0;
        virtual void Visit(AstProcedure& ast)               = 0;
        virtual void Visit(AstReturn& ast)                  = 0;
        virtual void Visit(AstIf& ast)                      = 0;
        virtual void Visit(AstBuiltin& ast)                 = 0;
    };

    class AstSearcher: public AstVisitor {
    public:
        virtual ~AstSearcher() = default;
        virtual void Visit(AstFile& ast) override;
        virtual void Visit(AstBlock& ast) override;
        virtual void Visit(AstUnary& ast) override;
        virtual void Visit(AstBinary& ast) override;
        virtual void Visit(AstFieldAccess& ast) override;
        virtual void Visit(AstIndex& ast) override;
        virtual void Visit(AstCall& ast) override;
        virtual void Visit(AstGenericInstantiation& ast) override;
        virtual void Visit(AstParenthesisedExpression& ast) override;
        virtual void Visit(AstDeclaration& ast) override;
        virtual void Visit(AstConstDeclaration& ast) override;
        virtual void Visit(AstName&) override;
        virtual void Visit(AstWildcard&) override;
        virtual void Visit(AstInteger&) override;
        virtual void Visit(AstFloat&) override;
        virtual void Visit(AstString&) override;
        virtual void Visit(AstFunction& ast) override;
        virtual void Visit(AstProcedure& ast) override;
        virtual void Visit(AstReturn& ast) override;
        virtual void Visit(AstIf& ast) override;
        virtual void Visit(AstBuiltin&) override;
    };

    struct Ast {
        std::shared_ptr<Type> ResolvedType = nullptr;

        virtual ~Ast()                           = default;
        virtual void Accept(AstVisitor& visitor) = 0;
    };

    struct AstFile: Ast {
        std::vector<std::unique_ptr<Ast>> Expressions;
        Token EndOfFileToken;

        AstFile(std::vector<std::unique_ptr<Ast>> expressions, Token endOfFileToken)
            : Expressions(std::move(expressions)), EndOfFileToken(endOfFileToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstBlock: Ast {
        Token OpenBraceToken;
        std::vector<std::unique_ptr<Ast>> Expressions;
        Token CloseBraceToken;

        AstBlock(Token openBraceToken,
                 std::vector<std::unique_ptr<Ast>> expressions,
                 Token closeBraceToken)
            : OpenBraceToken(openBraceToken)
            , Expressions(std::move(expressions))
            , CloseBraceToken(closeBraceToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstUnary: Ast {
        Token OperatorToken;
        std::unique_ptr<Ast> Operand;

        AstUnary(Token operatorToken, std::unique_ptr<Ast> operand)
            : OperatorToken(operatorToken), Operand(std::move(operand)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstBinary: Ast {
        std::unique_ptr<Ast> Left;
        Token OperatorToken;
        std::unique_ptr<Ast> Right;

        AstBinary(std::unique_ptr<Ast> left, Token operatorToken, std::unique_ptr<Ast> right)
            : Left(std::move(left)), OperatorToken(operatorToken), Right(std::move(right)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstFieldAccess: Ast {
        std::unique_ptr<Ast> Operand;
        Token PeriodToken;
        Token FieldNameToken;

        AstFieldAccess(std::unique_ptr<Ast> operand, Token periodToken, Token fieldNameToken)
            : Operand(std::move(operand))
            , PeriodToken(periodToken)
            , FieldNameToken(fieldNameToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstIndex: Ast {
        std::unique_ptr<Ast> Operand;
        Token AtToken;
        std::unique_ptr<Ast> Indexer;

        AstIndex(std::unique_ptr<Ast> operand, Token atToken, std::unique_ptr<Ast> indexer)
            : Operand(std::move(operand)), AtToken(atToken), Indexer(std::move(indexer)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstCall: Ast {
        std::unique_ptr<Ast> Operand;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<Ast>> Arguments;
        Token CloseParenthesisToken;

        AstCall(std::unique_ptr<Ast> operand,
                Token openParenthesisToken,
                std::vector<std::unique_ptr<Ast>> arguments,
                Token closeParenthesisToken)
            : Operand(std::move(operand))
            , OpenParenthesisToken(openParenthesisToken)
            , Arguments(std::move(arguments))
            , CloseParenthesisToken(closeParenthesisToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstGenericInstantiation: Ast {
        std::unique_ptr<Ast> Operand;
        Token OpenSquareBracketToken;
        std::vector<std::unique_ptr<Ast>> GenericArguments;
        Token CloseSquareBracketToken;

        AstGenericInstantiation(std::unique_ptr<Ast> operand,
                                Token openSquareBracketToken,
                                std::vector<std::unique_ptr<Ast>> genericArguments,
                                Token closeSquareBracketToken)
            : Operand(std::move(operand))
            , OpenSquareBracketToken(openSquareBracketToken)
            , GenericArguments(std::move(genericArguments))
            , CloseSquareBracketToken(closeSquareBracketToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstParenthesisedExpression: Ast {
        Token OpenParenthesisToken;
        std::unique_ptr<Ast> Expression;
        Token CloseParenthesisToken;

        AstParenthesisedExpression(Token openParenthesisToken,
                                   std::unique_ptr<Ast> expression,
                                   Token closeParenthesisToken)
            : OpenParenthesisToken(openParenthesisToken)
            , Expression(std::move(expression))
            , CloseParenthesisToken(closeParenthesisToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstDeclaration: Ast {
        Token NameToken;
        Token ColonToken;
        std::unique_ptr<Ast> Type;
        bool IsGenericParameter = false;

        AstDeclaration(Token nameToken, Token colonToken, std::unique_ptr<Ast> type)
            : NameToken(nameToken), ColonToken(colonToken), Type(std::move(type)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
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

        AstConstDeclaration(
            Token constToken,
            Token nameToken,
            std::optional<Token> openSquareBracketToken,
            std::optional<std::vector<std::unique_ptr<AstDeclaration>>> genericParameters,
            std::optional<Token> closeSquareBracketToken,
            std::optional<Token> colonToken,
            std::optional<std::unique_ptr<Ast>> type,
            Token equalToken,
            std::unique_ptr<Ast> value)
            : ConstToken(constToken)
            , NameToken(nameToken)
            , OpenSquareBracketToken(openSquareBracketToken)
            , GenericParameters(std::move(genericParameters))
            , CloseSquareBracketToken(closeSquareBracketToken)
            , ColonToken(colonToken)
            , Type(std::move(type))
            , EqualToken(equalToken)
            , Value(std::move(value)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstName: Ast {
        Token NameToken;
        Ast* ResolvedDeclaration;

        AstName(Token nameToken) : NameToken(nameToken), ResolvedDeclaration(nullptr) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstWildcard: Ast {
        Token WildcardToken;

        AstWildcard(Token wildcardToken) : WildcardToken(wildcardToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstInteger: Ast {
        Token IntegerToken;

        AstInteger(Token integerToken) : IntegerToken(integerToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstFloat: Ast {
        Token FloatToken;

        AstFloat(Token floatToken) : FloatToken(floatToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstString: Ast {
        Token StringToken;

        AstString(Token stringToken) : StringToken(stringToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstFunction: Ast {
        Token FuncToken;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<AstDeclaration>> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<std::unique_ptr<AstBlock>> Body;

        AstFunction(Token funcToken,
                    Token openParenthesisToken,
                    std::vector<std::unique_ptr<AstDeclaration>> parameters,
                    Token closeParenthesisToken,
                    Token colonToken,
                    std::unique_ptr<Ast> returnType,
                    std::optional<std::unique_ptr<AstBlock>> body)
            : FuncToken(funcToken)
            , OpenParenthesisToken(openParenthesisToken)
            , Parameters(std::move(parameters))
            , CloseParenthesisToken(closeParenthesisToken)
            , ColonToken(colonToken)
            , ReturnType(std::move(returnType))
            , Body(std::move(body)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstProcedure: Ast {
        Token ProcToken;
        Token OpenParenthesisToken;
        std::vector<std::unique_ptr<AstDeclaration>> Parameters;
        Token CloseParenthesisToken;
        Token ColonToken;
        std::unique_ptr<Ast> ReturnType;
        std::optional<std::unique_ptr<AstBlock>> Body;

        AstProcedure(Token procToken,
                     Token openParenthesisToken,
                     std::vector<std::unique_ptr<AstDeclaration>> parameters,
                     Token closeParenthesisToken,
                     Token colonToken,
                     std::unique_ptr<Ast> returnType,
                     std::optional<std::unique_ptr<AstBlock>> body)
            : ProcToken(procToken)
            , OpenParenthesisToken(openParenthesisToken)
            , Parameters(std::move(parameters))
            , CloseParenthesisToken(closeParenthesisToken)
            , ColonToken(colonToken)
            , ReturnType(std::move(returnType))
            , Body(std::move(body)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstReturn: Ast {
        Token ReturnToken;
        std::optional<std::unique_ptr<Ast>> Value;

        AstReturn(Token returnToken, std::optional<std::unique_ptr<Ast>> value)
            : ReturnToken(returnToken), Value(std::move(value)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstIf: Ast {
        Token IfToken;
        std::unique_ptr<Ast> Condition;
        std::unique_ptr<AstBlock> ThenBlock;
        std::optional<Token> ElseToken;
        std::optional<std::unique_ptr<Ast>> ElseScope;

        AstIf(Token ifToken,
              std::unique_ptr<Ast> condition,
              std::unique_ptr<AstBlock> thenBlock,
              std::optional<Token> elseToken,
              std::optional<std::unique_ptr<Ast>> elseScope)
            : IfToken(ifToken)
            , Condition(std::move(condition))
            , ThenBlock(std::move(thenBlock))
            , ElseToken(elseToken)
            , ElseScope(std::move(elseScope)) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    struct AstBuiltin: Ast {
        Token BuiltinToken;
        Token StringToken;

        AstBuiltin(Token builtinToken, Token stringToken)
            : BuiltinToken(builtinToken), StringToken(stringToken) {}

        virtual void Accept(AstVisitor& visitor) override {
            return visitor.Visit(*this);
        }
    };

    void DumpAst(Ast& ast);

}
