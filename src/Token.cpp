#define KEEP_TOKEN_KINDS
#include "Token.hpp"

namespace Langite {

    std::string TokenKind_ToString(TokenKind kind) {
        switch (kind) {
#define TOKEN_KIND(kind, string) \
    case TokenKind::kind:        \
        return string;
#define TOKEN_KIND_KEYWORD(kind, string) \
    case TokenKind::kind:                \
        return string;
#define TOKEN_KIND_SINGLE(kind, string, character) \
    case TokenKind::kind:                          \
        return string;
#define TOKEN_KIND_DOUBLE(kind, firstCharacter, doubleKind, secondCharacter) \
    case TokenKind::kind:                                                    \
        return std::string(1, firstCharacter);                               \
    case TokenKind::doubleKind:                                              \
        return std::string(1, firstCharacter) + secondCharacter;
#define TOKEN_KIND_TRIPLE(                                                        \
    kind, firstCharacter, doubleKind, secondCharacter, thirdKind, thirdCharacter) \
    case TokenKind::kind:                                                         \
        return std::string(1, firstCharacter);                                    \
    case TokenKind::doubleKind:                                                   \
        return std::string(1, firstCharacter) + secondCharacter;                  \
    case TokenKind::thirdKind:                                                    \
        return std::string(1, firstCharacter) + thirdCharacter;
            TOKEN_KINDS
#undef TOKEN_KIND
#undef TOKEN_KIND_KEYWORD
#undef TOKEN_KIND_SINGLE
#undef TOKEN_KIND_DOUBLE
#undef TOKEN_KIND_TRIPLE
        }
    }

}
