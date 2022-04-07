using System;
using System.Collections.Generic;
using Langite.Syntax.Ast;

namespace Langite.Resolving
{
    internal sealed class ConstDeclarationScopeSearcher : Searcher<ValueTuple>
    {
        private readonly List<ConstDeclaration> _constDeclarations;

        private ConstDeclarationScopeSearcher()
        {
            _constDeclarations = new List<ConstDeclaration>();
        }
        
        public static IList<ConstDeclaration> Search(Node node)
        {
            var searcher = new ConstDeclarationScopeSearcher();
            node.Accept(searcher, default);
            return searcher._constDeclarations;
        }

        public override ValueTuple Visit(Syntax.Ast.File file, ValueTuple indent)
        {
            return default;
        }

        public override ValueTuple Visit(Declaration declaration, ValueTuple indent)
        {
            return default;
        }

        public override ValueTuple Visit(ConstDeclaration constDeclaration, ValueTuple indent)
        {
            _constDeclarations.Add(constDeclaration);
            return default;
        }

        public override ValueTuple Visit(Function function, ValueTuple indent)
        {
            return default;
        }

        public override ValueTuple Visit(Procedure procedure, ValueTuple indent)
        {
            return default;
        }

        public override ValueTuple Visit(Block block, ValueTuple indent)
        {
            return default;
        }
    }
}
