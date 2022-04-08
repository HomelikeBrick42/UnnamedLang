using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public abstract class Node
    {
        public abstract SourceLocation Location { get; }
        public abstract IEnumerable<Node> GetChildren();
        public abstract T Accept<T, U>(Visitor<T, U> visitor, U arg);
    }
}
