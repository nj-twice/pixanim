# What's this?

`pixanim` is a simple program to help you make pixel art animations!

For now, it doesn't do much, but in its current form, it's enough for me!

# How to use it?

Its main purpose is to compile a bunch of individual image files
in the current directory into a spritesheet.

Inside a directory of images, simply run:

```sh
pixanim make
```

You can get more info by running:

```sh
pixanim help
```

## Naming conventions

In order for `pixanim` to correctly compile your images, they must
follow a simple set of conventions.

Your file names must have the following format: `x_y.png`, where both
`x` and `y` are non-zero natural numbers.

Moreover, the numbers should follow a natural progression, without
any leaps or omissions.

As an example:
`1_1.png`, `1_2.png`, `2_1.png` and `2_2.png`
is a valid set of file names, whereas
`1_1.png`, `1_3.png`, `2_1.png` and `2_2.png`
or even
`1_1.png`, `1_2.png`, `3_1.png` and `3_2.png`
are **not**.

## What are these numbers?

`x` represents the row in the final spritesheet, that is, an animation and
`y` represents the column, that is, an individual frame of an animation.

# Compiling

As for any Rust project, simply run:

```sh
cargo build --release
```

# Alternatives

Of course, there are [better programs](https://www.aseprite.org/) for
creating pixel art that come with a lot of bells and whistles.

If this little tool isn't enough for you, consider using them instead.
