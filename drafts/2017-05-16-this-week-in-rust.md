Title: This Week in Rust 182
Number: 182
Date: 2017-05-16
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

* [Mles - a new protocol in Rust](http://mles.io/blog).

# Crate of the Week

This week's crate of the week is [PX8](https://github.com/Gigoteur/PX8), a Rust implementation of an Open Source fantasy console. Thanks to [hallucino](https://users.rust-lang.org/users/hallucino) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Crate evaluation for 2017-05-16: log](https://internals.rust-lang.org/t/crate-evaluation-for-2017-05-16-log/5185). People are needed to help fill out the log crate evaluation, write cookbook recipes for the log crate, and generally offer their opinions.
* [cross: OpenSSL missing for i686-musl](https://github.com/japaric/cross/issues/27). cross allows you to do "zero setup" cross compilation and "cross testing" of Rust crates.
* [cross: libmusl with static OpenSSL](https://github.com/japaric/cross/issues/21).
* [easy] [url: Implement Debug for many types](https://github.com/servo/rust-url/issues/305). Servo's `url` is a URL parser library for Rust.
* [url: Implement `Default` for `ParseOptions` and `ParseOptions::new`](https://github.com/servo/rust-url/issues/301).
* [easy] [url: Modify docs to put error conditions into `Errors` sections](https://github.com/servo/rust-url/issues/314).
* [url: Better documentation for quirks module](https://github.com/servo/rust-url/issues/311).
* [easy] [url: Improvements to `Origin` docs](https://github.com/servo/rust-url/issues/310).
* [easy] [url: Add examples to `Url` methods](https://github.com/servo/rust-url/issues/309).
* [easy] [url: Add examples to `ParseOptions`](https://github.com/servo/rust-url/issues/308).
* [url: Modify `define_encode_set` to support private definitions](https://github.com/servo/rust-url/issues/307).
* [url: Document the percent_encoding module](https://github.com/servo/rust-url/issues/298).
* [easy] [rust-cookbook: Switch error handling setup to quick_main! macro from error-chain](https://github.com/brson/rust-cookbook/issues/59). Rust Cookbook is a collection of simple examples that demonstrate good practices to accomplish common programming tasks.
* [memmap: Overhaul API](https://github.com/danburkert/memmap-rs/issues/33). memmap-rs is a Rust library for cross-platform memory-mapped file IO.
* [memmap: Document error conditions for fallible methods in a separate "Errors" section](https://github.com/danburkert/memmap-rs/issues/37).
* [memmap: Add examples to methods](https://github.com/danburkert/memmap-rs/issues/34).
* [memmap: Expand crate-level documentation](https://github.com/danburkert/memmap-rs/issues/32).
* [easy] [tokei: AutoHotKey support](https://github.com/Aaronepower/tokei/issues/106). Tokei is a program that displays statistics about your code.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

125 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-05-08..2017-05-15

* [disallow `._` in float literal](https://github.com/rust-lang/rust/pull/41946) (breaking change, but...who does this?), also
  [`illegal_floating_point_literal_pattern` compat lint](https://github.com/rust-lang/rust/pull/41293)
* [enforce well-formedness after generalizing](https://github.com/rust-lang/rust/pull/41716)
* [`eprint!(..)` / `eprintln!(..)`](https://github.com/rust-lang/rust/pull/41192)
* [faster `[u8].reverse()`](https://github.com/rust-lang/rust/pull/41764)
* [polymorphic `span_label`](https://github.com/rust-lang/rust/pull/41745) (reduces plugin ceremony)
* [diagnostics: allow multiple suggestions](https://github.com/rust-lang/rust/pull/41876)
* [remove no longer needed libsyntax features](https://github.com/rust-lang/rust/pull/41729)
* [remove unused macros](https://github.com/rust-lang/rust/pull/41934)
* [more crate metadata queries](https://github.com/rust-lang/rust/pull/41724)
* [include crate root in `save-analysis`](https://github.com/rust-lang/rust/pull/41919)
* [finer crate hashing for incremental compilation](https://github.com/rust-lang/rust/pull/41709)
* [Windows io::Error improvements](https://github.com/rust-lang/rust/pull/41684)
* [`impl Clone for .split_whitespace()`](https://github.com/rust-lang/rust/pull/41659)
* [allow bare CR in doc comments](https://github.com/rust-lang/rust/pull/41827)
* [box large MIR variants](https://github.com/rust-lang/rust/pull/41926)
* [fix exponential LLVM code growth on inlining drops](https://github.com/rust-lang/rust/pull/41920)
* [RLS crash fixed](https://github.com/rust-lang/rust/pull/41969)
* [fix lvalue ops](https://github.com/rust-lang/rust/pull/41939)
* [fix error with `-Z treat-err-as-bug`](https://github.com/rust-lang/rust/pull/41942)
* [don't deny outer type parameters in embedded constants](https://github.com/rust-lang/rust/pull/41939)
* [equate items ignoring variance](https://github.com/rust-lang/rust/pull/41913)
* [new `-Z force-unstable-if-unmarked` flag](https://github.com/rust-lang/rust/pull/41847)
* [stabilize `-C target-feature=+crt-static`](https://github.com/rust-lang/rust/pull/41757)
* [cargo fetches only registry index master branch](https://github.com/rust-lang/cargo/pull/4024)
* [cargo now ignores malformed manifests on git deps](https://github.com/rust-lang/cargo/pull/3998)
* [cargo build script search path fixed](https://github.com/rust-lang/cargo/pull/3974)
* [cargo now retries requests that failed with 5XX](https://github.com/rust-lang/cargo/pull/4032)
* [cargo no longer checks out the whole crates.io index](https://github.com/rust-lang/cargo/pull/4026)
* [(mostly) rewrite the Rust installer in Rust](https://github.com/rust-lang/rust-installer/pull/62)

## New Contributors

* acdenisSK
* Bobby Holley
* Charlie Sheridan
* Jing Zhao
* Joshua Sheard
* Masaki Hara
* Migi
* Raphaël Huchet
* Tommy Ip

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

* [disposition: close] [Allow any Displayable type for expect](https://github.com/rust-lang/rfcs/pull/1968).
* [disposition: merge] [Expand and stabilize `impl Trait`](https://github.com/rust-lang/rfcs/pull/1951).
* [disposition: merge] [Specify `#[repr(transparent)]`](https://github.com/rust-lang/rfcs/pull/1758).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).

## New RFCs

* [`Result::pass()`, turning `Result<T,E>` into `Result<U,F>`, if `From` is set up](https://github.com/rust-lang/rfcs/pull/1996).
* [Opaque Data structs for FFI](https://github.com/rust-lang/rfcs/pull/1993).
* [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [Match branch semicolon](https://github.com/rust-lang/rfcs/pull/1994).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

We're making good progress and the style is coming together. If you want to see the style in practice, check out [our example](https://github.com/rust-lang-nursery/fmt-rfcs/blob/master/example/lists.rs) or use the [Integer32 Playground](https://play.integer32.com/) and select 'Proposed RFC' from the 'Format' menu. Be aware that implementation is work in progress.

Issues in final comment period:

* [Spaces around `=` in attributes](https://github.com/rust-lang-nursery/fmt-rfcs/issues/82)
* [Return type on new line](https://github.com/rust-lang-nursery/fmt-rfcs/issues/77) - proposed to close
* [Single-line where](https://github.com/rust-lang-nursery/fmt-rfcs/issues/74) - proposed to close
* [Attribute/doc comment ordering](https://github.com/rust-lang-nursery/fmt-rfcs/issues/72)
* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71)
* [Function calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/65) and [chains of calls](https://github.com/rust-lang-nursery/fmt-rfcs/issues/66)
* [Combining opening and closing delims](https://github.com/rust-lang-nursery/fmt-rfcs/issues/61)
* [Where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get involved

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)


# Upcoming Events

* [May 10. Rust Rome - Rust Meetup #2 - Intro + Rocket.rs](https://www.meetup.com/it-IT/Rust-Roma/events/239513275/).
* [May 10. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 10. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 11. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlywhbpb/).
* [May 11. Rust DC - Building high performance REST APIs with Rust and Rocket](https://www.meetup.com/RustDC/events/239115583/).
* [May 13. Mozilla Philippines - Introduction to Rust (Programming Language)](https://www.eventbrite.com/e/introduction-to-rust-programming-language-tickets-33749248912).
* [May 15. Rust Sydney Meetup - Happy Birthday Rust](https://www.meetup.com/Rust-Sydney/events/239659974/)!
* [May 16. Tokyo Rust Meetup #7 - Rust Birthday Party](https://www.meetup.com/Tokyo-Rust-Meetup/events/239301821/)!
* [May 16. Rust Paris Meetup #37](https://www.meetup.com/Rust-Paris/events/239723704/).
* [May 16. 1st Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/239688416/).
* [May 17. Rust LA May Meetup - Rust Birthday Party](https://www.meetup.com/Rust-Los-Angeles/events/239616841/)!
* [May 17. South Florida Rust - Rust Birthday Party](https://www.meetup.com/South-Florida-Rust-Meetup/events/239036595/)!
* [May 17. Rust Atlanta - Heterogeneous Collections in Rust at Tech Square Labs (Midtown)](https://www.meetup.com/Rust-ATL/events/239205124/).
* [May 17. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/239666001/).
* [May 17. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 17. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [May 18. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [May 20. Rust Bangalore community meetup](https://www.eventshigh.com/detail/Bangalore/9a49c6be73b6591e770d1cece7eec6fe-Rust-Bangalore-First-Meetup).
* [May 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [May 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Anixe](https://rustjobs.rs/jobs/21/anixe-rust-developer)
* [Rust Legend at Between Lines](https://rustjobs.rs/jobs/20/between-lines-ltd-rust-legend)
* [Research Engineer (Big Data) at Hadean](https://rustjobs.rs/jobs/18/hadean-research-engineer-big-data)
* [Core Developer for Parity Technologies](https://parity.io/#rust-dev-job)


*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The answer is obvious: it's the intersection of trust and frustration.

— [/u/kibwen answering - What does Rust mean?](https://www.reddit.com/r/rust/comments/69zca2/what_does_rust_mean/dhb20yb/).

Thanks to [Jean](https://github.com/cmr/this-week-in-rust/issues/438#issuecomment-300025367) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
