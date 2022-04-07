using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace Langite.Syntax
{
    public sealed class Parser
    {
        private readonly Lexer _lexer;

        private Parser(string filepath, string source)
        {
            _lexer = new Lexer(filepath, source);
        }

        public static Ast.File Parse(string filepath, string source)
        {
            var parser = new Parser(filepath, source);
            return parser.ParseFile();
        }

        private Ast.File ParseFile()
        {
            var expressions = new List<Ast.Node>();
            while (_lexer.PeekKind() != TokenKind.EndOfFile)
            {
                AllowMultipleNewlines();
                expressions.Add(ParseExpression());
                ExpectNewline();
            }

            var endOfFileToken = ExpectToken(TokenKind.EndOfFile);
            return new Ast.File(expressions, endOfFileToken);
        }

        private Ast.Block ParseBlock()
        {
            var openBraceToken = ExpectToken(TokenKind.OpenBrace);
            var expressions = new List<Ast.Node>();
            while (_lexer.PeekKind() != TokenKind.CloseBrace)
            {
                AllowMultipleNewlines();
                expressions.Add(ParseExpression());
                ExpectNewline();
            }

            var closeBraceToken = ExpectToken(TokenKind.CloseBrace);
            return new Ast.Block(openBraceToken, expressions, closeBraceToken);
        }

        private Ast.Declaration ParseDeclaration()
        {
            var nameToken = ExpectOneOf(new[] {TokenKind.Name, TokenKind.Wildcard});
            var colonToken = ExpectToken(TokenKind.Colon);
            var type = ParseLeastExpression();
            return new Ast.Declaration(nameToken, colonToken, type);
        }

        private Ast.If ParseIf()
        {
            var ifToken = ExpectToken(TokenKind.If);
            var condition = ParseExpression();
            var thenBlock = ParseBlock();
            Token? elseToken = null;
            Ast.Node? elseNode = null;
            if (_lexer.PeekKind() == TokenKind.Else)
            {
                elseToken = _lexer.NextToken();
                if (_lexer.PeekKind() == TokenKind.If)
                    elseNode = ParseIf();
                else
                    elseNode = ParseBlock();
            }

            return new Ast.If(ifToken, condition, thenBlock, elseToken, elseNode);
        }

        private Ast.Node ParseExpression()
        {
            return ParseBinaryExpression(0);
        }

        private Ast.Node ParseLeastExpression()
        {
            return ParseBinaryExpression(uint.MaxValue);
        }

        private Ast.Node ParseBinaryExpression(uint parentPrecedence)
        {
            Ast.Node left;
            var unaryPrecedence = _lexer.PeekKind().GetUnaryOperatorPrecedence();
            if (unaryPrecedence > 0)
            {
                var operatorToken = _lexer.NextToken();
                AllowNewline();
                var operand = ParseBinaryExpression(unaryPrecedence);
                left = new Ast.Unary(operatorToken, operand);
            }
            else
            {
                left = ParsePrimaryExpression();
            }

            while (true)
                switch (_lexer.PeekKind())
                {
                    case TokenKind.OpenParenthesis:
                    {
                        var openParenthesisToken = _lexer.NextToken();
                        AllowNewline();
                        var arguments = new List<Ast.Node>();
                        while (_lexer.PeekKind() != TokenKind.CloseParenthesis)
                        {
                            arguments.Add(ParseExpression());
                            ExpectCommaOrNewline();
                        }

                        var closeParenthesisToken = ExpectToken(TokenKind.CloseParenthesis);
                        left = new Ast.Call(left, openParenthesisToken, arguments, closeParenthesisToken);
                        break;
                    }


                    case TokenKind.OpenSquareBracket:
                    {
                        var openSquareBracketToken = _lexer.NextToken();
                        AllowNewline();
                        var genericArguments = new List<Ast.Node>();
                        while (_lexer.PeekKind() != TokenKind.CloseSquareBracket)
                        {
                            genericArguments.Add(ParseExpression());
                            ExpectCommaOrNewline();
                        }

                        var closeSquareBracketToken = ExpectToken(TokenKind.CloseSquareBracket);
                        left = new Ast.GenericInstantiation(left, openSquareBracketToken, genericArguments,
                            closeSquareBracketToken);
                        break;
                    }

                    case TokenKind.Period:
                    {
                        var periodToken = _lexer.NextToken();
                        var nameToken = ExpectToken(TokenKind.Name);
                        left = new Ast.FieldAccess(left, periodToken, nameToken);
                        break;
                    }

                    default:
                    {
                        var binaryPrecedence = _lexer.PeekKind().GetBinaryOperatorPrecedence();
                        if (binaryPrecedence <= parentPrecedence)
                            goto End;

                        var operatorToken = _lexer.NextToken();
                        AllowNewline();
                        var right = ParseBinaryExpression(binaryPrecedence);
                        left = new Ast.Binary(left, operatorToken, right);
                        break;
                    }
                }

            End:

            return left;
        }

        private Ast.Node ParsePrimaryExpression()
        {
            switch (_lexer.PeekKind())
            {
                case TokenKind.OpenParenthesis:
                {
                    var openParenthesisToken = _lexer.NextToken();
                    AllowNewline();
                    var expression = ParseExpression();
                    AllowNewline();
                    var closeParenthesisToken = ExpectToken(TokenKind.CloseParenthesis);
                    return new Ast.ParenthesisedExpression(openParenthesisToken, expression, closeParenthesisToken);
                }

                case TokenKind.Const:
                {
                    var constToken = _lexer.NextToken();
                    var nameToken = ExpectOneOf(new[] {TokenKind.Name, TokenKind.Wildcard});
                    Token? openSquareBracketToken = null;
                    IList<Ast.GenericParameter>? genericParameters = null;
                    Token? closeSquareBracketToken = null;
                    if (_lexer.PeekKind() == TokenKind.OpenSquareBracket)
                    {
                        openSquareBracketToken = _lexer.NextToken();
                        AllowNewline();
                        genericParameters = new List<Ast.GenericParameter>();
                        while (_lexer.PeekKind() != TokenKind.CloseSquareBracket)
                        {
                            var name = ExpectToken(TokenKind.Name);
                            var colon = ExpectToken(TokenKind.Colon);
                            var typ = ParseExpression();
                            genericParameters.Add(new Ast.GenericParameter(name, colon, typ));
                            ExpectCommaOrNewline();
                        }

                        closeSquareBracketToken = ExpectToken(TokenKind.CloseSquareBracket);
                    }

                    Token? colonToken = null;
                    Ast.Node? type = null;
                    if (_lexer.PeekKind() == TokenKind.Colon)
                    {
                        colonToken = _lexer.NextToken();
                        AllowNewline();
                        type = ParseExpression();
                    }

                    var equalsToken = ExpectToken(TokenKind.Equal);
                    AllowNewline();
                    var value = ParseExpression();
                    return new Ast.ConstDeclaration(constToken, nameToken, openSquareBracketToken, genericParameters,
                        closeSquareBracketToken, colonToken, type, equalsToken, value);
                }

                case TokenKind.Builtin:
                {
                    var builtinToken = _lexer.NextToken();
                    var stringToken = ExpectToken(TokenKind.String);
                    return new Ast.Builtin(builtinToken, stringToken);
                }

                case TokenKind.BuiltinArray:
                {
                    var builtinArrayToken = _lexer.NextToken();
                    var openSquareBracketToken = ExpectToken(TokenKind.OpenSquareBracket);
                    var type = ParseExpression();
                    ExpectCommaOrNewline();
                    var length = ParseExpression();
                    var closeSquareBracketToken = ExpectToken(TokenKind.CloseSquareBracket);
                    return new Ast.BuiltinArray(builtinArrayToken, openSquareBracketToken, type, length,
                        closeSquareBracketToken);
                }

                case TokenKind.Name:
                {
                    var nameToken = _lexer.NextToken();
                    if (_lexer.PeekKind() == TokenKind.Colon)
                    {
                        var colonToken = _lexer.NextToken();
                        var type = ParseLeastExpression();
                        return new Ast.Declaration(nameToken, colonToken, type);
                    }

                    return new Ast.Name(nameToken);
                }

                case TokenKind.Integer:
                {
                    var integerToken = _lexer.NextToken();
                    return new Ast.Integer(integerToken);
                }

                case TokenKind.Float:
                {
                    var floatToken = _lexer.NextToken();
                    return new Ast.Float(floatToken);
                }

                case TokenKind.String:
                {
                    var stringToken = _lexer.NextToken();
                    return new Ast.String(stringToken);
                }

                case TokenKind.Func:
                {
                    var funcToken = _lexer.NextToken();
                    var openParenthesisToken = ExpectToken(TokenKind.OpenParenthesis);
                    var parameters = new List<Ast.Declaration>();
                    AllowNewline();
                    while (_lexer.PeekKind() != TokenKind.CloseParenthesis)
                    {
                        parameters.Add(ParseDeclaration());
                        ExpectCommaOrNewline();
                    }

                    var closeParenthesisToken = ExpectToken(TokenKind.CloseParenthesis);
                    var colonToken = ExpectToken(TokenKind.Colon);
                    AllowNewline();
                    var type = ParseLeastExpression();
                    Ast.Block? body = null;
                    if (_lexer.PeekKind() == TokenKind.OpenBrace)
                        body = ParseBlock();

                    return new Ast.Function(funcToken, openParenthesisToken, parameters, closeParenthesisToken,
                        colonToken, type, body);
                }

                case TokenKind.Proc:
                {
                    var procToken = _lexer.NextToken();
                    var openParenthesisToken = ExpectToken(TokenKind.OpenParenthesis);
                    var parameters = new List<Ast.Declaration>();
                    AllowNewline();
                    while (_lexer.PeekKind() != TokenKind.CloseParenthesis)
                    {
                        parameters.Add(ParseDeclaration());
                        ExpectCommaOrNewline();
                    }

                    var closeParenthesisToken = ExpectToken(TokenKind.CloseParenthesis);
                    var colonToken = ExpectToken(TokenKind.Colon);
                    AllowNewline();
                    var type = ParseLeastExpression();
                    Ast.Block? body = null;
                    if (_lexer.PeekKind() == TokenKind.OpenBrace)
                        body = ParseBlock();

                    return new Ast.Procedure(procToken, openParenthesisToken, parameters, closeParenthesisToken,
                        colonToken, type, body);
                }

                case TokenKind.Return:
                {
                    var returnToken = _lexer.NextToken();
                    Ast.Node? value = null;
                    var current = _lexer.PeekKind();
                    if (current is not TokenKind.CloseBrace and not TokenKind.CloseParenthesis and not
                        TokenKind.CloseSquareBracket and not TokenKind.Newline and not TokenKind.EndOfFile)
                        value = ParseExpression();
                    return new Ast.Return(returnToken, value);
                }

                case TokenKind.If:
                    return ParseIf();

                default:
                {
                    var token = _lexer.NextToken();
                    throw new CompileError(token.Location, $"Expected an expression, but got '{token}'");
                }
            }
        }

        private Token ExpectToken(TokenKind kind)
        {
            var token = _lexer.NextToken();
            if (token.Kind == kind)
                return token;
            throw new CompileError(token.Location, $"Expected '{kind.AsString()}', but got '{token}'");
        }

        private Token ExpectOneOf(ICollection<TokenKind> kinds)
        {
            Debug.Assert(kinds.Count > 0);
            if (kinds.Count == 1)
                return ExpectToken(kinds.First());
            var token = _lexer.NextToken();
            if (kinds.Contains(token.Kind))
                return token;
            var message = $"Expected one of [";
            var first = true;
            foreach (var kind in kinds)
            {
                if (first)
                    first = false;
                else
                    message += ", ";
                message += $"'{kind.AsString()}'";
            }

            message += $"], but got '{token}'";
            throw new CompileError(token.Location, message);
        }

        private void AllowNewline()
        {
            if (_lexer.PeekKind() == TokenKind.Newline)
                _lexer.NextToken();
        }

        private void AllowMultipleNewlines()
        {
            while (_lexer.PeekKind() == TokenKind.Newline)
                _lexer.NextToken();
        }

        private void ExpectNewline()
        {
            var current = _lexer.PeekKind();
            if (current is not TokenKind.CloseParenthesis and not TokenKind.CloseBrace and not
                TokenKind.CloseSquareBracket and not TokenKind.EndOfFile)
                ExpectToken(TokenKind.Newline);
        }

        private void ExpectCommaOrNewline()
        {
            var current = _lexer.PeekKind();
            if (current is not TokenKind.Newline and not TokenKind.CloseParenthesis and not TokenKind.CloseBrace and not
                TokenKind.CloseSquareBracket and not TokenKind.EndOfFile)
                ExpectToken(TokenKind.Comma);
            AllowNewline();
        }
    }
}
