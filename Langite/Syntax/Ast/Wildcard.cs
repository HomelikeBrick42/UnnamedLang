using System.Collections.Generic;
using System.Linq;

namespace Langite.Syntax.Ast
{
    public sealed class Wildcard: Node
    {
        public Wildcard(Token wildcardToken)
        {
            WildcardToken = wildcardToken;
        }

        public Token WildcardToken { get; }

        public override SourceLocation Location => WildcardToken.Location;
        
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
