namespace Langite.Syntax.Ast
{
    public abstract class Visitor<T, U>
    {
        public abstract T Visit(File file, U arg);
        public abstract T Visit(Unary unary, U arg);
        public abstract T Visit(Binary binary, U arg);
        public abstract T Visit(Integer integer, U arg);
        public abstract T Visit(Float @float, U arg);
        public abstract T Visit(String @string, U arg);
        public abstract T Visit(Name name, U arg);
        public abstract T Visit(Wildcard wildcard, U arg);
        public abstract T Visit(Declaration declaration, U arg);
        public abstract T Visit(GenericParameter genericParameter, U arg);
        public abstract T Visit(ConstDeclaration constDeclaration, U arg);
        public abstract T Visit(Function function, U arg);
        public abstract T Visit(Procedure procedure, U arg);
        public abstract T Visit(Block block, U arg);
        public abstract T Visit(Return @return, U arg);
        public abstract T Visit(If @if, U arg);
        public abstract T Visit(Call call, U arg);
        public abstract T Visit(GenericInstantiation genericInstantiation, U arg);
        public abstract T Visit(ParenthesisedExpression parenthesisedExpression, U arg);
        public abstract T Visit(FieldAccess fieldAccess, U arg);
        public abstract T Visit(Builtin builtin, U arg);
        public abstract T Visit(BuiltinArray builtinArray, U arg);
    }
}
