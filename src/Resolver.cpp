#include "Resolver.hpp"
#include "CompileError.hpp"

#include <cassert>
#include <sstream>

namespace Langite {

    void NameResolver::Visit(AstFile& ast) {
        assert(Constants.size() == 0);
        assert(Variables.size() == 0);
        Constants.push_back({});
        Variables.push_back({});
        bool oldResolving  = ResolvingConstants;
        ResolvingConstants = true;
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
        Variables.pop_back();
        Variables.push_back({});
        ResolvingConstants = false;
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
        ResolvingConstants = oldResolving;
        Constants.pop_back();
        Variables.pop_back();
        assert(Constants.size() == 0);
        assert(Variables.size() == 0);
    }

    void NameResolver::Visit(AstBlock& ast) {
        Constants.push_back({});
        Variables.push_back({});
        bool oldResolving  = ResolvingConstants;
        ResolvingConstants = true;
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
        Variables.pop_back();
        Variables.push_back({});
        ResolvingConstants = false;
        for (auto& expression : ast.Expressions)
            expression->Accept(*this);
        ResolvingConstants = oldResolving;
        Constants.pop_back();
        Variables.pop_back();
    }

    void NameResolver::Visit(AstUnary& ast) {
        ast.Operand->Accept(*this);
    }

    void NameResolver::Visit(AstBinary& ast) {
        ast.Left->Accept(*this);
        ast.Right->Accept(*this);
    }

    void NameResolver::Visit(AstFieldAccess& ast) {
        ast.Operand->Accept(*this);
    }

    void NameResolver::Visit(AstIndex& ast) {
        ast.Operand->Accept(*this);
        ast.Indexer->Accept(*this);
    }

    void NameResolver::Visit(AstCall& ast) {
        ast.Operand->Accept(*this);
        for (auto& argument : ast.Arguments)
            argument->Accept(*this);
    }

    void NameResolver::Visit(AstGenericInstantiation& ast) {
        ast.Operand->Accept(*this);
        for (auto& genericArgument : ast.GenericArguments)
            genericArgument->Accept(*this);
    }

    void NameResolver::Visit(AstParenthesisedExpression& ast) {
        ast.Expression->Accept(*this);
    }

    void NameResolver::Visit(AstDeclaration& ast) {
        ast.Type->Accept(*this);
        auto& name = std::get<std::string_view>(ast.NameToken.Data);
        if (Constants[Constants.size() - 1].contains(name))
            throw CompileError{
                .Location = ast.NameToken.Location,
                .Message  = (std::stringstream{} << "Redefinition of '" << name << '\'').str(),
            };
        if (Variables[Variables.size() - 1].contains(name))
            throw CompileError{
                .Location = ast.NameToken.Location,
                .Message  = (std::stringstream{} << "Redefinition of '" << name << '\'').str(),
            };
        Variables[Variables.size() - 1][name] = &ast;
    }

    void NameResolver::Visit(AstConstDeclaration& ast) {
        Constants.push_back({});
        Variables.push_back({});
        if (ast.GenericParameters)
            for (auto& genericParameter : *ast.GenericParameters)
                Constants[Constants.size() - 1]
                         [std::get<std::string_view>(genericParameter->NameToken.Data)] =
                             &*genericParameter;
        if (ast.Type)
            (*ast.Type)->Accept(*this);
        ast.Value->Accept(*this);
        Constants.pop_back();
        Variables.pop_back();
        if (ResolvingConstants) {
            auto& name = std::get<std::string_view>(ast.NameToken.Data);
            if (Constants[Constants.size() - 1].contains(name))
                throw CompileError{
                    .Location = ast.NameToken.Location,
                    .Message  = (std::stringstream{} << "Redefinition of '" << name << '\'').str(),
                };
            if (Variables[Variables.size() - 1].contains(name))
                throw CompileError{
                    .Location = ast.NameToken.Location,
                    .Message  = (std::stringstream{} << "Redefinition of '" << name << '\'').str(),
                };
            Constants[Constants.size() - 1][name] = &ast;
        }
    }

    void NameResolver::Visit(AstName& ast) {
        if (!ast.ResolvedDeclaration) {
            auto& name = std::get<std::string_view>(ast.NameToken.Data);
            if (ResolvingConstants) {
                for (auto it = Constants.rbegin(); it != Constants.rend(); it++) {
                    if (it->contains(name)) {
                        ast.ResolvedDeclaration = it->at(name);
                        break;
                    }
                }
            } else {
                auto constIt = Constants.rbegin();
                auto varIt   = Variables.rbegin();
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
        }
    }

    void NameResolver::Visit(AstWildcard&) {}

    void NameResolver::Visit(AstInteger&) {}

    void NameResolver::Visit(AstFloat&) {}

    void NameResolver::Visit(AstString&) {}

    void NameResolver::Visit(AstFunction& ast) {
        ast.ReturnType->Accept(*this);
        Constants.push_back({});
        auto oldVariables = std::move(Variables);
        Variables         = {};
        Variables.push_back({});
        bool oldResolving  = ResolvingConstants;
        ResolvingConstants = false;
        for (auto& parameter : ast.Parameters)
            parameter->Accept(*this);
        ResolvingConstants = oldResolving;
        if (ast.Body)
            (*ast.Body)->Accept(*this);
        Constants.pop_back();
        Variables = std::move(oldVariables);
    }

    void NameResolver::Visit(AstProcedure& ast) {
        ast.ReturnType->Accept(*this);
        Constants.push_back({});
        auto oldVariables = Variables;
        Variables.erase(Variables.begin() + 1, Variables.end());
        Variables.push_back({});
        bool oldResolving  = ResolvingConstants;
        ResolvingConstants = false;
        for (auto& parameter : ast.Parameters)
            parameter->Accept(*this);
        ResolvingConstants = oldResolving;
        if (ast.Body)
            (*ast.Body)->Accept(*this);
        Constants.pop_back();
        Variables = std::move(oldVariables);
    }

    void NameResolver::Visit(AstReturn& ast) {
        if (ast.Value)
            (*ast.Value)->Accept(*this);
    }

    void NameResolver::Visit(AstIf& ast) {
        ast.Condition->Accept(*this);
        ast.ThenBlock->Accept(*this);
        if (ast.ElseScope)
            (*ast.ElseScope)->Accept(*this);
    }

    void NameResolver::Visit(AstBuiltin&) {}

}
