Title: This Week in Rust 218
Number: 218
Date: 2018-01-23
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

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [actix-web](https://github.com/actix/actix-web), a small fast pragmatic open-source Rust web framework. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

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

144 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-15..2018-01-22

* [implement repr(transparent)](https://github.com/rust-lang/rust/pull/47158)
* [compute LLVM argument indices correctly in face of padding](https://github.com/rust-lang/rust/pull/47401)
* [whitelist x86 fxsr feature](https://github.com/rust-lang/rust/pull/47514)
* [rustc_trans: reorganize CrateContext and rename context types](https://github.com/rust-lang/rust/pull/47209)
* [remove noop landing pads in cleanup shims](https://github.com/rust-lang/rust/pull/47467)
* [custom error when moving arg outside of its closure](https://github.com/rust-lang/rust/pull/47144)
* [tweaks to invalid ctor messages](https://github.com/rust-lang/rust/pull/47116)
* [rename std::ptr::Shared to NonNull and stabilize it](https://github.com/rust-lang/rust/pull/46952)
* [point at unused arguments for format string](https://github.com/rust-lang/rust/pull/47481)
* [do not suggest to make `mut` binding external to `Fn` closure](https://github.com/rust-lang/rust/pull/47468)
* [add transpose conversions for nested Option and Result](https://github.com/rust-lang/rust/pull/47193)
* [deprecate std::net::lookup_host](https://github.com/rust-lang/rust/pull/47510)
* [optimize `slice::`{`position`, `rposition`} result bounds check](https://github.com/rust-lang/rust/pull/47333)
* [implement "only-<platforms>" for test headers](https://github.com/rust-lang/rust/pull/47487)
* [cargo: allow packaging of crates with unstable features](https://github.com/rust-lang/cargo/pull/4955)
* [rustdoc: switch to pulldown as default markdown renderer](https://github.com/rust-lang/rust/pull/47398)
* [rust-installer: Stream the parallel xz/gz tarball generation](https://github.com/rust-lang/rust-installer/pull/76)

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
