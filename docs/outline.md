# Syntax
To avoid complexity with parsing, this language uses exclusively s-expressions.

```
(module . [main])
(let main (Console.print "Hello world!"))
```

## Identifiers
Identifiers can be exactly one of the following

- Numeric literal. `1.4`, `0`, and `-82` are all examples of numeric literals. There is no distinction between `Int`s and `Float`s for 2 reasons

    1. The context always gives us enough information to figure out the type of number
    2. The type system can facilitate more types of numbers than "Whole" and "Not Whole".

    If you're evaluating in a `repl` and you don't provide any type information it will default to either `Int` or `Float`, depending if you put decimals in the float.

- String literal. `"Hello World"`, `""`, and `"this contains\ntwo lines"` are examples. They work the same as any other language.

- Bindings. You probably understand this as "variable" names, but due to the semantics of the language that would be a misnomer. When we create a value, say with `(let x 1)`, we're *binding* the name `x` to the *value* `1`, so that when we use `x`, we can *identify* the underlying value of `1`. For this reason, we call `x` a "Binding identifier". These identifiers may have any name, so long as they fulfil a few rules

    1. They must start with either `_` or an alphabetic character.
    2. They must be followed exclusively by `_` or any alpha*numeric* character.
    3. They *can* end with a `?`.

    This last one is a little atypical, but it makes certain functions read nicer. You'll experience it more as you use different APIs.

- Types. Types work the same as bindings, however they must start with an uppercase alphabetic character and cannot end with `?`.

- Modules. Modules work the same as bindings, however they must start with an uppercase alphabetic character and cannot end with `?`.

- Keywords. These can be one of the following

    - `if`
    - `module`
    - `when`
    - `is`
    - `as`
    - `let`

## Symbols
3 sets symbols are used in this language, `()`, `[]`, and `{}`.

- `()` is for logical grouping- evaluate what's inside before anything else. It works the same as in mathematics.
- `[]` is used for declaring a list like so `(let list [1, 2, 3])`.
- `{}` is used to declare a `record`. This works just like `JSON`, or `struct`s from other languages.

    ```
    (let player {position {x 0, y 0}, health 0, power 9000})
    ```

There is one additional symbol used in the language- `\`. This is used to declare lambdas, for example, the following 2 definitions are semantically identical

```
(let (solve_quadratic a b c) ...)
(let solve_quadratic (\(a b c) ...))
```

## Modules
The beginning of every file begins with a module statement. This is a special case of a more general syntax

```
(module ModuleName [<interface>]
    (let ... ...)
    (let ... ...)
    ...)
```

However, it's typical to use files to organise the module system so the following is semantically the same.

```
(module . [<interface>])
(let ... ...)
(let ... ...)
...
```

Where this is located in a file called `ModuleName.lang`. The `interface` portion is just a list of things which this module exposes to other modules. For example, we may have the following module

```
(module Math []
    (let (sqrt x) (sqrt_newton x 1))
    (let (sqrt_newton x guess) ...))
```

Ideally, you would want other modules to be able to access the `sqrt` function, but not the specific implementation details (that is, that the `sqrt` function is implemented using Newton's method). If we assumed that all functions could be accessed, it would be impossible to hide this implementation detail without nesting `sqrt_newton` inside `sqrt` like so-

```
(module Math []
    (let (sqrt x)
        (let (sqrt_newton x guess) ...))
        (sqrt_newton x 1))
```

Which would work, but if we needed `sqrt_newton` in any other part of the module (like a benchmark of various `sqrt` implementations) we can't. This is where interfaces come into play.

```
(module Math [sqrt]
    (let (sqrt x) (sqrt_newton x 1)
    (let (sqrt_newton x guess) ...)))
```

Now we can use `Math.sqrt` in other modules, but not `Math.sqrt_newton`. In our previous examples neither would have worked actually, since `(module Math [] ...)` means "declare a module called `Math` which doesn't expose anything" which is pretty useless.

## Imports
You can import functions and values from other modules like so

```
(import Game)
```

And access them throughout the rest of the script with `Game.member` where `member` is anything declared in the `Game` module. Importing modules is seldom as simple as just importing one library though. Usually, you want to import not only things from a single module, but also things from that modules' submodules. Typically, this is done by specifying the full "directory" of everything you want to import. A good example is the `XNA` game framework in `C#`. For example, imports in `MonoGame` (a descendant of `XNA`) can typically look like this

