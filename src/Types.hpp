#pragma once

#include <variant>
#include <memory>

namespace Langite {

    struct TypeVoid;
    struct TypeType;
    struct TypeInteger;
    struct TypeFloat;
    struct TypeString;
    struct TypeBool;
    struct TypeArray;
    struct TypePlaceholder;

    using Type = std::variant<TypeVoid,
                              TypeType,
                              TypeInteger,
                              TypeFloat,
                              TypeString,
                              TypeBool,
                              TypeArray,
                              TypePlaceholder>;

    struct TypeVoid {};

    struct TypeType {};

    struct TypeBool {};

    struct TypeInteger {};

    struct TypeFloat {};

    struct TypeString {};

    struct TypeArray {
        std::shared_ptr<Type> InnerType;
        size_t Length;
    };

    struct TypePlaceholder {};

}
