using System.Collections.Generic;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class Builtin : Node
    {
        public Builtin(Token builtinToken, Token stringToken)
        {
            BuiltinToken = builtinToken;
            StringToken = stringToken;
        }

        public Token BuiltinToken { get; }
        public Token StringToken { get; }

        public override SourceLocation Location => BuiltinToken.Location;

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
