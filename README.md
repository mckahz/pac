# Pixel Art Console

PAC is a virtual console for creating modern pixel art games.

PAC consists of 3 parts-

(TODO: add links to other pages.)
- [**Language**](): A fully featured programming language used to specify data as well as behaviour.
- [**Engine**](): Pixel Art rendering, audio, input handling, cross-platform.
- [**UI**](): Level editor, animation editor, hitbox editor, extensible.

See the above links for more information.

# Why PAC?

With any new game engine and *especially* new programming language comes the need to justify its existence.
Fundamentally this engine solves (TODO: HOW MANY FUCKING ISSUES AM I SOLVING HERE?) big issues.

## Poor rendering for pixel art by default in other engines.
In many, *many* indie titles using engines like [Game Maker](https://gamemaker.io/en), [Unity](https://unity.com), and to a much lesser extent in [Godot](https://godotengine.org), the pixel art rending has many faults. Quite often, you'll see basic mistakes such as-

- Mixels. Different sized pixels on screen at the same time. This can look okay as a way to show deformation of a sprite like in [The Binding of Isaac: Rebirth](https://en.wikipedia.org/wiki/The_Binding_of_Isaac:_Rebirth) where items can warp your character, or in [Celeste](https://en.wikipedia.org/wiki/Celeste_(video_game)) when Madeline fast-falls. Sometimes games want to be more dynamic than being rigidly locked to a grid, and it's great to have the ability to change that. However, ameture pixel-artists / game designers will change the size of pixels on a whim. Basically all modern engines handle this well though, so it's not too much of a problem.

- Pixel-Locking. In Game Maker and many purely code based engines the problem of what constitutes an onscreen pixel is simple- you lock everything to a grid. This way pixels will never be misaligned, because misaligned pixels look garbage! And while this is true of static objects, that is 2 objects with misaligned pixels placed next to eachother will look quite jarring- it's not true of an object in motion. We only notice misaligned pixels when they stop moving, so a character in [Shovel Knight](https://en.wikipedia.org/wiki/Shovel_Knight:_Plague_of_Shadows), or Celeste moving fluidly doesn't look jarring because when your character stops, you will quickly re-align with the pixels. Most of the time, at least. Again, pixel locking isn't a *bad* feature, but it shouldn't be the default way to do things. Setting up any of the aformentioned 2d game engines to allow for misaligned pixels is surprisingly painful.

- Nearest-Neighbor Scaling.

    When scaling up pixel art there are many right ways to do it.
    If I'm displaying an `8x8` image on a `16x16` screen,
    each pixel of the image (texel) can easily be `2x2` pixels and it works fine.
    For this reason, an unbelievable amount of 2d rendering uses one simple way to scale images- nearest neighbor.

    For high definition images, when they are scaled up on a screen it's best to blur it a little bit so that the fact that you're looking at an array of color values awkwardly scaled to fit your screen isn't so apparent.
    This is why you need to make sure your background is the right size, because if it isn't it will look blurry.
    This is called "bilinear filtering".

    For pixel art the choice is obvious. Bilinear filtering looks rubbish. So engines like Godot and Game Maker will just choose nearest neighbor and call it a day.

    The problem with this approach is that pixel art isn't as obvious as that- you need some amount of anti aliasing almost everywhere.
    - Around the edges of sprites if they're offset a fractional amount
    - Between pixels of different colors
    - On texels who's sizes are some non-whole-number multiple size of on-screen pixels

    It's quite easy to achieve this effect with 2 render phases, 1 with nearest neighbor and another with bilinear filtering.

    The problem with the big engines which everyone uses is this- they only do the first part by default, you need to do the next part by yourself.

    Seems simple enough, but there's more to the story and it's enough to keep me awake at night- it seems like half the planet had a brain heomarrage and decided that this was impossible. If you go looking for ways to scale pixel art probably, it is a rabbit hole of emotion. Wait this needs it's own section-

### The Collective Brain Heomarrage of the Game Dev Community
Due to the tone of the next section I hope that you appreciate the lack of examples I provide.
So many games look like ameture rubbish because of this mass lie that people have been told.
In the documentation of quite a few game engines you'll find the same rubbish.

> "In order to scale pixel art, you need to make sure the size of the in-game screen is some factor of your computer screen."
>     - someone who shouldn't be allowed on the internet.

The actual code to do this is a single shader, and yet the majority of the internet seems to think it's impossible.
There are so many examples of people doing it correctly and when you have it pointed out to you it's so obvious it hurts.
How is this still a pervasive issue in 90% of the games I see?

Well, this is why- big game engines don't support it by default, so people don't use it.
This is why proper, modern pixel art scaling will be the default in pac.

## Out of the box everything
Everything that you need to create a project, create something amazing, and publish it is accessible straight out of the box.
No looking for libraries to do the basics.

## Simple data representation
Due to the generality of the objects in most game engines, they pack a lot of features that aren't necessary. Whether it's a mandatory transform in Unity or an entire physics body in Game Maker. Now, these rarely become performance constraints- indeed games from both engines can run spectacularly well. The problem is that everything is a black box. This makes sense with general purpose engines as you need some common interface for working with game objects. However, this has some weeknesses-

- Referring to another object becomes an expensive operation which you want to do as little as possible, even though all it amounts to is a pointer.
- Ceremony in creating game objects. You *should* do it with the UI because the underlying data representation is specified so elaborately that it must first be serialized in some way.

However, games in `pac` must be small enough that serializing the input isn't necessary.
The UI is simply complimentary in that it parses your code, allows you to modify some values, and writes back to the parsed file.
This means you can refer to any game object the same as any other piece of code, which allows for a much more flexible and modular design.

## Debugging
When debugging a time based issue like physics or animation it can be frustrating to "hook into" the code at the right time.
It makes for a painful articulation of exactly under what conditions you're waiting for just so you can set a debug marker and stop *only* when you want to.
In theory, it would be nice to have the ability to stop whenever you wanted and rewind to before the bug occurred then step through. So that's a feature of PAC- rewind debugging.

This feature feels like a fever dream for little old me learning to program in Game Maker, but it turns out that not only is it [possible](TODO: insert proof), it's actually the driving force behind amazing technologies like [GGPO](https://www.ggpo.net), and conceptually it's quite simple- keep the old state of your program for as long as you want to be able to rewind. There are certain constraints you need to abide in order to make this feasable, and it's a large reason as to why PAC uses a dedicated language for its game code.

## Language
Most programming languages used by game engines are general purpose languages. This is okay, but game engines have enough domain specific ideas that a language built around them could be quite powerful. On top of that, the languages that people make game in are mostly the same, the biggest difference being the memory management stratergy.

Godot and Game Maker both use their own proprietary scripting languages, but they leave much to be desired. They both have very basic type systems yet a lot of emergent complexity from their initial design.

PAC is by design as simple as it can be. You have basic values (characters, booleans, numbers, strings), functions, ways to combine values (data structures), and a few syntactic nicities on top of that.

This allows it to be a versatile language for both data specification (i.e. levels, animations, hitboxes, etc.) and behaviour specifcation (rendering, input handling, audio, collision, networking, etc.).
