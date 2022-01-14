Title: This Week in Rust 426
Number: 426
Date: 2022-01-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Research

### Rust Walkthroughs

### Miscellaneous

[Rewriting my toy blockchain in Rust](https://ezzeriesa.notion.site/Rewriting-my-toy-blockchain-in-Rust-9a130f225666488491ba497004821fbb)

## Crate of the Week

This week's crate is [rustix](https://github.com/bytecodealliance/rustix), a crate with safe bindings to POSIX-ish syscalls.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1003) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

266 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-01-03..2022-01-10

* [allow `_` as the length of array types and repeat expressions](https://github.com/rust-lang/rust/pull/91907) (this implements the closed RFC PR [#2545](https://github.com/rust-lang/rfcs/pull/2545))
* [cg: split dwarf for crate dependencies](https://github.com/rust-lang/rust/pull/89819)
* [suggest single quotes when char expected, str provided](https://github.com/rust-lang/rust/pull/92507)
* [add a query for resolving an impl item from the trait item](https://github.com/rust-lang/rust/pull/90639)
* [miri: exclude mutable references to !Unpin types from uniqueness guarantees](https://github.com/rust-lang/miri/pull/1952)
* [perf: do not use LEB128 for encoding u16 and i16](https://github.com/rust-lang/rust/pull/92314)
* [rustc_metadata: optimize and document module children decoding](https://github.com/rust-lang/rust/pull/92086)
* [normalize generator-local types with unevaluated constants](https://github.com/rust-lang/rust/pull/92636)
* [normalize struct tail type when checking Pointee trait](https://github.com/rust-lang/rust/pull/92248)
* [constify `Box<T, A>` methods](https://github.com/rust-lang/rust/pull/91884)
* [do not hash leading zero bytes of i64 numbers in Sip128 hasher](https://github.com/rust-lang/rust/pull/92103)
* [implement `TryFrom<char>` for `u8`](https://github.com/rust-lang/rust/pull/84640)
* [implement const casts of raw pointers](https://github.com/rust-lang/rust/pull/92657)
* [stabilize `#[feature(available_parallelism)]`](https://github.com/rust-lang/rust/pull/92632)
* [stabilize `result_cloned` and `result_copied`](https://github.com/rust-lang/rust/pull/92483)
* [modifications to `std::io::Stdin` on Windows so that there is no longer a 4-byte buffer minimum in read()](https://github.com/rust-lang/rust/pull/91754)
* [core::ops::unsize: improve docs for DispatchFromDyn](https://github.com/rust-lang/rust/pull/91587)
* [hashbrown: don't hash the key when searching in an empty table](https://github.com/rust-lang/hashbrown/pull/305)
* [cargo: support rustflags per profile](https://github.com/rust-lang/cargo/pull/10217)
* [cargo: be resilient to most IO error and filesystem loop while walking dirs](https://github.com/rust-lang/cargo/pull/10214)
* [rustdoc: introduce a resolver cache for sharing data between early doc link resolution and later passes](https://github.com/rust-lang/rust/pull/92608)
* [rustdoc: resolve associated traits for non-generic primitive types](https://github.com/rust-lang/rust/pull/92443)
* [docs.rs: add "y" shortcut for permalink](https://github.com/rust-lang/docs.rs/pull/1583)
* [clippy: better detect when a field can be moved from in `while_let_on_iterator`](https://github.com/rust-lang/rust-clippy/pull/8221)
* [clippy: consider auto-deref when linting `manual_swap`](https://github.com/rust-lang/rust-clippy/pull/8220)
* [clippy: fix `type_repetition_in_bounds`](https://github.com/rust-lang/rust-clippy/pull/8224)
* [clippy: cover trait for `trait_duplication_in_bounds`](https://github.com/rust-lang/rust-clippy/pull/8252)
* [clippy: fix `iter_not_returning_iterator`](https://github.com/rust-lang/rust-clippy/pull/8228)

### Rust Compiler Performance Triage

Improvements generally outweighed regressions with most regressions coming in the form of correctness fixes (where performance regressions are generally less of a concern). The biggest win was arguably a change to the Sip128 hasher implementation which seemed to have decent performance implications for many real world crates. 

Triage done by **@rylev**.
Revision range: [2b681ac..72e74d7](https://perf.rust-lang.org/?start=2b681ac06b1a6b7ea39525e59363ffee0d1a68e5&end=72e74d7b9cf1a7901650227e74650f1fcc797600&absolute=false&stat=instructions%3Au)

3 Regressions, 7 Improvements, 2 Mixed; 4 of them in rollups
28 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-01-12.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [doc: guarantee call order for sort_by_cached_key](https://github.com/rust-lang/rust/pull/89621)

### [New RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Rename {Option, Result}::expect to unwrap_or_panic](https://github.com/rust-lang/rfcs/pull/3218)
* [Add RFC for providing ignore test message](https://github.com/rust-lang/rfcs/pull/3217)
* [Allow using for<'a> syntax when declaring closures](https://github.com/rust-lang/rfcs/pull/3216)

## Upcoming Events

Rusty Events between 1/12/2022 - 2/9/2022 🦀

### Online

* [January 12, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydccbqb/)
* [January 12, 2022 | Frankfurt, DE | **Rust for bare-metal embedded systems** | Developing Embedded Systems in Rhein-Main](https://www.meetup.com/Developing-Embedded-Systems-in-Rhein-Main/events/282321009)
* [January 12, 2022 | Los Angeles, CA, US | **Live Coding Session - Mob Programming a Rust Code Kata [Virtual] Jan. 2022** | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/282580016/)
* [January 12, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/gjrtqsydccbqb/)
* [January 13, 2022 | Charlottesville, VA, US | **Higher kinded polymorphism** | Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/events/282990814)
* [January 13, 2022 | Minneapolis, MN, US | **Safe Systems Programming in Rust: The Promise and the Challenge** | Software Engineering Reading Group](https://www.meetup.com/meetup-group-bxuhnetv/events/282770888)
* [January 13, 2022 | San Diego, CA, US | **San Diego Rust Rust January 2022 Tele-Meetup** | San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/283032744)
* [January 13, 2022 | Warsaw, PL | **Rust Warsaw #4** | Rust Warsaw](https://www.meetup.com/pl-PL/Rust-Warsaw/events/282879405/)
* [January 18, 2022 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 2 (finishing) & Chapter 3 (beginning)** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/283210346/)
* [January 18, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/vdhxgsydccbxb/)
* [January 19, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydccbzb)
* [January 20, 2022 | Cardiff, UK | **Rust Book Study Session - Functional Language Features & Cargo and Crates.io** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/283204522/)
* [January 25, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrydccbhc/)
* [January 27, 2022 | Nürnberg, DE | **Rust Nürnberg online #9**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/283118050/)
* [January 27, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545254)
* [February 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283011769)
* [February 9, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545292)

### North America

* [January 12, 2022 | Atlanta, GA, US | **Grab a beer with fellow Rustaceans** | Rust Atlanta](https://www.meetup.com/Rust-ATL/events/lhpkmsydccbqb/)
* [January 13, 2022 | Columbus, OH, US | **Monthly Meeting** | Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgrydccbrb/)
* [January 18, 2022 | San Francisco, CA, US | **Rust Hacking in Person** | San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/283208406/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Language stability is not just about semver compatibility. It's also about not burdening developers to have to make new decisions when looking at old code. \[Language instability\] creates churn and debate about things that previously didn't require it.

– [skysch on rust-internals](https://internals.rust-lang.org/t/rust-2030-christmas-list-inout-methods/15944/3)

Thanks to [Christopher Durham](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1165) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
