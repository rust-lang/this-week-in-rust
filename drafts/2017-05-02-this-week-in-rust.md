Title: This Week in Rust 180
Number: 180
Date: 2017-05-02
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

* Last week's [visual guide for rustfmt’s configs](https://github.com/regexident/rustfmt-configs-guide) has been [merged into rustfmt project](https://github.com/rust-lang-nursery/rustfmt/blob/master/Configurations.md) along with 120 unit tests.

# Crate of the Week

This week's crate of the week is [indicatif](https://crates.io/crates/indicatif), a crate to show nice progress bars on the console. Thanks to [Willi Kappler](https://users.rust-lang.org/users/willi_kappler) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [What topics would you like to see covered in a video course about Rust](https://users.rust-lang.org/t/what-topics-would-you-like-to-see-covered-in-a-video-course-about-rust/10500)?
* [rust: Debian Rust packages](https://github.com/rust-lang/rust/issues/28307#issuecomment-295283017).
* [rdedup](https://github.com/dpc/rdedup) - a data deduplication with compression and public key encryption library, is [looking for contributors](https://users.rust-lang.org/t/twir-call-for-participation/4821/42) who are interested in crypto, command line, and backups.
* [PumpkinDB](https://github.com/PumpkinDB/PumpkinDB) has a list of [starter issues](https://github.com/PumpkinDB/PumpkinDB/issues?q=is%3Aissue+is%3Aopen+label%3AWhatCanIStartWith%3F) for [people interested in an event sourcing database engine](https://users.rust-lang.org/t/twir-call-for-participation/4821/43).
* [easy] [maud: Remove `error!` macro](https://github.com/lfairy/maud/issues/84). Maud is an HTML template engine for Rust.
* [easy] [maud: Document toggled classes](https://github.com/lfairy/maud/issues/85).
* [less easy] [rust-bindgen: Add in-worklist bits to the analysis runner](https://github.com/servo/rust-bindgen/issues/664).
* [easy] [flate2: Use distinct Flush types for `Compress::compress` vs `Decompress::decompress`](https://github.com/alexcrichton/flate2-rs/issues/79). flate2 implements FLATE, Gzip, and Zlib bindings for Rust.
* [easy] [flate2: Write usage examples](https://github.com/alexcrichton/flate2-rs/issues/76).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

120 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-04-24..2017-05-01

* [LLVM upgraded to 4.0](https://github.com/rust-lang/rust/pull/40123) (Hooray!)
* [rust-windbg.cmd script added](https://github.com/rust-lang/rust/pull/39983) (Windows™ users rejoice)
* [file path remapping for debuginfo and reproducible builds](https://github.com/rust-lang/rust/pull/41508)
* [compile OpenSSL with one core only](https://github.com/rust-lang/rust/pull/41563) (File under: things that wouldn't happen in Rust)
* [associated consts are not object safe](https://github.com/rust-lang/rust/pull/41494)
* [parse trait objects startin with a lifetime bound](https://github.com/rust-lang/rust/pull/41542)
* [typeck now resolves vars before `try_index_step`](https://github.com/rust-lang/rust/pull/41578)
* [ICE on cross-crate associated type mismatch fixed](https://github.com/rust-lang/rust/pull/41583)
* [adjust `OperandPair` struct field index](https://github.com/rust-lang/rust/pull/41529)
* [fix lvalue ops handling](https://github.com/rust-lang/rust/pull/41651)
* [`#[used]` items are never dead code](https://github.com/rust-lang/rust/pull/41637)
* [improve error on moving out of indexed content](https://github.com/rust-lang/rust/pull/41564)
* [point to variables moved by closure on error](https://github.com/rust-lang/rust/pull/41523)
* queries for [`def_span`](https://github.com/rust-lang/rust/pull/41593), [`describe_def`](https://github.com/rust-lang/rust/pull/41534),
also [ergonomic improvements](https://github.com/rust-lang/rust/pull/41504)
* [we can `default impl` traits](https://github.com/rust-lang/rust/pull/37860)
* [`Step::replace_one` no longer puts a zero instead of one](https://github.com/rust-lang/rust/pull/41493)
* [`traits::select` filters out other traits' predicates](https://github.com/rust-lang/rust/pull/41486) (for faster compiles)
* [cache symbols in `ty::maps`](https://github.com/rust-lang/rust/pull/41507)
* [stricter privacy checks for trait items](https://github.com/rust-lang/rust/pull/41332)
* [reduce stack requirement for floating-point formatting](https://github.com/rust-lang/rust/pull/41509)
* [more methods, less transmute for `Box<`{`[u8]`, `str`}`>`](https://github.com/rust-lang/rust/pull/41258)
* [`io::`{`Chain`, `Take`}`::into_inner()`](https://github.com/rust-lang/rust/pull/41463)
* [{`Vec`, `String`}`::splice(..)`](https://github.com/rust-lang/rust/pull/40434)
* [`impl From<&mut [T]> for Vec<T>`](https://github.com/rust-lang/rust/pull/41530)
* [`FromIterator` and `extend` for `Cow<str>`](https://github.com/rust-lang/rust/pull/41449)
* [`cargo` {`build`, `test`, ...} `--all-`{`bins`, `tests`, `benches`, `examples`}](https://github.com/rust-lang/cargo/pull/3901)
* [The rust installer now has xz-compressed tarballs](https://github.com/rust-lang/rust-installer/pull/57) (will allow for smaller downloads in the future)
* [Haiku is now a supported OS target](https://github.com/rust-lang/rust-installer/pull/58)


## New Contributors

* Dylan Maccora
* Maxwell Paul Brickner
* Nicolas Bigaouette

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1733: Trait alias](https://github.com/rust-lang/rfcs/pull/1733).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: close] [Check future-proofing of `macro_rules!` using FIRST sets](https://github.com/rust-lang/rfcs/pull/1746).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Proposal for default crate recommendation ranking](https://github.com/rust-lang/rfcs/pull/1824).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).
* [disposition: merge] [A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [disposition: postpone] [Add `SafeDeref` and `SafeDerefMut`, equivalent to `Deref` and `DerefMut` but which are guaranteed to always return the same object](https://github.com/rust-lang/rfcs/pull/1873).
* [disposition: postpone] [Unsafe lifetime](https://github.com/rust-lang/rfcs/pull/1918). Add a new special lifetime, `'unsafe`, that implicitly satisfies any constraint, but may only be instantiated within an unsafe context.

## New RFCs

* [Make RangeInclusive just a two-field struct (amend 1192)](https://github.com/rust-lang/rfcs/pull/1980).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

PRs:

* [Statements](https://github.com/rust-lang-nursery/fmt-rfcs/pull/81)

Issues in final comment period:

* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71)
* [Struct and tuple literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64)
* [Array literals](https://github.com/rust-lang-nursery/fmt-rfcs/issues/63)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)
* [Imports (`use`)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/24)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)

Other interesting issues:

* [Function calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/64) and [chains of calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/66)
* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)

# Upcoming Events

* [Apr 27. Rust Mexico City Meetup #5](https://www.meetup.com/es-ES/Rust-MX/events/239279107/).
* [Apr 27. Rust Stockholm - Rust meetup @ Distil Networks](https://www.meetup.com/ruststhlm/events/238207716/).
* [Apr 27. Rust Meetup Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/28).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**
* [Apr 30. Kickstart to Rust Mozilla Gujarat User Group](https://reps.mozilla.org/e/kickstart-to-rust-mozilla-gujarat-user-group/).
* [May  2. Rust Workshop - RustBridge - Dortmund, Germany](https://www.meetup.com/Softwerkskammer-Ruhrgebiet/events/239380377/).
* [May  3. Intro to Rust for Java programmers - Code@LTH](https://www.facebook.com/events/1395576530485976/).
* [May  3. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238448447/).
* [May  3. Boston Rust: Rust 1.0 Anniversary Party and Hack Night](https://www.meetup.com/BostonRust/events/239319480/).
* [May  3. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May  3. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May  4. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May  4. Rust Bay Area: Using Rust at Dropbox to make Magic Pocket](https://www.meetup.com/Rust-Bay-Area/events/239222217/).
* [May  8. Prague Rust Meetup #3](https://www.meetup.com/rust-prague/events/239129625/).
* [May  8. Seattle Rust - Monthly Meetup](https://www.meetup.com/Seattle-Rust-Meetup/).
* [May 10. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 10. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywhbpb/).
* [May 11. Rust DC - Building high performance REST APIs with Rust and Rocket](https://www.meetup.com/RustDC/events/239115583/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [RustJobs.rs](https://rustjobs.rs) - a website dedicated to Rust jobs. There are 15 open Rust positions plus a list of companies using Rust.
* [Systems Programmer at Hadean](https://rustjobs.rs/jobs/17/hadean-systems-programmer) - C and Rust
* [Software Engineer (Full Stack) at One Codex](https://rustjobs.rs/jobs/16/one-codex-software-engineer-full-stack-) - new code in Rust
* [Rust Software Engineer at resin.io](https://resin.workable.com/j/ACF748D4A2).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> There are many ways in which Rust is like a version of C/C++ that mutated when Haskell was injected into its veins.

— [Lokathor on reddit](https://www.reddit.com/r/rust/comments/667ocp/why_are_some_people_comparing_rust_to_haskell/dggbked/).

Thanks to [Johan Sigfrids](https://users.rust-lang.org/t/twir-quote-of-the-week/328/392) and [liquidivy](https://www.reddit.com/r/rust/comments/667ocp/why_are_some_people_comparing_rust_to_haskell/dgge8vj/) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
