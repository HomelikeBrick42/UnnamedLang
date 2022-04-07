using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class Integer : Node
    {
        public Integer(Token integerToken)
        {
            Debug.Assert(integerToken.Kind == TokenKind.Integer);
            IntegerToken = integerToken;
        }

        public Token IntegerToken { get; }

        public override SourceLocation Location => IntegerToken.Location;

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
