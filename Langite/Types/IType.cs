namespace Langite.Types
{
    public interface IType
    {
        public TypeKind Kind { get; }
        public bool Resolved { get; }
        public ulong Size { get; }
    }
}
