using System;
using System.Collections.Generic;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class Name : Node
    {
        public DeclarationBase? ResolvedDeclaration = null;

        public Name(Token nameToken)
        {
            NameToken = nameToken;
        }

        public Token NameToken { get; }

        public string NameString => NameToken.Value as string ?? throw new InvalidOperationException();

        public override SourceLocation Location => NameToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            return Enumerable.Empty<Node>();
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
