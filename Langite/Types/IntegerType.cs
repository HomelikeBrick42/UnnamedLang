namespace Langite.Types
{
    public readonly struct IntegerType : IType
    {
        public TypeKind Kind => TypeKind.Integer;
        public bool Resolved => true;
        public ulong Size => 8;
    }
}
