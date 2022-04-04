#pragma once

namespace Langite {

    struct TypeVoid;
    struct TypeType;
    struct TypeInteger;
    struct TypeFloat;
    struct TypeString;
    struct TypeBool;

    class TypeVisitor {
    public:
        virtual ~TypeVisitor()                = default;
        virtual void Visit(TypeVoid& type)    = 0;
        virtual void Visit(TypeType& type)    = 0;
        virtual void Visit(TypeInteger& type) = 0;
        virtual void Visit(TypeFloat& type)   = 0;
        virtual void Visit(TypeString& type)  = 0;
        virtual void Visit(TypeBool& type)    = 0;
    };

    struct Type {
        virtual ~Type()                           = default;
        virtual void Accept(TypeVisitor& visitor) = 0;
    };

    struct TypeVoid: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

    struct TypeType: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

    struct TypeBool: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

    struct TypeInteger: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

    struct TypeFloat: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

    struct TypeString: Type {
        virtual void Accept(TypeVisitor& visitor) {
            visitor.Visit(*this);
        }
    };

}
