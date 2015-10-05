Title: This Week in Rust 99
Number: 99
Date: 2015-10-05
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

This week's edition was edited by: [nasa42](https://github.com/nasa42), [brson](https://github.com/brson), and [llogiq](https://github.com/llogiq).


# Updates from Rust Community

## News & Blog Posts

* [When Rust makes sense, or the state of typed languages](https://m50d.github.io/2015/09/28/when-rust-makes-sense.html).
* [podcast] [Rusty radio: Episode 4](http://rustyrad.io/podcast/4/). Raft, Paxos, and Distributed Systems in Rust.
* [This week in Servo 35](http://blog.servo.org/2015/09/28/twis-35/).
* [Resurrecting impl Trait](https://aturon.github.io/blog/2015/09/28/impl-trait/).
* [Combining Rust and Haskell](http://tab.snarc.org/posts/haskell/2015-09-29-rust-with-haskell.html).
* [video] [Using Rust with Ruby, a deep dive with Yehuda Katz](https://www.youtube.com/watch?v=IqrwPVtSHZI).
* [How to print a struct in Rust](https://joelmccracken.github.io/entries/how-to-print-a-struct-in-rust/).
* [Trying Rust for web services](https://blog.wearewizards.io/trying-rust-for-web-services).
* [An introduction to differential dataflow, part 1](https://github.com/frankmcsherry/blog/blob/master/posts/2015-09-29.md).
* [Experiences building an OS in Rust](https://mostlytyped.com/posts/experiences-building-an-os-in-ru).
* [Ownership is theft: Experiences building an embedded OS in Rust](http://amitlevy.com/papers/tock-plos2015.pdf).
* [Rust faster!](https://llogiq.github.io/2015/10/03/fast.html)
* [podcast] [New Rustacean podcast episode 01](http://www.newrustacean.com/show_notes/e01/index.html). Documentation in general, and `rustdoc` and `cargo doc` in particular.
* [Rusty queens](http://jd.ekstrandom.net/blog/2015/10/rusty-queens). An n-queens solver in Rust.

## Notable New Crates & Projects

* [Redox](https://github.com/redox-os/redox). A Rust Operating System.
* [Webrender](https://github.com/glennw/webrender/wiki). An experimental renderer for Servo that aims to draw web content like a modern game engine.
* [Coroutine I/O](https://github.com/zonyitoo/coio-rs). Coroutine scheduling with work-stealing algorithm.
* [Rustation](https://github.com/simias/rustation). PlayStation emulator in Rust.

# Updates from Rust Core

102 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-09-28..2015-10-05

## Notable changes

* [Backporting accepted PRs to beta](https://github.com/rust-lang/rust/pull/28802).
* [Use the *adjusted* callee type in effect checking](https://github.com/rust-lang/rust/pull/28790).
* [Derive `Clone` for `Peekable`](https://github.com/rust-lang/rust/pull/28738).
* [Make `fs::canonicalize` work on directories on Windows](https://github.com/rust-lang/rust/pull/28729).
* [Don't crash on non-existent path in constant](https://github.com/rust-lang/rust/pull/28686).
* [Un-regress conflicting destructors](https://github.com/rust-lang/rust/pull/28681).
* [Don't use jemalloc when crossing to MSVC](https://github.com/rust-lang/rust/pull/28668).
* [Implement `AsMut` for `Vec`](https://github.com/rust-lang/rust/pull/28663).
* [Fix Cygwin support on Windows 10](https://github.com/rust-lang/rust-installer/pull/44).
* [Don't display duplicate trait errors](https://github.com/rust-lang/rust/pull/28645).
* [Early-prohibit objects with Self-containing supertraits](https://github.com/rust-lang/rust/pull/28629).
* [Swap link order of native libs/rust deps](https://github.com/rust-lang/rust/pull/28605).
* [Add support for the rumprun unikernel](https://github.com/rust-lang/rust/pull/28593).
* [Don't ICE if an archive isn't actually an archive](https://github.com/rust-lang/rust/pull/28673).
* [Avoid unnecessary temporaries when ref'ing a DST value](https://github.com/rust-lang/rust/pull/28787).
* [Cargo: Do not skip the root path if it's a dotdir](https://github.com/rust-lang/cargo/pull/2019).

## New Contributors

* Andreas Sommer
* Dato Simó
* James Bell
* Jethro Beekman
* Seeker14491
* Ted Mielczarek
* Will Speak
* Willy Aguirre

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

* [Amend #911 const-fn to allow unsafe const functions](https://github.com/rust-lang/rfcs/pull/1245).
* [Place left arrow syntax (`place <- expr`)](https://github.com/rust-lang/rfcs/pull/1228).
* [Allow a re-export for `main`](https://github.com/rust-lang/rfcs/pull/1260).

## New RFCs

* [Incremental compilation](https://github.com/rust-lang/rfcs/pull/1298).
* [Add a `SharedSender` to `std::sync::mpsc` that implements `Sync`](https://github.com/rust-lang/rfcs/pull/1299).
* [Define the general semantics of intrinsic functions](https://github.com/rust-lang/rfcs/pull/1300).
* [Add a `let...else` expression, similar to Swift's `guard let...else`](https://github.com/rust-lang/rfcs/pull/1303).
* [Abstract output type parameters](https://github.com/rust-lang/rfcs/pull/1305).
* [Add some additional utility methods to `OsString` and `OsStr`](https://github.com/rust-lang/rfcs/pull/1307).

# Upcoming Events

* [10/12. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [10/13. San Diego Rust Meetup #9](http://www.meetup.com/San-Diego-Rust/events/225389095/).
* 10/14. RustBerlin Hack and Learn.
* [10/19. Rust Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week, Crate of the Week is [Itertools](https://github.com/bluss/rust-itertools). Thanks go to [llogiq](https://github.com/llogiq) for the suggestion. In his own words:

> So today I'll write about [Itertools](https://github.com/bluss/rust-itertools). Because iterators in Rust are awesome, and this crates makes them even awesome-r. If you want to do something with iterators that seems to be slightly impossible using the `std` APIs, chances are Itertools already implements a way that is both fast and elegant. Knowing your itertools APIs will level up your Rust-fu.
> 
> For a (very small and simple) example, haven't you wished to zip two iterators, but don't stop iteration after the shorter iterator has run out? With Itertools you can just say `x.zip_longest(y)` and get an iterator of `EitherOrBoth<X, Y>`.

# Quote of the Week

*In programming (as opposed to politics), safety=freedom.* — [llogiq on /r/rust](https://www.reddit.com/r/rust/comments/3mofy0/when_rust_makes_sense_or_the_state_of_typed/cvgpwke).

Thanks to [birkenfeld](https://users.rust-lang.org/users/birkenfeld) for the tip. [Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
