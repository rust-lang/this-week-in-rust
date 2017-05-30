Title: This Week in Rust 128
Number: 128
Date: 2016-05-02
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

This week's edition was edited by: [Vikrant](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).

# Updates from Rust Community

## News & Blog Posts

* [Introducing MIR](http://blog.rust-lang.org/2016/04/19/MIR.html).
* [Myths and legends about integer overflow in Rust](https://huonw.github.io/blog/2016/04/myths-and-legends-about-integer-overflow-in-rust/).
* [How to run Ruby inside a Rust crate](http://anima-engine.org/blog/how-to-run-ruby-inside-a-crate/).
* [Porting a Haskell graphics framework to Rust](http://phaazon.blogspot.in/2016/04/porting-haskell-graphics-framework-to.html).
* [Mapping over Arrays in Rust](https://llogiq.github.io/2016/04/28/arraymap.html).
* [Optimizing matrix multiplication in Rust](http://www.suchin.co/2016/04/25/Matrix-Multiplication-In-Rust-Pt-1/).
* [Multithreaded matrix multiplication in Rust - Part II](https://athemathmo.github.io/2016/04/25/multithreading-multiplication-2.html).
* [Segfaults are our friends and teachers](http://kamalmarhubi.com/blog/2016/04/25/segfaults-are-our-friends-and-teachers/).
* [This week in Redox 14](http://www.redox-os.org/news/this-week-in-redox-14/).
* [This Week in Ruma 2016-04-24](https://www.ruma.io/news/this-week-in-ruma-2016-04-24/). Ruma is a Matrix client-server API written in Rust.
* [The state of Rust docs](https://facility9.com/2016/04/the-state-of-rust-docs-2016/).

## Notable New Crates & Project Updates

* [Rust project changelog for 2016-04-29](https://users.rust-lang.org/t/rust-project-changelog-for-2016-04-29/5613). Updates to bitflags, lazy_static, regex, rust-mode, rustup, uuid.
* [Xi Editor](https://github.com/google/xi-editor). A modern editor with a backend written in Rust.
* [rure](https://github.com/rust-lang-nursery/regex/tree/master/regex-capi). A C API for the regex crate.
* [cassowary-rs](https://github.com/dylanede/cassowary-rs). A Rust implementation of the Cassowary constraint solving algorithm.
* [Sapper](https://github.com/sappworks/sapper). A lightweight web framework built on async hyper, implemented in Rust language.
* [servo-vdom](https://github.com/LorenVS/servo-vdom). A modified servo browser which accepts content patches over an IPC channel.
* [rustr and rustinr](http://rustr.org/). Rust library for working with R, and an R package to generate Rust interfaces.
* [Rorschach](https://github.com/meh/rorschach). Pretty print binary blobs based on common layout definition.

# Crate of the Week

This week's Crate of the Week is [arrayvec](https://crates.io/crates/arrayvec), which gives us a `Vec`-like interface over plain arrays for those instances where you don't want the indirection. Thanks to [ehiggs](https://users.rust-lang.org/users/ehiggs) for the suggestion!

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [Help improve DuckDuckGo's Rust-related searches](https://www.reddit.com/r/rust/comments/4gujbf/help_improve_duckduckgos_rustrelated_searches/).
* [easy] [rust: Add error explanations for all error codes](https://github.com/rust-lang/rust/issues/32777).
* [easy] [servo/highfive: Add a comment when a PR receives a push](https://github.com/servo/highfive/issues/101).
* [easy] [servo/devices: Minimize duplication for unsupported platforms](https://github.com/servo/devices/issues/8).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

92 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-04-25..2016-05-02

* [new `Entry::key()` method](https://github.com/rust-lang/rust/pull/33148)
* [`Clone::clone()` no longer deep-clones `Copy` types](https://github.com/rust-lang/rust/pull/31414)
* [`IPV6_V6ONLY` removed](https://github.com/rust-lang/rust/pull/33263) (breaking change!)
* [`-Z save_analysis` now uses JSON format](https://github.com/rust-lang/rust/pull/33208)
* [`trans::collector` improvements](https://github.com/rust-lang/rust/pull/33171)
* [`pub(restricted)` on tuple struct fields](https://github.com/rust-lang/rust/pull/33161)
* [simplify AST→HIR lowering by removing reproducibility](https://github.com/rust-lang/rust/pull/33296)
* [New "Rust Friends" page](https://github.com/rust-lang/rust-www/pull/346)

## New Contributors

* Andy Russell
* Brayden Winterton
* Demetri Obenour
* Ergenekon Yigit
* Jonathan Turner
* Michael Tiller
* Timothy McRoy
* Tomáš Hübelbauer

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week!*.

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Float-free libcore](https://github.com/rust-lang/rfcs/pull/1596).
* [`as_millis` function on `std::time::Duration`](https://github.com/rust-lang/rfcs/pull/1547).
* [Add `TryFrom` and `TryInto` traits](https://github.com/rust-lang/rfcs/pull/1542).
* [Add workspaces to Cargo](https://github.com/rust-lang/rfcs/pull/1525).
* [Specifying that `<T as Clone>::clone(&t)` where `T: Copy` should be equivalent to `ptr::read(&t)`](https://github.com/rust-lang/rfcs/pull/1521).
* [Proposal for thread affinity](https://github.com/rust-lang/rfcs/pull/1480).
* [Add `#[repr(align = "N")]`](https://github.com/rust-lang/rfcs/pull/1358).

## New RFCs

* [Introduce `dyn` keyword](https://github.com/rust-lang/rfcs/pull/1603).
* [Amend RFC 1268 with a more feasible proposal post-specialization](https://github.com/rust-lang/rfcs/pull/1600).
* [Associated type constructors (a form of higher-kinded polymorphism)](https://github.com/rust-lang/rfcs/pull/1598).

# Upcoming Events

* 5/4. Rust Community Team Meeting at #rust-community on irc.mozilla.org.
* [5/4. OpenTechSchool Berlin: Rust Hack and Learn](http://www.meetup.com/opentechschool-berlin/).
* [5/4. Cologne / Germany: Rust Anniversary Meetup](http://www.meetup.com/de-DE/Rust-Cologne-Bonn/events/230641335/).
* [5/4. Wellington Rust meetup](http://www.meetup.com/Wellington-Rust-Meetup/events/230650719/).
* 5/5. Rust London Meetup #5 at Mozilla London.
* [5/5. Rust Sydney Hack-night](http://www.meetup.com/Rust-Sydney/events/230536981/).
* [5/9. Seattle Rust Meetup](https://www.eventbrite.com/e/mozilla-rust-seattle-meetup-tickets-12222326307?aff=erelexporg).
* [5/10. (San Diego) Eat– Drink– Rust! Downtown Rust Meetup](http://www.meetup.com/San-Diego-Rust/events/230492925/).
* [5/10. Rust NYC: Systems Programming (in Rust) and Tasting (in Beer)](http://www.meetup.com/Rust-NYC/events/230401806/).
* [5/11. Rust Boulder/Denver Monthly Meeting](http://www.meetup.com/Rust-Boulder-Denver/).
* [5/11. Rust LA Monthly Meetup - Hack Night](http://www.meetup.com/Rust-Los-Angeles/events/230365091/).
* [5/12. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [5/12. Rust Bay Area](http://www.meetup.com/Rust-Bay-Area/events/230624722/).
* [5/13. Rust Meetup Darmstadt](http://www.meetup.com/de-DE/Rust-Rhein-Main/events/230396961/).
* [5/13. Hack Chiapas, Autonomous University of Chiapas, Mexico](http://hackchiapas.com/).
* [5/14. Rust Minsk](http://www.meetup.com/Rust-%D0%B2-%D0%9C%D0%B8%D0%BD%D1%81%D0%BA%D0%B5/events/229283305/).
* [5/16. Rust Paris](http://www.meetup.com/Rust-Paris).
* [5/17. Moscow Rust Conference](https://rustycrate.ru/%D0%BD%D0%BE%D0%B2%D0%BE%D1%81%D1%82%D0%B8/2016/04/25/colaboratory-rust.html).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In general, enough layers of Rc/RefCell will make anything work.

[gkoz on TRPLF](https://users.rust-lang.org/t/how-to-get-static-lifetime/5552/8).

Thanks to [birkenfeld](https://users.rust-lang.org/users/birkenfeld) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
