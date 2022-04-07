namespace Langite.Syntax
{
    public readonly struct SourceLocation
    {
        public SourceLocation(string filepath, int position, int line, int column)
        {
            Filepath = filepath;
            Position = position;
            Line = line;
            Column = column;
        }

        public string Filepath { get; }
        public int Position { get; }
        public int Line { get; }
        public int Column { get; }

        public override string ToString()
        {
            return $"{Filepath}:{Line}:{Column}";
        }
    }
}
