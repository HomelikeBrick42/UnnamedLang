namespace Langite.Types
{
    public readonly struct ArrayType : IType
    {
        public ArrayType(Type innerType, ulong length)
        {
            InnerType = innerType;
            Length = length;
        }

        public Type InnerType { get; }
        public ulong Length { get; }
        
        public TypeKind Kind => TypeKind.Array;
        public bool Resolved => InnerType.Resolved;
        public ulong Size => InnerType.Size * Length;
    }
}
