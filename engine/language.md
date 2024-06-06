# Idea
The language for this game engine should be a simple ml language with enough sophistication in the type system to allow for seamless mutation while reaping the benefits of immutability such as rewind debugging and testing.

# Syntax
## Hello World!
The syntax is basically Haskell syntax where each block is implicitly a `do` block. Here is a small example of what a `Hello World` application looks like in this language.
```hs
main : Task { console: Console } ()
main = printLn "Hello World"
```

Note that we can do subsequent operations by repeating statements
```hs
main =
    printLn "Hello World!"
    print "What is your name? "
    name <- input
    printLn ("Hello " ++ name ++ "!")
```

## Functions
Functions are declared like so
```hs
f : a -> b -> c -> d
f a b c = d
```

where `f` is the name of the function, `a` through `c` is the name of the arguments. Note that the same names are given to the types of the parameters. This is because values and types have identifiers belonging to different namespaces.

## Operators
The number of operators in this language is very small, as to allow for syntactic brevity for common operations. You have your standard arithmetic operators,
- `+`
- `-`
- `*`
- `/`
- `%`

but you also have function application / composition operators.
- `|>` used to feed arguments from the left side of the function.
    The following 3 lines are equivalent:
    ```hs
    f (g x)
    g x |> f
    x |> g |> f
    ```
- `|->` the same as `|>` but implicitly `map`ing the function on the right hand side.
    The following 3 lines are equivalent:
    ```hs
    x |-> f
    x |> map f
    map f x

- `|=>` the same as `|>` but implicitly `flatMap`ing the function on the right hand side.
    The following 3 lines are equivalent:
    ```hs
    x |=> f
    x |> flatMap f
    flatMap f x
    ```

Each of these operators has their corresponding reversed versions `<|`, `<-|`, `<=|`. These can be useful to avoid parenthesizing everything. For example, the following are all equivalent-
```hs
h (g (f x))
f x |> g |> h
h <| g <| f x
```

This just depends on readability. The first example reads basically the same as the last- a top down view of what we're doing to `x`. It's the result of applying `h` to the result of `g` of `f` of `x`. The final version just has less parentheses. The second version is a more sequential view of what we're doing to `x`. First, we're `f`ing it, then we're `g`ing it, then `h`ing. This is simply a matter of style and does not effect the output of the program.

## Types
Types are provided just above the definition of a function- they're the part that comes after `:`. For example
```hs
main : Task { console: Console } ()
main = ...
```

means that `main` is a value representing a `Task` which can read/write to the console. Since `Task`s can occasionally be used to return a value, there is an argument for the return type of the function, but since we don't explicitly care about that we just return `()`, which represents nothing, more or less. A traditional `C` function's type signature may look like
```hs
main : Task { console: Console } int
```

Since `main` in `C` (and other derivative languages) can return an exit code. This isn't syntactically valid though, since types require `PascalCase` naming conventions, like so
```hs
main : Task { console: Console } Int
```

# Semantics
The biggest difference between this language and the ones you're likely familiar with is immutability. If you were coding in `C` or `Python` or `Assembly` some things would be much more direct to express. If you have a player and you want them to take damage, you can simply do `health -= 1`, which means "make `health` one less than it was before."

This is faulty for a few quite subtle reasons. Let's say that I had several potential enemies which can damage the player. Where in my code base does this damage modification go? There are no correct answers here, but one common one is to say that the only thing which can modify the player's state should belong to the part of the code base responsible for controlling the player. This is fine, but what if your player can trade hits with the enemy? No problem, the enemy just needs to have it's own `health` functionality. But now let's say that if there's a trade, i.e. the enemy and the player hit eachother on the same frame, it does less damage. Now the mechanics of "the player getting hurt" extends beyond the idea of "the player got hit by something" and the player now has to know information about the thing which hit it. We could keep going with this solution, but what a lot of game developers eventually arrive on is some sort of `Manager` class, say a `CombatManager`- in other words "a thing that does something".

The problem I have with `Manager`/`Do-er` classes is that they're complicating an already existing language construct which does the same thing. "A thing which does something" isn't a "manager", it's a function! A function takes some input, does stuff to it, then gives you the new thing back. If we have a function like this
```hs
playerEnemyCollision : List Enemy -> Player -> Player
```

Then you can think of it as a function which modifies the player given some enemies. If we introduce the same constraint as with our previous example, we'll need to return not only the new state of the player, but also the new state of all the enemies-
```hs
playerEnemyCollision : List Enemy -> Player -> (List Enemy, Player)
```

This can quickly become unwieldy. The essential idea where trying to express is "what does this action operate on?" Because of this, we could simply pass a record of everything that we're changing
```hs
playerEnemyCollision : { enemies: List Enemy, player: Player } -> { enemies: List Enemy, player: Player }
```

This is nice because it has the symmetry of the initial rendition. Additionally, if we had another system which acted upon the same entities, we can now compose it with this one seemlessly. Say we wanted to damage the entities, then resolve the collision. If we had an additional function, such as
```hs
damageEntities : { enemies: List Enemy, player: Player } -> { enemies: List Enemy, player: Player }
```

We can update our world simply by doing
```hs
world = { enemies = [...], player = ... }
updateWorld world = playerEnemyCollision (damageEntities world)
```

Or to embrace function composition a little more
```hs
updateWorld world = world |> damageEntities |> playerEnemyCollision
```

And this reads quite well. If I'm updating the world, I take the world as it is, damage the entities which need damaging, then resolve the collisions between the players and enemies. This approach is much better for 2 reasons-
1. The logic for this system doesn't "belong" to either the enemy or the player. There is some point in our game where we want to update the world, and when we do there are certain actions we need to take which depend on / modify `enemies` and the `player`. This is conceptually as simple as it gets.
2. We can compose different tasks which happen in our world in an easy to read sequential order.

Our solution isn't perfect though. Lets introduce some more actions we may want to take as our game progresses.
```hs
checkForJump : { player: Player, input: Input, level_geometry: LevelGeometry } -> { player: Player, input: Input, level_geometry: LevelGeometry }
resolveCollisions : { player: Player, enemies: List Enemies, level_geometry: LevelGeometry } -> { player: Player, enemies: List Enemies, level_geometry: LevelGeometry }
```

ideally, you would want to be able to insert this right into the pipeline
```hs
updatedWorld world = world |> damageEntities |> checkForJump |> playerEnemyCollision |> resolveCollisions
```

The compiler will complain because your `World` doesn't have any concept of `Input` or `LevelGeometry`. If we provide those fields for our world
```hs
world = { enemies = [...], player = ..., input = ..., level_geometry = ... }
```

Now it will work as anticipated, even though the function types don't line up. This is a bit subtle so I'll go through it step by step-
Take `damageEntities` and `checkForJump`. These functions both operate with different data. As far as `damageEntities` is concerned, `world` doesn't have any notion of `Input`, or `LevelGeometry`. However, because the compiler can see that eventually in this pipeline of functions `world` is required to have those fields in `checkForJump`, all of their type signatures get automatically converted to a super-type which includes all of them. The compiler will give all the functions the same type signature behind the scenes
```hs
: { enemies : List Enemy, player : Player, input : Input, level_geometry : LevelGeometry } -> { enemies : List Enemy, player : Player, input : Input, level_geometry : LevelGeometry }
```

This is a really handy feature when composing actions, because it means that we can compose actions which have different effects easily. However, there's still one glaring issue- we have to copy this records' type signature in the type signature of the action. We can solve this quite simply with a type alias-
```hs
Task a = a -> a
```

We can then write the type signatures much more succinctly
```hs
: Task { enemies : List Enemy, player : Player, input : Input, level_geometry : LevelGeometry }
```

Again, this makes sense linguistically. `damageEntities` is a function. This function is a `Task` which modifies `enemies` and `players`. Therefore, it has a type signature of
```hs
damageEntities : Task { enemies: List Enemy, player: Player }
```

If you've seen the "Hello World!" example, you know that there's one extra parameter to `Task`. This is because functions aren't just used to modify stuff, they're also used to get information about stuff. Imagine you wanted to create a bullet in your world. What type signature would `createBullet` have? If we used our naive `Task` definition, we wouldn't be able to get a reference to the bullet we've just created. So for this reason, `Task`s aren't simply a function from the old state to the new state, it's a function from the old state to the new state *and* some value.
```hs
Task a v = a -> (v, a)
```

And we could say `createBullet` has type
```hs
createBullet : Task { projectiles: List Projectile } Projectile
```

or the de-sugared form
```hs
createBullet : { projectiles: List Projectile } -> (Projectile, { projectiles: List Projectile })
```

Now we can actually get information out of a given `Task`. The biggest problem with this solution now is that we can't compose our `Task`s as easily as we did before. Our `updateWorld` function has gained several layers of complexity, for values which we aren't even using for the most part-
```hs
updateWorld world0 =
    (_, world1) = damageEntities world0
    (_, world2) = checkForJump world1
    (bullet, world3) = createBullet world2
    (_, world4) = playerEnemyCollision world3
    (_, world5) = projectileEnemyCollision world4
    (_, world6) = projectileWallCollision bullet world5
    (_, world7) = resolveCollision world6
    world7
