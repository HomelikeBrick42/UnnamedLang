namespace Langite.Syntax.Ast
{
    public abstract class Visitor<T, U>
    {
        public abstract T Visit(File file, U indent);
        public abstract T Visit(Unary unary, U indent);
        public abstract T Visit(Binary binary, U indent);
        public abstract T Visit(Integer integer, U indent);
        public abstract T Visit(Float @float, U indent);
        public abstract T Visit(String @string, U indent);
        public abstract T Visit(Name name, U indent);
        public abstract T Visit(Declaration declaration, U indent);
        public abstract T Visit(GenericParameter genericParameter, U indent);
        public abstract T Visit(ConstDeclaration constDeclaration, U indent);
        public abstract T Visit(Function function, U indent);
        public abstract T Visit(Procedure procedure, U indent);
        public abstract T Visit(Block block, U indent);
        public abstract T Visit(Return @return, U indent);
        public abstract T Visit(If @if, U indent);
        public abstract T Visit(Call call, U indent);
        public abstract T Visit(GenericInstantiation genericInstantiation, U indent);
        public abstract T Visit(ParenthesisedExpression parenthesisedExpression, U indent);
        public abstract T Visit(FieldAccess fieldAccess, U indent);
        public abstract T Visit(Builtin builtin, U indent);
        public abstract T Visit(BuiltinArray builtinArray, U indent);
    }
}
