# CUTE-lang
## "C Unified Transpiled Environment Language"

C.U.T.E is a scripting language designed for simple integration into C environments, hence the abbreviation.

Instead of compiling to C-interopable bytecode, like languages such as Lua, CUTE compiles directly to C and supports more extensible and advanced project and management files.

### The problems
1. **Speed.**  Most integratable scripting languages (like Lua and Python) are either integratable due to compilation to bytecode or by parsing like normally.
2. **Ease of integration.**  Most scripting languages are integrated by including a parser for bytecode or source code, this causes bulk in the final binary and slows down the build process.

### The solutions
1. **Speed.**  CUTE is integrated either by compiling to a C script or a C header file that can be included by other programs. This keeps the speed of C while not including a bulky runtime.
2. **Ease of integration.**  CUTE is easily integrated as all that is necessary is a `cute.toml` file and the `cute` compiler/utility on the host system.

## Why does CUTE exist?
CUTE is built to be integrated within a game engine environment

