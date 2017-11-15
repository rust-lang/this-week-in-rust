Title: This Week in Rust 208
Number: 208
Date: 2017-11-14
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

* [Fearless concurrency in Firefox Quantum](https://blog.rust-lang.org/2017/11/14/Fearless-Concurrency-In-Firefox-Quantum.html).
* [Zoxc joins Rust compiler team](https://internals.rust-lang.org/t/please-welcome-zoxc-to-the-compiler-team/6207).
* [Rust futures: a short tutorial - part 1](https://dev.to/mindflavor/rust-futures-an-uneducated-short-and-hopefully-not-boring-tutorial---part-1-3k3).
* [Cross-compiling Rust for the Raspberry Pi on macOS](https://akappel.github.io/2017/11/07/rpi-crosstool.html).
* [A linear hashing implementation in Rust](https://samrat.me/posts/2017-11-09-kvstore-rust-hashtable/).
* [This week in Rust docs 81](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-81).
* [Impl period newsletter 3](https://internals.rust-lang.org/t/impl-period-newsletter-3/6185).
* [video] [How Rust gets polymorphism right](https://www.youtube.com/watch?v=VSlBhAOLtFA).
* [podcast] [Rusty Spike Podcast - episode 7](https://rusty-spike.blubrry.net/2017/11/08/episode-7-nov-8-2017/). Stanford using Rust, the Rust AMA, RLS release dates, crates.io growth, and Servo audio.

# Crate of the Week

This week's crate is [failure](https://github.com/withoutboats/failure), a crate to deal with... you guessed it, failure. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Help us benchmark saturating float casts](https://internals.rust-lang.org/t/help-us-benchmark-saturating-float-casts/6231).
* [Want to learn more about Wayland? Here are some detailed issues on how to help implement a safe wlroots wrapper in Rust](https://github.com/swaywm/wlroots-rs/issues/22).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

137 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-11-06..2017-11-13

* [add `Option::filter()`](https://github.com/rust-lang/rust/pull/45863) (RFC [#2124](https://github.com/LukasKalbertodt/rfcs/blob/8857fc3aa021058084e2a16af457e43249cc50ce/text/2124-option-filter.md))
* [refactor Option::filter method](https://github.com/rust-lang/rust/pull/45933)
* [MIR-borrowck: fix diagnostics for closures](https://github.com/rust-lang/rust/pull/45927)
* [compiletest: Fix a couple of test re-run issues](https://github.com/rust-lang/rust/pull/45917)
* [fix test case header parsing code in presence of multiple revisions](https://github.com/rust-lang/rust/pull/45914)
* [rustbuild: Disable ThinLTO for libtest](https://github.com/rust-lang/rust/pull/45908)
* [make saturating u128 -> f32 casts the default behavior](https://github.com/rust-lang/rust/pull/45900)
* [check::method - unify receivers before normalizing method signatures](https://github.com/rust-lang/rust/pull/45890)
* [fix core for targets with max-atomic-width = 0](https://github.com/rust-lang/rust/pull/45882)
* [restore move out dataflow, add report of move out errors](https://github.com/rust-lang/rust/pull/45877)
* [implement arbitrary_self_types](https://github.com/rust-lang/rust/pull/45870)
* [incr.comp.: Verify stability of incr. comp. hashes and clean up various other things](https://github.com/rust-lang/rust/pull/45867)
* [disable `mmap` in `libbacktrace` on Apple platforms](https://github.com/rust-lang/rust/pull/45866)
* [fix help for duplicated names: `extern crate (...) as (...)`](https://github.com/rust-lang/rust/pull/45856)
* [disable LLVM assertions on Nightly, enable them in "alt" builds](https://github.com/rust-lang/rust/pull/45810)
* [make positional argument error in format! clearer](https://github.com/rust-lang/rust/pull/45807)
* [deduplicate projection error (E0271) messages](https://github.com/rust-lang/rust/pull/45952)
* [add missing div and rem vector intrinsics](https://github.com/rust-lang/rust/pull/45804)
* [prefer libproc_macro APIs to libsyntax ones in the quasi-quoter](https://github.com/rust-lang/rust/pull/45791)
* [fixes to MIR effectck](https://github.com/rust-lang/rust/pull/45785)
* [display all emission types in error msg if user inputs invalid option](https://github.com/rust-lang/rust/pull/45782)
* [accept interpolated patterns in trait method parameters](https://github.com/rust-lang/rust/pull/45775)
* [add error for `...` in expressions](https://github.com/rust-lang/rust/pull/45773)
* [resolve: Use same rules for disambiguating fresh bindings in `match` and `let`](https://github.com/rust-lang/rust/pull/45050)
* [change MIR dump filenames from `rustc.nodeN...` to `rustc.<DefPath>`](https://github.com/rust-lang/rust/pull/45757)
* [fix MIR CopyPropagation errneously propagating assignments to function args](https://github.com/rust-lang/rust/pull/45753)
* [handle anon lifetime arg being returned with named lifetime return type](https://github.com/rust-lang/rust/pull/45751)
* [refactor internal suggestion API](https://github.com/rust-lang/rust/pull/45741)
* [extend NLL with preliminary support for free regions on functions](https://github.com/rust-lang/rust/pull/45668)
* [allow overriding the thread-local statics model](https://github.com/rust-lang/rust/pull/45666)
* [use a `Set<T>` instead of a `Map<T, bool>`](https://github.com/rust-lang/rust/pull/45736)
* [regenerate libcore/char_private.rs](https://github.com/rust-lang/rust/pull/45571)
* [detect `=` → `:` typo in let bindings](https://github.com/rust-lang/rust/pull/45452)
* [forbid casting to/from a pointer of unknown kind](https://github.com/rust-lang/rust/pull/45735)
* [working towards a libc-less (wasm32) libstd](https://github.com/rust-lang/rust/pull/45725)
* [rustc: add item name to deprecated lint warning](https://github.com/rust-lang/rust/pull/45707)
* [RwLock guards are Sync if T is](https://github.com/rust-lang/rust/pull/45682)
* [remove `T: Sized` on pointer `as_ref()` and `as_mut()`](https://github.com/rust-lang/rust/pull/44932)
* [impl FromIterator<()> for ()](https://github.com/rust-lang/rust/pull/45379)
* [improve SliceExt::binary_search performance](https://github.com/rust-lang/rust/pull/45333)
* [saturating casts between integers and floats](https://github.com/rust-lang/rust/pull/45205)
* [`OccupiedEntry::replace_entry`](https://github.com/rust-lang/rust/pull/45152)
* [cargo: List available binary names](https://github.com/rust-lang/cargo/pull/4673)
* [rustdoc: Fix duplicated impls with generics](https://github.com/rust-lang/rust/pull/45620)
* [rustdoc: Search over generic types in docs](https://github.com/rust-lang/rust/pull/45673)
* [rustdoc: add more elements in the sidebar](https://github.com/rust-lang/rust/pull/45766)
* [rustdoc: add `#[allow(unused)]` to every doctest](https://github.com/rust-lang/rust/pull/45764)

## New Contributors

* Alec Theriault
* Alkis Evlogimenos
* John Ford
* John-John Tedro
* Sebastian Dröge
* Sébastien Santoro
* Shotaro Yamada

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2124: Add `Option::filter` to the standard library](https://github.com/rust-lang/rfcs/pull/2124).

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

* [Minimal target feature unsafe](https://github.com/rust-lang/rfcs/pull/2212).

# Upcoming Events

* [Nov 16. Cambridge Rust Meetup #5](https://www.meetup.com/Cambridge-Rust-Meetup/events/244114730/).
* [Nov 16. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Nov 19. Rust India Community Monthly Call](https://reps.mozilla.org/e/rust-india-monthly-call/).
* [Nov 21. Beginning Rust and Rust Hack Night @ Valtech Stockholm Sweden](https://www.meetup.com/ruststhlm/events/244792464/).
* [Nov 21. Rust Zürich - Intro to Rust: November Community Meetup](https://www.meetup.com/Rust-Zurich/events/244698503/).
* [Nov 22. Rust Milano - Meet Rust Language with a live coding session](https://www.meetup.com/rust-language-milano/events/245059623).
* [Nov 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 24. Monkey Tech Days Toulouse, France - Explore Languages (Go Vs Rust) - MKTD#5](https://www.meetup.com/Monkey-Tech-Days/events/237545492/).
* [Nov 25. Rust Bangalore - Rust Concurrency (part 2 of 2)](https://www.meetup.com/rustox/events/244782966/).
* [Nov 27. Triangle Rustaceans Durham, NC - Algebraic Data Types in Practice and Theory](https://www.meetup.com/triangle-rustaceans/events/kkjnpnywpbkc).
* [Nov 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 29. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywpbmc).
* [Nov 30. Rust Munich: Rust Machine Learning with Juice](https://www.meetup.com/rust-munich/events/244580709/).
* [Nov 30. Rust Detroit - Introducing Tock OS 1.0](https://www.meetup.com/rust-detroit/events/244855856/).
* [Nov 30. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developers at stokebrain](https://users.rust-lang.org/t/rust-developers-wanted-for-startup/13784).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
