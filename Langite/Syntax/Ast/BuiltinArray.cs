using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class BuiltinArray : Node
    {
        public BuiltinArray(Token builtinArrayToken, Token openSquareBracketToken, Node innerType, Node length,
            Token closeSquareBracketToken)
        {
            BuiltinArrayToken = builtinArrayToken;
            OpenSquareBracketToken = openSquareBracketToken;
            InnerType = innerType;
            Length = length;
            CloseSquareBracketToken = closeSquareBracketToken;
        }

        public Token BuiltinArrayToken { get; }
        public Token OpenSquareBracketToken { get; }
        public Node InnerType { get; }
        public Node Length { get; }
        public Token CloseSquareBracketToken { get; }

        public override SourceLocation Location => BuiltinArrayToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return InnerType;
            yield return Length;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
