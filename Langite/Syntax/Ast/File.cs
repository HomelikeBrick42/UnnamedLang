using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class File : Node
    {
        public File(IList<Node> expressions, Token endOfFileToken)
        {
            Expressions = expressions;
            EndOfFileToken = endOfFileToken;
        }

        public IList<Node> Expressions { get; }
        public Token EndOfFileToken { get; }

        public override SourceLocation Location => new(EndOfFileToken.Location.Filepath, 0, 1, 1);

        public override IEnumerable<Node> GetChildren()
        {
            return Expressions;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
