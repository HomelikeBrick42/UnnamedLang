using System;
using System.Diagnostics;

namespace Langite.Syntax.Ast
{
    public abstract class DeclarationBase : Node
    {
        protected DeclarationBase(Token nameToken)
        {
            Debug.Assert(nameToken.Kind is TokenKind.Name or TokenKind.Wildcard);
            NameToken = nameToken;
        }

        public Token NameToken { get; }
        public string Name => NameToken.Value as string ?? throw new InvalidOperationException();
        public override SourceLocation Location => NameToken.Location;
    }
}
