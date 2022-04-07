#include "Ast.hpp"

#include <iostream>

namespace Langite {

    void AstSearcher::Visit(AstFile& ast) {
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
    }

    void AstSearcher::Visit(AstBlock& ast) {
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
    }

    void AstSearcher::Visit(AstUnary& ast) {
        ast.Operand->Accept(*this);
    }

    void AstSearcher::Visit(AstBinary& ast) {
        ast.Left->Accept(*this);
        ast.Right->Accept(*this);
    }

    void AstSearcher::Visit(AstFieldAccess& ast) {
        ast.Operand->Accept(*this);
    }

    void AstSearcher::Visit(AstIndex& ast) {
        ast.Operand->Accept(*this);
        ast.Indexer->Accept(*this);
    }

    void AstSearcher::Visit(AstCall& ast) {
        ast.Operand->Accept(*this);
        for (auto& argument : ast.Arguments)
            argument->Accept(*this);
    }

    void AstSearcher::Visit(AstGenericInstantiation& ast) {
        ast.Operand->Accept(*this);
        for (auto& genericArgument : ast.GenericArguments)
            genericArgument->Accept(*this);
    }

    void AstSearcher::Visit(AstParenthesisedExpression& ast) {
        ast.Expression->Accept(*this);
    }

    void AstSearcher::Visit(AstDeclaration& ast) {
        ast.Type->Accept(*this);
    }

    void AstSearcher::Visit(AstConstDeclaration& ast) {
        if (ast.GenericParameters)
            for (auto& genericParameter : *ast.GenericParameters)
                genericParameter->Accept(*this);
        if (ast.Type)
            (*ast.Type)->Accept(*this);
        ast.Value->Accept(*this);
    }

    void AstSearcher::Visit(AstName&) {}

    void AstSearcher::Visit(AstWildcard&) {}

    void AstSearcher::Visit(AstInteger&) {}

    void AstSearcher::Visit(AstFloat&) {}

    void AstSearcher::Visit(AstString&) {}

    void AstSearcher::Visit(AstFunction& ast) {
        for (auto& parameter : ast.Parameters)
            parameter->Accept(*this);
        ast.ReturnType->Accept(*this);
        if (ast.Body)
            (*ast.Body)->Accept(*this);
    }

    void AstSearcher::Visit(AstProcedure& ast) {
        for (auto& parameter : ast.Parameters)
            parameter->Accept(*this);
        ast.ReturnType->Accept(*this);
        if (ast.Body)
            (*ast.Body)->Accept(*this);
    }

    void AstSearcher::Visit(AstReturn& ast) {
        if (ast.Value)
            (*ast.Value)->Accept(*this);
    }

    void AstSearcher::Visit(AstIf& ast) {
        ast.Condition->Accept(*this);
        ast.ThenBlock->Accept(*this);
        if (ast.ElseScope)
            (*ast.ElseScope)->Accept(*this);
    }

    void AstSearcher::Visit(AstBuiltin&) {}

    class AstDumper: public AstVisitor {
        size_t Indent = 0;

        void PrintIndent() {
            for (size_t i = 0; i < Indent; i++) {
                std::cout << "    ";
            }
        }
    public:
        virtual void Visit(AstFile& ast) override {
            PrintIndent();
            std::cout << "- File" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Expressions:" << std::endl;
            Indent++;
            for (auto& expression : ast.Expressions) {
                expression->Accept(*this);
            }
            Indent--;
            Indent--;
        }

        virtual void Visit(AstBlock& ast) override {
            PrintIndent();
            std::cout << "- Block" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Expressions:" << std::endl;
            Indent++;
            for (auto& expression : ast.Expressions) {
                expression->Accept(*this);
            }
            Indent--;
            Indent--;
        }

        virtual void Visit(AstUnary& ast) override {
            PrintIndent();
            std::cout << "- Unary" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "OperatorToken: '" << TokenKind_ToString(ast.OperatorToken.Kind) << '\''
                      << std::endl;
            PrintIndent();
            std::cout << "Operand:" << std::endl;
            Indent++;
            ast.Operand->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstBinary& ast) override {
            PrintIndent();
            std::cout << "- Binary" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "OperatorToken: '" << TokenKind_ToString(ast.OperatorToken.Kind) << '\''
                      << std::endl;
            PrintIndent();
            std::cout << "Left:" << std::endl;
            Indent++;
            ast.Left->Accept(*this);
            Indent--;
            PrintIndent();
            std::cout << "Right:" << std::endl;
            Indent++;
            ast.Right->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstFieldAccess& ast) override {
            PrintIndent();
            std::cout << "- Field Access" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Field Name: " << std::get<std::string_view>(ast.FieldNameToken.Data)
                      << std::endl;
            PrintIndent();
            std::cout << "Operand:" << std::endl;
            Indent++;
            ast.Operand->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstIndex& ast) override {
            PrintIndent();
            std::cout << "- Index" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Operand:" << std::endl;
            Indent++;
            ast.Operand->Accept(*this);
            Indent--;
            PrintIndent();
            std::cout << "Indexer:" << std::endl;
            Indent++;
            ast.Indexer->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstCall& ast) override {
            PrintIndent();
            std::cout << "- Call" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Operand:" << std::endl;
            Indent++;
            ast.Operand->Accept(*this);
            Indent--;
            PrintIndent();
            std::cout << "Arguments:" << std::endl;
            Indent++;
            for (auto& argument : ast.Arguments) {
                argument->Accept(*this);
            }
            Indent--;
            Indent--;
        }

