Title: This Week in Rust 86
Date: 2015-07-06
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

* [Things rust shipped without](https://graydon2.dreamwidth.org/218040.html). Rust released 1.0 without these "features", and for good reasons.
* [Rust Never Sleeps: How Mozilla Could Become Cool Again](http://readwrite.com/2015/07/02/mozilla-rust-programming-language-potential). Mozilla has almost faded from memory, but Rust could make it hip again.
* [Why Go and Rust are not competitors](http://dave.cheney.net/2015/07/02/why-go-and-rust-are-not-competitors).
* [A Rusting Rubyist](https://medium.com/@mfpiccolo/a-rubyist-rusting-db6e7e9c8f36). Mike Piccolo documents his attempt to create a web scraping library in Rust that can be called from a Ruby module.
* [A Rusting Rubyist II](https://medium.com/@mfpiccolo/a-rubyist-rusting-ii-f72dd8b0ed97).
* [A Pythonist getting Rusty these days... (Part 1)](https://wafflespeanut.github.io/blog/2015/07/05/a-pythonist-getting-rusty-these-days-dot/). Rust from a Python developer's perspective.
* [A Simple Web App in Rust, Part 3 -- Integration](https://joelmccracken.github.io/entries/a-simple-web-app-in-rust-pt-3/). The third part in a series on writing a very simple web application in Rust.
* [Understanding Lifetime in Rust – Part I](https://mobiarch.wordpress.com/2015/06/29/understanding-lifetime-in-rust-part-i/).
* [[PDF] Parallelization in Rust with fork-join and friends](http://publications.lib.chalmers.se/records/fulltext/219016/219016.pdf).

# New Releases & Project Updates

* [capgun](https://github.com/softprops/capgun). A simple utility that watches files and fires a specified command when they do.
* [pirate](https://github.com/zcdziura/pirate). A command-line arrrrguments parser, written in Rust.
* [rust-worldgen](https://github.com/YeyaSwizaw/rust-worldgen). Noise and World Generation library for Rust.
* [plex](https://github.com/goffrie/plex). A parser and lexer generator as a Rust syntax extension.

# What's cooking on nightly?

107 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-06-29..2015-07-06

# New Contributors



# Approved RFCs

* [Add some of `[T]`’s methods to strings and vice versa](https://github.com/rust-lang/rfcs/pull/1152).

# Final Comment Period

Every week the teams announce a 'final comment period' for RFCs which
are reaching a decision. Express your opinions now. [This week's][fcp]
RFCs entering FCP are:

[fcp]: https://github.com/rust-lang/rfcs/pulls?q=is%3Aopen+is%3Apr+label%3Afinal-comment-period

* [Expand the std::net module](https://github.com/rust-lang/rfcs/pull/1158). Expand the surface area of std::net to bind more low-level interfaces and provide more advanced customization and configuration of sockets.
* [Rename `connect` to `join`](https://github.com/rust-lang/rfcs/pull/1102).
* [Replace `slice::tail()`/`init()` with new methods](https://github.com/rust-lang/rfcs/pull/1058).
* [Redirect `stdio` of child processes to open file handles](https://github.com/rust-lang/rfcs/pull/1055).
* [Allow macros in types](https://github.com/rust-lang/rfcs/pull/873).
* [Allow closure expressions to expand to a `&` or `&mut` temporary](https://github.com/rust-lang/rfcs/pull/756). Modify the `||` expression sugar so that it can expand to either `F`, `&F`, or `&mut F`, where `F` is a fresh struct type implementing one of the `Fn`/`FnMut`/`FnOnce` traits.

# New RFCs

* [Add a high-level intermediate representation (HIR) to the compiler.](https://github.com/rust-lang/rfcs/pull/1191).
* [Style: How should we format function declarations?](https://github.com/rust-lang/rfcs/pull/1190).
* [Stabilize the `#![no_std]` attribute](https://github.com/rust-lang/rfcs/pull/1184).
* [Anonymous/placeholder lifetime `'_`](https://github.com/rust-lang/rfcs/pull/1177). Allow using an undeclared '_ wherever an explicit lifetime can be used, but is optional, such as function argument/return types and any path inside a function.
* [Create `IntoRaw{Fd, Socket, Handle}` trait to complement `AsRaw*`](https://github.com/rust-lang/rfcs/pull/1174).
* [Allow changing the default allocator](https://github.com/rust-lang/rfcs/pull/1183). Add support to the compiler to override the default allocator, allowing a different allocator to be used by default in Rust programs.
* [Propose `Interior<T>` data-type, to allow moves out of the dropped value during the drop hook](https://github.com/rust-lang/rfcs/pull/1180).

# Upcoming Events

* [7/7, San Diego Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/223145739/)
* [7/8, Rust in Production - San Francisco](http://www.meetup.com/Rust-Bay-Area/events/222260315/)
* [7/13, Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [7/15, Rust Los Angeles Monthly Meetup](http://www.meetup.com/Rust-Los-Angeles/events/223341178)
* [7/20, Rust Paris](http://www.meetup.com/Rust-Paris).
* [7/22, Columbus Rust Society](http://www.meetup.com/columbus-rs/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Greek constitution to be rewritten in #rustlang to deal with their ownership and borrowing problem."* — [@bigthingist](https://twitter.com/bigthingist/status/616826349634908160)

[Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
