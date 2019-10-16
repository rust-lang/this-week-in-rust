Title: This Week in Rust 308
Number: 308
Date: 2019-10-15
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

- [Vulnerability in sodiumoxide: generichash::Digest::eq always return true](https://www.reddit.com/r/rust/comments/dguqt3/vulnerability_in_sodiumoxide_generichashdigesteq/)
- [Adventures in motion control: The communications system part 2](http://adventures.michaelfbryan.com/posts/comms-part-2/)

# Crate of the Week

This week, we don't have one, nor two, but *three* crates of the week! There's [Watt](https://github.com/dtolnay/watt), a fast WASM-based proc-macro runtime, [Anyhow](https://github.com/dtolnay/anyhow), yet another error handling crate and [spotify-tui](https://github.com/Rigellute/spotify-tui), a console user interface for spotify.

Thanks to [Aloso](https://users.rust-lang.org/t/crate-of-the-week/2704/649), [zicklag](https://users.rust-lang.org/t/crate-of-the-week/2704/645) and [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/644) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hacktoberfest issues from EmbarkStudios](https://github.com/search?q=user:EmbarkStudios+label:hacktoberfest+state:open).
* [rustc: Deprecation warning emitted from derive without a span](https://github.com/rust-lang/rust/issues/56195).
* [async-std: Add `task::yield_now`](https://github.com/async-rs/async-std/issues/290).
* [async-std: Add `sync::CondVar`](https://github.com/async-rs/async-std/issues/217).
* [async-std: Add `path::{Path,PathBuf}`](https://github.com/async-rs/async-std/issues/183).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

302 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-10-07..2019-10-14

* [Add support for `const unsafe? extern fn`](https://github.com/rust-lang/rust/pull/64906)
* [Split non-CAS atomic support off into `target_has_atomic_load_store`](https://github.com/rust-lang/rust/pull/65214)
* [deriving: Avoid dummy Span on an artificial `type_ident` path](https://github.com/rust-lang/rust/pull/65310)
* [Print lifetimes with backticks](https://github.com/rust-lang/rust/pull/65292)
* [Fix suggested bound addition diagnostic](https://github.com/rust-lang/rust/pull/65289)
* [Note when a mutable trait object is needed](https://github.com/rust-lang/rust/pull/65077)
* [Use structured suggestion for removal of `as_str()` call](https://github.com/rust-lang/rust/pull/65194)
* [Fix const generic arguments not displaying in types mismatch diagnostic](https://github.com/rust-lang/rust/pull/65154)
* [Improve message when attempting to instantiate tuple structs with private fields](https://github.com/rust-lang/rust/pull/65153)
* [Suggest dereferencing boolean reference when used in `if` or `while`](https://github.com/rust-lang/rust/pull/65150)
* [When suggesting assoc function with type params, include turbofish](https://github.com/rust-lang/rust/pull/65145)
* [self-profiling: Add events for everything except trait selection](https://github.com/rust-lang/rust/pull/65208)
* [Avoid `SmallVec::collect`](https://github.com/rust-lang/rust/pull/64949)
* [Speed up `TokenStream` concatenation](https://github.com/rust-lang/rust/pull/65198)
* [Implement `Clone::clone_from` for `VecDeque`](https://github.com/rust-lang/rust/pull/65069)
* [Stabilize `slice::repeat`](https://github.com/rust-lang/rust/pull/64877)
* [Stabilize `mem::take`](https://github.com/rust-lang/rust/pull/64716)
* [Implement (`HashMap`) `Entry::insert`](https://github.com/rust-lang/rust/pull/64656)
* [improve performance of signed `saturating_mul`](https://github.com/rust-lang/rust/pull/65312)
* [dist: minimize the `rust-std` component](https://github.com/rust-lang/rust/pull/64823)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize proc macros generating `macro_rules` items](https://github.com/rust-lang/rust/pull/64035).
* [disposition: merge] [Stabilize `slice::repeat` (feature `repeat_generic_slice`)](https://github.com/rust-lang/rust/pull/64877).

## New RFCs

* [Variadic tuples](https://github.com/rust-lang/rfcs/pull/2775).
* [Scope prints in diagnostics](https://github.com/rust-lang/rfcs/pull/2777).
* [Initial cargo-plugin-fields](https://github.com/rust-lang/rfcs/pull/2776).

# Upcoming Events

### Europe

* [Oct 16. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/nxdpgryznbvb/).
* [Oct 17. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/265509739/)
* [Oct 18. Stuttgart, DE - Rust Meetup Hack and Learn](https://www.meetup.com/de-DE/Rust-Community-Stuttgart/events/265526369/).
* [Oct 19.-23. Rome, IT - Rust+GNOME 2019 Hackfest #6](https://wiki.gnome.org/Hackfests/Rust2019-2#preview).
* [Oct 23. Stockholm, SE - Stockholm Rust - Rust Meetup @Embark Studios](https://www.meetup.com/Stockholm-Rust/events/265322700/).
* [Oct 24. Turin, IT - Mozilla Torino - Gruppo di studio Rust](https://www.meetup.com/Mozilla-Torino/events/265207841).
* [Oct 24. Vienna, AT - Rust Vienna - Rust Townsquare Gathering Oktober](https://www.meetup.com/Rust-Vienna/events/265535638/).


### North America

* [Oct 16. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rwcpfryznbvb/).
* [Oct 16. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscryznbfc/).
* [Oct 18 & 19. Dayton, OH, US - Rust Belt Rust](https://www.rust-belt-rust.com/).
* [Oct 23. Portland, OR, US - PDXRust - Hack Night](https://www.meetup.com/PDXRust/events/265043014/).
* [Oct 30. San Francisco, US - Rust in Blockchain Workshop Day (SFBW)](https://www.meetup.com/Rust-in-Blockchain-San-Francisco/events/265362152/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Blockchain Engineer at Nervos, Hangzhou, CN (Remote available)](https://angel.co/company/nervos-1/jobs/589230-senior-blockchain-engineer).
* [Rust/Core Developer at Parity, Berlin, DE (Remote available)](https://www.parity.io/jobs/#berlin-rust-core-developer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> If the Rust community has an ethos, it's that software should have strong static typing, but people should have soft dynamic typing.

– [Kyle Strand on Twitter](https://twitter.com/BatmanAoD/status/1174799660134699008)

Thanks to [Kyle Strand](https://users.rust-lang.org/t/twir-quote-of-the-week/328/710) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
