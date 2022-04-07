using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Binary : Node
    {
        public Binary(Node left, Token operatorToken, Node right)
        {
            Left = left;
            OperatorToken = operatorToken;
            Right = right;
        }

        public Node Left { get; }
        public Token OperatorToken { get; }
        public Node Right { get; }

        public override SourceLocation Location => OperatorToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Left;
            yield return Right;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
