# TODO

Language features to implement from [the specification](spec/grammar.md).

## Imports

- [x] Parser: parse angle-bracket and quoted `using` paths.
- [ ] Codegen: emit the corresponding C++ includes.

## Namespaces

- [ ] Parser: parse namespace declarations and nested declarations.
- [ ] Codegen: emit C++ namespaces.

## Variables and constants

- [ ] Parser: parse explicitly typed declarations, optional initializers, and
      constants.
- [ ] Codegen: emit declarations and initializers with the right C++ types and
      constness.

## Types

- [ ] Parser: parse built-in, user-defined, pointer, array, and aggregate
      types (`list`, `map`, `tuple`, `optional`, `result`).
- [ ] Codegen: map CUTE types to C++ types, including required standard-library
      and `cute::result` support.

## Functions

- [ ] Parser: parse functions, typed parameters, return types, and bodies.
- [ ] Codegen: emit function declarations and definitions, parameters, and
      return types.

## Structs and classes

- [ ] Parser: parse members, inheritance, and nested functions and variables.
- [ ] Codegen: emit structs, classes, inheritance, and members.

## Visibility

- [ ] Parser: parse `public` and `private` member modifiers.
- [ ] Codegen: emit equivalent C++ access sections.

## Constructors and destructors

- [ ] Parser: parse constructors and `~` destructors within aggregates.
- [ ] Codegen: emit C++ constructors and destructors.

## Enums

- [ ] Parser: parse variants with optional assigned values.
- [ ] Codegen: emit C++ enum declarations and values.

## Expressions and operators

- [ ] Parser: parse literals, identifiers, calls, member and array access,
      unary and binary operators with the specified precedence.
- [ ] Codegen: emit equivalent C++ expressions while preserving precedence
      and behavior.

## Arrays

- [ ] Parser: parse array type sizes, array access, and array literals.
- [ ] Codegen: emit C++ array declarations and access.

## Returns and blocks

- [ ] Parser: parse blocks, return-with-value, and bare return statements.
- [ ] Codegen: emit C++ blocks and return statements.

## Conditional statements

- [ ] Parser: parse `if`, `else if`, and `else` branches.
- [ ] Codegen: emit equivalent C++ conditionals.

## Loops

- [ ] Parser: parse classic `for`, range `for`, and `while` loops.
- [ ] Codegen: emit equivalent C++ loops.

## Run blocks

- [ ] Parser: parse `run` blocks.
- [ ] Codegen: emit the specified unconditional loop.

## Labels and jumps

- [ ] Parser: parse labels and `jump` statements.
- [ ] Codegen: emit C++ labels and `goto` statements.

