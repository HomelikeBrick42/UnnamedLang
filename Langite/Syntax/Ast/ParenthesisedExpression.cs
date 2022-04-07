using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class ParenthesisedExpression : Node
    {
        public ParenthesisedExpression(Token openParenthesisToken, Node expression, Token closeParenthesisToken)
        {
            OpenParenthesisToken = openParenthesisToken;
            Expression = expression;
            CloseParenthesisToken = closeParenthesisToken;
        }

        public Token OpenParenthesisToken { get; }
        public Node Expression { get; }
        public Token CloseParenthesisToken { get; }

        public override SourceLocation Location => OpenParenthesisToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Expression;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
