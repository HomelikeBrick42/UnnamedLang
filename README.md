# Unnamed Lang

This is an expression based-lang with newlines instead of semicolons

## Variables

#### Examples

```c
foo: int
bar: int

// assignments can happen both ways and are always evaluated left-to-right

foo <- 5

1 + 2 * 3 -> bar

// swap foo and bar
foo <-> bar
```

<br/>

## Constants

#### Examples

```c
const foo = 5
const bar = foo * 56
```

## Functions

- Functions have no side effects and will only have access to their parameters and other constants
- Given the same input they will always produce the same output
- They also can be run at compile time for the initalization of other constants or the types of variables
- You cannot call procedures from inside of a function

### Examples

```c
const add = func(a, b: int): int {
	return a + b
}

const five_plus_six = add(5, 6)
```

```c
const int_or_bool = func(condition: bool): type {
	if condition {
		return int
	} else {
		return bool
	}
}

some_variable: int_or_bool(true)
```

```c
const some_constant = 5

a: int <- some_constant

const get_some_constant = func(): int {
	// this is not allowed because functions only have access to their parameters and local variables
	// return a

	// this is allowed because it is a constant and will never change
	return some_constant
}
```

<br/>

## Procedures

Procedures can have side effects and dictate the main control flow of the program because they can have access external input/output

### Examples

```c
const greet_user = proc(): void {
	print("What is your name: ")
	name: string <- read_line_from_console(stdin)
	print("Hello, %\n", name)
}
```

## Wildcard

A wildcard can be used in the place of a type (and maybe pattern matching in the future) and it will let the compiler infer the type from usage

### Examples

```c
foo: _ <- 5 // type will be infered to `float` from further usage
bar: float <- foo

// this will error because `string` is not compatable with the already infered type `float`
// foo <- "some string"

// the type will be `untyped_integer`
// but because there is no usage that it can infer the type from
// it will be given the type `int`
baz: _ <- 1 + 2 * 3
```
