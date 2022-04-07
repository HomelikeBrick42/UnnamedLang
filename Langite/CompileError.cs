using System;
using Langite.Syntax;

namespace Langite
{
    public sealed class CompileError : Exception
    {
        public CompileError(SourceLocation location, string message)
            : base($"{location}: {message}")
        {
            Location = location;
        }

        public SourceLocation Location { get; }
    }
}
