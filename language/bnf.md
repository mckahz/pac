# Operators

```
|---------------------------------------------|
| 10  | f x                           | Left  |
|---------------------------------------------|
|  9  | >> <<                         | Right |
|---------------------------------------------|
|  8  | ^                             | Right |
|---------------------------------------------|
|  7  | * / %                         | Left  |
|---------------------------------------------|
|  6  | + -                           | Left  |
|---------------------------------------------|
|  5  | :: ++                         | Right |
|---------------------------------------------|
|  4  | == != < <= >= >               |       |
|---------------------------------------------|
|  3  | &&                            | Right |
|---------------------------------------------|
|  2  | ||                            | Right |
|---------------------------------------------|
|  1  | monad operators               | Left  |
|---------------------------------------------|
|  0  | <| |>                         | Right |
|---------------------------------------------|
```

# Getting Started

To get up and running, clone this repository and execute the interpreter for some file.

```
$ git clone (TODO: insert web address)
$ cd lang
$ (TODO: insert 
```

To create a program, you must have a `Main.lang` file. This will be the entry point for everything which happens in your program.
This is the simplest program you could write in any programming language- "Hello World!"

```
module Main [main];

let main = Task.println "Hello World";
```

We'll get into the exact details of what this means but for now you can execute it with the following command-

```
$ cargo run -- path/to/Main.lang
```

# Hello World



