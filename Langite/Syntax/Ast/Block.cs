using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Block : Node
    {
        public Block(Token openBraceToken, IList<Node> expressions, Token closeBraceToken)
        {
            OpenBraceToken = openBraceToken;
            Expressions = expressions;
            CloseBraceToken = closeBraceToken;
        }

        public Token OpenBraceToken { get; }
        public IList<Node> Expressions { get; }
        public Token CloseBraceToken { get; }

        public override SourceLocation Location => OpenBraceToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            foreach (var expression in Expressions)
                yield return expression;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
