using System;
using System.Collections.Generic;
using Langite.Syntax;
using Langite.Syntax.Ast;
using Langite.Types;
using String = Langite.Syntax.Ast.String;
using Type = Langite.Types.Type;

namespace Langite.Resolving
{
    internal sealed class TypeEvaluator : Visitor<Type, ValueTuple>
    {
        public override Type Visit(Syntax.Ast.File file, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Unary unary, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Binary binary, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Integer integer, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Float @float, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(String @string, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Name name, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Wildcard wildcard, ValueTuple arg)
        {
            return new Type(new PlaceholderType());
        }

        public override Type Visit(Declaration declaration, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(GenericParameter genericParameter, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(ConstDeclaration constDeclaration, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Function function, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Procedure procedure, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Block block, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Return @return, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(If @if, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Call call, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(GenericInstantiation genericInstantiation, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(ParenthesisedExpression parenthesisedExpression, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(FieldAccess fieldAccess, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(Builtin builtin, ValueTuple arg)
        {
            throw new NotImplementedException();
        }

        public override Type Visit(BuiltinArray builtinArray, ValueTuple arg)
        {
            throw new NotImplementedException();
        }
    }

    public sealed class TypeResolver : Visitor<ValueTuple, Type?>
    {
        public static readonly Type DefaultVoidType = new(new VoidType());
        public static readonly Type DefaultTypeType = new(new TypeType());
        public static readonly Type DefaultBoolType = new(new BoolType());
        public static readonly Type DefaultCharType = new(new CharType());
        public static readonly Type DefaultIntegerType = new(new IntegerType());
        public static readonly Type DefaultFloatType = new(new FloatType());
        public static readonly Type DefaultStringType = new(new StringType());
        public static readonly Dictionary<(Type, ulong), Type> DefaultArrayTypes = new();

        private TypeResolver()
        {
        }

        public static void Resolve(Node node)
        {
            var typeResolver = new TypeResolver();
            node.Accept(typeResolver, null);
            CheckTypesResolved(node);
        }

        private static void CheckTypesResolved(Node node)
        {
            foreach (var child in node.GetChildren())
                CheckTypesResolved(child);
            if (node.ResolvedType is null)
                throw new CompileError(node.Location, "Unable to resolve type");
        }

        private static Type IfSameKindOrDefault(Type? suggested, Type @default)
        {
            if (suggested is not null && suggested.Kind == @default.Kind)
                return suggested;

            if (suggested is not null && suggested.Kind == TypeKind.Placeholder)
            {
                suggested.Value = @default.Value;
                return suggested;
            }

            return @default;
        }

        public override ValueTuple Visit(Syntax.Ast.File file, Type? expectedType)
        {
            file.ResolvedType = IfSameKindOrDefault(expectedType, DefaultVoidType);
            foreach (var expression in file.Expressions)
                expression.Accept(this, null);
            return default;
        }

        public override ValueTuple Visit(Unary unary, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(Binary binary, Type? expectedType)
        {
            binary.Left.Accept(this, expectedType);
            binary.Right.Accept(this, binary.Left.ResolvedType);
            switch (binary.OperatorToken.Kind)
            {
                case TokenKind.LeftArrow:
                {
                    if (binary.Left.ResolvedType!.Kind == TypeKind.Placeholder)
                    {
                        binary.Left.ResolvedType.Value = binary.Right.ResolvedType!.Value;
                    }
                    else if (binary.Right.ResolvedType!.Kind == TypeKind.Placeholder)
                    {
                        binary.Right.ResolvedType.Value = binary.Left.ResolvedType!.Value;
                    }

                    if (binary.Left.ResolvedType! != binary.Right.ResolvedType!)
                        throw new CompileError(binary.Location,
                            $"Cannot assign type '{binary.Right.ResolvedType}' to type '{binary.Left.ResolvedType}'");

                    binary.ResolvedType = binary.Left.ResolvedType;
                    break;
                }

                case TokenKind.RightArrow:
                {
                    if (binary.Left.ResolvedType!.Kind == TypeKind.Placeholder)
                    {
                        binary.Left.ResolvedType.Value = binary.Right.ResolvedType!.Value;
                    }
                    else if (binary.Right.ResolvedType!.Kind == TypeKind.Placeholder)
                    {
                        binary.Right.ResolvedType.Value = binary.Left.ResolvedType!.Value;
                    }

                    if (binary.Left.ResolvedType! != binary.Right.ResolvedType!)
                        throw new CompileError(binary.Location,
                            $"Cannot assign type '{binary.Left.ResolvedType}' to type '{binary.Right.ResolvedType}'");

                    binary.ResolvedType = binary.Right.ResolvedType;
                    break;
                }

                case TokenKind.Plus:
                case TokenKind.Minus:
                case TokenKind.Asterisk:
                case TokenKind.Slash:
                case TokenKind.Percent:
                {
                    throw new NotImplementedException();
                }

                default:
                    throw new NotImplementedException();
            }

            return default;
        }

        public override ValueTuple Visit(Integer integer, Type? expectedType)
        {
            integer.ResolvedType = IfSameKindOrDefault(expectedType, DefaultIntegerType);
            return default;
        }

        public override ValueTuple Visit(Float @float, Type? expectedType)
        {
            @float.ResolvedType = IfSameKindOrDefault(expectedType, DefaultFloatType);
            return default;
        }

        public override ValueTuple Visit(String @string, Type? expectedType)
        {
            @string.ResolvedType = IfSameKindOrDefault(expectedType, DefaultStringType);
            return default;
        }

        public override ValueTuple Visit(Name name, Type? expectedType)
        {
            name.ResolvedType = name.ResolvedDeclaration!.ResolvedType;
            return default;
        }

        public override ValueTuple Visit(Wildcard wildcard, Type? expectedType)
        {
            wildcard.ResolvedType = expectedType ?? new Type(new PlaceholderType());
            return default;
        }

        public override ValueTuple Visit(Declaration declaration, Type? expectedType)
        {
            declaration.Type.Accept(this, DefaultTypeType);
            if (declaration.Type.ResolvedType!.Kind != TypeKind.Type)
                throw new CompileError(declaration.Type.Location, $"The declaration type must be a type but got type '{declaration.Type.ResolvedType}'");
            declaration.ResolvedType = EvalType(declaration.Type);
            return default;
        }

        private static Type EvalType(Node type)
        {
            return type.Accept(new TypeEvaluator(), default);
        }

        public override ValueTuple Visit(GenericParameter genericParameter, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(ConstDeclaration constDeclaration, Type? expectedType)
        {
            constDeclaration.ResolvedType = IfSameKindOrDefault(expectedType, DefaultVoidType);
            return default;
        }

        public override ValueTuple Visit(Function function, Type? expectedType)
        {
            if (function.Body is not null)
            {
                throw new NotImplementedException();
            }
            else
            {
                throw new NotImplementedException();
            }

            return default;
        }

        public override ValueTuple Visit(Procedure procedure, Type? expectedType)
        {
            if (procedure.Body is not null)
            {
                throw new NotImplementedException();
            }
            else
            {
                throw new NotImplementedException();
            }

            return default;
        }

        public override ValueTuple Visit(Block block, Type? expectedType)
        {
            block.ResolvedType = IfSameKindOrDefault(expectedType, DefaultVoidType);
            foreach (var expression in block.Expressions)
                expression.Accept(this, null);
            return default;
        }

        public override ValueTuple Visit(Return @return, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(If @if, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(Call call, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(GenericInstantiation genericInstantiation, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(ParenthesisedExpression parenthesisedExpression, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(FieldAccess fieldAccess, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(Builtin builtin, Type? expectedType)
        {
            throw new NotImplementedException();
        }

        public override ValueTuple Visit(BuiltinArray builtinArray, Type? expectedType)
        {
            throw new NotImplementedException();
        }
    }
}
