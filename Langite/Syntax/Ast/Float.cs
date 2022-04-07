using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class Float : Node
    {
        public Float(Token floatToken)
        {
            Debug.Assert(floatToken.Kind == TokenKind.Float);
            FloatToken = floatToken;
        }

        public Token FloatToken { get; }

        public override SourceLocation Location => FloatToken.Location;

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
