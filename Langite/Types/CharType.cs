namespace Langite.Types
{
    public readonly struct CharType : IType
    {
        public TypeKind Kind => TypeKind.Char;
        public bool Resolved => true;
        public ulong Size => 4;
    }
}
