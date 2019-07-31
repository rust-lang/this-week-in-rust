Title: This Week in Rust 297
Number: 297
Date: 2019-07-30
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

* [The Rust compiler is still getting faster](https://blog.mozilla.org/nnethercote/2019/07/25/the-rust-compiler-is-still-getting-faster/).
* [Unsafe as a human-assisted type system](https://matklad.github.io/2019/07/25/unsafe-as-a-type-system.html).
* [Why does the Rust compiler not optimize code assuming that two mutable references cannot alias](https://stackoverflow.com/questions/57259126/why-does-the-rust-compiler-not-optimize-code-assuming-that-two-mutable-reference)?
* [Python vs Rust for neural networks](https://ngoldbaum.github.io/posts/python-vs-rust-nn/).
* [Dependency management and trust scaling](http://lucumr.pocoo.org/2019/7/29/dependency-scaling/).

# Crate of the Week

This week's crate is [async-trait](https://github.com/dtolnay/async-trait), a procedural macro to allow `async fn`s in trait methods.
Thanks to [Ehsan M. Kermani](https://users.rust-lang.org/t/crate-of-the-week/2704/592) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust in Blockchain - Call for Contributors](https://rustinblockchain.org/2019/07/30/call-for-contributors/).
* [RustFest Barcelona - Call for Proposals is open](https://blog.rustfest.eu/cfp-for-barcelona).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

324 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-07-22..2019-07-29

* [Add support for UWP targets](https://github.com/rust-lang/rust/pull/60260)
* [Add `riscv32i-unknown-none-elf` target](https://github.com/rust-lang/rust/pull/62784)
* [Update wasm32 support for LLVM 9](https://github.com/rust-lang/rust/pull/62809)
* [Move unescape module to rustc_lexer](https://github.com/rust-lang/rust/pull/62851)
* [Make the parser TokenStream more resilient after mismatched delimiter recovery](https://github.com/rust-lang/rust/pull/62887)
* [Improve diagnostics for _ const/static declarations](https://github.com/rust-lang/rust/pull/62804)
* [Avoid ICE when referencing desugared local binding in borrow error](https://github.com/rust-lang/rust/pull/63051)
* [Suggest trait bound on type parameter when it is unconstrained](https://github.com/rust-lang/rust/pull/62772)
* [Allow lifetime elision in `Pin<&(mut) Self>`](https://github.com/rust-lang/rust/pull/61207)
* [Stop bare trait lint applying to macro call sites](https://github.com/rust-lang/rust/pull/63014)
* [Add note suggesting to borrow a String argument to find](https://github.com/rust-lang/rust/pull/62981)
* [Add method disambiguation help for trait implementation](https://github.com/rust-lang/rust/pull/62921)
* [miri: Enable Intrptrcast by default](https://github.com/rust-lang/miri/pull/851)
* [Don't access a static just for its size and alignment](https://github.com/rust-lang/rust/pull/62982)
* [Use const array repeat expressions for `uninit_array`](https://github.com/rust-lang/rust/pull/62799)
* [Stabilize the `type_name` intrinsic in `core::any`](https://github.com/rust-lang/rust/pull/60066)
* [Constantly improve the `Vec`(`Deque`) array `PartialEq` impls](https://github.com/rust-lang/rust/pull/63061)
* [hashbrown: Do not grow the container if an insertion is on a tombstone](https://github.com/rust-lang/hashbrown/pull/106)
* [rust-bindgen: Cleanup `wchar_t` layout computation to happen later](https://github.com/rust-lang/rust-bindgen/pull/1596)
* [rustdoc: Make `#[doc(include)]` relative to the containing file](https://github.com/rust-lang/rust/pull/60938)
* [docs.rs: Fix weird layout workflow issues on firefox](https://github.com/rust-lang/docs.rs/pull/358)
* [Force clippy to run every time](https://github.com/rust-lang/cargo/pull/7157) (finally!)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2515: Permit impl Trait in type aliases](https://github.com/rust-lang/rfcs/pull/2515).
* [RFC 2574: SIMD vectors in FFI](https://github.com/rust-lang/rfcs/pull/2574).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No new RFCs were proposed this week.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize checked_duration_since for 1.38.0](https://github.com/rust-lang/rust/pull/62860).
* [disposition: merge] [Stabilize duration_float](https://github.com/rust-lang/rust/pull/62756).
* [disposition: merge] [Implement DoubleEndedIterator for iter::{StepBy, Peekable, Take](https://github.com/rust-lang/rust/pull/61457).
* [disposition: merge] [Give built-in macros stable addresses in the standard library](https://github.com/rust-lang/rust/pull/63056).
* [disposition: merge] [Add a few trait impls for AccessError](https://github.com/rust-lang/rust/pull/61491).

## New RFCs

* [Add `no_entry` attribute to omit entry point symbol](https://github.com/rust-lang/rfcs/pull/2735).

# Upcoming Events

### Africa

* [Aug  7. Johannesburg, ZA - Johannesburg Rust Meetup - futures](https://www.meetup.com/Johannesburg-Rust-Meetup/events/dgqmbryzlbkb/).

### Asia Pacific

* [Aug  5. Auckland, NZ - Rust AKL August - Rust usage in Firefox](https://www.meetup.com/rust-akl/events/259480991/).
* [Aug 10. Singapore, SG - Rust Meetup](https://www.eventbrite.com/e/rust-meetup-tickets-65358532129).
* [Aug 17. Taipei, TW - "Everything in Rust" at COSCUP 2019](https://coscup.org/2019/en/).

### Europe

* [Aug  4. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/whmxrqyzlbgb).
* [Aug  7. Erlangen, DE - Rust Franken Meetup #1](https://www.meetup.com/Rust-NERF/events/263163435/).
* [Aug  7. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzlbkb/).

### North America

* [Aug  7. Portland, OR, US - PDXRust - Trees = Boxes + Enums + Iterators](https://www.meetup.com/PDXRust/events/263383260/).
* [Aug  7. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzlbkb/).
* [Aug  7. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzlbkb/).
* [Aug  7. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzlbkb/).
* [Aug  8. San Diego, CA, US - San Diego Rust August Meetup](https://www.meetup.com/San-Diego-Rust/events/263267320/).
* [Aug  8. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzlblb/).
* [Aug  8. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/262296008).
* [Aug 13. Toronto, ON, CA - Rust Toronto - Toronto Rustaceans Tech and Talk](https://www.meetup.com/Rust-Toronto/events/263395708).
* [Aug 13. Denver, CO, US - Rust Boulder/Denver - Hack 'N Snack](https://www.meetup.com/Rust-Boulder-Denver/events/263156621/).
* [Aug 13. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdryzlbrb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Platform Engineer - Layout as Mozilla, Portland, US](https://careers.mozilla.org/position/gh/1787784/).
* [Senior Software Engineer at ConsenSys R&D, Remote](https://consensys.net/open-roles/1792013/).
* [Rust Developer at Finhaven, Vancouver, CA](https://finhaven.humi.ca/job-board/engineering/1306).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust clearly popularized the ownership model, with similar implementations being considered in D, Swift and other languages. This is great news for both performance and memory safety in general.
>
> Also let's not forget that Rust is not the endgame. Someone may at one point find or invent a language that will offer an even better position in the safety-performance-ergonomics space. We should be careful not to get too attached to Rust, lest we stand in progress' way.

– [llogiq on reddit](https://reddit.com/r/rust/comments/cfeng7/the_redmonk_programming_language_rankings_june/euann96/)

Thanks to [Vikrant](https://users.rust-lang.org/t/twir-quote-of-the-week/328/676) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/ckagpq/this_week_in_rust_297/).</small>
