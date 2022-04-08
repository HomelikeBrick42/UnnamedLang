using System;

namespace Langite.Syntax.Ast
{
    public class Searcher<U> : Visitor<ValueTuple, U>
    {
        public override ValueTuple Visit(File file, U indent)
        {
            foreach (var child in file.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Unary unary, U indent)
        {
            foreach (var child in unary.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Binary binary, U indent)
        {
            foreach (var child in binary.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Integer integer, U indent)
        {
            foreach (var child in integer.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Float @float, U indent)
        {
            foreach (var child in @float.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(String @string, U indent)
        {
            foreach (var child in @string.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Name name, U indent)
        {
            foreach (var child in name.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Wildcard wildcard, U indent)
        {
            foreach (var child in wildcard.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Declaration declaration, U indent)
        {
            foreach (var child in declaration.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(GenericParameter genericParameter, U indent)
        {
            foreach (var child in genericParameter.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(ConstDeclaration constDeclaration, U indent)
        {
            foreach (var child in constDeclaration.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Function function, U indent)
        {
            foreach (var child in function.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Procedure procedure, U indent)
        {
            foreach (var child in procedure.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Block block, U indent)
        {
            foreach (var child in block.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Return @return, U indent)
        {
            foreach (var child in @return.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(If @if, U indent)
        {
            foreach (var child in @if.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Call call, U indent)
        {
            foreach (var child in call.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(GenericInstantiation genericInstantiation, U indent)
        {
            foreach (var child in genericInstantiation.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(ParenthesisedExpression parenthesisedExpression, U indent)
        {
            foreach (var child in parenthesisedExpression.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(FieldAccess fieldAccess, U indent)
        {
            foreach (var child in fieldAccess.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(Builtin builtin, U indent)
        {
            foreach (var child in builtin.GetChildren())
                child.Accept(this, indent);
            return default;
        }

        public override ValueTuple Visit(BuiltinArray builtinArray, U indent)
        {
            foreach (var child in builtinArray.GetChildren())
                child.Accept(this, indent);
            return default;
        }
    }
}
