using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Procedure : Node
    {
        public Procedure(Token procToken, Token openParenthesisToken, IList<Declaration> parameters,
            Token closeParenthesisToken, Token colonToken, Node returnType, Block? body)
        {
            ProcToken = procToken;
            OpenParenthesisToken = openParenthesisToken;
            Parameters = parameters;
            CloseParenthesisToken = closeParenthesisToken;
            ColonToken = colonToken;
            ReturnType = returnType;
            Body = body;
        }

        public Token ProcToken { get; }
        public Token OpenParenthesisToken { get; }
        public IList<Declaration> Parameters { get; }
        public Token CloseParenthesisToken { get; }
        public Token ColonToken { get; }
        public Node ReturnType { get; }
        public Block? Body { get; }

        public override SourceLocation Location => ProcToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            foreach (var parameter in Parameters)
                yield return parameter;
            yield return ReturnType;
            if (Body is not null)
                yield return Body;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
