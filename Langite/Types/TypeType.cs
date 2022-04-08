namespace Langite.Types
{
    public readonly struct TypeType : IType
    {
        public TypeKind Kind => TypeKind.Type;
        public bool Resolved => true;
        public ulong Size => 8;
    }
}
