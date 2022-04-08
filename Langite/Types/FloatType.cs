namespace Langite.Types
{
    public readonly struct FloatType : IType
    {
        public TypeKind Kind => TypeKind.Float;
        public bool Resolved => true;
        public ulong Size => 8;
    }
}
