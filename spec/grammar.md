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

var x: i32 = 5;
```

---

# Keywords

The following identifiers are reserved:

```text
using
namespace
class
inherits
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

So are these variable types:
```text
void
bool
iptr
uptr
i8
i16
i32
i64
u8
u16
u32
u64
f32
f64
char
c16
c32
err

string
stringview
list
map
tuple
optional
result
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

A variable must have:

* a type declaration
* an initial value

Valid:

```cute
var x: i32 = 5;
var y: f64 = 5.0;
var z: i64;
```

Invalid:

```cute
x = 5;
i32 y = 5;
var z;
```

# Constants

Constants use the `const` keyword.

Constants must always have an explicit type.

Syntax:

```text
const name: type = value;
```

Valid:

```cute
const max_players: i32 = 16;
const title: string = "CUTE";
```

Invalid:

```cute
const max_players = 16;
```

Constants cannot be reassigned.

---

# Types

The following is a list over all variable types and what C++ types they match to:

```text
void - void
bool - bool
iptr - intptr_t
uptr - uintptr_t
i8 - int8_t
i16 - int16_t
i32 - int32_t
i64 - int64_t
u8 - uint8_t
u16 - uint16_t
u32 - uint32_t
u64 - uint64_t
f32 - float
f64 - double
char - char
c16 - char16_t
c32 - char32_t
err - uint8_t

string - std::string
stringview - std::string_view
list T - std::vector<T>
map K V - std::unordered_map<K, V>
tuple T1 T2 T3 T4 [...] std::tuple<T1, T2, T3, T4, ...>
optional T - std::optional<T>
result T - cute::result<T>
```

`cute::result<T>` is defined in C++ as:

```c++
namespace cute {
template <typename T>
class result {
    std::optional<T> value;
    uint8_t err = 255;

public:
    bool exists() const {
        return value.has_value();
    }

    uint8_t get_err() const {
        return err;
    }

    T& unwrap() & {
        if (!value)
            cute::panic("Tried unwrapping non-existent value");

        return *value;
    }

    const T& unwrap() const & {
        if (!value)
            cute::panic("Tried unwrapping non-existent value");

        return *value;
    }

    T&& unwrap() && {
        if (!value)
            cute::panic("Tried unwrapping non-existent value");

        return std::move(*value);
    }

    T expect(std::string_view message) const {
        if (!value)
            cute::panic(std::string(message));

        return *value;
    }

    void set_error(uint8_t e) {
        value.reset();
        err = e;
    }

    void set_val(T val) {
        value = std::move(val);
        err = 255;
    }
};
}
```

Examples:

```cute
var health: i32;
var name: string;
var active: bool;
```

However, some have been remapped with other keywords
```cute
var lines: list string; # Same as "std::vector<std::string> list;"
var lines_arr: string[50]; # Same as "string lines_arr[50];"
```

User-defined types are created with:

* `struct`
* `class`
* `enum`

They also follow the same syntax as C++

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
class name [inherits] [other class] {
    members
};
```

Example:

```cute
class Player inherits Entity {
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
>>
<<
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
>>=
<<=
```

Operator precedence follows C23.

---

# Conditions

## If statements

Syntax:

```text
if (condition) {
}
else if (condition) {
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
for (var i: int = 0; i < 10; i++) {
    print(i);
}
```

---

## Range for loop

CUTE adds a range-based loop syntax.

Example:

```cute
for (var i: int -> range(0, 10)) {
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

Imports are handled through the `using` keyword and a name that works the same as in C++.

Example:

```cute
using <iostream>
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
