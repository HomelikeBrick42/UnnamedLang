namespace Langite.Types
{
    public sealed class Type
    {
        public Type(IType value)
        {
            Value = value;
        }

        public IType Value;

        public TypeKind Kind => Value.Kind;
        public bool Resolved => Value.Resolved;
        public ulong Size => Value.Size;

        private bool Equals(Type other)
        {
            return Value.Equals(other.Value);
        }

        public override bool Equals(object? obj)
        {
            return ReferenceEquals(this, obj) || obj is Type other && Equals(other);
        }

        public override int GetHashCode()
        {
            return 0;
        }

        public override string ToString()
        {
            return Value.ToString()!;
        }

        public static bool operator ==(Type a, Type b)
        {
            return Equals(a, b);
        }

        public static bool operator !=(Type a, Type b)
        {
            return !Equals(a, b);
        }
    }
}
