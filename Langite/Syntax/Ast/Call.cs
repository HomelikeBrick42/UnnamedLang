using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Call : Node
    {
        public Call(Node operand, Token openParenthesisToken, IList<Node> arguments, Token closeParenthesisToken)
        {
            Operand = operand;
            OpenParenthesisToken = openParenthesisToken;
            Arguments = arguments;
            CloseParenthesisToken = closeParenthesisToken;
        }

        public Node Operand { get; }
        public Token OpenParenthesisToken { get; }
        public IList<Node> Arguments { get; }
        public Token CloseParenthesisToken { get; }

        public override SourceLocation Location => OpenParenthesisToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Operand;
            foreach (var argument in Arguments)
                yield return argument;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
