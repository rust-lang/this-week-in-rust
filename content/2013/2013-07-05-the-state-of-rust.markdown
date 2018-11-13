Title: The State of Rust 0.7
Date: 2013-07-05 09:51
Category: Rust

Given the influx of newcomers from the 0.7 release, I thought it'd be a good
idea to summarize the condition of Rust, its libraries, and its documentation.
bstrie said it best I think, "basically, any question of the form 'is there a
reason for this stupid and terrible thing' is 'no, sorry, we're working on
it'&nbsp;".

<!-- more -->

# Iterators

External iterators are a main feature of the 0.7 release, and there is ongoing
work to remove all of the library features that use internal iterators. Most
of them are gone, and there are few uses of internal iterators. However, they
are clunky to use. The `for` loop semantics are going to change from internal
iteration to external iteration, but they are still internal iteration right
now, which means the `advance` adaptor is necessary for most uses of
iterators. Additionally, the `iter()` helper function is necessary to actually
return an iterator. This will be obviated by an `Iterable` trait that many
things will hopefully implement. Due to
[5898](https://github.com/mozilla/rust/issues/5898), many methods are oddly
named or have an underscore appended (ie, `transform` instead of `map`,
`position_` instead of `position`).

# IO

IO has been a bit of a losing proposition since at least 0.2. The interface is
very primitive and inefficient. It requires using `@Trait` objects (`@Reader`
/ `@Writer`). It's also very undocumented. But the situation isn't going to
improve much because all of that code is getting torn out when brson/the
interns finish their work on the new runtime. On the plus side, we'll have
shiny new IO when they're done! If you want to contribute, there's plenty of
work to be done in this area:

- [issue 6169](https://github.com/mozilla/rust/issues/6169)
- [issue 6850](https://github.com/mozilla/rust/issues/6850)
- [issue 4419](https://github.com/mozilla/rust/issues/4419)
- [brson's status report for June](https://mail.mozilla.org/pipermail/rust-dev/2013-May/004305.html)

# Compiler

The compiler is still buggy and inefficient. Lots of things work, but lots of
things don't. There's still some resolve bugs (the one mentioned above, as
well as some others, and perpetually poor error messages), default methods
don't work, debuginfo is incomplete, the compiler has quadratic codegen when
using `match`, so on and so forth. There's a lot of work to be done here but
it's not easy. I'm writing a series about the compiler that should help new
contributors get started and grok how it all fits together. The only thing
making Rust usable right now is LLVM's fantastic optimization. Our no-opt
builds run slower than our opt builds under Valgrind. Ponder that for a
minute.

# `rustpkg`

rustpkg is still heavily in-progress and unfinished. It's usable, but not
everything is implemented, and the documentation is incomplete. It's rather
unintuitive to use right now, but it does work! Read the
[manual](https://github.com/mozilla/rust/blob/master/doc/rustpkg.md) carefully
if you want to use it.

# Documentation

Documentation is poor. `rustdoc` is really bad, which doesn't help. I'm
working on a new rustdoc ([I log my progress
here](http://rustlog.octayn.net)), but it won't be ready for some weeks. Lots
of things are undocumented or near impossible to find because of how bad
`rustdoc` is. The tutorials need lots of work, and a guided tour of the
libraries would be nice, as well as a "Rust By Example," showing how to
accomplish common goals. These aren't really hard to do, it's just that nobody
has done them yet.

# Libraries

There are few robust libraries or bindings to libraries, for anything, besides
what is in std/extra. There's the stuff servo uses (glfw, skia, sdl), and an
opengl binding somewhere, but beyond that, you're on your own. The
[rust-bindgen](https://github.com/crabtw/rust-bindgen) tool can help with
wrapping a C library. There is no GUI library available yet, no real
networking. It's easy to wrap a C library, you just need to be careful with
your `unsafe` blocks. Once again, these things aren't hard, they just haven't
been done yet.

# The future

Basically everything is heavily WIP, but it's constantly improving. We always
need more contributors, ask in IRC if you're interested. Rust 0.7 is pre-alpha
quality, IMO. It would be silly to use Rust for something important, but don't
let that stop you from trying to do something ambitious. If you feel Rust is
still too young to get involved, but you want to track its progress, I write
[This Week in Rust](http://cmr.github.io/blog/categories/this-week-in-rust/),
which is an easy way to track our progress. 1.0 is tentatively planned for
first-quarter 2014, last I heard, and I don't think that is out of reach. 1.0
corresponds to "maturity #2" at the absolute minimum (the maturiy levels are
listed [as milestones](https://github.com/mozilla/rust/issues/milestones)).

But don't be discouraged. It was far worse before! Rust is in a good position,
it's just not quite all there yet. This post may seem pessimistic, but the
progress Rust has made is astonishing. Here's to a wonderful 0.8!

# Some links

- [Rust subreddit](http://www.reddit.com/r/rust/)
- [Mailing list](https://mail.mozilla.org/listinfo/rust-dev)
- [Mailing list archives](http://blog.gmane.org/gmane.comp.lang.rust.devel)
- [IRC
  Channel](http://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust)
  (it's `#rust` on irc.mozilla.org)
