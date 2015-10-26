Title: This Week in Rust 102
Number: 102
Date: 2015-10-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [Design patterns in Rust](http://www.ncameron.org/blog/design-patterns-in-rust/).
* [Managing connection state with mio](http://hermanradtke.com/2015/10/23/managing-connection-state-with-mio-rust.html).
* [Writing an OS in Rust](http://blog.phil-opp.com/).
* [Good practices for writing Rust libraries](https://pascalhertleif.de/artikel/good-practices-for-writing-rust-libraries/).
* [Rust and Swift](http://www.chriskrycho.com/2015/rust-and-swift-i.html).
* [This week in Redox 3](http://www.redox-os.org/news/this-week-in-redox-3/).
* [This week in Servo 38](http://blog.servo.org/2015/10/19/twis-38/).
* [Things I learned from Rust](https://dfockler.github.io/#learned-rust).
* [Messy error handling in Rust with `try!`](http://mortoray.com/2015/10/21/messy-error-handling-in-rust-with-try/).
* [podcast] [New Rustacean podcast episode 02](http://www.newrustacean.com/show_notes/e002/). The `struct` data type constructor, and the basics of Rust's "ownership" concept and "borrowing" and "moving".
* [podcast] [New Rustacean podcast episode 03](http://www.newrustacean.com/show_notes/e003/). Enumerated (`enum`) types, pattern matching, and meaningful return values.
* [Simple_parallel 0.3: Revisiting k-NN](https://huonw.github.io/blog/2015/10/simple_parallel-revisiting-knn/).

## Notable New Crates & Projects

* [Rust OS comparison](https://github.com/flosse/rust-os-comparison). A comparison of operating systems written in Rust.
* [Nucleon](https://github.com/NicolasLM/nucleon). Dynamic load balancer written in Rust.
* [Typenum](https://github.com/paholg/typenum). Compile time numbers in Rust.
* [Pumpkin](https://github.com/zcdziura/pumpkin). A cryptographically secure prime number generator.
* [Brotli-rs](https://github.com/ende76/brotli-rs). Brotli decompression in pure, safe Rust.
* [Multirust-rs](https://github.com/Diggsey/multirust-rs). A reimplementation of multirust in Rust.

# Updates from Rust Core

101 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-10-19..2015-10-26

See the [subteam report for 2015-10-23][subteam] for details.

[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-10-23/2821

## Notable changes

* [Implement RFC 1229: Turn statically known erroneous code into a warning and continue normal code-generation](https://github.com/rust-lang/rust/pull/28845).
* [Move desugarings to lowering step](https://github.com/rust-lang/rust/pull/28857).
* [Cargo: Implement `cargo install`](https://github.com/rust-lang/cargo/pull/2026).
* [Add the PNaCl/JS targets to the backend](https://github.com/rust-lang/rust/pull/28355).
* [Implement drain over a range for `VecDeque`](https://github.com/rust-lang/rust/pull/27723).
* [Owned conversions for CString](https://github.com/rust-lang/rust/pull/28977).
* [Remove `#[derive(Show)]`](https://github.com/rust-lang/rust/pull/29148).
* [std: Stabilize library APIs for 1.5](https://github.com/rust-lang/rust/pull/29254).

## New Contributors

* arcnmx
* Bryce Van Dyk
* Chris Drake
* Emanuel Czirai
* Irving A.J. Rivas Z.
* James McGlashan
* Lee Jenkins
* Michael Howell
* Philipp Oppermann
* skeleten
* Stefan O'Rear

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Allow overlapping implementations for marker traits](https://github.com/rust-lang/rfcs/pull/1268).
* [Promote the `libc` crate from the nursery](https://github.com/rust-lang/rfcs/pull/1291).
* [Enable the compiler to cache incremental workproducts](https://github.com/rust-lang/rfcs/pull/1298).
* [Add some additional utility methods to `OsString` and `OsStr`](https://github.com/rust-lang/rfcs/pull/1307).

## New RFCs

* [`src/grammar` for the canonical grammar of the Rust language](https://github.com/rust-lang/rfcs/pull/1331).

# Upcoming Events

* [10/27. Dinner at Tied House](http://www.meetup.com/Rust-Bay-Area/events/226146024/).
* [10/28. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [10/28. Rust Amsterdam](http://www.meetup.com/Rust-Amsterdam/events/225117486/).
* [10/28. RustBerlin Hack and Learn](http://www.meetup.com/Rust-Berlin/events/225614991/).
* [10/31. Rust Meetup Tokyo](https://rust-of-us.doorkeeper.jp/events/32615).
* [11/2. Women Who Code, Codemotion Berlin](http://berlin2015.codemotionworld.com/news/women-who-code-workshop-introduction-to-rust/).
* [11/4. PDXRust](http://www.meetup.com/PDXRust/events/225745776/).
* [11/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate & Quote of the Week

This week's Crate of the Week is [winapi-rs](https://github.com/retep998/winapi-rs). Thanks to [DanielKeep](https://users.rust-lang.org/t/crate-of-the-week/2704/64) for the suggestion. [Submit your suggestions for next week][submit_crate]!

> [@retep998](https://users.rust-lang.org/users/retep998) has, by and large, taken it upon himself to try and bind the Windows API using the sadistically horrible official `windows.h` header.  By hand.

> Having chipped away at the surface myself, let me tell you some of the horrors I've seen.  `windows.h` contains sub-headers that implicitly depend on other headers having been imported in a particular order.  Get the order wrong, and the definitions change.  This makes isolating *anything*, let alone defining where it canonically is supposed to come from a Sisyphean task.  It contains symbols that are deliberately defined multiple times.  Sometimes, with different types.  Sometimes, with different values.  Because `windows.h` was written by shambling horrors from beyond time and space.  That's not even touching the titanic, C'thuloid mass of brain-twisting agony that is the conditional compilation definitions and annotations sprinkled throughout like some kind of virulent pox.  There are obscure conditional flags that change or omit functions based on bizarre edge cases buried in some obscure file which is included three different ways by paths so obtuse you could make a credible argument for a satellite map of the damn thing except it'd have to be a six dimensional monstrosity that would drive you mad just from *looking* at it.  Then there's the vast tracts of API surface that are exposed via COM which Rust does absolutely *jack* toward making in any way usable so you have to write the damn vtables *by hand* and heaven *help* you if you run into an interface with multiple base interfaces because at that point you might as well give up and just use C because *at least* that has an IDL generator for it.  That's even *before* you realise that LLVM's code generation isn't even *correct* on Windows, and this whole time, it's been silently generating bad code for stdcall methods which you'd *think* would be just like functions but *nooo* that's now how Visual C++ works, so methods with particular return types are incompatible at the binary level, unless you know about this and manually correct for it by fudging the type signatures in the Rust binding code and really it's no wonder he's a rabbit because a human would have been driven so deep into madness they'd need be halfway to the Earth's firey core.

> To put it another way: I think it'd be kinda nice to give him a proverbial pat on the back for his herculean efforts thus far.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704
