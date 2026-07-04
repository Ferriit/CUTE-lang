# CUTE-lang
## "C Unified Transpiled Environment Language"

C.U.T.E is a scripting language designed for simple integration into C environments, hence the abbreviation.

Instead of compiling to C(++)-interopable bytecode, like languages such as Lua, CUTE compiles directly to C and supports more extensible and advanced project and management files.

## Why does CUTE exist?
CUTE is built to be integrated within a game engine environment and provides a lightweight alternative to traditional scripting solutions.

Instead of embedding a full interpreter or virtual machine, CUTE generates native C-compatible code that can be compiled directly into the host application. This allows engine developers to maintain full control over memory, performance, and execution flow while still providing a flexible scripting layer for gameplay logic.

## Design goals of CUTE
1. **Minimal runtime overhead.** - No embedded interpreter or virtual machine required, only the transpiler.
2. **Native integration.** - Outputs native C++ source code or header files.
3. **Customizable execution model.** - Can use a standard `main` entry point or use a different C stub for execution.

## Project configuration
CUTE projects are defined using a `cute.toml` file, which specifies source directories, output targets, C stub, and C compiler settings.

example:
```toml
[project]
name = "test"
version = "0.1.0"

[source]
dir = "src/"
entrypoint = "src/main.cute"
stub = "standard" # Can be any C++ file or "standard"

[build]
output = "build/"
```

## Current status
CUTE is still in early development. The language design, compiler behavior, and runtime interface are subject to change as the project evolves.


## Specification and Documentation
Full language specification and documentation can be found in `spec/` and `docs/` respectively.

## Closing note
CUTE is an experiment in treating scripting not as a separate runtime, but as a compile-time extension of C itself. The goal is to keep systems simple, fast, and fully transparent from source code to execution.
