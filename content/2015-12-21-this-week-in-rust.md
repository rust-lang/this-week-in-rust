Title: This Week in Rust 110
Number: 110
Date: 2015-12-21
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

* [What one must understand to be productive with Rust](https://medium.com/@ericdreichert/what-one-must-understand-to-be-productive-with-rust-e9e472116728).
* [Rayon: Data parallelism in Rust](http://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/).
* [Programming with Rust](http://hackaday.com/2015/12/18/programming-with-rust/). Rust for hardware hackers.
* [Rendering an animation in Rust](http://www.willusher.io/2015/12/16/rendering-an-animation-in-rust/).
* [Procedural macros, framework](http://www.ncameron.org/blog/procedural-macros-framework/). Nick's continued efforts for a better macro system in Rust.
* [Differential geometry in Rust](http://ebvalaim.mydevil.net/en/2015/12/18/differential-geometry-in-rust/).
* [Adding community-driven Wayland support to Servo](http://blogs.s-osg.org/community-driven-wayland-support-servo/).
* [Surfaces and signatures: Component privacy versus dependence](http://blog.pnkfx.org/blog/2015/12/19/signatures-and-surfaces-thoughts-on-privacy-versus-dependency/).

## Notable New Crates & Project Updates

* [rustfmt](https://github.com/rust-lang-nursery/rustfmt) now comes with a Cargo subcommand `fmt`.
* [Rayon](https://github.com/nikomatsakis/rayon). A data parallelism library for Rust.
* [Ruplicity](https://github.com/mbrt/ruplicity). Rust library managing duplicity backups.
* [Floki](https://github.com/arthurprs/floki). A message queue in Rust, inspired by Apache Kafka and Amazon SQS.
* [rustlearn](https://github.com/maciejkula/rustlearn). A machine learning package for Rust.
* [Corange-rs](https://github.com/lucidscape/corange-rs). Rust bindings for the Corange game engine.

# Updates from Rust Core

109 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2015-12-14..2015-12-21

See the [triage digest][triage] and [subteam reports][subteam] for more details.

[triage]: https://internals.rust-lang.org/t/triage-digest-mon-dec-21-2015/3021
[subteam]: https://internals.rust-lang.org/t/subteam-reports-2015-12-21/3020

## Notable changes

* [`memchr` and `memrchr` in std](https://github.com/rust-lang/rust/pull/30381).
* [privacy: Rewrite VisiblePrivateTypesVisitor](https://github.com/rust-lang/rust/pull/29973).
* [Book: First draft of primitive types](https://github.com/rust-lang/book/pull/34).
* [Book: New chapter: Comments](https://github.com/rust-lang/book/pull/32).
* [Book: New chapter: Functions](https://github.com/rust-lang/book/pull/31).
* [Make RFC 1214 warnings into errors](https://github.com/rust-lang/rust/pull/30389). [RFC 1214](https://github.com/rust-lang/rfcs/blob/master/text/1214-projections-lifetimes-and-wf.md): Clarify (and improve) rules for projections and well-formedness.
* [Better support for `--llvm-root`](https://github.com/rust-lang/rust/pull/27937).
* [Ensure borrows of fn/closure params do not outlive invocations](https://github.com/rust-lang/rust/pull/30341).
* [Make name resolution errors non-fatal](https://github.com/rust-lang/rust/pull/30320).
* [Move built-in syntax extensions to a separate crate](https://github.com/rust-lang/rust/pull/30300).
* [Implement `#[deprecated]` attribute (RFC 1270)](https://github.com/rust-lang/rust/pull/30206).
* [Partially implement type ascription](https://github.com/rust-lang/rust/pull/30184).
 
## New Contributors

* Ed Clarke
* faineance
* fbergr
* Shiney
* Steve Wooster
* Zach Reizner

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1328: Allow a custom panic handler](https://github.com/rust-lang/rfcs/pull/1328).
* [Amend RFC 550 with (expanded) abstract specification rather than algorithm](https://github.com/rust-lang/rfcs/pull/1384).

## Final Comment Period

Every week [the team](https://rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/issues?utf8=%E2%9C%93&q=is%3Apr+org%3Arust-lang+label%3Afinal-comment-period+is%3Aopen

* [Improve Cargo target-specific dependencies](https://github.com/rust-lang/rfcs/pull/1361).
* [Add a `IndexAssign` trait that allows overloading "indexed assignment" expressions like `a[b] = c`](https://github.com/rust-lang/rfcs/pull/1129).
* [Allow eliding more type parameters](https://github.com/rust-lang/rfcs/pull/1196).
* [Add an `alias` attribute to `#[link]` and `-l`](https://github.com/rust-lang/rfcs/pull/1296).

## New RFCs

* [Generalize the delayed resolution of language items to arbitrary items](https://github.com/rust-lang/rfcs/pull/1408).
* [Add a `noalias` language item](https://github.com/rust-lang/rfcs/pull/1410).
* [Add Rvalue-static-promotion](https://github.com/rust-lang/rfcs/pull/1414).
* [Deprecate type aliases in `std::os::*::raw`](https://github.com/rust-lang/rfcs/pull/1415).
* [Safe `memcpy`, `memset` for slices `([T]::{ copy_from, fill })`](https://github.com/rust-lang/rfcs/pull/1419).
* [pub(restricted) item](https://github.com/rust-lang/rfcs/pull/1422). Expand the current `pub`/non-`pub` categorization of items with the ability to say "make this item visible *solely* to a (named) module tree.

# Upcoming Events

*No upcoming events for next two weeks.*

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

* [Technical Lead/Manager](https://ebip.co.uk/careers) at EBI Portfolios.
* [Research Engineer - Servo](https://careers.mozilla.org/en-US/position/ozy21fwU) at Mozilla.
* [Senior Research Engineer - Rust](https://careers.mozilla.org/en-US/position/o0H41fww) at Mozilla.
* [Open Source Software Engineer](http://maidsafe.net/careers) at MaidSafe.
* [Mulitple positions](http://rust.jobboard.io/employers/6824-ironnet-cybersecurity) at IronNet Cybersecurity.

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Crate of the Week

This week's Crate of the Week is [clippy](https://github.com/Manishearth/rust-clippy), a collection of lints for better Rust code. Alas, being a plugin, it only runs on nightly Rust, but that's easy to do with [multirust](https://github.com/brson/multirust). Thanks to [leodasvacas](https://users.rust-lang.org/users/leodasvacas) for the suggestion.

[Submit your suggestions for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Quote of the Week

> Do or do not. There is a try!

— Jonathan Turner in a [blog post](http://www.jonathanturner.org/2015/11/learning-to-try-things-in-rust.html).

Thanks to [Vikrant](https://users.rust-lang.org/users/nasa42) for the tip.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
