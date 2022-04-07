using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Declaration : DeclarationBase
    {
        public Declaration(Token nameToken, Token colonToken, Node type) : base(nameToken)
        {
            ColonToken = colonToken;
            Type = type;
        }

        public Token ColonToken { get; }
        public Node Type { get; }

        public override IEnumerable<Node> GetChildren()
        {
            yield return Type;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
