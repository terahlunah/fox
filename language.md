# Language overview

---

## Comments

Comments in Fox use the `#` symbol

`# This is a comment`

Comments will be used throughout this page to add context to a line of code or to represent the program stack.


---

## Literals

There is 4 types if literals in Fox:

- Integers of type `Int`
    ```
    42
    ```
- Floating-point numbers of type `Float`
    ```
    3.14
    ```
- Characters of type `Char`
    ```
    'a'
    ```
- Strings of type `String`, which is a type alias for `[Char]`
    ```
    "Hello\n"
    # Equivalent to ['h','e','l','l','o','\n']
    ```

---

## Tuples

Tuples are statically sized collections of heterogeneously typed elements.

They are defined by a comma separated list of elements, surrounded by parenthesis `( )`.
The comma after the last element is optional.
```
(1, 2)
```
Tuples have the type `(a, b, ...)` where `a`, `b` are generic types.  
The type `(a, b)` is an alias for the internal type `Tuple2 a b`, same for `(a, b, c)` and `Tuple3`, etc.
Tuples can have up to 9 elements.

For example:
- `(1, 3.14)` has type `(Int, Float)`
- `("Hello", (1))` has type `([Char], (Int))`


---

## Lists

Lists are dynamically sized collections of homogeneously typed elements.

They are defined by a comma separated list of `key:value` pairs, surrounded by square brackets `[ ]`.
The comma after the last element is optional.
```
[1, 2, 3]
```

The compact form `[1, 2, 3]`  is desugared as:

```
List.empty
1 List.push
2 List.push
3 List.push
```

Lists have the type `[a]` (an alias for the internal type `List a`) where `a` is a generic type.


---

## Tables

Tables are key-value stores.

They are defined by a comma separated list of elements of the same type, surrounded by square brackets `[ ]`.
The comma after the last element is optional.
```
[
  "a" : 1,
  "b" : 2
]
```

The compact form `["a" : 1, "b" : 2]`  is desugared as:

```
Table.empty
"a" 1 Table.set
"b" 2 Table.set
```

Tables have the type `[a:b]` (an alias for the internal type `Table a b`) where `a` and `b` are a generic type.



---

## Functions

You can define new functions with the `def` keyword

```
def square = dup *

4 square 
# 16
```


Since manipulating the stack is central in Fox, it comes with a few builtin functions for stack manipulation.
Here are some the most common ones:

- `dup` duplicate the value at the top of the stack  
  ```
  1
  # stack: [1]
  dup
  # stack: [1 1]
  ```

- `drop` remove the value at the top of the stack
  ```
  1 2
  # stack: [1 2]
  drop
  # stack: [1]
  ```

- `swap` swap the 2 values at the top of the stack
  ```
  1 2
  # stack: [1 2]
  swap
  # stack: [2 1]
  ```

## Typing

By default, functions types will be inferred as generically as possible.  
This means a function like `def square = dup *` will accept any type for which `dup` and `*` is implemented.

To restrict a function to a specific type, it is possible to add type annotation in parentheses:  
```
def square (Float -> Float) = 
    dup *
```

## Anonymous functions

Anonymous functions can be created on the fly by surrounding a chunk of code with `{}`

```
[1, 2] { 3 + } map
# [4, 5]
```

Named functions can also be passed by value by wrapping them in `{}`
```
def add3 = 3 +

[1, 2] { add3 } map
# [4, 5]
```

Since this is a really common pattern, a specialized `\function` syntax can be used for this purpose
```
def add3 = 3 +

[1, 2] \add3 map
# [4, 5]
```


---

## Conditionals

Using the `?` and `??` functions: 
- `?` for working with values: `def ? (Bool t t -> t) = ...`
- `??` for lazy evaluation: `def ?? (Bool (-> t) (-> t) -> t) = ...`

```
def positive? = 0 >

def printSign = positive? "positive" "negative" ? print

# "positive"
1 printSign

# "negative"
-1 printSign
```

Using the `then else` special syntax

```
def printSign = 
  positive? then { 
    "positive"
  } else {
    "negative"
  } 
  print
```
This `then { a } else { b }` syntax is desugared to `{ a } { b } ??` 


---

## Locals

Values can be popped from the stack and pushed to named locals for convenience.
This is useful for working with complex functions.

The local store operator `->` is used for that purpose.

`-> x` will pop the top value of the stack and create a local function named `x` which can be called in the rest of the function to push the value of `x` on the stack.

- point free
    ```
    def square = dup *
    def add3 = + +
    ```  

- with locals
    ```
    def square = 
        -> x
        x x *
    def add3 = 
        -> a,b,c
        a b c +
    ```
    `-> a,b,c` will expand to `-> a -> b -> c`

For these simple examples, the point free version is better.
But complex mathematical formulas are often more readable with locals.


---

## Ref

`Values in Fox are immutable, and always manipulated "by value".`

`Ref` safely encapsulate a reference to a value, and makes it possible to share a mutable value.  
When a `Ref` is copied, both the new `Ref` and the original `Ref` reference the same value.
The is Fox equivalent of a smart pointer.

Compare the following code without `Ref`
```
0 -> x                  # type: Int

x print                 # 0

x { 1 + } apply
print                   # 1

x print                 # 0
```
to this one with `Ref`
```
0 ref -> x              # type: Int Ref

x read print            # 0

x { 1 + } map
read print              # 1

x read print            # 1
```

`Ref` are manipulated with the functions `read`, `write`, `map` and `iter`.


---

## Custom types

TODO

## Examples

Multiple constructors with no fields
```
# Single line syntax
type Bool = False | True

# Multi line syntax
type Bool {
    | True
    | False
}
```

Multiple constructors with different numbers of fields
```
type Option o = None | Some o
```

Single constructor
```
type Pair a b = Pair a b
```

Single record constructor
```
type Vec2 t = Vec2 {x: t, y: t}
```


---

## Pattern matching

TODO

```
2 Some
match {
    | None: none
    | Some: print     # will print 2
}

```


---

## Modules

TODO

---

## Traits

TODO

---

## Seq (Iterable)