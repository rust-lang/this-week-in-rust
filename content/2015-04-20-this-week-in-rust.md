Title: This Week in Rust 78
Date: 2015-04-20
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors or omissions in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# What's cooking on master?

108 pull requests were [merged in the last week][merged], and 5 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-13..2015-04-20
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-13..2015-04-20

Now you can follow breaking changes *[as they happen][BitRust]*! Or, [in html][BitRust2].

[BitRust]: http://rawgit.com/mrmonday/bitrust/gh-pages/index.html
[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Add `Sync` to the bounds of `io::Error`](https://github.com/rust-lang/rust/pull/24133).
* [Make `FromRawFd::from_raw_fd` unsafe](https://github.com/rust-lang/rust/pull/24251).
* [Destabilize `format_args!` internals](https://github.com/rust-lang/rust/pull/24312).
* [Check for overflow in arithmetic negation](https://github.com/rust-lang/rust/pull/24500).

# Other Changes

* [Add `Default` trait for `AtomicBool`, `AtomicIsize`, `AtomicUsize`](https://github.com/rust-lang/rust/pull/24329).
* [rustdoc: Another round of improvements](https://github.com/rust-lang/rust/pull/24396).
* [Move optimized `String::from_str` to `String::from`](https://github.com/rust-lang/rust/pull/24517).
* Several patches adding extended diagnostics have appeared: [1](https://github.com/rust-lang/rust/pull/24542), [2](https://github.com/rust-lang/rust/pull/24552), [3](https://github.com/rust-lang/rust/pull/24525). Thanks to michaelsproul, GuillaumeGomez, meqif and others!

# New Contributors

* Abhishek Chanda
* Andrew Seidl
* Aram Visser
* Avdi Grimm
* fenduru
* James Perry
* Manuel Hoffmann
* Mickaël Salaün
* Nelo Onyiah
* Nick Hamann
* pez
* Robin Kruppe
* rundrop1
* Theo Belaire
* Thomas Jespersen
* Ting-Yu Lin

# Approved RFCs

* [RFC 771: std::iter::once](https://github.com/rust-lang/rfcs/pull/771).
* [RFC 1030: 1.0 prelude additions](https://github.com/rust-lang/rfcs/pull/1030).
* [RFC 1048: split up fs::soft_link into os::unix::fs::symlink and os::windows::fs::{symlink_file, symlink_dir} ](https://github.com/rust-lang/rfcs/pull/1048).
* [RFC 1054: Rename or replace `str::words` to side-step the ambiguity of “a word”](https://github.com/rust-lang/rfcs/pull/1054).
* [RFC 1057: Add Sync to io::Error](https://github.com/rust-lang/rfcs/pull/1057).

# New RFCs

* [Constants that depend on type parameters in generic code](https://github.com/rust-lang/rfcs/pull/1062).
* [Alter mem::forget to be safe](https://github.com/rust-lang/rfcs/pull/1066).
* [Scaling Rust's governance](https://github.com/rust-lang/rfcs/pull/1068).

# Notable Links

* [A page of useful links for new contributors](http://www.ncameron.org/rust.html).
* [Priorities after 1.0](http://internals.rust-lang.org/t/priorities-after-1-0/1901/54). This is where we're going. Your opinion matters.
* [Mixing matching, mutation, and moves in Rust](http://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html).
* [How should we talk about mutability?](http://internals.rust-lang.org/t/how-should-we-talk-about-mutability/1882)
* [Rust 1.0 launch event details](http://users.rust-lang.org/t/rust-1-0-launch-event-details-action-required-for-event-organizers/1025). There will be meetups to attend.
* [Regression report beta-2015-04-03 vs. beta-2015-04-17](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-beta-2015-04-17/1931).
* [Array slice strangeness](http://www.reddit.com/r/rust/comments/330tu1/array_slice_strangeness/). Just a question, but behold the tag team answer by huon, Gankro, and pnkfelix.
* [`std::thread::scoped` found to be unsound](http://www.reddit.com/r/rust/comments/32jmf8/stdthreadscoped_found_to_be_unsound/). That awesome thing Rust can do? It can't actually.
* [Aaron Turon's Stanford talk (video)](http://www.reddit.com/r/rust/comments/32fxlo/aaron_turon_stanfords_talk_on_rust/).
* [Help write Rust error explanations!](http://www.reddit.com/r/rust/comments/32jdq9/help_write_rust_error_explanations/). Michael Sproul is on the prowl. For better error messages.
* [My Python's a little Rusty](https://www.youtube.com/watch?v=3CwJ0MH-4MA). Dan Callahan at PyCon 2015. Video.
* seanmonstar has created [crates.io badges to plaster your README's with](http://seanmonstar.com/post/116574828167/merit-badge).
* [Rust vs. Ruby: building an API](http://serdardogruyol.com/rust-vs-ruby-building-an-api/).
* [Building Rust programs with Docker - ad-hoc talk@containera.io (video)](https://www.youtube.com/watch?v=JJdevVjAmmQ)

# Project Updates

* [A skeletal animation demo in Piston](https://www.reddit.com/r/rust_gamedev/comments/32g6h7/piston_skeletal_animation_demo/).
* [wrapping_macros](https://github.com/lfairy/wrapping_macros). Macros for wrapping arithmetic.
* [tempfile](http://www.reddit.com/r/rust/comments/32n864/tempfile_temporary_file_library/). Secure, cross-platform, temporary files.
* [LlamaDB](http://www.reddit.com/r/rust/comments/32wqa7/wip_llamadb_a_simple_sql_database_written_in_rust/). A SQL database.
* [twilio-rs](http://www.reddit.com/r/rust/comments/336syv/twiliors_rust_library_for_working_with_twilio/). Library for working with Twilio.
* [elmesque](https://github.com/mitchmindtree/elmesque). [Elm](http://elm-lang.org)'s std graphics modules ported to Rust and rendering to GL.

# Upcoming Events

* [4/20. Paris](http://www.meetup.com/Rust-Paris).
* [4/22. The Columbus Rust Society](http://www.meetup.com/columbus-rs/).

A number of meetups will be [celebrating
1.0](http://users.rust-lang.org/t/rust-1-0-launch-event-details-action-required-for-event-organizers/1025/6). Watch this space!

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

"`unsafe` restricts which code could contain undefined behavior, but it doesn't isolate the effects of that undefined behavior." - [kmc on the limits of unsafety](https://www.reddit.com/r/rust/comments/32wqa7/wip_llamadb_a_simple_sql_database_written_in_rust/cqfoh41).

Thanks to tshepang for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
