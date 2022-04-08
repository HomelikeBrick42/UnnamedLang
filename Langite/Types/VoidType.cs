namespace Langite.Types
{
    public readonly struct VoidType : IType
    {
        public TypeKind Kind => TypeKind.Void;
        public bool Resolved => true;
        public ulong Size => 0;
    }
}