        virtual void Visit(AstGenericInstantiation& ast) override {
            PrintIndent();
            std::cout << "- Generic Instantiation" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Operand:" << std::endl;
            Indent++;
            ast.Operand->Accept(*this);
            Indent--;
            PrintIndent();
            std::cout << "Generic Arguments:" << std::endl;
            Indent++;
            for (auto& argument : ast.GenericArguments) {
                argument->Accept(*this);
            }
            Indent--;
            Indent--;
        }

        virtual void Visit(AstParenthesisedExpression& ast) override {
            PrintIndent();
            std::cout << "- Parenthesised Expression" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Expression:" << std::endl;
            Indent++;
            ast.Expression->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstDeclaration& ast) override {
            PrintIndent();
            std::cout << "- Declaration: " << static_cast<void*>(&ast) << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Name: '" << std::get<std::string_view>(ast.NameToken.Data) << '\''
                      << std::endl;
            PrintIndent();
            std::cout << "Type:" << std::endl;
            Indent++;
            ast.Type->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstConstDeclaration& ast) override {
            PrintIndent();
            std::cout << "- Const Declaration: " << static_cast<void*>(&ast) << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Name: '" << std::get<std::string_view>(ast.NameToken.Data) << '\''
                      << std::endl;
            if (ast.GenericParameters) {
                PrintIndent();
                std::cout << "Generic Parameters:" << std::endl;
                Indent++;
                for (auto& parameter : *ast.GenericParameters) {
                    parameter->Accept(*this);
                }
                Indent--;
            }
            if (ast.Type) {
                PrintIndent();
                std::cout << "Type:" << std::endl;
                Indent++;
                (*ast.Type)->Accept(*this);
                Indent--;
            }
            PrintIndent();
            std::cout << "Value:" << std::endl;
            Indent++;
            ast.Value->Accept(*this);
            Indent--;
            Indent--;
        }

        virtual void Visit(AstName& ast) override {
            PrintIndent();
            std::cout << "- Name: '" << std::get<std::string_view>(ast.NameToken.Data) << '\''
                      << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Resolved Declaration: " << ast.ResolvedDeclaration << std::endl;
            Indent--;
        }

        virtual void Visit(AstWildcard&) override {
            PrintIndent();
            std::cout << "- Wildcard" << std::endl;
        }

        virtual void Visit(AstInteger& ast) override {
            PrintIndent();
            std::cout << "- Integer: " << std::get<size_t>(ast.IntegerToken.Data) << std::endl;
        }

        virtual void Visit(AstFloat& ast) override {
            PrintIndent();
            std::cout << "- Float: " << std::get<double>(ast.FloatToken.Data) << std::endl;
        }

        virtual void Visit(AstString& ast) override {
            PrintIndent();
            auto& string = std::get<std::vector<char>>(ast.StringToken.Data);
            std::cout << "- String: \"" << std::string_view{ string.data(), string.size() } << '"'
                      << std::endl;
        }

        virtual void Visit(AstFunction& ast) override {
            PrintIndent();
            std::cout << "- Function" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Parameters:" << std::endl;
            Indent++;
            for (auto& parameter : ast.Parameters) {
                parameter->Accept(*this);
            }
            Indent--;
            PrintIndent();
            std::cout << "Return Type:" << std::endl;
            Indent++;
            ast.ReturnType->Accept(*this);
            Indent--;
            if (ast.Body) {
                PrintIndent();
                std::cout << "Body:" << std::endl;
                Indent++;
                (*ast.Body)->Accept(*this);
                Indent--;
            }
            Indent--;
        }

        virtual void Visit(AstProcedure& ast) override {
            PrintIndent();
            std::cout << "- Procedure" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Parameters:" << std::endl;
            Indent++;
            for (auto& parameter : ast.Parameters) {
                parameter->Accept(*this);
            }
            Indent--;
            PrintIndent();
            std::cout << "Return Type:" << std::endl;
            Indent++;
            ast.ReturnType->Accept(*this);
            Indent--;
            if (ast.Body) {
                PrintIndent();
                std::cout << "Body:" << std::endl;
                Indent++;
                (*ast.Body)->Accept(*this);
                Indent--;
            }
            Indent--;
        }

        virtual void Visit(AstReturn& ast) override {
            PrintIndent();
            std::cout << "- Return" << std::endl;
            Indent++;
            if (ast.Value) {
                PrintIndent();
                std::cout << "Value:" << std::endl;
                Indent++;
                (*ast.Value)->Accept(*this);
                Indent--;
            }
            Indent--;
        }

        virtual void Visit(AstIf& ast) override {
            PrintIndent();
            std::cout << "- If" << std::endl;
            Indent++;
            PrintIndent();
            std::cout << "Condition:" << std::endl;
            Indent++;
            ast.Condition->Accept(*this);
            Indent--;
            PrintIndent();
            std::cout << "Then Block:" << std::endl;
            Indent++;
            ast.ThenBlock->Accept(*this);
            Indent--;
            if (ast.ElseScope) {
                PrintIndent();
                std::cout << "Else Scope:" << std::endl;
                Indent++;
                (*ast.ElseScope)->Accept(*this);
                Indent--;
            }
            Indent--;
        }

        virtual void Visit(AstBuiltin& ast) override {
            PrintIndent();
            std::cout << "- Builtin" << std::endl;
            Indent++;
            PrintIndent();
            auto& string = std::get<std::vector<char>>(ast.StringToken.Data);
            std::cout << "String: \"" << std::string_view{ string.data(), string.size() } << '"'
                      << std::endl;
            Indent--;
        }
    };

    void DumpAst(Ast& ast) {
        AstDumper dumper;
        ast.Accept(dumper);
    }

}
