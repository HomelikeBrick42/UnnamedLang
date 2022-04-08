using System;
using Langite;
using Langite.Resolving;
using Langite.Syntax;
using Langite.Syntax.Ast;
using File = System.IO.File;

if (args.Length != 1)
{
    Console.ForegroundColor = ConsoleColor.Red;
    Console.WriteLine("Usage: langite.exe <file>");
    Console.ResetColor();
    return 1;
}

var filepath = args[0];
var source = File.ReadAllText(filepath);

try
{
#if false
    var lexer = new Lexer(filepath, source);
    while (true)
    {
        var token = lexer.NextToken();
        Console.WriteLine($"{token} {token.Value}");
        if (token.Kind == TokenKind.EndOfFile)
            break;
    }
#else
    var ast = Parser.Parse(filepath, source);
    NameResolver.Resolve(ast);
    PrettyPrinter.Print(ast);
#endif
}
catch (CompileError e)
{
    Console.ForegroundColor = ConsoleColor.Red;
    Console.WriteLine(e.Message);
    Console.ResetColor();
    return 1;
}
catch (Exception e)
{
    Console.ForegroundColor = ConsoleColor.Red;
    Console.WriteLine(e);
    Console.ResetColor();
    return -1;
}

return 0;
