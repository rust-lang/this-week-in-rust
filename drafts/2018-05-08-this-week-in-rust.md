Title: This Week in Rust 233
Number: 233
Date: 2018-05-08
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

* [Refactoring Apache Arrow to use traits and generics](https://andygrove.io/2018/05/apache-arrow-traits-generics/).

# Crate of the Week

This week's crate is [semverver](https://crates.io/crates/semverver), a currently nightly-only cargo subcommand to detect semver violations. Thanks to [Dzmitry Malyshau](https://users.rust-lang.org/u/kvark) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Clippy](https://github.com/rust-lang-nursery/rust-clippy) has a lot of [good first issues](https://github.com/rust-lang-nursery/rust-clippy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) to get started.
* [mutagen has more 'good first issues'](https://github.com/llogiq/mutagen/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [hard] [tarpaulin: Link error with undefined reference to main on Arch](https://github.com/xd009642/tarpaulin/issues/23). Tarpaulin is a code coverage tool for Rust projects.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

145 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-04-30..2018-05-07

* [stable release 1.26.0](https://github.com/rust-lang/rust/pull/50510)
* [add some groundwork for cross-language LTO](https://github.com/rust-lang/rust/pull/50000)
* [fix ICE in assertion macro](https://github.com/rust-lang/rust/pull/50474)
* [fix ICE when using `a..=b` in a closure](https://github.com/rust-lang/rust/pull/50421)
* [forbid constructing empty identifiers from concat_idents](https://github.com/rust-lang/rust/pull/50406)
* [proc_macro: explicitly make everything !Send/Sync](https://github.com/rust-lang/rust/pull/50453)
* [introduce `-Znll-facts` to dump base-facts for the NLL analysis](https://github.com/rust-lang/rust/pull/50370)
* [immutably and implicitly borrow all pattern ids for their guards (NLL only)](https://github.com/rust-lang/rust/pull/49870)
* [fix a warning in libcore on 16bit targets](https://github.com/rust-lang/rust/pull/50369)
* [stabilize `#[must_use]` for functions and must-use comparison operators](https://github.com/rust-lang/rust/pull/48925) (RFC [#1940](https://rust-lang.github.io/rfcs/1940-must-use-functions.html))
* [reduce maximum repr(align(N)) to 2^29](https://github.com/rust-lang/rust/pull/50378)
* [correct initial field alignment for `repr(C)`/`repr(int)`](https://github.com/rust-lang/rust/pull/50354)
* [use `escape_default()` for strings in `LitKind::token()`](https://github.com/rust-lang/rust/pull/50391)
* [extend `Printer::buf` on demand](https://github.com/rust-lang/rust/pull/50339)
* [always inline simple `BytePos` and `CharPos` methods](https://github.com/rust-lang/rust/pull/50407)
* [rustc: return iterators from `Terminator(Kind)::successors(_mut)`](https://github.com/rust-lang/rust/pull/50278)
* [treat generators as if they have an arbitrary destructor](https://github.com/rust-lang/rust/pull/49943)
* [`RangeInclusive::{new, start, end}` methods](https://github.com/rust-lang/rust/pull/49724)
* [use `ManuallyDrop` instead of `Option` in `BinaryHeap` Hole implementation](https://github.com/rust-lang/rust/pull/50487)
* [remove the deprecated `std::net::`{`lookup_host`,`LookupHost`}](https://github.com/rust-lang/rust/pull/50435)
* [cargo: show elapsed time in minutes if >= 60 secs](https://github.com/rust-lang/cargo/pull/5456)
* [rustdoc: resolve nested `impl Trait`s](https://github.com/rust-lang/rust/pull/50419)
* [rustbuild: allow quick testing of libstd and libcore at stage0](https://github.com/rust-lang/rust/pull/50466)

## New Contributors

* Aaron Aaeng
* Irina Popa
* James Sanderson
* Pazzaz
* Philipp Hansch
* Ralf Biedert
* z4v1er

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2318: Custom test frameworks](https://github.com/rust-lang/rfcs/pull/2318).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Reserve `try` for `try { .. }` block expressions](https://github.com/rust-lang/rfcs/pull/2388).
* [disposition: merge] [Async/await notation for ergonomic asynchronous IO](https://github.com/rust-lang/rfcs/pull/2394).
* [disposition: merge] [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).

## New RFCs

* [Add Async trait and task system to libcore](https://github.com/rust-lang/rfcs/pull/2418).
* [Unreserve `proc`](https://github.com/rust-lang/rfcs/pull/2420).
* [Keyword unreservations (pure, sizeof, alignof, offsetof)](https://github.com/rust-lang/rfcs/pull/2421).
* [`throw` expressions](https://github.com/rust-lang/rfcs/pull/2426).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [May  3. Utrecht, NL - Rust Workshop](https://www.meetup.com/Rust-Utrecht/events/248995086/).
* [May  6. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbjb/).
* [May  7. Sydney, AU - Rust Sydney - Meetup 13](https://www.meetup.com/Rust-Sydney/events/249764935/).
* [May  8. Helsinki, FI - Finland Rust-lang Group - May Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/250129359/).
* [May  8. São Paulo, BR - Encontro de Comunidades - Guru-SP e RustLangBR na TOTVS](https://www.meetup.com/Guru-SP-Grupo-de-Usuarios-Ruby-de-Sao-Paulo/events/249463627/).
* [May  8. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [May  9. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May  9. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [May 10. Redwood City, US - Bay Area - Where "Self-Driving" Database Meets a "Rusty" Distributed Key-Value Store](https://www.meetup.com/Bay-Area-NewSQL-Database-Meetup/events/249676562/).
* [May 10. Arlington, US - Rust DC - Learn+Try: parsing with nom](https://www.meetup.com/RustDC/events/249883820).
* [May 10. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/lcsdqpyxhbnb/).
* [May 10. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/249783590/).
* [May 13. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxhbrb/).
* [May 14. Seattle, US - Seattle Rust Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxhbsb/).
* [May 16. Denver, US - Rust Boulder/Denver - Rust Denver May Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/249098925/).
* [May 16. Rust Community Team Meeting at #rust-community on irc.mozilla.org](irc://irc.mozilla.org/rust-community).
* [May 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/249497881/).
* [May 16. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxhbvb/).
* [May 17. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxhbwb/).
* **[May 27. Paris, FR - RustFest Paris 2018](https://paris.rustfest.eu/)**.
* [May 30./31. Rust/WASM course around JSConf.EU](https://ti.to/asquera-event-ug/rust-wasm-wwwtf-2018/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> last time i talked to the infra team they made a bot to replace kennytm. i fear if I ask them to write a rust based unikernel with a custom os to host the docs they’ll actually do it

— [@killercup on Twitter](https://twitter.com/killercup/status/988894247075155968).

Thanks to [skade](https://users.rust-lang.org/t/twir-quote-of-the-week/328/516) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