```
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using Microsoft.Xna.Framework.Media;
using Microsoft.Xna.Framework.Input.Touch;
using Microsoft.Xna.Framework.Content;
```

Which in our language would look something like this

```
(import Microsoft.Xna.Framework)
(import Microsoft.Xna.Framework.Graphics)
(import Microsoft.Xna.Framework.Input)
(import Microsoft.Xna.Framework.Media)
(import Microsoft.Xna.Framework.Input.Touch)
(import Microsoft.Xna.Framework.Content)
```

Which is obviously quite redundant. Luckily, text is a very nice format to describe hierarchical data such as this, so there's a special syntax sugar for just that.

```
(import Microsoft.Xna.Framework.(self, Graphics, Input.(self, Touch), Media, Content)
```

Where `self` is a keyword meaning "the module this is inside of", so the first `self` refers to `Microsoft.Xna.Framework` and the second to `Microsoft.Xna.Framework.Input`.

Even though it tends to be bad practice, it's quite convenient for development to import everything from a module so there is a special syntax for that too

```
(import Microsoft.Xna.Framework.(..))
```

Note that `..` doesn't import `self`, so to be able to also use `Microsoft.Xna.Framework.anything` in this case you need to include `self` in the import, although there really isn't any reason to do this.

- Note: This is only bad practice because of namespace pollution. It makes it less clear where functions, values, etc. come from, and it makes imports order dependent.

We can also rename modules using the `as` keyword. Let's say we wanted to refer to `Microsoft.Xna.Framework` as `FW`. We could write

```
(import Microsoft.Xna.Framework as FW)
```

This also works in nested definitions

```
(import Microsoft.Xna.Framework.(self as FW, Graphics as G, Input.(self as I, Touch as TI), Media as M, Content as C)
```

Whether this is a good way to name your imports is up for discussion, but you can name them whatever makes sense for your project.

## Statements
Each module is a list of statements.
Statements can be either

- value definitions
- function definitions (which is the same as value definitions since functions are first class)
- `import` statements
- `module` declarations

Function definitions look almost the same as value definitions, and both use the `let` keyword.

```
(let a 1)
(let b 3)
(let c 2)
(let (solve_quadratic a b c)
    (/  (+  (- 0 b)
            (sqrt (-    (* b b)
                        (* 4 (* a c)))))
        (* 2 a)))
```

In this case, the first 3 lines are declaring values `a`, `b`, and `c`. The rest is declaring a function `solve_quadratic` with arguments `a`, `b`, and `c`.
The general syntax is

```
(let value_name <body>)
```

or

```
(let (function_name <optional arguments>+) <body>)
```


- Note: Incidentally, variable shadowing is allowed, although you will get an editor suggestion to change it. This means that the `a`, `b`, and `c` in the definition of `solve_quadratic` refer to the arguments, not the above definitions.

## Types
You can declare the type of a function / value like so

```
(let :Int x 0)
(let :(Fn Float Bool) (less_than_2 x) (< x 2))
(let :(Fn Float Float) (sqrt x) (..))
```

Where `Fn a b` means "the type of a function which takes an `a` and returns a `b`.
Multivalued functions have weirder type signatures. Let's just say we could use `Fn` again.

```
(let :(Fn Float Float Float) (add a b) (+ a b))
```

This makes sense to us, but to the compiler it's nonsense. What is `Fn`? Does it take 2 type arguments (the argument type and the return type), or 3 (2 argument types and a return type)? The number of inputs expected for a type like this known as its *arity* and functions which can accept a different number of arguments are called *variadic* functions. Note that I'm saying `functions` even though I'm referring to a type, the same principle applies. We could add this as a special construct to the language, and indeed many languages do, but for various reasons this language does not support variadic functions nor variadic types.

The upshot is, if you want to declare a function with multiple arguments, you'll have to use the `->` syntax sugar

```
(let :(Float -> Float -> Float) (add a b) (+ a b))
```

This gets de-sugared into

```
(let :(Fn Float (Fn Float Float)) (add a b) (+ a b))
```

Which may feel weird if you're not familiar with partial application, but it does compile.

