using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class Function : Node
    {
        public Function(Token funcToken, Token openParenthesisToken, IList<Declaration> parameters,
            Token closeParenthesisToken, Token colonToken, Node returnType, Block? body)
        {
            FuncToken = funcToken;
            OpenParenthesisToken = openParenthesisToken;
            Parameters = parameters;
            CloseParenthesisToken = closeParenthesisToken;
            ColonToken = colonToken;
            ReturnType = returnType;
            Body = body;
        }

        public Token FuncToken { get; }
        public Token OpenParenthesisToken { get; }
        public IList<Declaration> Parameters { get; }
        public Token CloseParenthesisToken { get; }
        public Token ColonToken { get; }
        public Node ReturnType { get; }
        public Block? Body { get; }

        public override SourceLocation Location => FuncToken.Location;

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
