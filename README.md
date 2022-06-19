# 🦊 Fox 

Fox is a statically typed, stack-based concatenative programming language.  
It is designed to balance minimalism and ergonomics.

Design was inspired by Kitten, Rust, Haskell and F#.

---

## Features

- ### Concatenative
  The main operation in Fox is function composition.

- ### Stack Based
  Functions work by operating on the stack. 
  Code is mostly written in point free style, but Fox also support named binding.

- ### Statically Typed
  The type checking happens at compile time. This ensures code correctness.
  Fox typed include primitives, sum types and product types.

- ### Standalone interpreter
  The standalone interpreter allows `.fox` files to be run directly from the command line  
  ```$> fox my_script.fox```

- ### Rust interface
  Fox is designed to be easily used and extendable from Rust.


---

## Examples

- Hello world
```
"Hello world!" print                # `print` consume a `String` from the top of the stack and prints it
```  

- Greeter
```
def greet = 
    @name                           # pop the top stack value and store it in `name`
    ["Hello, ", name, "!"] concat   # concat convert a `String List` to a `String`
    print                           # `print` consume a `String` from the top of the stack and prints it

"What is your name? " prompt        # `prompt` push a `String` on the stack from user input
greet                               # `greet` consume a `String` from the top of the stack and greets the user
```