#pragma once

#include "Ast.hpp"

#include <vector>
#include <unordered_map>

namespace Langite {

    class NameResolver: public AstVisitor {
        bool ResolvingConstants = false;

        std::vector<std::unordered_map<std::string_view, Ast*>> Constants;
        std::vector<std::unordered_map<std::string_view, Ast*>> Variables;

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
        virtual void Visit(AstName& ast) override;
        virtual void Visit(AstWildcard& ast) override;
        virtual void Visit(AstInteger& ast) override;
        virtual void Visit(AstFloat& ast) override;
        virtual void Visit(AstString& ast) override;
        virtual void Visit(AstFunction& ast) override;
        virtual void Visit(AstProcedure& ast) override;
        virtual void Visit(AstReturn& ast) override;
        virtual void Visit(AstIf& ast) override;
        virtual void Visit(AstBuiltin& ast) override;
    };

}
