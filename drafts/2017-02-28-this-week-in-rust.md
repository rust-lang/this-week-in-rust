Title: This Week in Rust 171
Number: 171
Date: 2017-02-28
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

* [Rust is now the fastest language on k-nucleotide](https://benchmarksgame.alioth.debian.org/u64q/performance.php?test=knucleotide). Using [OrderMap](https://github.com/bluss/ordermap) crate, Rust is now the fastest language at The Computer Language Benchmarks Game's k-nucleotide benchmark.
* [Deploying Rust in a large codebase](https://medium.com/@rillian/deploying-rust-in-a-large-codebase-7e50328074e8). Shipping Rust in Firefox.
* [Non-lexical lifetimes using liveness and location](http://smallcultfollowing.com/babysteps/blog/2017/02/21/non-lexical-lifetimes-using-liveness-and-location/).
* [Demangling C++ symbols in Rust](http://fitzgeraldnick.com/2017/02/22/cpp-demangle.html).
* [Rust asynchronous HTTP server with tokio and hyper](https://blog.guillaume-gomez.fr/articles/2017-02-22+Rust+asynchronous+HTTP+server+with+tokio+and+hyper).
* [Simulating LIDAR driving with Rust and OpenAI](https://medium.com/@andrew_subarctic/simulating-lidar-driving-with-rust-and-openai-19a8dcbc2ad8).
* [Shar: One year with Rust](https://www.reddit.com/r/rust_gamedev/comments/5vqlln/shar_one_year_with_rust/). SHAR is an action combination of tactical and sports game in a destructible world.
* [Objective-C from Rust: Statically verified type encodings without allocation](http://sasheldon.com/blog/2017/02/20/objective-c-from-rust-type-encodings/).
* [How we made our CSV processing 142x faster](http://blog.faraday.io/how-we-made-our-csv-processing-142x-faster/).
* [One year with Rust - Developing a full featured application in Rust](https://vitiral.github.io/2017/02/25/one-year-with-rust.html).
* [The System Programming Enclosure Movement](https://llogiq.github.io/2017/02/21/enclosure.html).
* [How we made TensorFlow run on a Raspberry Pi using Rust](https://medium.com/snips-ai/how-we-made-tensorflow-run-on-a-raspberry-pi-using-rust-7478f7a31329).
* [The first Crate polishing workshop: Report](https://llogiq.github.io/2017/02/27/cpw.html).

## weekly

* [This Week in Rust Docs 45](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-45).
* [These weeks in Redox 19](https://redox-os.org/news/visual-refresh-19/). Visual refresh.
* [Ferris Makes Emulators 22 - Sample Extraction](https://www.youtube.com/watch?v=Cd6yrfI82y8).
* [What's coming up in imag 23](https://beyermatthias.de/blog/2017/02/20/whats-coming-up-in-imag-23/).
* [This week in TiKV 2010-02-27](http://weekly.pingcap.com/2017/02/27/tidb-weekly/#weekly-update-in-tikv)

# Crate of the Week

This week's crate of the week is [nalgebra](https://crates.io/crates/nalgebra), a linear algebra library in and for Rust. Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [discussion] [rust-book: Rust & OOP patterns](https://users.rust-lang.org/t/what-does-rust-oop-mean-to-you/9633).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).
* [medium] [clippy: Lint `.into_iter()` if that only forwards to `.iter()`](https://github.com/Manishearth/rust-clippy/issues/1565).
* [medium] [clippy: Warn on `let _ = x.lock();`](https://github.com/Manishearth/rust-clippy/issues/1574).
* [hard] [clippy: Lint crates that can be `#![no_std]` but aren't](https://github.com/Manishearth/rust-clippy/issues/1569).
* [easy/hard] [clippy: Lint functions taking references as arguments but only use them to create an owned value](https://github.com/Manishearth/rust-clippy/issues/1563).
* [easy] [clippy: Lint for iterating over a slice with one (or zero) element](https://github.com/Manishearth/rust-clippy/issues/1540).
* [easy] [clippy: useless_transmute being raised when it's doing multiple casts](https://github.com/Manishearth/rust-clippy/issues/1545).
* [easy] [clippy: Lint to suggest `.saturating_add/sub(x)` for `.checked_add/sub(x).unwrap_or(MAX/MIN)`](https://github.com/Manishearth/rust-clippy/issues/1557).
* [easy] [clippy: Lint against const atomics](https://github.com/Manishearth/rust-clippy/issues/1560).
* [easy] [crates.io: Add rustfmt to run on travis and fail the build](https://github.com/rust-lang/crates.io/issues/575).
* [easy] [crates.io: Run rustfmt on the whole codebase and send in the changes](https://github.com/rust-lang/crates.io/issues/574).
* [easy] [crates.io: Document applying categories/adding new categories](https://github.com/rust-lang/crates.io/issues/544).
* [medium] [crates.io: Yanking a crate should update its max_version](https://github.com/rust-lang/crates.io/issues/76).
* [medium] [crates.io: Be able to search within a keyword or category](https://github.com/rust-lang/crates.io/issues/491).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

124 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-20..2017-02-27

* [fix LLVM x86 interrupt calling convention](https://github.com/rust-lang/llvm/pull/63)
* [fix ICE in trans](https://github.com/rust-lang/rust/pull/40064)
* [properly display expected details on type mismatch](https://github.com/rust-lang/rust/pull/39905)
* [`-C overflow-checks` option](https://github.com/rust-lang/rust/pull/40037)
* [stabilize `static_recursion`](https://github.com/rust-lang/rust/pull/40027)
* [allow tools to run test runners programmatically](https://github.com/rust-lang/rust/pull/39815)
* [allow coercion of non-capturing closures to `fn`s](https://github.com/rust-lang/rust/pull/40025)
* [improved lib defaults handling](https://github.com/rust-lang/rust/pull/40022)
* [make `Unique<T>` `UnwindSafe` even for unsized types](https://github.com/rust-lang/rust/pull/40020)
* [set LLVM metadata for vtables](https://github.com/rust-lang/rust/pull/39995) (allows better optimizations)
* [track the `-Z sanitizer` flag over incremental compilations](https://github.com/rust-lang/rust/pull/39993)
* [incremental compilation: detect bootstrap outputs](https://github.com/rust-lang/rust/pull/40038)
* [adaptive hashmap: consider displacement instead of shift length](https://github.com/rust-lang/rust/pull/39988)
* [don't treat privately uninhabited types as uninhabited](https://github.com/rust-lang/rust/pull/39980)
* [improved type inference error reporting](https://github.com/rust-lang/rust/pull/39913)
* [verify all sysroot crates are unstable](https://github.com/rust-lang/rust/pull/39851)
* [Cargo: fix required features vs. dependencies' features interference](https://github.com/rust-lang/cargo/pull/3737)
* [`cargo check --all`](https://github.com/rust-lang/cargo/pull/3731)
* [migrate Cargo from rustc-serialize to serde](https://github.com/rust-lang/cargo/pull/3682)
* [docs: port the reference to mkbook](https://github.com/rust-lang/rust/pull/39855)

## New Contributors

* Daniel Xu
* er-1
* Hiroki Kobayashi
* Josef Brandl
* Paul Merrill
* Peter Wagenet
* Tom Anderson
* topecongiro

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

* [disposition: postpone] [Introduce _pattern synonyms_ - used to create new patterns we can pattern match against from real patterns](https://github.com/rust-lang/rfcs/pull/1895).
* [disposition: close] [Add a `Transmute<T>` trait for representing types that can be transmuted to `T`](https://github.com/rust-lang/rfcs/pull/1891).
* [disposition: merge] [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: close] [Add 'else match' blocks to if expressions](https://github.com/rust-lang/rfcs/pull/1712).
* [disposition: postpone] [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: postpone] [Add the ability to define closures that are generic over types](https://github.com/rust-lang/rfcs/pull/1650).
* [disposition: close] [Add `&move` pointers, the `DerefMove` trait, and the unsafe `DerefPure` traits](https://github.com/rust-lang/rfcs/pull/1646).

## New RFCs

* [dependent-types (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1931).
* [Introduce `with` bounds for pi types](https://github.com/rust-lang/rfcs/pull/1932).
* [Fully dependent pi types](https://github.com/rust-lang/rfcs/pull/1933).
* [Allow an optional vert at the beginning of a match branch](https://github.com/rust-lang/rfcs/pull/1925).
* [Reduce the number of constraints repeated when writing `T: SomeTrait`](https://github.com/rust-lang/rfcs/pull/1927).
* [Tuple-based variadic generics](https://github.com/rust-lang/rfcs/pull/1935).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [structs and unions](https://github.com/rust-lang-nursery/fmt-rfcs/pull/53)

Issues in final comment period:

* [`extern` vs `extern "C"`](https://github.com/rust-lang-nursery/fmt-rfcs/issues/52)
* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [types](https://github.com/rust-lang-nursery/fmt-rfcs/issues/15)

Other significant issues:

* [ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

# Upcoming Events

* [Mar  1. Rust User Group Cologne - Web development in Rust](http://rust.cologne/2017/03/01/web-dev.html).
* [Mar  1. South Florida Rust - Intro to Ownership and Borrowing](https://www.meetup.com/South-Florida-Rust-Meetup/events/237559303/).
* [Mar  1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  2. GPU glyph rasterization, Rocket, and the orphan rules](https://www.meetup.com/Rust-Bay-Area/events/237709786/).
* [Mar  7. Mozilla Meetup Switzerland - Rust on the Rumprun Unikernel](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/237757802/).
* [Mar  7. Rust Oslo - What's New - Focus on web services](https://www.meetup.com/Rust-Oslo/events/237849579/).
* [Mar  8. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658966/).
* [Mar  8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/237525355/).
* [Mar  9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 11. Rust NYC - Rust Hack & Learn](https://www.meetup.com/Rust-NYC/events/238057861/).
* [Mar 13. Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/237058819/).
* [Mar 15. Rust Meetup Hamburg - Rust/Ethereum Meetup](https://www.meetup.com/Rust-Meetup-Hamburg/events/237858112/).
* [Mar 15. Rust Los Angeles - Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/237757181/).
* [Mar 15. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 15. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 16. Thompson Rivers University, BC Canada - Get Rusty](https://www.eventbrite.ca/e/get-rusty-tickets-31407199780).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> What about the Quote of the Week? I noticed it's missing quite often these days.

— [llogiq on reddit](https://www.reddit.com/r/rust/comments/5vh4uk/this_week_in_rust_170/de2j085/).

Thanks to [tibodelor for the suggestion](https://www.reddit.com/r/rust/comments/5vh4uk/this_week_in_rust_170/de3ppdd/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
