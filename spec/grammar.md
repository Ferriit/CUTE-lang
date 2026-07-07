# CUTE grammar

## Language model

CUTE is a programming language that compiles directly to C++.

Unless explicitly stated otherwise:

* C++ semantics apply.
* Classes and structs behave like their C++ equivalents.
* Constructors and destructors behave like C++ constructors and destructors.
* Operator behavior follows C++ rules.
* Memory management follows C++ rules.
* Generated C++ preserves the behavior of the original CUTE program.

This document only describes CUTE-specific syntax.

---

# Miscellaneous

* All statements must end with `;`.
* All operators use the same priority as C23 (ISO/IEC 9899:2024).
* CUTE is case-sensitive.
* Whitespace is ignored except inside string literals.
* Comments begin with `#` and continue until the end of the line.
* C++ behavior applies unless CUTE syntax differs.

Example:

```cute
# This is a comment

var x: int = 5;
```

---

# Keywords

The following identifiers are reserved:

```text
namespace
class
struct
enum
func
var
const
public
private
return
if
else
for
while
run
jump
true
false
```

---

# Identifiers

Identifiers:

* may contain letters, numbers, and `_`
* may not begin with a number
* are case-sensitive

Valid:

```cute
player
Player
player2
_internal
```

Invalid:

```cute
2player
my-variable
```

---

# Variables

Variables are declared using the `var` keyword.

Syntax:

```text
var name [: type] [= value];
```

A variable must have either:

* a type declaration
* an initial value
* both

Valid:

```cute
var x: int = 5;
var y = 5;
var z: int;
```

Invalid:

```cute
x = 5;
int y = 5;
var z;
```

When the type is omitted, CUTE uses type inference.

Example:

```cute
var value = 10;
```

Equivalent:

```cute
var value: int = 10;
```

---

# Constants

Constants use the `const` keyword.

Constants must always have an explicit type.

Syntax:

```text
const name: type = value;
```

Valid:

```cute
const max_players: int = 16;
const title: string = "CUTE";
```

Invalid:

```cute
const max_players = 16;
```

Constants cannot be reassigned.

---

# Types

CUTE supports normal C++ types.

Examples:

```cute
var health: int;
var name: string;
var active: bool;
```

User-defined types are created with:

* `struct`
* `class`
* `enum`

Example:

```cute
var player: Player;
```

---

# Functions

Functions use the `func` keyword.

Syntax:

```text
func name(parameters): return_type {
    statements
}
```

Example:

```cute
func add(a: int, b: int): int {
    return a + b;
}
```

Parameters require explicit types.

Invalid:

```cute
func add(a, b): int {
}
```

Functions without return values use `void`.

Example:

```cute
func print_message(): void {
    print("Hello");
}
```

---

# Return statements

Values are returned using `return`.

Example:

```cute
func get_number(): int {
    return 10;
}
```

Void functions may return without a value.

Example:

```cute
func stop(): void {
    return;
}
```

---

# Namespaces

Namespaces group declarations.

Syntax:

```text
namespace name {
    declarations
};
```

Example:

```cute
namespace math {
    func add(a: int, b: int): int {
        return a + b;
    }
};
```

Members are accessed using `.`.

Example:

```cute
math.add(1, 2);
```

Namespaces follow C++ namespace behavior.

---

# Structs

Structs behave like C++ structs.

Syntax:

```text
struct name {
    members
};
```

Example:

```cute
struct Vector {
    var x: int;
    var y: int;

    func length(): float {
        return 0.0;
    }
};
```

Structs support:

* member variables
* member functions
* constructors
* destructors
* inheritance
* operator overloading

All behavior follows C++ rules.

---

# Classes

Classes behave like C++ classes.

Syntax:

```text
class name {
    members
};
```

Example:

```cute
class Player {
    private var health: int;

    public func damage(amount: int): void {
        health -= amount;
    }
};
```

Classes support:

* constructors
* destructors
* inheritance
* polymorphism
* operator overloading
* member access

All behavior follows C++ rules.

---

# Visibility

Members may be marked as:

```text
public
private
```

Example:

```cute
class Player {
    private var health: int;

    public func get_health(): int {
        return health;
    }
};
```

Visibility behaves like C++.

---

# Constructors and destructors

Constructors use the class name.

Example:

```cute
class Player {
    Player() {
    }
};
```

Destructors use `~`.

Example:

```cute
class Player {
    ~Player() {
    }
};
```

They behave exactly like C++ constructors and destructors.

---

# Enums

Enums behave like C++ enums.

Example:

```cute
enum State {
    Idle,
    Running,
    Dead
};
```

Values may be assigned manually.

Example:

```cute
enum Error {
    None = 0,
    Failed = 1
};
```

---

# Arrays

Arrays use C++ array syntax.

Example:

```cute
var numbers: int[10];
```

Access:

```cute
numbers[0];
```

Array behavior follows C++ rules.

---

# Member access

Members are accessed using `.`.

Example:

```cute
player.health;
player.attack();
```

Member access can be chained.

Example:

```cute
game.player.weapon.damage();
```

---

# Operators

Operators behave like C++ operators.

Supported operators include:

## Arithmetic

```text
+
-
*
/
%
```

## Comparison

```text
==
!=
<
>
<=
>=
```

## Logical

```text
&&
||
!
```

## Increment and decrement

```text
++
--
```

## Assignment

```text
=
+=
-=
*=
/=
```

Operator precedence follows C23.

---

# Conditions

## If statements

Syntax:

```text
if (condition) {
}
else {
}
```

Example:

```cute
if (value == 10) {
    print("ten");
}
else {
    print("not ten");
}
```

---

# Loops

## Classic for loop

CUTE supports C-style for loops.

Example:

```cute
for (var i = 0; i < 10; i++) {
    print(i);
}
```

---

## Range for loop

CUTE adds a range-based loop syntax.

Example:

```cute
for (var i <- range(0, 10)) {
    print(i);
}
```

The generated behavior follows C++ range loop behavior.

---

## While loop

Example:

```cute
while (true) {
    print("loop");
}
```

---

# Run blocks

`run` creates an unconditional loop.

Example:

```cute
run {
    print("running");
}
```

Equivalent behavior:

```cpp
while (true) {
    std::cout << "running" << std::endl;
}
```

---

# Jump statements

`jump` transfers execution to a label.

Example:

```cute
jump end;

end:
```

Labels are written as:

```text
identifier:
```

---

# Imports

Imports are not written inside CUTE source files.

Dependencies are managed using `cute.toml`.

Example:

```toml
[source]
cpp_includes = ["<string>", "<iostream>"]

[dependencies]
library = "1.0"
```

The build system handles translation into C++ includes and dependencies.

---

# C++ translation model

CUTE syntax maps directly to equivalent C++.

Example:

CUTE:

```cute
func add(a: int, b: int): int {
    return a + b;
}
```

C++:

```cpp
int add(int a, int b) {
    return a + b;
}
```

---

CUTE:

```cute
var x: int = 5;
```

C++:

```cpp
int x = 5;
```

---

# Complete example

```cute
namespace program {
    const version: int = 1;

    class Player {
        private var health: int;

        Player() {
            health = 100;
        }

        public func damage(amount: int): void {
            health -= amount;
        }
    };

    func main(): int {
        var player = Player();

        if (true) {
            player.damage(10);
        }

        return 0;
    }
};
```
