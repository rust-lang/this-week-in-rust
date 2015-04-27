Title: This Week in Rust 79
Date: 2015-04-27
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

119 pull requests were [merged in the last week][merged], and 2 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-20..2015-04-27
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-20..2015-04-27

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [std: Add Default/IntoIterator/ToOwned to the prelude](https://github.com/rust-lang/rust/pull/24541)
* [Enforce the comma after lifetime arguments and before type arguments](https://github.com/rust-lang/rust/pull/24547)
* [Remove for deprecated functionality](https://github.com/rust-lang/rust/pull/24636)
* [Make stability attributes an error](https://github.com/rust-lang/rust/pull/24646)

# Other Changes

* [Rename std::fs::soft_link to std::fs::symlink](https://github.com/rust-lang/rust/pull/24222)
* [Introduce a `FreeRegionMap` to store relations between free regions](https://github.com/rust-lang/rust/pull/24553)
* [implement rfc 1054: split_whitespace() fn, deprecate words()](https://github.com/rust-lang/rust/pull/24563)
* [implement set_tcp_keepalive for linux](https://github.com/rust-lang/rust/pull/24594)
* [Call skolemize_late_bound_regions only after fast rejection](https://github.com/rust-lang/rust/pull/24615). Faster type checking.

# New Contributors

* Andrzej Janik
* Geoffry Song
* Heejong Ahn
* John Van Enk
* Krzysztof Drewniak
* Lee Aronson
* Michael Rosenberg
* Przemysław Wesołek
* Skyler
* tynopex
* Young Wu

# Approved RFCs

* [RFC 1044: Expand the scope of `std::fs`](https://github.com/rust-lang/rfcs/pull/1044)

# New RFCs

* [Scoped threads, take 2](https://github.com/rust-lang/rfcs/pull/1084)
* [Leak and Destructor Guarantees](https://github.com/rust-lang/rfcs/pull/1085)

# Betawatch!

The current beta is `1.0.0-beta.3 (5241bf9c3 2015-04-25)`.

There were 2 PRs this week landing backports to beta.

* [24708](https://github.com/rust-lang/rust/pull/24708).
* [24814](https://github.com/rust-lang/rust/pull/24814).

# Notable Links

* [Rust once, run everywhere](http://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html).
* [Weekly meetings 2015-04-21][mtg]. Servo licensing, iter overflow, float formatting, branching.
* [Core team meeting 2015-04-22](https://github.com/rust-lang/meeting-minutes/blob/master/core-team/meeting-2015-04-22.md). The core team is summarizing their weekly meeting.
* [Let's build a web service and client in Rust](https://github.com/brson/httptest)
* [Regression report beta-2015-04-03 vs. beta-2015-04-24](http://internals.rust-lang.org/t/regression-report-beta-2015-04-03-vs-nightly-2015-04-24/1967/2).
* It will [soon](http://www.reddit.com/r/rust/comments/33boew/weekend_experiment_link_rust_programs_against/) be possibly to [link Rust against MUSL](https://github.com/rust-lang/rust/pull/24777) instead of glibc on Linux in order to make completely self-contained binaries.
* [hlua's stack handling](https://medium.com/@tomaka/hlua-s-stack-handling-6b15ab309b17)
* [Release channels, bit branching, and the release process](http://internals.rust-lang.org/t/release-channels-git-branching-and-the-release-process/1940).
* [Cohort analysis of Rust contributors](http://sanxiyn.blogspot.com/2015/04/cohort-analysis-of-rust-contributors.html)
* [Pros and cons: Rust vs. C++](https://plus.google.com/+nialldouglas/posts/AXFJRSM8u2t)
* [Interactive 2D applications with Carboxyl and Elmesque](http://blog.ebopp.de/blog/2015/04/23/interactive-2d-apps/)
* [Servo: building a high-performance, safe web browser](http://blogs.s-osg.org/servo-adapting-c-to-work-on-the-web/).
* [A map of Rust contributors](https://github.com/jakub-/github-contributors-geojson/blob/master/rust.geojson). Thanks jakub-!
* [Reminder: the trains are running](http://internals.rust-lang.org/t/reminder-the-trains-are-running/1959)
* [My failed attempt to build a digital audio workstation in Rust](http://genesisdaw.org/post/progress-so-far.html)
* [Cross-compiling Rust for Raspberry Pi](https://github.com/Ogeon/rust-on-raspberry-pi)

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-04-21.md


# Project Updates

* [term-painter](http://www.reddit.com/r/rust/comments/32pgci/my_first_crate_easy_coloring_of_the_terminal/). Library for coloring and formatting terminal output, from Lukas Kalbertodt.
* [elmesque](http://www.reddit.com/r/rust/comments/339876/elmesque_elms_std_graphics_modules_ported_to_rust/). A graphics library based on Elm's.
* kvark [posted a new screenshot](https://www.reddit.com/r/rust_gamedev/comments/33w4ny/claymore_grid_screenshot_just_to_break_the_silence/) of his game, Claymore.
* tomaka released a [terminal backend for Glutin](https://www.reddit.com/r/rust_gamedev/comments/33xe2x/libcaca_backend_for_glutin/), based on [libcaca](http://r.duckduckgo.com/l/?kh=-1&uddg=http%3A%2F%2Fcaca.zoy.org%2Fwiki%2Flibcaca).
* ivanceras posted a new [screenshot](https://www.reddit.com/r/rust_gamedev/comments/3406qe/update_here_is_a_quick_cellshading_effect_on_my/) of his voxel renderer, [balisong](https://github.com/ivanceras/balisong).
* [rust-dispatch](https://github.com/SSheldon/rust-dispatch). Bindings to [Grand Central Dispatch](https://en.wikipedia.org/wiki/Grand_Central_Dispatch)
* [rustc noodling for code completion](http://phildawes.net/blog/2015/04/21/racer-rustc/). An update on code completion in [Racer](https://github.com/phildawes/racer).
* [This Week in Servo 31](http://blog.servo.org/2015/04/23/twis-31/)
* [RustDT IDE 0.2.0 released](http://users.rust-lang.org/t/rustdt-0-2-0-released-auto-complete-with-racer/1109)
* [itertools 0.3.0 released](http://bluss.github.io/rust/2015/04/25/releasing-itertools.0.3.0/)

# Upcoming Events

* [April 28. Sydney Hack Night](http://www.meetup.com/Rust-Sydney/events/221993570/)
* [May 5. San Diego](https://sandiego.rs)
* [May 11. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

```text
< Ms2ger> And note, unsafe code isn't for violating Rust's invariants, it's for maintaining them manually
```

Ms2ger in #rust.

Thanks to bluss for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
