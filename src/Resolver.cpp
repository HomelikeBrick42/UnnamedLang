#include "Resolver.hpp"
#include "CompileError.hpp"

#include <cassert>
#include <sstream>

namespace Langite {

    class ConstDeclarationInCurrentScopeSearcher: public AstSearcher {
    public:
        std::vector<AstConstDeclaration*> ConstDeclarations;
    private:
        virtual void Visit(AstFile&) override {}
        virtual void Visit(AstBlock&) override {}

        virtual void Visit(AstConstDeclaration& ast) override {
            ConstDeclarations.push_back(&ast);
            AstSearcher::Visit(ast);
        }

        virtual void Visit(AstFunction&) override {}
        virtual void Visit(AstProcedure&) override {}
    };

    void NameResolver::Visit(AstFile& ast) {
        Constants.push_back({});
        Variables.push_back({});
        ConstDeclarationInCurrentScopeSearcher searcher;
        for (auto& expression : ast.Expressions)
            expression->Accept(searcher);
        for (auto& constDeclaration : searcher.ConstDeclarations)
            Constants[Constants.size() - 1]
                     [std::get<std::string_view>(constDeclaration->NameToken.Data)] =
                         constDeclaration;
        AstSearcher::Visit(ast);
        Constants.pop_back();
        Variables.pop_back();
    }

    void NameResolver::Visit(AstBlock& ast) {
        Constants.push_back({});
        Variables.push_back({});
        ConstDeclarationInCurrentScopeSearcher searcher;
        for (auto& expression : ast.Expressions)
            expression->Accept(searcher);
        for (auto& constDeclaration : searcher.ConstDeclarations)
            Constants[Constants.size() - 1]
                     [std::get<std::string_view>(constDeclaration->NameToken.Data)] =
                         constDeclaration;
        AstSearcher::Visit(ast);
        Constants.pop_back();
        Variables.pop_back();
    }

    void NameResolver::Visit(AstDeclaration& ast) {
        const auto& name = std::get<std::string_view>(ast.NameToken.Data);
        if (Constants[Constants.size() - 1].contains(name) ||
            Variables[Variables.size() - 1].contains(name))
            throw CompileError{
                .Location = ast.NameToken.Location,
                .Message = (std::stringstream{} << "Redeclaration of name '" << name << '\'').str(),
            };
        if (ast.IsGenericParameter) {
            Constants[Constants.size() - 1][name] = &ast;
        } else {
            Variables[Variables.size() - 1][name] = &ast;
        }
        AstSearcher::Visit(ast);
    }

    void NameResolver::Visit(AstName& ast) {
        if (!ast.ResolvedDeclaration) {
            const auto& name = std::get<std::string_view>(ast.NameToken.Data);
            auto constIt     = Constants.rbegin();
            auto varIt       = Variables.rbegin();
            while (constIt != Constants.rend() || varIt != Variables.rend()) {
                if (constIt != Constants.rend()) {
                    if (constIt->contains(name)) {
                        ast.ResolvedDeclaration = constIt->at(name);
                        break;
                    }
                    constIt++;
                }
                if (varIt != Variables.rend()) {
                    if (varIt->contains(name)) {
                        ast.ResolvedDeclaration = varIt->at(name);
                        break;
                    }
                    varIt++;
                }
            }
            if (!ast.ResolvedDeclaration)
                throw CompileError{
                    .Location = ast.NameToken.Location,
                    .Message =
                        (std::stringstream{} << "Unable to find name '" << name << '\'').str(),
                };
        }
        AstSearcher::Visit(ast);
    }

    void NameResolver::Visit(AstFunction& ast) {
        Constants.push_back({});
        auto oldVariables = std::move(Variables);
        Variables.clear();
        Variables.push_back({});
        AstSearcher::Visit(ast);
        Variables = std::move(oldVariables);
        Constants.pop_back();
    }

    void NameResolver::Visit(AstProcedure& ast) {
        Constants.push_back({});
        auto oldVariables = Variables;
        Variables.erase(Variables.begin() + 1, Variables.end());
        Variables.push_back({});
        AstSearcher::Visit(ast);
        Variables = std::move(oldVariables);
        Constants.pop_back();
    }

}
