using System.Collections.Generic;

namespace Langite.Syntax.Ast
{
    public sealed class If : Node
    {
        public If(Token ifToken, Node condition, Block thenBlock, Token? elseToken, Node? elseNode)
        {
            IfToken = ifToken;
            Condition = condition;
            ThenBlock = thenBlock;
            ElseToken = elseToken;
            ElseNode = elseNode;
        }

        public Token IfToken { get; }
        public Node Condition { get; }
        public Block ThenBlock { get; }
        public Token? ElseToken { get; }
        public Node? ElseNode { get; }

        public override SourceLocation Location => IfToken.Location;

        public override IEnumerable<Node> GetChildren()
        {
            yield return Condition;
            yield return ThenBlock;
            if (ElseNode is not null)
                yield return ElseNode;
        }

        public override T Accept<T, U>(Visitor<T, U> visitor, U arg)
        {
            return visitor.Visit(this, arg);
        }
    }
}
