using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Unary : Node
    {
        public Unary(Token operatorToken, Node operand)
        {
            OperatorToken = operatorToken;
            Operand = operand;
        }

        public Token OperatorToken { get; }
        public Node Operand { get; }

        public override SourceLocation Location => OperatorToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Operand;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
