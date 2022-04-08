namespace Langite.Types
{
    public sealed class Type
    {
        public Type(Type value)
        {
            Value = value;
        }

        public Type Value { get; set; }
        
        public long Size => Value.Size;
    }
}
