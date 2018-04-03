Title: This Week in Rust 228
Number: 228
Date: 2018-04-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.25](https://blog.rust-lang.org/2018/03/29/Rust-1.25.html). 🎉🎈
* [Tokio: New Timer implementation](https://tokio.rs/blog/2018-03-timers/).
* [Refactoring some repetitive code to a Rust macro](https://people.gnome.org/~federico/blog/refactoring-some-repetitive-code-to-a-macro.html).
* [History of the Rand crate](https://www.reddit.com/r/rust/comments/87qy40/history_of_the_rand_crate/).
* [HTTP upgrades with hyper](http://seanmonstar.com/post/172531530657/http-upgrades-with-hyper).
* [Porting an academic C++ library to Rust to do analysis on Doom maps](https://eev.ee/blog/2018/03/30/a-geometric-rust-adventure/).
* [Building tiny Rust binaries for embedded Linux](https://jamesmunns.com/update/2018/04/01/tinyrocket.html).
* [Speeding Up `dwarfdump` With Rust](https://robert.ocallahan.org/2018/03/speeding-up-dwarfdump-with-rust.html).
* [Analysing crates.io data for top dependencies](https://tirkarthi.github.io/rust/2018/03/30/analyzing-crates-data.html).
* [This Week in Rust Docs 99](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-99).
* [The Embedded Working Group Newsletter 2](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-2/7176).
* [podcast] [New Rustacean: Rust 1.25](https://newrustacean.com/show_notes/news/rust_1_25/). Paths and matches and SIMD, cargo new changes, and tons of community-driven learning materials.

# Crate of the Week

This week's crate is [Ditto](https://github.com/alex-shapiro/ditto) - CRDTs for common data structures like maps, vecs, sets, text, and JSON. Thanks to [nasa42](https://users.rust-lang.org/u/nasa42) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* If you’re interested in language design, please help polish up [the new Delegation RFC draft](https://internals.rust-lang.org/t/new-rfc-for-delegation-anyone-interested-in-contributing/6644/8).
* [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide) is a project to write a short guide about how the rust compiler works, and it needs your help. There are some [easier issues](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AEasy), [issues which might require a bit of investigation/code reading](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3AMedium), and [issues which probably require some advanced knowledge or a lot of time](https://github.com/rust-lang-nursery/rustc-guide/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+label%3AHard).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

149 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-03-26..2018-04-02

* [Stabilize `std::process::id()`](https://github.com/rust-lang/rust/pull/49574).
* [Stabilize method `String::retain`](https://github.com/rust-lang/rust/pull/49243).
* [Stabilize `match_default_bindings`](https://github.com/rust-lang/rust/pull/49394).
* [Stabilize `fs::read` and `fs::write`](https://github.com/rust-lang/rust/pull/49422).
* [Rename `fs::read_string` to `read_to_string` and stabilize](https://github.com/rust-lang/rust/pull/49522).
* [Stabilize underscore lifetimes](https://github.com/rust-lang/rust/pull/49458).
* [Enable target_feature on any LLVM 6+](https://github.com/rust-lang/rust/pull/49428).
* [Forbid `#[inline(always)]` with `#[target_feature]`](https://github.com/rust-lang/rust/pull/49425).
* [Stabilize TryFrom / TryInto, and tweak impls for integers](https://github.com/rust-lang/rust/pull/49305).
* [Revert "Add TryFrom and TryInto to the prelude"](https://github.com/rust-lang/rust/pull/49518).
* [Add basic PGO support](https://github.com/rust-lang/rust/pull/48346).
* [Add `slice::sort_by_cached_key` as a memoised `sort_by_key`](https://github.com/rust-lang/rust/pull/48639).
* [Rename RangeArgument to RangeBounds, move it and Bound to libcore](https://github.com/rust-lang/rust/pull/49163).
* [Group linked libraries where needed](https://github.com/rust-lang/rust/pull/49316).
* [Cargo: Add description for each `-Z` flag in cargo help](https://github.com/rust-lang/cargo/pull/5235).
* [Add `is_whitespace` and `is_alphanumeric` to str](https://github.com/rust-lang/rust/pull/49381).
* [Cargo: Run `rustc` for information fewer times](https://github.com/rust-lang/cargo/pull/5249).
* [Expand Attributes on Statements and Expressions](https://github.com/rust-lang/rust/pull/49124).
* [Move the `alloc::allocator` module to `core::heap`](https://github.com/rust-lang/rust/pull/49481).
* [Introduce trait engine](https://github.com/rust-lang/rust/pull/49202).
* [Fail the build if we build Cargo twice](https://github.com/rust-lang/rust/pull/49053).

## New Contributors

* Anders Pitman
* matthew

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Unnamed fields of struct and union type](https://github.com/rust-lang/rfcs/pull/2102).
* [disposition: merge] [Semantic build scripts for Cargo](https://github.com/rust-lang/rfcs/pull/2196).
* [disposition: merge] [Extend pattern API to OsStr](https://github.com/rust-lang/rfcs/pull/2295).
* [disposition: merge] [Type alias enum variants](https://github.com/rust-lang/rfcs/pull/2338).
* [disposition: postpone] [Formalise reborrows](https://github.com/rust-lang/rfcs/pull/2364).
* [disposition: postpone] [Non-selfexhausting Drain](https://github.com/rust-lang/rfcs/pull/2369).

## New RFCs

* [Allow Items to be grouped into "Item-level scopes"](https://github.com/rust-lang/rfcs/pull/2377).
* [Lint Reasons](https://github.com/rust-lang/rfcs/pull/2383).
* [Unless/Until](https://github.com/rust-lang/rfcs/pull/2384).
* [Implied `#[derive(SuperTrait)]`](https://github.com/rust-lang/rfcs/pull/2385).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Apr  5. Madrid, ES - Segundo meetup de MadRust](https://www.meetup.com/MadRust/events/248884690/).
* [Apr  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr  8. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgblb/).
* [Apr  9. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxgbmb/).
* [Apr 10. Rome, IT - CLI App + Error Handling - Rust Roma #Aperitech](https://www.meetup.com/Rust-Roma/events/249232048/).
* [Apr 10. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Apr 11. Munich, DE - Fun with Rust and Numerical Methods](https://www.meetup.com/rust-munich/events/248055969/).
* [Apr 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 11. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Apr 11. Denver, US - April Meetup in Boulder](https://www.meetup.com/Rust-Boulder-Denver/events/248792627/).
* [Apr 11. Orange County, US - Compression and Serialization Benchmarks in Rust. Q&A on dark corners of Rust](https://www.meetup.com/oc-rust/events/249137682/).
* [Apr 12. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxgbqb/).
* [Apr 12. Arlington, US - Rust DC - Learn+Try: Rust in the Browser via WebAssembly](https://www.meetup.com/RustDC/events/248552247/).
* [Apr 15. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxgbtb/).
* [Apr 17. Brussels, BE - Error, logs and more](https://www.meetup.com/Belgium-Rust-user-group/events/248297132/).
* [Apr 18. Paris, FR - Devoxx 2018 - Hands-on Labs - Rust introduction](http://cfp.devoxx.fr/2018/talk/QAL-4376/Atelier_Rust).
* [Apr 18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [Apr 18. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247388143/).
* [Apr 18. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxgbxb/).
* [Apr 19. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 19. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxgbzb/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Rust Developer at ETCDEV](https://www.etcdevteam.com/job-senior-rust.html).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I guess #rustleaks are memory safe since you just mem::forget them

— [@mgattozzi on Twitter](https://twitter.com/mgattozzi/status/979516899791986688).

Thanks to [@RustDevLuke for the suggestion](https://twitter.com/RustDevLuke/status/979518801648611328)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
