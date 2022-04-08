using System;

namespace Langite.Syntax.Ast
{
    public sealed class PrettyPrinter : Visitor<ValueTuple, ulong>
    {
        private PrettyPrinter()
        {
        }

        public static void Print(Node node)
        {
            var prettyPrinter = new PrettyPrinter();
            node.Accept(prettyPrinter, 0UL);
        }

        private static void PrintIndent(ulong indent)
        {
            for (var i = 0UL; i < indent; i++) Console.Write("  ");
        }

        private static void PrintHeader(Node node, ulong indent, string title)
        {
            PrintIndent(indent);
            Console.WriteLine(title);
            /*
            if (node.ResolvedType is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Resolved Type:");
                PrintIndent(indent + 2);
                Console.WriteLine(node.ResolvedType);
            }
            */
        }

        public override ValueTuple Visit(File file, ulong indent)
        {
            PrintHeader(file, indent, $"- File: '{file.Location.Filepath}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Expressions:");
            foreach (var expression in file.Expressions)
                expression.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Unary unary, ulong indent)
        {
            PrintHeader(unary, indent, $"- Unary: '{unary.OperatorToken}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Operand:");
            unary.Operand.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Binary binary, ulong indent)
        {
            PrintHeader(binary, indent, $"- Binary: '{binary.OperatorToken}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Left:");
            binary.Left.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Right:");
            binary.Right.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Integer integer, ulong indent)
        {
            PrintHeader(integer, indent, $"- Integer: {integer.IntegerToken.Value as long?}");
            return default;
        }

        public override ValueTuple Visit(Float @float, ulong indent)
        {
            PrintHeader(@float, indent, $"- Float: {@float.FloatToken.Value as double?}");
            return default;
        }

        public override ValueTuple Visit(String @string, ulong indent)
        {
            PrintHeader(@string, indent, $"- String: \"{@string.StringToken.Value as string}\"");
            return default;
        }

        public override ValueTuple Visit(Name name, ulong indent)
        {
            PrintHeader(name, indent, $"- Name: '{name.NameString}'");
            return default;
        }

        public override ValueTuple Visit(Wildcard wildcard, ulong indent)
        {
            PrintHeader(wildcard, indent, "- Wildcard");
            return default;
        }

        public override ValueTuple Visit(Declaration declaration, ulong indent)
        {
            PrintHeader(declaration, indent, $"- Declaration: '{declaration.Name}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Type:");
            declaration.Type.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(GenericParameter genericParameter, ulong indent)
        {
            PrintHeader(genericParameter, indent, $"- Generic Parameter: '{genericParameter.Name}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Type:");
            genericParameter.Type.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(ConstDeclaration constDeclaration, ulong indent)
        {
            PrintHeader(constDeclaration, indent, $"- Const Declaration: '{constDeclaration.Name}'");

            if (constDeclaration.GenericParameters is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Generic Parameters:");
                foreach (var genericParameter in constDeclaration.GenericParameters)
                    genericParameter.Accept(this, indent + 2);
            }

            if (constDeclaration.Type is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Type:");
                constDeclaration.Type.Accept(this, indent + 2);
            }

            PrintIndent(indent + 1);
            Console.WriteLine("Value:");
            constDeclaration.Value.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Function function, ulong indent)
        {
            PrintHeader(function, indent, "- Function");
            PrintIndent(indent + 1);
            Console.WriteLine("Parameters:");
            foreach (var parameter in function.Parameters)
                parameter.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Return Type:");
            function.ReturnType.Accept(this, indent + 2);
            if (function.Body is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Body:");
                function.Body.Accept(this, indent + 2);
            }

            return default;
        }

        public override ValueTuple Visit(Procedure procedure, ulong indent)
        {
            PrintHeader(procedure, indent, "- Procedure");
            PrintIndent(indent + 1);
            Console.WriteLine("Parameters:");
            foreach (var parameter in procedure.Parameters)
                parameter.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Return Type:");
            procedure.ReturnType.Accept(this, indent + 2);
            if (procedure.Body is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Body:");
                procedure.Body.Accept(this, indent + 2);
            }

            return default;
        }

        public override ValueTuple Visit(Block block, ulong indent)
        {
            PrintHeader(block, indent, "- Block");
            PrintIndent(indent + 1);
            Console.WriteLine("Expressions:");
            foreach (var expression in block.Expressions)
                expression.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Return @return, ulong indent)
        {
            PrintHeader(@return, indent, "- Return");
            if (@return.Value is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("Value:");
                @return.Value.Accept(this, indent + 2);
            }

            return default;
        }

        public override ValueTuple Visit(If @if, ulong indent)
        {
            PrintHeader(@if, indent, "- If");
            PrintIndent(indent + 1);
            Console.WriteLine("Condition:");
            @if.Condition.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("ThenBlock:");
            @if.ThenBlock.Accept(this, indent + 2);
            if (@if.ElseNode is not null)
            {
                PrintIndent(indent + 1);
                Console.WriteLine("ElseNode:");
                @if.ElseNode.Accept(this, indent + 2);
            }

            return default;
        }

        public override ValueTuple Visit(Call call, ulong indent)
        {
            PrintHeader(call, indent, "- Call");
            PrintIndent(indent + 1);
            Console.WriteLine("Operand:");
            call.Operand.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Arguments:");
            foreach (var argument in call.Arguments)
                argument.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(GenericInstantiation genericInstantiation, ulong indent)
        {
            PrintHeader(genericInstantiation, indent, "- GenericInstantiation");
            PrintIndent(indent + 1);
            Console.WriteLine("Operand:");
            genericInstantiation.Operand.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Generic Arguments:");
            foreach (var genericArgument in genericInstantiation.GenericArguments)
                genericArgument.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(ParenthesisedExpression parenthesisedExpression, ulong indent)
        {
            PrintHeader(parenthesisedExpression, indent, "- Parenthesised Expression");
            PrintIndent(indent + 1);
            Console.WriteLine("Expression:");
            parenthesisedExpression.Expression.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(FieldAccess fieldAccess, ulong indent)
        {
            PrintHeader(fieldAccess, indent, $"- Field Access: '{fieldAccess.Name}'");
            PrintIndent(indent + 1);
            Console.WriteLine("Operand:");
            fieldAccess.Operand.Accept(this, indent + 2);
            return default;
        }

        public override ValueTuple Visit(Builtin builtin, ulong indent)
        {
            PrintHeader(builtin, indent, $"- Builtin: \"{builtin.StringToken.Value as string}\"");
            return default;
        }

        public override ValueTuple Visit(BuiltinArray builtinArray, ulong indent)
        {
            PrintHeader(builtinArray, indent, "- Builtin Array");
            PrintIndent(indent + 1);
            Console.WriteLine("Type:");
            builtinArray.InnerType.Accept(this, indent + 2);
            PrintIndent(indent + 1);
            Console.WriteLine("Length:");
            builtinArray.Length.Accept(this, indent + 2);
            return default;
        }
    }
}
