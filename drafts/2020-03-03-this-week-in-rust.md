Title: This Week in Rust 328
Number: 328
Date: 2020-03-03
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

[The RustConf 2020 CFP is now open!](https://cfp.rustconf.com/events/rustconf-2020) We'd love to hear from you at RustConf!
- [Porting a JavaScript App to WebAssembly with Rust (Part 3)](https://www.slowtec.de/posts/2020-02-28-porting-javascript-to-rust-part-3.html)

# Crate of the Week

This week's crates are [wundergraph](https://crates.io/crates/wundergraph), a GraphQL interface library, and [kibi](https://github.com/ilai-deutel/kibi), a text editor in thousand lines of Rust.

Thanks to [Georg Semmler](https://users.rust-lang.org/t/crate-of-the-week/2704/732) and [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/734) for the suggestions!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [good first issue] [sqlx: Implement support for time-rs 0.2](https://github.com/launchbadge/sqlx/issues/115).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

307 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-02-17..2020-02-24

* [configure: set LLVM flags with a value](https://github.com/rust-lang/rust/pull/69244)
* [parse: unify item parsing & filter illegal item kinds](https://github.com/rust-lang/rust/pull/69366)
* [parse: allow `type Foo: Ord` syntactically](https://github.com/rust-lang/rust/pull/69361)
* [parse: fuse associated and extern items up to defaultness](https://github.com/rust-lang/rust/pull/69194)
* [`recursion_limit` parsing handles overflows](https://github.com/rust-lang/rust/pull/67272)
* [fix generator miscompilations](https://github.com/rust-lang/rust/pull/69302)
* [don't eliminate frame pointers on thumb targets](https://github.com/rust-lang/rust/pull/69248)
* [tweak binding lifetime suggestion text](https://github.com/rust-lang/rust/pull/69305)
* [on mismatched argument count point at arguments](https://github.com/rust-lang/rust/pull/68877)
* [do not emit note suggesting to implement operation trait to foreign type](https://github.com/rust-lang/rust/pull/69217)
* [split non macro portion of `unused_doc_comment` from macro part into two passes/lints](https://github.com/rust-lang/rust/pull/69084)
* [combine `HaveBeenBorrowedLocals` and `IndirectlyMutableLocals` into one dataflow analysis](https://github.com/rust-lang/rust/pull/69113)
* [fix printing of `Yield` terminator](https://github.com/rust-lang/rust/pull/69200)
* [querify `object_safety_violations`](https://github.com/rust-lang/rust/pull/69242)
* [change const eval to just return the value](https://github.com/rust-lang/rust/pull/69181)
* [allow trait methods to be called on concrete types in a const context](https://github.com/rust-lang/rust/pull/68847)
* [perf: miscellaneous inlining improvements](https://github.com/rust-lang/rust/pull/69256)
* [perf: O(log n) lookup of associated items by name](https://github.com/rust-lang/rust/pull/69072)
* [add `LinkedList::remove`](https://github.com/rust-lang/rust/pull/68705)
* [change `FromStr` for `String` to use `Infallible` directly](https://github.com/rust-lang/rust/pull/67925)
* [make `u8::is_ascii` a stable `const fn`](https://github.com/rust-lang/rust/pull/68984)
* [make integer exponentiation methods unstably const](https://github.com/rust-lang/rust/pull/68978)
* [simplify `Skip::nth` and `Skip::last` implementations](https://github.com/rust-lang/rust/pull/68597)
* [stabilize `Once::is_completed`](https://github.com/rust-lang/rust/pull/68945)
* [stabilize {`f32`, `f64`}::{`LOG2_10`, `LOG10_2`}](https://github.com/rust-lang/rust/pull/69249)
* [git2: add `Branch::get_mut`](https://github.com/rust-lang/git2-rs/pull/522)
* [futures: relax bounds for `FuturesUnordered`](https://github.com/rust-lang/futures-rs/pull/2085)
* [futures: add `StreamExt::flat_map`](https://github.com/rust-lang/futures-rs/pull/2068)
* [cargo: add new feature resolver](https://github.com/rust-lang/cargo/pull/7820)
* [cargo: add an option to include crate versions to the generated docs](https://github.com/rust-lang/cargo/pull/7903)
* [cargo: improvements to `StringList` config handling](https://github.com/rust-lang/cargo/pull/7891)
* [rustfmt: support formatting half open ranges](https://github.com/rust-lang/rustfmt/pull/4044)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo report future-incompat](https://github.com/rust-lang/rfcs/pull/2834).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Permit negative impls for non-auto traits](https://github.com/rust-lang/rust/pull/68004).
* [disposition: merge] [Stabilize const for integer {to,from}_{be,le,ne}_bytes methods](https://github.com/rust-lang/rust/pull/69373).

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Africa

* [Mar  4. Johannesburg, ZA - Johannesburg Rust Meetup - Coffee and a chat about Rust](https://www.meetup.com/Johannesburg-Rust-Meetup/events/268960482/).

### Asia Pacific

* [Mar  5. Melbourne, AU - Rust Melbourne - Hack Night, Talks, and Networking](https://www.meetup.com/Rust-Melbourne/events/268002615/).

### Europe

* [Mar  4. Dublin, IE - Rust Dublin - Reboot pub meetup: The Duke](https://www.meetup.com/Rust-Dublin/events/237883717/).
* [Mar  4. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcfbgb/).
* [Mar  9. Karlsruhe, DE - Rust Meetup](https://www.meetup.com/Rust-Hack-Learn-Karlsruhe/events/268299172/)
* [Mar 11. Oslo, NO - Rust Oslo - Lightning talks](https://www.meetup.com/Rust-Oslo/events/268738879).
* [Mar 12. Turin, IT - Mozilla Torino - Gruppo di studio Rust alla Torino hacknight](https://www.meetup.com/Mozilla-Torino/events/268822794).

### North America

* [Mar  4. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpybcfbgb/).
* [Mar 10. Denver, CO, US - Rust Boulder/Denver - March Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/267834799/).
* [Mar 10. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup in Redmond](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdrybcfbnb/).
* [Mar 11. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcfbpb/).
* [Mar 11. Houston, TX, US - Houston Linux Users Group - Rust Study Group](https://www.facebook.com/events/469382520642102).
* [Mar 12. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcfbqb/).
* [Mar 12. Lehi, UT, US - Utah Rust - The Blue Pill: Rust on Microcontrollers](https://www.meetup.com/utah-rust/events/268567961/).
* [Mar 18. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcfbxb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Yoda must have hit his head, though. `if let 42 = x {}` "if let forty-two equals x"

– [Hutch on rust-internals](https://internals.rust-lang.org/t/using-if-let-to-check-for-equality/11750/19)

Thanks to [Kornel](https://users.rust-lang.org/t/twir-quote-of-the-week/328/821) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
