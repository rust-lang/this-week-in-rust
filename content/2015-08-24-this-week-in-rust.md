Title: This Week in Rust 93
Number: 93
Date: 2015-08-24
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

* [Rust-101](https://www.ralfj.de/projects/rust-101/main.html). A hands-on Rust tutorial.
* [The glium library](https://medium.com/@tomaka/the-glium-library-5be149d87dc1). Challenges in exposing a safe API over an unsafe C library.
* [Building an iOS App in Rust, Part 2: Passing Primitive Data Between Rust and iOS](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-2/). Read Part 1 [here](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/).
* [Mozilla’s Servo Lets Rust Shine](http://thenewstack.io/mozillas-servo-lets-rust-shine/).
* [Three months of Rust](http://scattered-thoughts.net/blog/2015/06/04/three-months-of-rust/).
* [Configuring Emacs for Rust](http://bassam.co/emacs/2015/08/24/rust-with-emacs/).
* [Dissecting Crates.io: Bare Minimum Mirror](https://gmjosack.github.io/posts/dissecting-cratesio-minimum-mirror/).
* [Using Docker to Test Rust on Linux](http://hermanradtke.com/2015/08/23/using-docker-to-test-rust-on-linux.html).
* [Community Versions for Rust](http://words.steveklabnik.com/community-versions-for-rust).
* [Virtual Structs Part 3: Bringing Enums and Structs Together](http://smallcultfollowing.com/babysteps/blog/2015/08/20/virtual-structs-part-3-bringing-enums-and-structs-together/).
* [Parser Combinator Experiments in Rust](https://m4rw3r.github.io/parser-combinator-experiments-rust/).
* [Closures as Anti-Lifetime-Gluteal-Bite-Device](https://llogiq.github.io/2015/08/19/closure.html).

# New Releases & Project Updates

* [Rust by Example](http://rustbyexample.com/) now [tracks stable Rust](https://github.com/rust-lang/rust-by-example/pull/636), [uses container-based Travis infrastructure](https://github.com/rust-lang/rust-by-example/pull/637), and [is tested against all three channels](https://github.com/rust-lang/rust-by-example/pull/638).
* [octavo](https://github.com/hauleth/octavo). Highly modular & configurable hash & crypto library written in pure Rust.
* [Ammonia](https://github.com/notriddle/rust-ammonia). A whitelist-based HTML sanitization library.
* [mm](https://www.reddit.com/r/rust/comments/3i4qd7/mm_general_purpose_math_and_multimedia_libraries/). General purpose math and multimedia libraries.
* [etcd](https://github.com/jimmycuadra/rust-etcd). A Rust client library for etcd.
* [proc](https://github.com/danburkert/proc-rs). A Rust library for accessing Linux process and system information.
* [scoped-threadpool-rs](https://github.com/Kimundi/scoped-threadpool-rs). A library for scoped and cached threadpools.
* [imgui-rs](https://github.com/Gekkio/imgui-rs). Rust bindings for ImGui.

# What's cooking on nightly?

86 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-08-17..2015-08-24

# New Contributors

* jotomicron
* Marc-Antoine Perennou
* Martin Wernstål

# Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1200: Add `cargo install`](https://github.com/rust-lang/rfcs/pull/1200).
* [RFC 1211: Introduce a mid-level IR (MIR) in the compiler](https://github.com/rust-lang/rfcs/pull/1211).

# Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen+updated%3A2015-08-17..2015-08-24

* [Policy for rust-lang crates](https://github.com/rust-lang/rfcs/pull/1242).
* [Stabilize `catch_panic`](https://github.com/rust-lang/rfcs/pull/1236).
* [`CoerceUnsized` should ignore `PhantomData` fields](https://github.com/rust-lang/rfcs/pull/1234).
* [Turn statically known erroneous code into a warning and continue normal code-generation](https://github.com/rust-lang/rfcs/pull/1229).
* [Define a "line" as terminated by either `\n` or `\r\n`](https://github.com/rust-lang/rfcs/pull/1212).
* [Pretty print `Debug` of tuples, tuple structs and enum variants in a single line](https://github.com/rust-lang/rfcs/pull/1198).
* [Add item recovery collection APIs](https://github.com/rust-lang/rfcs/pull/1194).
* [Allow a `x...y` expression to create an inclusive range](https://github.com/rust-lang/rfcs/pull/1192).
* [Implement raw fat pointer comparisons](https://github.com/rust-lang/rfcs/pull/1135).
* [Add the family of `[Op]Assign` traits to allow overloading assignment operations like `a += b`](https://github.com/rust-lang/rfcs/pull/953).

# New RFCs

* [New `str` and `String` API](https://github.com/rust-lang/rfcs/issues/1253).
* [`RawOs` marker traits](https://github.com/rust-lang/rfcs/issues/1256).
* [Missing edge-case when destructuring](https://github.com/rust-lang/rfcs/issues/1261).
* [Splitting variants of `str::starts_with` and `ends_with`](https://github.com/rust-lang/rfcs/issues/1262).
* [Add a lint to warn about negative literal / method call precedence](https://github.com/rust-lang/rfcs/issues/1263).
* [Add `CStr::from_bytes`](https://github.com/rust-lang/rfcs/issues/1264).

# Upcoming Events

* [8/26. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [8/31. Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*"`unsafe` is as viral and pernicious as pop music, though obviously not as dangerous."* — [Daniel Keep](https://users.rust-lang.org/users/DanielKeep) at [TRPLF](https://users.rust-lang.org/t/pure-rust-and-safe-badges/2451/27).

Thanks to [llogiq](https://users.rust-lang.org/users/llogiq) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
