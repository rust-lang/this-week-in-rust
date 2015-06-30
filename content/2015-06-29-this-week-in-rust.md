Title: This Week in Rust 85
Date: 2015-06-29
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# From the Blogosphere

* [Dealing With the Rust Shift in Perspective](https://joelmccracken.github.io/entries/dealing-with-the-rust-shift-in-perspective/).
* [rusty radio: Episode 1: A Rusty Start](http://rustyrad.io/podcast/1/).
* [Rust, the Language for Growth](https://joelmccracken.github.io/entries/rust-the-language-for-growth/).

# Tips & Tricks

* [How to Use Rust with Node.js When Performance Matters](http://blog.risingstack.com/how-to-use-rust-with-node-when-performance-matters/). Write performance critical code in Rust and use it via FFI in Node.js.
* [Effectively Using Iterators In Rust](http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html). Iterate over vector and slice types using `.iter()` and `.into_iter()`.
* [First Rust Program Pain (So you can avoid it…)](http://dtrace.org/blogs/ahl/2015/06/22/first-rust-program-pain/). Tips for Rust beginners on how to avoid a fight with the compiler.
* [Implement Traits on Generics](https://mr-byte.github.io/blog/blog/2015/06/27/traits-on-generics/).
* [The Unsafe Rust Programming Language](http://cglab.ca/~abeinges/blah/turpl/_book/README.html). An excellent document about advanced functionality and low-level development practices in the Rust Programming Language.

# In the News

* [Rust 1.1 stable, the Community Subteam, and RustCamp](http://blog.rust-lang.org/2015/06/25/Rust-1.1.html).
* [Tickets are on sale now for RustCamp!](http://rustcamp.com/).

# New Releases & Project Updates

* [rust-timsort](https://github.com/notriddle/rust-timsort). Rust implementation of the modified MergeSort used in Python and Java.
* [trust](https://github.com/Wmaxlees/trust). Rust automated test runner.
* [mongo-rust-driver](https://github.com/thijsc/mongo-rust-driver). Mongo Rust driver built on top of the Mongo C driver.
* [rust-ffi-omnibus](http://jakegoulding.com/rust-ffi-omnibus/). A collection of examples of using code written in Rust from other languages.
* [hyper is now at v0.6](http://seanmonstar.com/post/122441373502/hyper-v0-6). An HTTP/S library for Rust.
* [rust-throw](https://github.com/daboross/rust-throw). A new experimental rust error handling library, meant to assist and build on existing error handling systems.
* [burrito](https://github.com/withoutboats/burrito). A monadic IO interface in Rust.
* [mimty](https://bitbucket.org/joshmorin/mimty). Fast, safe, self-contained MIME Type Identification for C and Rust.

# What's cooking on master?

95 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-06-22..2015-06-29

# Breaking Changes

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Other Changes

* [std: Add support for Windows XP](https://github.com/rust-lang/rust/pull/26601).
* [Reset signal behavior before starting children with std::process](https://github.com/rust-lang/rust/pull/25784). Resets Rust's SIGPIPE handler, as well as any signal mask that may have been set, before spawning a child.
* [MSVC: Implement runtime support for unwinding](https://github.com/rust-lang/rust/pull/26569). A key aspect is missing, however, which is that unwinding is still turned off by default for MSVC.
* [Suggest missing trait bounds when a method exists but the bounds aren't satisfied](https://github.com/rust-lang/rust/pull/26435).

# New Contributors

* Andy Grover
* Brody Holden
* Christian Persson
* Cruz Julian Bishop
* Dirkjan Ochtman
* Gulshan Singh
* Jake Hickey
* Makoto Kato
* Yongqian Li

# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs which
are reaching a decision. Express your opinions now. [This week's][fcp]
RFCs entering FCP are:

[fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period

* [Add some of `[T]`’s methods to strings and vice versa](https://github.com/rust-lang/rfcs/pull/1152).
* [Implement `FromIterator` for the unit type](https://github.com/rust-lang/rfcs/pull/1130).
* [Rename `connect` to `join`](https://github.com/rust-lang/rfcs/pull/1102).
* [Add `read_into_buf` and `get_buf` to `BufRead`](https://github.com/rust-lang/rfcs/pull/1015).
* [read_all](https://github.com/rust-lang/rfcs/pull/980).
* [Allow macros in types](https://github.com/rust-lang/rfcs/pull/873).
* [Allow closure expressions to expand to a `&` or `&mut` temporary](https://github.com/rust-lang/rfcs/pull/756).

# New RFCs

* [Return `Result` from `main`](https://github.com/rust-lang/rfcs/issues/1176).

# Upcoming Events

* [7/7, San Diego Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/223145739/)
* [7/8, Rust in Production - San Francisco](http://www.meetup.com/Rust-Bay-Area/events/222260315/)
* [7/13, Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [7/15, Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/223341178)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com