# Semantics
## Scope
You can define a value like so

```
(let name value)
```

Now the value `name` can be accessed by any other definition declared in the same file. If you declare a value within another declaration, such as

```
(let (outer)
    (let inner 0)
    ...)
```

`inner` can only be used by other definitions / expressions within `outer`, but not by any other definition with the same scope or higher than `outer`.

## Immutability
All values are immutable. This means that once a value is set, there's nothing you can do to change that. On obvious question is "well how do I do anything" and that's a good question. When thinking about code through a lense of immutability, we don't think of values as changing, we think of functions as taking the old thing and giving the new thing. A good example is `map`-

```
(let singles [1, 2, 3])
(let doubles (map (\(x) (* x x)) singles))
```

In order to get the new thing, `doubles`, we take the old thing `singles`, copy it and change it however we need. You might think this is impractical- to copy every argument to every function, and you would be right! The compiler secretly optimises a lot of this copying away, and due to everything being immutable we can take advantage of parallelism much easier than the alternatives. This language is for creating simple games though, and while performance is a big priority for me, the language developer, for you it would be best to forget about performance while learning the basics.

In general though, a function which changes something, lets say `a`, can be thought of as a function *from* `a` *to* `a`, or `Fn a a` or `a -> a`.
Let's look at another example- the `main` function-

```
(let main (Console.println "Hello world!"))
```

This function is modifying something- the `Console`. In line with our model of mutability, we could think of this as a function from the old state of the console to the new state-

```
(let :(Console -> Console)
    main (Console.println "Hello world!"))
```

But this won't type check, since `main` is a value, not a function. Functions and values can be used interchangeably, and sometimes things that look like values are secretly functions- which is indeed the case for `main`, so let's assume that `main` and `Console.println` are functions-

```
(let :(Console -> Console) (main old_console)
    (let new_console (Console.println "Hello world!" old_console))
    new_console)
```

This works to model mutation, but where do we actually *update* the console? Well, you don't. The runtime of the language deals with all of that. Here's an example with pythonic-pseudocode

```py
def run_main():
    old_console = get_console()
    new_console = my_app.main(old_console)
    old_console.update(new_console)
```

This may feel quite roundabout to do something so basic, but the benefits don't reveal themselves until you have a bigger codebase.

The main idea behind representing changes (to the world, to the characters in your game, to the files on your system, etc.) like this is that we're putting information about what a function *does* into the type system. It's essentially like file permissions- unless a function takes a `Console`, it can't do anything to the console. This means that when you encounter a bug you're guaranteed to only have a small surface area in which that bug could occur.
- Is there a problem with the way something is printing to the console? The problem is likely in a function which modifies the `Console`.
- Is your save data not updating? The problem is likely in a function which modifies the `FileSystem`.
- Does the player jump too high after getting hit by an enemy? The problem is likely in a function which modifies `Player`, it also probably deals with `Enemy`s or `Projectile`s.
You get the idea. However, this last example raises an important question- how do we deal with functions which change multiple things?
Using the following `main` as an example

```
(let (main ???)
    (let name (Console.input ???))
    (Console.write (++ "Hello " name) ???)
    (FileSystem.write "example.txt" name ???)
    ???)
```

What are the inputs? It operates on the `Console` and the `FileSystem`, so it would make sense that the arguments reflect that

```
(let (main old_console old_fs)
    (let name (Console.input old_console))
    (Console.write (++ "Hello " name) old_console)
    (FileSystem.write "example.txt" name old_fs)
    ???)
```

But what do I return? The runtime needs to know what the new `Console` looks like, and also what the new `FileSystem` looks like, so I suppose I can pack them both into a tuple and return that

```
(let (main old_console old_fs)
    (let new_console1 (let name (Console.input old_console)))
    (let new_console2 (Console.write (++ "Hello " name) new_console1))
    (let new_fs (FileSystem.write "example.txt" name old_fs))
    (new_console2, new_fs))
```

And while this *could* work, it's obviously very clunky. We need to keep track of the state of both the console and the file system with clunky names. `main` needs to be variadic, and the return type has no structure. How about we try a record as both the input, and output?

