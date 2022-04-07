using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class String : Node
    {
        public String(Token stringToken)
        {
            Debug.Assert(stringToken.Kind == TokenKind.String);
            StringToken = stringToken;
        }

        public Token StringToken { get; }

        public override SourceLocation Location => StringToken.Location;

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
