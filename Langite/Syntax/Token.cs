namespace Langite.Syntax
{
    public readonly struct Token
    {
        public Token(TokenKind kind, SourceLocation location, int length, object? value = null)
        {
            Kind = kind;
            Location = location;
            Length = length;
            Value = value;
        }

        public TokenKind Kind { get; }
        public SourceLocation Location { get; }
        public int Length { get; }
        public object? Value { get; }

        public override string ToString()
        {
            return Kind.AsString();
        }
    }
}