```
(let Environment { console Console, fs FileSystem })

(let :(Env -> Env) (main env0)
    (let env1 (let name (Console.input env0)))
    (let env2 (Console.write (++ "Hello " name) env1))
    (let env3 (FileSystem.write "example.txt" name env2))
    env3)
```

This is better, but it's still very prone to error. The order in which these commands are written should be enough to determine when they're executed, not some index within the name of a binding of an abstract concept. This is just a fundamental limitation to functions- you need to specify their inputs, and specify their outputs. The problem with this approach is that we're thinking too atomically. We're trying to describe how `main` does what it does, but not what it actually is. Here's a valid program which does exactly that-

```
(let main
    (let! name Console.input)
    (let! _ (Console.write (++ "Hello " name)))
    (FileSystem.write "example.txt" name))
```

But what on earth is `let!`? Simply put, it means `let` `name` come from the result of `Console.input`. The last 2 lines don't use the results, so we simple ignore them with `_`. In fact, it's common enough to write `(let! _ ...)` that there's a syntax sugar for it- `(do ...)`.

```
(let main
    (let! name Console.input)
    (do Console.write (++ "Hello " name))
    (FileSystem.write "example.txt" name))
```

What does `let!` actually *do* though? Well, pretty much exactly what we were doing before. This is what it desugars to-

```
(let main
    (Console.input (\(name)
        (Console.write (++ "Hello " name) (\(_)
            (FileSystem.write "example.txt" name))))))
```

If you've ever used callbacks before then this should look familiar. This is how `Promise`s work in `JavaScript`, for example. What this means is that the type signatures for these functions are a little different than simple `Env -> Env`, then take the environment *and* some function which describes what to do with the return type when it's finished with it's operation. So `Console.input` has type `Env -> (String -> (Env -> Env)) -> Env`, for example. This is pretty unreadable, but there's a concept emerging out of this abstraction- something which does stuff. We have a word for that- it's called a `Task`. A `Task` has 2 things- what it uses, and what it's used for. For `Console.input`, it uses the `console`, and it's used to produce string. Therefore it's *actual* type signature is `Task { console Console } String`, which is much easier to understand. `Console.write` also uses the console, but it's not used for its return value, thus it has type `String -> Task { console Console } ()`. Likewise, `FileSystem.write` has type `String -> String -> Task { fs FileSystem } ()`. By being able to describe actions with one type, we can compose them conveniently using the `let!` syntax.

It should come as no surprise that `main` has type `Task { console Console, fs FileSystem } ()` since in order to run our program we need access to both the console and the file system, and our program isn't used to produce any value. One downside is that the final `Task` in `main` can't have `do`/`let!`, since there's nothing afterwards for the syntax to desugar. We can solve that by adding a `Task` to the end which does nothing-

```
(let :(Task { console Console, fs FileSystem } ()) main
    (let! name Console.input)
    (do (Console.write (++ "Hello " name)))
    (do (FileSystem.write "example.txt" name))
    (Console.write ""))
```

Although this is a bit ugly, and it only works for `Task`s which use the `Console`. What we want is a generic `Task` we can use anywhere. This is what `return` does, it takes a value and produces a `Task` which does nothing but returns that value.

```
(let :(Task { console Console, fs FileSystem } ()) main
    (let! name Console.input)
    (do (Console.write (++ "Hello " name)))
    (do (FileSystem.write "example.txt" name))
    (return ()))
```

Much nicer.

## Structural Typing
Something we brushed over in the last section was how `main` which expects an environment which looks like `{ console Console, fs FileSystem }` could use a `Task` like `Console.write` which expects an environment which looks like `{ console Console }`. These are 2 different types, so one may assume that they aren't compatible. Indeed in most languages that would be correct however this language is *structurally typed*. This means that the type signature of `Console.write` isn't saying "I can be used in any `Task` which *only* operates on the console", rather "I can be used in any `Task` which can *at least* operate on the console". When compiling the following program

```
(let :(Task { console Console, fs FileSystem } ()) main
    (let! name Console.input)
    (do (Console.write (++ "Hello " name)))
    (do (FileSystem.write "example.txt" name))
    (return ()))
```

we actually create new versions of `Console.input`, `Console.write` and `FileSystem.write` which all return `Task { console Console, fs FileSystem } ???`. This allows us to compose these functions together easily, and write way less restricted `Task`s.
