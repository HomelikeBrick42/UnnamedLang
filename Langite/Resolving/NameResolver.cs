using System;
using System.Collections.Generic;
using System.Linq;
using Langite.Syntax.Ast;

namespace Langite.Resolving
{
    public sealed class NameResolver : Searcher<ValueTuple>
    {
        private readonly List<Dictionary<string, DeclarationBase>> _constDeclarations;
        private List<Dictionary<string, DeclarationBase>> _declarations;

        private NameResolver()
        {
            _declarations = new List<Dictionary<string, DeclarationBase>>();
            _constDeclarations = new List<Dictionary<string, DeclarationBase>>();
        }

        private void PushScope()
        {
            _declarations.Add(new Dictionary<string, DeclarationBase>());
            _constDeclarations.Add(new Dictionary<string, DeclarationBase>());
        }

        private void PopScope()
        {
            _declarations.RemoveAt(_declarations.Count - 1);
            _constDeclarations.RemoveAt(_constDeclarations.Count - 1);
        }

        public static void Resolve(Node node)
        {
            var nameResolver = new NameResolver();
            node.Accept(nameResolver, default);
        }

        public override ValueTuple Visit(File file, ValueTuple indent)
        {
            PushScope();
            foreach (var expression in file.Expressions)
            foreach (var constDeclaration in ConstDeclarationScopeSearcher.Search(expression))
            {
                if (_constDeclarations[^1].ContainsKey(constDeclaration.Name))
                    throw new NotImplementedException();
                _constDeclarations[^1].Add(constDeclaration.Name, constDeclaration);
            }

            foreach (var expression in file.Expressions)
                expression.Accept(this, indent);
            PopScope();
            return default;
        }

        public override ValueTuple Visit(Name name, ValueTuple indent)
        {
            var varIndex = _declarations.Count - 1;
            var constIndex = _constDeclarations.Count - 1;
            while (varIndex >= 0 || constIndex >= 0)
            {
                if (varIndex >= 0)
                {
                    if (_declarations[varIndex].ContainsKey(name.NameString))
                    {
                        name.ResolvedDeclaration = _declarations[varIndex][name.NameString];
                        break;
                    }

                    varIndex--;
                }

                if (constIndex >= 0)
                {
                    if (_constDeclarations[constIndex].ContainsKey(name.NameString))
                    {
                        name.ResolvedDeclaration = _constDeclarations[constIndex][name.NameString];
                        break;
                    }

                    constIndex--;
                }
            }

            if (name.ResolvedDeclaration is null)
                throw new CompileError(name.Location, $"Unable to find name '{name.NameString}'");

            return default;
        }

        public override ValueTuple Visit(Declaration declaration, ValueTuple indent)
        {
            if (_declarations[^1].ContainsKey(declaration.Name) ||
                _constDeclarations[^1].ContainsKey(declaration.Name))
                throw new NotImplementedException();
            _declarations[^1].Add(declaration.Name, declaration);
            PushScope();
            declaration.Type.Accept(this, indent);
            PopScope();
            return default;
        }

        public override ValueTuple Visit(GenericParameter genericParameter, ValueTuple indent)
        {
            if (_declarations[^1].ContainsKey(genericParameter.Name) ||
                _constDeclarations[^1].ContainsKey(genericParameter.Name))
                throw new NotImplementedException();
            _constDeclarations[^1].Add(genericParameter.Name, genericParameter);
            PushScope();
            genericParameter.Type.Accept(this, indent);
            PopScope();
            return default;
        }

        public override ValueTuple Visit(ConstDeclaration constDeclaration, ValueTuple indent)
        {
            PushScope();
            foreach (var child in constDeclaration.GetChildren())
                child.Accept(this, indent);
            PopScope();
            return default;
        }

        public override ValueTuple Visit(Function function, ValueTuple indent)
        {
            PushScope();
            var oldScope = _declarations.Select(scope => scope.ToDictionary(entry => entry.Key, entry => entry.Value))
                .ToList();
            _declarations = new List<Dictionary<string, DeclarationBase>> {new()};
            foreach (var parameter in function.Parameters)
                parameter.Accept(this, indent);
            function.ReturnType.Accept(this, indent);
            function.Body?.Accept(this, indent);
            _declarations = oldScope;
            PopScope();
            return default;
        }

        public override ValueTuple Visit(Procedure procedure, ValueTuple indent)
        {
            PushScope();
            var oldScope = _declarations.Select(scope => scope.ToDictionary(entry => entry.Key, entry => entry.Value))
                .ToList();
            _declarations.RemoveRange(1, _declarations.Count - 1);
            foreach (var parameter in procedure.Parameters)
                parameter.Accept(this, indent);
            procedure.ReturnType.Accept(this, indent);
            procedure.Body?.Accept(this, indent);
            _declarations = oldScope;
            PopScope();
            return default;
        }

        public override ValueTuple Visit(Block block, ValueTuple indent)
        {
            PushScope();
            foreach (var expression in block.Expressions)
            foreach (var constDeclaration in ConstDeclarationScopeSearcher.Search(expression))
                _constDeclarations[^1].Add(constDeclaration.Name, constDeclaration);
            foreach (var expression in block.Expressions)
                expression.Accept(this, indent);
            PopScope();
            return default;
        }
    }
}