```

If you take a step back though, all we're trying to do is compose `Task`s. That's not a complicated idea, it should be easy to communicate. But now we have a whole bunch of state we have to manage and it can easily be messed up. Luckily, there's ways we can simplify the code above. Let's just create a function to deal with this mess in the background.
```hs
updateworld world =
    world
    |> then damageentities
    |> then checkforjump
    |> then playerenemycollision
    |> then resolvecollisions
```

what type does `then` have? The first argument is clearly a `Task`, but what about the second argument? What's the key thing being fed through the pipe? We can figure this out with a bit of deduction. Obviously, `updateWorld` is a `Task`. The following is clearly also a `Task`
```hs
updateworld world =
    world
    |> then damageentities
    |> then checkforjump
    |> then playerenemycollision
```
Therefore, the final line
```hs
    |> then resolvecollisions
```
Must be expecting a `Task`. So `then` takes 2 `Task`s- the next `Task` to do, and the `Task` up until that point, and returns a `Task` which describes doing both in order.
```hs
then : Task -> Task -> Task
```

The first function application is actually a type error, since `world` is a record of the current state of the world, not a `Task`. We can turn `world` into a `Task` which operates on the `world` without actually doing anything by using the `wrap` function.
```hs
wrap : a -> Task a ()

updateWorld world =
    wrap world
    |> then damageEntities
    |> then checkForJump
    |> then playerEnemyCollision
    |> then resolveCollisions
```

Now that the types line up... well, we're done actually. That's valid code.
