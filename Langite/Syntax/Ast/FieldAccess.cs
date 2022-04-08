using System;
using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class FieldAccess : Node
    {
        public FieldAccess(Node operand, Token periodToken, Token nameToken)
        {
            Operand = operand;
            PeriodToken = periodToken;
            NameToken = nameToken;
        }

        public Node Operand { get; }
        public Token PeriodToken { get; }
        public Token NameToken { get; }

        public string Name => NameToken.Value as string ?? throw new InvalidOperationException();

        public override SourceLocation Location => PeriodToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Operand;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
