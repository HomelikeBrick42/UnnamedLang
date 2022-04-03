#include "SourceLocation.hpp"
#include "Token.hpp"
#include "Lexer.hpp"
#include "CompileError.hpp"
#include "Ast.hpp"
#include "Parsing.hpp"

#include <iostream>

using namespace Langite;

int main(int, char**) {
    std::string_view filepath = "test.lang";
    std::string_view source   = R"###(
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

test: Array[int, 5]
test@0 <- 5
1 + 2 * 3 -> test@3
test@(the_length-1) <- the_length

const the_length = test.length // test.length is a constant
)###";
    try {
        std::unique_ptr<AstFile> file = ParseFile(filepath, source);
        (void)file;
    } catch (CompileError e) {
        std::cerr << e.Location.Filepath << ':' << e.Location.Line << ':' << e.Location.Column
                  << ": " << e.Message << std::endl;
    }
    return 0;
}
