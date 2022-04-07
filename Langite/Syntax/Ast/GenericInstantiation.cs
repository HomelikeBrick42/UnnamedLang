using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class GenericInstantiation : Node
    {
        public GenericInstantiation(Node operand, Token openSquareBracketToken, IList<Node> genericArguments, Token closeSquareBracketToken)
        {
            Operand = operand;
            OpenSquareBracketToken = openSquareBracketToken;
            GenericArguments = genericArguments;
            CloseSquareBracketToken = closeSquareBracketToken;
        }

        public Node Operand { get; }
        public Token OpenSquareBracketToken { get; }
        public IList<Node> GenericArguments { get; }
        public Token CloseSquareBracketToken { get; }

        public override SourceLocation Location => OpenSquareBracketToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Operand;
            foreach (var argument in GenericArguments)
                yield return argument;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
