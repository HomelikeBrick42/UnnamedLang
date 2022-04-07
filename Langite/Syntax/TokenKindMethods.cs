using System;

namespace Langite.Syntax
{
    public static class TokenKindMethods
    {
        public static TokenKind? FromChar(char chr)
        {
            return chr switch
            {
                '\0' => TokenKind.EndOfFile,
                '\n' => TokenKind.Newline,
                '(' => TokenKind.OpenParenthesis,
                ')' => TokenKind.CloseParenthesis,
                '{' => TokenKind.OpenBrace,
                '}' => TokenKind.CloseBrace,
                '[' => TokenKind.OpenSquareBracket,
                ']' => TokenKind.CloseSquareBracket,
                ':' => TokenKind.Colon,
                ',' => TokenKind.Comma,
                '.' => TokenKind.Period,
                '@' => TokenKind.At,
                '+' => TokenKind.Plus,
                '-' => TokenKind.Minus,
                '*' => TokenKind.Asterisk,
                '/' => TokenKind.Slash,
                '%' => TokenKind.Percent,
                '<' => TokenKind.LessThan,
                '>' => TokenKind.GreaterThan,
                '=' => TokenKind.Equal,
                '!' => TokenKind.ExclamationMark,
                _ => null,
            };
        }

        public static TokenKind? FromDoubleChar(char first, char second)
        {
            return (first, second) switch
            {
                ('<', '=') => TokenKind.LessThanEqual,
                ('>', '=') => TokenKind.GreaterThanEqual,
                ('=', '=') => TokenKind.EqualEqual,
                ('!', '=') => TokenKind.ExclamationMarkEqual,
                ('<', '-') => TokenKind.LeftArrow,
                ('-', '>') => TokenKind.RightArrow,
                _ => null,
            };
        }

        public static TokenKind FromString(string str)
        {
            return str.Length switch
            {
                1 when FromChar(str[0]) is { } kind => kind,
                2 when FromDoubleChar(str[0], str[1]) is { } kind => kind,
                _ => str switch
                {
                    "_" => TokenKind.Wildcard,
                    "const" => TokenKind.Const,
                    "func" => TokenKind.Func,
                    "proc" => TokenKind.Proc,
                    "return" => TokenKind.Return,
                    "if" => TokenKind.If,
                    "else" => TokenKind.Else,
                    "__builtin" => TokenKind.Builtin,
                    "__builtin_array" => TokenKind.BuiltinArray,
                    _ => TokenKind.Name,
                },
            };
        }

        public static string AsString(this TokenKind kind)
        {
            return kind switch
            {
                TokenKind.EndOfFile => "{EOF}",
                TokenKind.Newline => "{newline}",
                TokenKind.Name => "{name}",
                TokenKind.Integer => "{integer}",
                TokenKind.Float => "{float}",
                TokenKind.String => "{string}",
                TokenKind.Wildcard => "_",
                TokenKind.Const => "const",
                TokenKind.Func => "func",
                TokenKind.Proc => "proc",
                TokenKind.Return => "return",
                TokenKind.If => "if",
                TokenKind.Else => "else",
                TokenKind.Builtin => "__builtin",
                TokenKind.OpenParenthesis => "(",
                TokenKind.CloseParenthesis => ")",
                TokenKind.OpenBrace => "{",
                TokenKind.CloseBrace => "}",
                TokenKind.OpenSquareBracket => "[",
                TokenKind.CloseSquareBracket => "]",
                TokenKind.Colon => ":",
                TokenKind.Comma => ",",
                TokenKind.Period => ".",
                TokenKind.At => "@",
                TokenKind.Plus => "+",
                TokenKind.Minus => "-",
                TokenKind.Asterisk => "*",
                TokenKind.Slash => "/",
                TokenKind.Percent => "%",
                TokenKind.LessThan => "<",
                TokenKind.LessThanEqual => "<=",
                TokenKind.GreaterThan => ">",
                TokenKind.GreaterThanEqual => ">=",
                TokenKind.Equal => "=",
                TokenKind.EqualEqual => "==",
                TokenKind.ExclamationMark => "!",
                TokenKind.ExclamationMarkEqual => "!=",
                TokenKind.LeftArrow => "<-",
                TokenKind.RightArrow => "->",
                TokenKind.BuiltinArray => "__builtin_array",
                _ => throw new ArgumentOutOfRangeException(nameof(kind), kind, null),
            };
        }

        public static uint GetUnaryOperatorPrecedence(this TokenKind kind)
        {
            return kind switch
            {
                TokenKind.Plus or TokenKind.Minus or TokenKind.ExclamationMark => 6,
                _ => 0,
            };
        }

        public static uint GetBinaryOperatorPrecedence(this TokenKind kind)
        {
            return kind switch
            {
                TokenKind.At => 5,
                TokenKind.Asterisk or TokenKind.Slash or TokenKind.Percent => 4,
                TokenKind.Plus or TokenKind.Minus => 3,
                TokenKind.EqualEqual or TokenKind.ExclamationMarkEqual => 2,
                TokenKind.LeftArrow or TokenKind.RightArrow => 1,
                _ => 0,
            };
        }
    }
}
