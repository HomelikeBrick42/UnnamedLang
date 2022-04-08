namespace Langite.Types
{
    public readonly struct BoolType : IType
    {
        public TypeKind Kind => TypeKind.Bool;
        public bool Resolved => true;
        public ulong Size => 1;
    }
}
