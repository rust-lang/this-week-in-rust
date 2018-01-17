Title: This Week in Rust 217
Number: 217
Date: 2018-01-16
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

* [Announcement: a new Release Team, and new leadership for the Infrastructure Team](https://internals.rust-lang.org/t/announcement-a-new-release-team-and-new-leadership-for-the-infrastructure-team/6562).
* [Clap's new year's weight loss program with cargo-bloat](https://clap.rs/2018/01/09/new-years-weight-loss/).
* [What’s Tokio and async IO all about](https://manishearth.github.io/blog/2018/01/10/whats-tokio-and-async-io-all-about/)?
* [Writing complex macros in Rust: Reverse Polish notation](https://rreverser.com/writing-complex-macros-in-rust/).
* [About the undefined behavior](https://vorner.github.io/undefined.html).
* [Benchmarking in stable Rust with criterion.rs](https://bheisler.github.io/post/benchmarking-with-criterion-rs/).
* [Rust and WebAssembly With Turtle](https://varblog.org/blog/2018/01/08/rust-and-webassembly-with-turtle/).
* [Debugging Rust programs with lldb on MacOS](https://bryce.fisher-fleig.org/blog/debugging-rust-programs-with-lldb/index.html).
* [A year with Rust & game dev](http://druerridge.com/?p=536).
* [This week in Rust docs 88](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-88).

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [artifact](https://github.com/vitrial/artifact), a design documentation tool. Thanks to [musicmatze](https://users.rust-lang.org/u/musicmatze) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [doc] [good first issue] [Help Rayon prepare for 1.0](https://users.rust-lang.org/t/rayon-1-0-on-feb-14/14950).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Tera: Allow filters in math operations in 0.11](https://github.com/Keats/tera/issues/244). Tera is a template engine for Rust based on Jinja2/Django.
* [good first issue] [Gutenberg: Make content::Section hold references](https://github.com/Keats/gutenberg/issues/205). Gutenberg is an opinionated static site generator with everything built-in.
* [good first issue] [Aardwolf: Routing for web templates](https://github.com/BanjoFox/aardwolf/issues/69). Aardwolf is a platform for creating new social networks, connected across the web.
* [good first issue] [miniz_oxide: Port CVE and other tests from zlib-ng](https://github.com/Frommi/miniz_oxide/issues/17). miniz_oxide is a Rust replacement for miniz deflate/zlib encoder/decoder.
* [mdBook: Add not found page](https://github.com/rust-lang-nursery/mdBook/issues/539).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

130 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-01..2018-01-08

* [delete the old docs, lift up the new](https://github.com/rust-lang/cargo/pull/4904)
* [generate code for unused const- and inline-fns if -Clink-dead-code is specified](https://github.com/rust-lang/rust/pull/46916)
* [allow non-alphabetic underscores in camel case](https://github.com/rust-lang/rust/pull/46907)
* [NLL fixes](https://github.com/rust-lang/rust/pull/46984)
* [only bump error count when we are sure that the diagnostic is not a repetition](https://github.com/rust-lang/rust/pull/47146)
* [limit style lint to non-synthetic generic params](https://github.com/rust-lang/rust/pull/47132)
* [try to improve LLVM pass ordering](https://github.com/rust-lang/rust/pull/46739)
  and [the pass manager order](https://github.com/rust-lang/llvm/pull/101)
* [use name-discarding LLVM context](https://github.com/rust-lang/rust/pull/47220)
* [force appropriate extension when converting from int to ptr](https://github.com/rust-lang/rust/pull/47147)
* [delay panic for aliasing violation for static items](https://github.com/rust-lang/rust/pull/47105)
  and [from incoherent drop implementation](https://github.com/rust-lang/rust/pull/47104)
* [add 'Span::parent()' and 'Span::source()' to proc_macro API](https://github.com/rust-lang/rust/pull/47099)
* [`CStore` switch `FxHashMap` to `IndexVec`](https://github.com/rust-lang/rust/pull/46913)
* [implement `TrustedRandomAccess` for `slice::`{`Chunks`, `ChunksMut`, `Windows`}](https://github.com/rust-lang/rust/pull/47142)

## New Contributors

* Alexander Regueiro
* Alexis Hunt
* Bulat Musin
* Dan Robertson
* Fenrir
* Kagamihime
* muvlon
* Neil Shen
* O01eg
* ritiek
* Ryan Cumming

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

* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Benchmarking / cargo bench](https://github.com/rust-lang/rfcs/pull/2287).
* [Associated type bounds of form `MyTrait<AssociatedType: Bounds>`](https://github.com/rust-lang/rfcs/pull/2289).
* [Add `std::mem::zero`](https://github.com/rust-lang/rfcs/pull/2291).
* [Allow `if let` guards in `match` expressions](https://github.com/rust-lang/rfcs/pull/2294).

# Upcoming Events

* [Jan 18. Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/245934654/).
* [Jan 18. Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/mgtcwnyxcbxb/).
* [Jan 20. Rust Bangalore - Rust for newbies (part 1 of 12)](https://www.meetup.com/rustox/events/246863039/).
* [Jan 22. Durham, NC - Triangle Rustaceans - Rust 101](https://www.meetup.com/triangle-rustaceans/events/kkjnpnyxcbdc/).
* [Jan 22. Lyon, France - TupperRust (registration required)](https://framaforms.org/inscription-obligatoire-tupperrust-de-janvier-2018-a-lens-lyon-1515789658).
* [Jan 23. A deep dive into Rust @ Facebook Developer Circle Ruhr](https://www.meetup.com/Facebook-Developer-Circle-Ruhr/events/246462601/).
* [Jan 23. Boston Rust - January Meetup at Amazon](https://www.meetup.com/BostonRust/events/246571213/).
* [Jan 24. Milano - Overload di funzioni in Rust - Come ho imparato a vivere felicemente senza](https://www.meetup.com/rust-language-milano/events/246439486/).
* [Jan 24. Rust NYC - Traits](https://www.meetup.com/Rust-NYC/events/246695372/).
* [Jan 24. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan 24. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Jan 25. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Jan 29. Rust London User Group - LDN Talks: January 2018](https://www.meetup.com/Rust-London-User-Group/events/246637221/).
* [Jan 31. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Jan 31. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

# Quote of the Week

> Anything that will make wasm nicer will be awesome, but honestly, I’m thrilled with what we’ve got. It feels absolutely insane that I can just compile this language that’s basically the opposite of JavaScript and it’s running in the browser.

— [Tomas Sedovic in a #Rust2018 post](https://aimlesslygoingforward.com/blog/2018/01/10/rust-2018/).

Thanks to [ErichDonGubler and CAD97 for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/482)!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
