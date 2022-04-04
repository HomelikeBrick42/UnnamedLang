#pragma once

#include "Ast.hpp"

#include <vector>
#include <unordered_map>

namespace Langite {

    class NameResolver: public AstSearcher {
        std::vector<std::unordered_map<std::string_view, Ast*>> Constants;
        std::vector<std::unordered_map<std::string_view, Ast*>> Variables;

        virtual void Visit(AstFile& ast) override;
        virtual void Visit(AstBlock& ast) override;
        virtual void Visit(AstDeclaration& ast) override;
        virtual void Visit(AstName& ast) override;
        virtual void Visit(AstFunction& ast) override;
        virtual void Visit(AstProcedure& ast) override;
    };

}
