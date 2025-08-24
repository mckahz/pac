# Constructors

a module should export constructors like it would any other member. For instance, contrast the following two examples

```
let Tree node = Leaf node | Branch (Tree node) (Tree node);

let leaf : node -> Tree node;
let leaf = Leaf;

let branch : Tree node -> Tree node -> Tree node;
let branch = Branch;
```

it's telling that the implementation of the constructors `leaf` and `branch` can be implemented directly in terms of the constructors `Leaf` and `Branch` respectively. This means that during semantic analysis of our code `Leaf` and `Branch` need to be understood basically the same way as regular values.

The definition for `Tree` in this context only really does 2 things. It tells us the type variables its constructors use, and declares what it's constructors' type signatures are. In this sense, a constructor could be defined (in pseudo code) like so

```
let Tree node = extern "Tree";

let leaf : node -> Tree node;
let leaf = extern "Tree.Leaf";
```

This means that any unions in our canonicalized tree don't need to contain information about the constructors it uses.

# When Expressions / Patterns

This is how my compiler currently exports a simple `rest` (or `tail` in Haskell) function-

```js
export const rest = (xs) => (() => {
    const __expr = xs;
    const __functions = [
        () => crash("cannot get rest of empty list"),
        (_,xs) => xs
    ];
    return __expr.args ? __functions[__expr.tag](... __expr.args) : __expr ? __functions[1]() : __functions[0]();
})();
```

this is obviously hideous.

It would probably be ideal if the output looked something like this

```js
export const rest = (xs) => (
  (xs == []) ? crash("cannot get rest of empty list") :
  (isCons(xs)) ? (() => {
    const _ = xs.args[0]; // Optimized out!!
    const xs = xs.args[1];
    return xs;
  })() :
  crash("never")
);
```
