using System;
using System.Diagnostics;

namespace Langite.Syntax
{
    public sealed class Lexer
    {
        private int _column = 1;
        private int _line = 1;

        private int _position = 0;

        public Lexer(string filepath, string source)
        {
            Filepath = filepath;
            Source = source;
        }

        public string Filepath { get; }
        public string Source { get; }

        private char CurrentChar => _position < Source.Length
            ? Source[_position]
            : '\0';

        public Token PeekToken()
        {
            var position = _position;
            var line = _line;
            var column = _column;
            var token = NextToken();
            _position = position;
            _line = line;
            _column = column;
            return token;
        }

        public TokenKind PeekKind()
        {
            return PeekToken().Kind;
        }

        public Token NextToken()
        {
            Start:
            while (char.IsWhiteSpace(CurrentChar) && CurrentChar != '\n')
                NextChar();

            var startLocation = new SourceLocation(Filepath, _position, _line, _column);

            if (CurrentChar == '"')
            {
                NextChar();

                var value = string.Empty;
                while (CurrentChar != '"' && CurrentChar != '\0')
                    if (CurrentChar == '\\')
                    {
                        NextChar();
                        value += CurrentChar switch
                        {
                            '0' => '\0',
                            't' => '\t',
                            'n' => '\n',
                            'r' => '\r',
                            _ => throw new CompileError(new SourceLocation(Filepath, _position, _line, _column),
                                $"Unknown escape character {NextChar()}"),
                        };
                    }
                    else
                    {
                        value += NextChar();
                    }

                var closeQuote = NextChar();
                if (closeQuote != '"')
                {
                    Debug.Assert(closeQuote == '\0');
                    throw new CompileError(startLocation, "String literal unclosed at end of file");
                }

                return new Token(TokenKind.String, startLocation, _position - startLocation.Position, value);
            }

            if (char.IsAscii(CurrentChar) && char.IsDigit(CurrentChar))
            {
                var @base = 10;
                if (CurrentChar == '0')
                {
                    NextChar();
                    switch (CurrentChar)
                    {
                        case 'b':
                            NextChar();
                            @base = 2;
                            break;

                        case 'o':
                            NextChar();
                            @base = 8;
                            break;

                        case 'd':
                            NextChar();
                            @base = 10;
                            break;

                        case 'x':
                            NextChar();
                            @base = 16;
                            break;
                    }
                }

                var intValue = 0L;

                while (char.IsAscii(CurrentChar) && char.IsLetterOrDigit(CurrentChar) || CurrentChar == '_')
                {
                    if (CurrentChar == '_')
                    {
                        NextChar();
                        continue;
                    }

                    var chr = CurrentChar;
                    var value = chr switch
                    {
                        >= '0' and <= '9' => (long) chr - '0',
                        >= 'A' and <= 'Z' => (long) chr - 'A' + 10,
                        >= 'a' and <= 'z' => (long) chr - 'a' + 10,
                        _ => throw new ArgumentOutOfRangeException(),
                    };

                    if (value >= @base)
                    {
                        var location = new SourceLocation(Filepath, _position, _line, _column);
                        NextChar();
                        throw new CompileError(location, $"Digit '{chr}' is too big for base {@base}");
                    }

                    intValue *= @base;
                    intValue += value;

                    NextChar();
                }

                if (CurrentChar == '.')
                {
                    var floatValue = (double) intValue;
                    var discriminant = 1L;

                    while (char.IsAscii(CurrentChar) && char.IsLetterOrDigit(CurrentChar) || CurrentChar == '_')
                    {
                        if (CurrentChar == '_')
                        {
                            NextChar();
                            continue;
                        }

                        var chr = CurrentChar;
                        var value = chr switch
                        {
                            >= '0' and <= '9' => (long) chr - '0',
                            >= 'A' and <= 'Z' => (long) chr - 'A' + 10,
                            >= 'a' and <= 'z' => (long) chr - 'a' + 10,
                            _ => throw new ArgumentOutOfRangeException(),
                        };

                        if (value >= @base)
                        {
                            var location = new SourceLocation(Filepath, _position, _line, _column);
                            NextChar();
                            throw new CompileError(location, $"Digit '{chr}' is too big for base {@base}");
                        }

                        discriminant *= @base;
                        floatValue += (double) value / discriminant;

                        NextChar();
                    }

                    return new Token(TokenKind.Float, startLocation, _position - startLocation.Position, floatValue);
                }

                return new Token(TokenKind.Integer, startLocation, _position - startLocation.Position, intValue);
            }

            if (char.IsLetterOrDigit(CurrentChar) || CurrentChar == '_')
            {
                while (char.IsLetterOrDigit(CurrentChar) || CurrentChar == '_')
                    NextChar();
                var length = _position - startLocation.Position;
                var name = Source.Substring(startLocation.Position, length);
                return new Token(TokenKindMethods.FromString(name), startLocation, length, name);
            }

            {
                var chr = NextChar();

                if (chr == '/' && CurrentChar == '/')
                {
                    while (CurrentChar != '\n')
                        NextChar();
                    goto Start;
                }

                if (chr == '/' && CurrentChar == '*')
                    throw new NotImplementedException();
                // goto Start;

                {
                    if (TokenKindMethods.FromDoubleChar(chr, CurrentChar) is { } kind)
                    {
                        NextChar();
                        return new Token(kind, startLocation, _position - startLocation.Position);
                    }
                }
                {
                    if (TokenKindMethods.FromChar(chr) is { } kind)
                        return new Token(kind, startLocation, _position - startLocation.Position);
                }

                throw new CompileError(startLocation, $"Unexpected character '{chr}'");
            }
        }

        private char NextChar()
        {
            var current = CurrentChar;
            if (current != '\0')
            {
                _position++;
                _column++;
                if (current == '\n')
                {
                    _line++;
                    _column = 1;
                }
            }

            return current;
        }
    }
}
