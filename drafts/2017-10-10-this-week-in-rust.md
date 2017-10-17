Title: This Week in Rust 203
Number: 203
Date: 2017-10-10
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

# Crate of the Week

This week's crate is [if_chain](https://crates.io/crates/if_chain) a macro that helps combat rightwards drift where code nests many `if`s and `if let`s. Since the
latter cannot be contracted with `&&`, this can be really helpful to make code more readable. Thanks to [Michael Budde](https://users.rust-lang.org/u/mbudde) for
the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [impl period opportunities: Diesel](https://medium.com/@sgrif/impl-version-1-0-for-diesel-10f5872c7be).
* [Way Cooler is looking for help with milestone 0.7](https://github.com/way-cooler/way-cooler/issues?q=is%3Aopen+is%3Aissue+milestone%3A0.7+label%3A%22Help+Wanted%22).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

163 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-10-09..2017-10-16

* [add `x86_64-unknown-linux-gnux32` target](https://github.com/rust-lang/rust/pull/45224)
* [inline `eq_slice` into `str::eq`](https://github.com/rust-lang/rust/pull/45005)
* [MIR-borrowck: moves of prefixes invalidate uses too](https://github.com/rust-lang/rust/pull/45025)
* [MIR borrowck: print lvalues in error messages in the same way that the AST borrowck](https://github.com/rust-lang/rust/pull/44985)
* [MIR-borrowck: add false edges to match arms](https://github.com/rust-lang/rust/pull/45200)
* [MIR-borrowck: migrate remaining AST diagnostics](https://github.com/rust-lang/rust/pull/45167)
* [querify `trans_fulfill_obligation`](https://github.com/rust-lang/rust/pull/44967)
* [querify Vtable methods](https://github.com/rust-lang/rust/pull/45137)
* [check namespaces when resolving associated items in typeck](https://github.com/rust-lang/rust/pull/45297)
* [rustc: Remove `used_mut_nodes` from `TyCtxt`](https://github.com/rust-lang/rust/pull/45283)
* [rustc: Fix some ThinLTO internalization](https://github.com/rust-lang/rust/pull/45215)
* [rustc: Update LLVM with a ThinLTO fix](https://github.com/rust-lang/rust/pull/45203)
* [rustc: Handle `#[inline(always)]` at `-O0`](https://github.com/rust-lang/rust/pull/45202)
* [rustc: Don't inline in CGUs at `-O0`](https://github.com/rust-lang/rust/pull/45075)
* [rustc: Reduce default CGUs to 16](https://github.com/rust-lang/rust/pull/45064)
* [incremental compilation auto assert (with except)](https://github.com/rust-lang/rust/pull/45104)
* [incremental compilation: Bring back output of -Zincremental-info](https://github.com/rust-lang/rust/pull/45063)
* [ensure `std::mem::Discriminant` is `Send + Sync`](https://github.com/rust-lang/rust/pull/45095)
* [fix `TcpStream::connect_timeout` on linux](https://github.com/rust-lang/rust/pull/45269)
* [improve performance of `spsc_queue` and stream](https://github.com/rust-lang/rust/pull/44963)
* [improve raw `Box` conversions](https://github.com/rust-lang/rust/pull/44877)
* [some hashmap cleanups](https://github.com/rust-lang/rust/pull/45263)
* [optimize comparison functions of `Iterator`](https://github.com/rust-lang/rust/pull/45007)
* [compiletest/runtest: format `ErrorKind` with `Display`](https://github.com/rust-lang/rust/pull/45258)
* [implement display_hint in gdb pretty printers](https://github.com/rust-lang/rust/pull/45071)
* [some low-hanging rustdoc optimizations](https://github.com/rust-lang/rust/pull/44613)
* [rustdoc: mobile sidebar improvements](https://github.com/rust-lang/rust/pull/45240)
* [let rustdoc print the crate version into docs](https://github.com/rust-lang/rust/pull/44989)
* [incr.comp.: Introduce `ensure` and `ensure` typeck_tables_of](https://github.com/rust-lang/rust/pull/45228)
* [enable building clippy in CI](https://github.com/rust-lang/rust/pull/45177) (one more step towards stable clippy!)
* [update grammar to parse current rust syntax](https://github.com/rust-lang/rust/pull/45125) (Language lawyers rejoice!)


## New Contributors

* Badel2
* Ben Cressey
* Daniel Klauer
* Jeroen Bollen
* Matthias Devlamynck
* Sean Prashad
* Tomas Nilsson
* Vitaly Vi Shukela

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2094: Non-lexical lifetimes](https://github.com/rust-lang/rfcs/pull/2094).
* [RFC 2137: Support defining C-compatible variadic functions in Rust](https://github.com/rust-lang/rfcs/pull/2137).
* [RFC 2141: Add support to Cargo for alternative registries](https://github.com/rust-lang/rfcs/pull/2141).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [impl-only-use](https://github.com/rust-lang/rfcs/pull/2166). The `use …::{… as …}` syntax can now accept `_` as alias to a trait to only import the implementations of such a trait.

# Upcoming Events

* [Oct  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Oct  7. Rust Bangalore SQL Data Binding Workshop](https://www.meetup.com/rustox/events/243387585/).
* [Oct  9. Rust Seattle Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/243267474/).
* [Oct 11. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 11. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 12. Rust Washington DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/243672292/).
* [Oct 12. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/243389836/).
* [Oct 18. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/243663198/).
* [Oct 18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct 18. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct 19. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Research Intern - University 2018 - Mozilla](https://careers.mozilla.org/position/gh/864822).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The compiler is grumpy for you so you don’t have to be

— Élisabeth Henry @RustFest Zürich

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/454) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
