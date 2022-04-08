namespace Langite.Types
{
    public readonly struct StringType : IType
    {
        public TypeKind Kind => TypeKind.String;
        public bool Resolved => true;
        public ulong Size => 16;
    }
}
