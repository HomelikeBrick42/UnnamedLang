using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Return : Node
    {
        public Return(Token returnToken, Node? value)
        {
            ReturnToken = returnToken;
            Value = value;
        }

        public Token ReturnToken { get; }
        public Node? Value { get; }

        public override SourceLocation Location => ReturnToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            if (Value is not null)
                yield return Value;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
