namespace Langite.Types
{
    public readonly struct PlaceholderType : IType
    {
        public TypeKind Kind => TypeKind.Placeholder;
        public bool Resolved => false;
        public ulong Size => 0;
    }
}
