using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class ConstDeclaration : DeclarationBase
    {
        public ConstDeclaration(Token constToken, Token nameToken, Token? openSquareBracketToken,
            IList<GenericParameter>? genericParameters, Token? closeSquareBracketToken, Token? colonToken,
            Node? type, Token equalsToken, Node value) : base(nameToken)
        {
            ConstToken = constToken;
            OpenSquareBracketToken = openSquareBracketToken;
            GenericParameters = genericParameters;
            CloseSquareBracketToken = closeSquareBracketToken;
            ColonToken = colonToken;
            Type = type;
            EqualsToken = equalsToken;
            Value = value;
        }

        public Token ConstToken { get; }
        public Token? OpenSquareBracketToken { get; }
        public IList<GenericParameter>? GenericParameters { get; }
        public Token? CloseSquareBracketToken { get; }
        public Token? ColonToken { get; }
        public Node? Type { get; }
        public Token EqualsToken { get; }
        public Node Value { get; }

        public override IEnumerable<Node> GetChildren()
        {
            if (GenericParameters is not null)
                foreach (var genericParameter in GenericParameters)
                    yield return genericParameter;
            if (Type is not null)
                yield return Type;
            yield return Value;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
