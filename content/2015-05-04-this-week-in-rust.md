Title: This Week in Rust 80
Date: 2015-05-04
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

115 pull requests were [merged in the last week][merged], and 2 [RFC PRs][rfcs].

[merged]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-27..2015-05-04
[rfcs]: https://github.com/rust-lang/rfcs/pulls?q=is%3Apr+is%3Amerged+merged%3A2015-04-27..2015-05-04

Now you can follow breaking changes *[as they happen][BitRust2]*!

[BitRust2]: http://killercup.github.io/bitrust/

# Breaking Changes

* [Inspect enum discriminant *after* calling its destructor](https://github.com/rust-lang/rust/pull/24765)
* [Add downcasting to std::error::Error](https://github.com/rust-lang/rust/pull/24793)
* [dropck: Remove `Copy` from special-cased traits](https://github.com/rust-lang/rust/pull/24906)
* [std: Remove index notation on slice iterators](https://github.com/rust-lang/rust/pull/25006)

# Other Changes

* [Implement associated constants](https://github.com/rust-lang/rust/pull/23606)
* [Expand the area of std::fs](https://github.com/rust-lang/rust/pull/24711)
* [Make `UnsafeCell`, `RefCell`, `Mutex`, and `RwLock` accept DSTs](https://github.com/rust-lang/rust/pull/24737)
* [Experimental MUSL target support](https://github.com/rust-lang/rust/pull/24777)
* [Implement Vec::drain from RFC 574](https://github.com/rust-lang/rust/pull/24781)
* [Add single-threaded fence intrinsics](https://github.com/rust-lang/rust/pull/24833)
* [Provide a Default implementation for AtomicPtr](https://github.com/rust-lang/rust/pull/24834)
* [std: Fix inheriting standard handles on windows](https://github.com/rust-lang/rust/pull/24873)
* [Add intrinsics for unchecked division and modulo](https://github.com/rust-lang/rust/pull/24886)
* robinst, bguiz and michaelsproul landed more extended error diagnostics: [1][e1], [2][e2], [3][e3].
* [derive: Give access to field attributes in ext::deriving](https://github.com/rust-lang/rust/pull/25027)
* [Implement String::drain(range) according to RFC 574](https://github.com/rust-lang/rust/pull/25028)

[e1]: https://github.com/rust-lang/rust/pull/24892
[e2]: https://github.com/rust-lang/rust/pull/24893
[e3]: https://github.com/rust-lang/rust/pull/24894
[e4]: https://github.com/rust-lang/rust/pull/24975

# New Contributors

* Brendan Graetz
* Carol (Nichols || Goulding)
* critiqjo
* Dominic van Berkel
* Hech
* Jan Bujak
* J Bailey
* jooert
* Jordan Humphreys
* Poga Po
* sinkuu
* Xuefeng Wu

# Approved RFCs

* [RFC 1040: Duration](https://github.com/rust-lang/rfcs/blob/master/text/1040-duration-reform.md)

# New RFCs

* [Guaranteed non-static dtors](https://github.com/rust-lang/rfcs/pull/1094)
* [Add unsafe Option::unwrap_unchecked](https://github.com/rust-lang/rfcs/pull/1095)
* [Remove static_assert](https://github.com/rust-lang/rfcs/pull/1096)
* [`panic!` handlers](https://github.com/rust-lang/rfcs/pull/1100)
* [Rename `connect` to `join`](https://github.com/rust-lang/rfcs/pull/1102)
* [Policy on semver and API evolution](https://github.com/rust-lang/rfcs/pull/1105)

# Betawatch!

The current beta is `rustc 1.0.0-beta.4 (850151a75 2015-04-30)`.

There were 2 PRs this week landing backports to beta.

* [25004](https://github.com/rust-lang/rust/pull/25004)
* [24943](https://github.com/rust-lang/rust/pull/24943)

# Notable Links

* [Where Rust really shines](http://manishearth.github.io/blog/2015/05/03/where-rust-really-shines/)
* [Using the `Option` type effectively](http://blog.8thlight.com/uku-taht/2015/04/29/using-the-option-type-effectively.html)
* [A Rust contributor tries their hand at Go](http://www.polyglotweekly.com/2015/04/24/thoughts-of-a-rustacean-learning-go.html)
* [On reference counting and leaks](http://smallcultfollowing.com/babysteps/blog/2015/04/29/on-reference-counting-and-leaks/) and [a few more remarks](http://smallcultfollowing.com/babysteps/blog/2015/04/30/a-few-more-remarks-on-reference-counting-and-leaks/).
* [Rust web lessons based on nickel.rs and Angular.js](https://github.com/Codenator81/nickel-demo). A tutorial on using nickel.
* [Helping Travis catch the rustc train](http://huonw.github.io/blog/2015/04/helping-travis-catch-the-rustc-train/) and [Travis on the train, part 2](http://huonw.github.io/blog/2015/05/travis-on-the-train-part-2/).
* [rustfmt - help wanted](http://www.reddit.com/r/rust/comments/34cyas/rustfmt_help_wanted/)
* [I got Rust working on a $13 ARM micro controller board. Here are my notes from the process](http://antoinealb.net/programming/2015/05/01/rust-on-arm-microcontroller.html)
* [Evaluation of performance and productivity metrics of potential programming languages in the HPC environment](https://github.com/MrFloya/thesis-ba/raw/master/tex/thesis.pdf). MrFloya's bachelor's thesis.
* [A journey into iterators](http://hoverbear.org/2015/05/02/a-journey-into-iterators/)
* [Abomination: terrifying serialization](http://www.frankmcsherry.org/serialization/2015/05/04/unsafe-at-any-speed.html)
* [Weekly meeting 2015-04-28](https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2015-04-28.md). Snapshots, transmute, static_assert.
* [Core team meeting 2015-04-29](https://github.com/rust-lang/meeting-minutes/blob/master/core-team/meeting-2015-04-29.md). Optimizing by default, mem::forget, Debug builders, security policy.

# Project Updates

* [Piston - skeletal animations with dual-quaternions](https://www.reddit.com/r/rust_gamedev/comments/34bdrc/piston_skeletal_animation_with_dualquaternions/). Video.
* [hprof](https://www.reddit.com/r/rust_gamedev/comments/34r6o0/hprof_a_simple_hierarchical_profiler/). A simple hierarchical profiler.
* [Baby steps at procedural tile generation in kvarkus's Claymore](https://www.reddit.com/r/rust_gamedev/comments/34sbqw/baby_steps_at_procedural_tile_generation_rivers/).
* [Google APIs for Rust - Dev Diary #2: Making CLIs (video)](https://youtu.be/wHlE1pNThjE)
* [extra_lints](http://www.reddit.com/r/rust/comments/349gf2/extra_lints_for_rust/). More lints!
* [RustyAndroid](https://github.com/pakoito/RustyAndroid). Sample Rust Android application.
* [Servo continues pushing forward](http://blogs.s-osg.org/servo-continues-pushing-forward/)
* [Racer on Rust beta!](http://phildawes.net/blog/2015/05/02/racer-on-rust-beta/)
* [bounded-spsc-queue](https://github.com/polyfractal/bounded-spsc-queue).
* [Horrorshow](https://users.rust-lang.org/t/horrorshow-a-poc-html-templating-library/1215). An HTML templating macro.
* [rust-chunked-transfer](https://github.com/frewsxcv/rust-chunked-transfer). Encoder and decoder for HTTP chunked transfer.
* [kademlia-rs](https://github.com/leejunseok/kademlia-rs). A Rust implementation of the Kademlia DHT.

# Upcoming Events

* [May 5. San Diego](https://sandiego.rs)
* [May 11. Seattle](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg)
* [May 15. Amsterdam](http://www.meetup.com/Rust-Amsterdam/events/221569942/)
* [May 15. Berlin](http://www.meetup.com/Rust-Berlin/events/222087122/)
* [May 15. Boston](http://www.meetup.com/Boston-Rust-Meetup-25317522aNpHwZdw/events/222181765/?a=ra1_te)
* [May 15. Columbus Rust Society](http://www.meetup.com/columbus-rs/events/222044818/)
* [May 15. Copenhagen](http://www.eventbrite.com/e/copenhagen-rust-launch-party-tickets-16663376608)
* [May 15. Florence (@develer)](http://workshop.develer.com/post/117155582878/15-maggio-italian-rust-1-0-release-party)
* [May 15. Florence (@Unifi)](http://www.lilik.it/aperitivo-di-lancio-per-rust-1-0/)
* [May 15. London](http://www.meetup.com/Rust-London-User-Group/events/221897841/)
* [May 15. Los Angeles](http://www.eventbrite.com/e/rust-10-meetup-los-angeles-tickets-16776976388)
* [May 15. Madrid](http://www.meetup.com/Rust-Madrid/events/222118916/)
* [May 15. Moscow](http://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%BE%D1%81%D0%BA%D0%B2%D0%B5/events/222092229/)
* [May 15. Munich](https://github.com/Byron/depot/issues/1)
* [May 15. NYC](http://www.meetup.com/Rust-NYC/events/222018027/)
* [May 15. Paris](http://www.meetup.com/Rust-Paris/events/219680068/)
* [May 15. Pittsburgh](https://www.eventbrite.com/e/rust-10-release-party-tickets-16790008367)
* [May 15. Portland](http://www.meetup.com/PDXRust/events/222057357/)
* [May 15. San Francisco](http://www.meetup.com/Rust-Bay-Area/events/221743903/)
* [May 15. Toronto](http://www.meetup.com/Rust-Toronto/events/222014412/)
* [May 15. Washington, DC](https://www.eventbrite.com/e/rust-10-celebration-dc-tickets-16715560692?ref=estw)
* [May 15. Warsaw](http://www.meetup.com/Rust-Warsaw/events/220171771/)
* [May 16. Chengdu](https://jinshuju.net/f/grrXK8)
* [May 16. Seoul](http://rust-kr.org/pages/meetup-2015-05-16)
* [May 16. Tokyo](http://rust-samurai.connpass.com/event/14649/)
* [May 19. Sydney](http://www.meetup.com/Rust-Sydney/events/221946886/)
* [May 22. Lambdaconf. Boulder, CO](http://www.degoesconsulting.com/lambdaconf-2015/). Jared Roesch - "Who got types in my systems programming?"
* [May 23. Bangalore](https://reps.mozilla.org/e/rust-1-0-release-party-bangalore/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# Quote of the Week

*"Ultimately, I think this all boils down to the fact that borrowck only cares about reachable
values. A leaked value isn't reachable, therefore it doesn't matter
that it had a lifetime associated with it and technically outlives that
lifetime, since it's not reachable no undefined behavior can be invoked."*

[Insight from kballard on the safety of linking](https://github.com/rust-lang/rfcs/pull/1084#issuecomment-96875651).

Thanks to Gankro for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
