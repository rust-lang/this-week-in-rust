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

* [WebRender capture infrastructure](https://kvark.github.io/webrender/debug/ron/2018/01/23/wr-capture-infra.html).

## #Rust2018

Find all #Rust2018 posts at [Read Rust](http://readrust.net/rust2018/).

# Crate of the Week

This week's crate is [cargo-bloat](https://github.com/RazrFalcon/cargo-bloat), a cargo subcommand to find out how much space crates/functions take up in an executable. Thanks to [Vikrant](https://users.rust-lang.org/u/nasa42) for the suggestion!

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

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-08..2018-01-15

* [add a "-Z offline" flag to Cargo, altering it's dependency resolution behavior](https://github.com/rust-lang/cargo/pull/4770)
* [`auto trait Foo { .. }`](https://github.com/rust-lang/rust/pull/47416)
* [fix std breakage with custom libdir](https://github.com/rust-lang/rust/pull/46592)
* [syntax: rewrite parsing of impls](https://github.com/rust-lang/rust/pull/46455)
* [add error code for unstable feature errors](https://github.com/rust-lang/rust/pull/47413)
* [type error method suggestions use whitelisted identity-like conversions](https://github.com/rust-lang/rust/pull/46461)
* [avoid panicking when invalid argument is passed to `cfg(..)`](https://github.com/rust-lang/rust/pull/47372)
* [NLL: bad error message when converting anonymous lifetime to `'static`](https://github.com/rust-lang/rust/pull/47329)
* [NLL: resolve type and region variables in "NLL dropck"](https://github.com/rust-lang/rust/pull/47322)
* [treat `#[path]` files as mod.rs files](https://github.com/rust-lang/rust/pull/47298)
* [shorten names of some compiler generated artifacts](https://github.com/rust-lang/rust/pull/47269)
* [account for `pub` in `const` → `static` suggestion](https://github.com/rust-lang/rust/pull/47262)
* [remove deprecated unstable attribute `#[simd]`](https://github.com/rust-lang/rust/pull/47251)
* [rustc: tweak `#[target_feature]` syntax](https://github.com/rust-lang/rust/pull/47223)
* [macros: improve 1.0/2.0 interaction](https://github.com/rust-lang/rust/pull/46551)
* [make double ended searchers use dependent fingers](https://github.com/rust-lang/rust/pull/47208)
* [add iterator method specialisations to `Range*`](https://github.com/rust-lang/rust/pull/47180)
* [fix built-in indexing not being used where index type wasn't "obviously" `usize`](https://github.com/rust-lang/rust/pull/47167)
* [add `slice::`{`ExactChunks`, `ExactChunksMut`} iterators](https://github.com/rust-lang/rust/pull/47126)
* [better Debug impl for io::Error](https://github.com/rust-lang/rust/pull/47120)
* [fix nested imports not included in the save_analysis output](https://github.com/rust-lang/rust/pull/47081)
* [implement AsRef<Path> for Component](https://github.com/rust-lang/rust/pull/46985)
* [`BufRead`: only flush the internal buffer if seeking outside of it](https://github.com/rust-lang/rust/pull/46832)
* [pre-allocate in `fs::`{`read`, `read_string`}](https://github.com/rust-lang/rust/pull/47324)
* [implement `Write` for `Cursor<&mut Vec<T>>`](https://github.com/rust-lang/rust/pull/46830)
* [fix off-by-one error in `BufWriter`](https://github.com/rust-lang/rust/pull/47330)
* [deprecate `[T]::rotate` in favor of `[T]::rotate_`{`left`, `right`}](https://github.com/rust-lang/rust/pull/46777)
* [add `HashMap::remove_entry`](https://github.com/rust-lang/rust/pull/47259)

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
