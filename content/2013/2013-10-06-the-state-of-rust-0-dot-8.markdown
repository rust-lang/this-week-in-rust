Title: The State of Rust 0.8
Date: 2013-10-06 20:32
Category: Rust

Another 3 months, another release. This is the first release that I've
witnessed in its entirety! This is a summary of Rust: its compiler,
libraries, documentation, and community. ([What is
Rust?](http://rust-lang.org))

<!-- more -->

# Compiler

The compiler is faring well. It received a lot of attention in compile speed,
although memory usage regressed significantly. It is now mostly fixed in
master. Time compiling `fn main() { }` went from 172ms to 112ms on my box.
When compiling programs of any significance, the gap is much larger. There's
also been some thought put into parallelizing rustc. Michael Woerister's GSoC
project was debuginfo, and it's almost in a fully-working state. As of 0.8,
it's not completely baked -- libstd can't be compiled with it, and stepping
through code isn't perfect -- but it's a huge step forward, and he created an
extensive testsuite, so it shouldn't regress. The pretty printer hasn't seen
much improvement. Default methods, one of the major things Michael Sullivan
worked on over the summer, are in a much better state. If there are any
remaining bugs in them, I haven't seen them.

# Iterators

Iterators are hugely improved for 0.8. The `for` loop syntax now uses the
Iterator trait. Additionally, most uses of vector iterators now compile to the
exact same code that indexing or iteration would in C or C++, including the
ability to be vectorized. Additionally, they now use default methods instead
of extension implementations. A bunch of other extensions to Iterator were
added, such as DoubleEndedIterator and RandomAccessIterator.

# Documentation

The documentation is in a much better state than it was 3 months ago. The new
rustdoc was started and finished. I started it, and Alex Crichton really
polished and finished it the last two weeks before the release. The API
documentation is now navigable, and one can actually see the relationship
between various types. A bunch of work also went into the tutorials, yielding
three new documents: error handling and conditions, iterators and containers,
and rustpkg.

# `rustpkg`

Rustpkg continues to advance. Tim put out the call for community involvement,
and it's getting significant traction in actual libraries. There are still a
few kinks when using it for development, but when just fetching and building
dependencies, it works very well. Servo is porting its whole mini-ecosystem
over to rustpkg, uncovering bunches of problems and deficiencies in the
process. If you're interested in helping out with Rust, rustpkg is a major
area. Tim is also super nice, and will happily help you get into the codebase.

# Libraries

Rust is slowly accreting more and more useful libraries: mostly coming from
the gamedev community, but sometimes other useful things as well. The
new runtime has completely replaced the old, a significant step forward for
Rust's maturity. `rust-http` is making some really nice strides, as well as
the opengl bindings. The standard libraries are becoming nicer to use.
`std::run`, in particular, stands out to me as something that's quite easy to
use, and `std::str` saw a lot of work making it more correct.

# The Future

This was a great release cycle, and I think the next one will be even better.
Alex Crichton was hired as a full-time Rust developer, and he has been doing a
lot of important work that just would have taken a while to happen otherwise.
I'm very optimistic about this release, much more so than 0.7. I think we
might be able to hit milestone 1 for 0.9 or 0.10, though that's just
speculation on my part.

# Is Rust Ready?

No. Rust is approaching maturity, but it isn't there yet. There are still
backwards-incompatible changes being made to try and get to milestone 2.  The
major things that come to mind are closure reform and privacy overhaul (which
is being worked on in master [right
now](https://github.com/mozilla/rust/pull/9735)). Additionally, people in the
gamedev community are starting to [hit walls with the type
system](http://www.reddit.com/r/rust/comments/1nxs1h/the_state_of_rust_08/ccnb8ya).
Those probably won't be fixed in the time leading up to 1.0, but this is also
speculation.

Experimenting with Rust is becoming more viable as time goes on, but using it
in production is a bad idea, especially if "low maintenance" is at all
valuable.
