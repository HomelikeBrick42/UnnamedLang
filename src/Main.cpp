#include "SourceLocation.hpp"
#include "Token.hpp"
#include "Lexer.hpp"
#include "CompileError.hpp"

#include <iostream>

using namespace Langite;

int main(int, char**) {
    Lexer lexer{ "test.lang", R"###(
const foo = 5

const do_something = func(a: int, b: int): int {
    return a + b
}

const greet_user = proc(): void {
    print("What is your name: ")
    name: string <- read_line_from_console(stdin)
    print("Hello, %\n", name)
}

const int_or_bool = func(condition: bool): type {
    if condition {
        return int
    } else {
        return bool
    }
}

const identity[T: type] = func(value: T): T {
    return value
}

bar: int <- identity[int](1 + 2 * 3)
baz: string <- identity("hello")

some_variable: int_or_bool(true)
)###" };
    try {
        while (true) {
            Token token = lexer.NextToken();
            std::cout << TokenKind_ToString(token.Kind) << std::endl;
            if (token.Kind == TokenKind::EndOfFile)
                break;
        }
    } catch (CompileError e) {
        std::cerr << e.Location.Filepath << ':' << e.Location.Line << ':' << e.Location.Column
                  << ": " << e.Message << std::endl;
    }
    return 0;
}
