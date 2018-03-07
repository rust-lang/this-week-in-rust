Title: This Week in Rust 224
Number: 224
Date: 2018-03-06
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

# Crate of the Week

This week's crate is [trace](https://github.com/gsingh93/trace), a crate to allow for quick debug outputs without `println!`. Thanks to [gilescope](https://users.rust-lang.org/u/gilescope) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [doc] [rustc-guide](https://github.com/rust-lang-nursery/rustc-guide): A guide to how rustc works and how to contribute to it [has issues that need help](https://github.com/rust-lang-nursery/rustc-guide/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22). The difficulty ranges from "copy the contents of the rustc READMEs" or "proofreading" to "write about the dark bowls of X in rustc". Prior knowledge of compiler internals is _not_ required.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

141 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-26..2018-03-05

* [comprehensively support trailing commas in std/core macros](https://github.com/rust-lang/rust/pull/48056)
* [type check defaults at declaration](https://github.com/rust-lang/rust/pull/46785)
* [fix spans ending not on UTF8 character bound](https://github.com/rust-lang/rust/pull/48522) (UTF8 is hard)
* [NLL: avoid borrowed value must be valid for lifetime '_#2r..." in errors](https://github.com/rust-lang/rust/pull/48592)
* [replace Rc with Lrc for shared data](https://github.com/rust-lang/rust/pull/48586)
* [support parentheses in patterns under feature gate](https://github.com/rust-lang/rust/pull/48500)
* [restrict the `Termination` impls to simplify stabilization](https://github.com/rust-lang/rust/pull/48497)
* [rustc: tweak funclet cleanups of FFI functions](https://github.com/rust-lang/rust/pull/48572)
* [add functionality for epoch lints; add epoch lint for dyn-trait](https://github.com/rust-lang/rust/pull/48461)
  and [use `dyn trait` everywhere](https://github.com/rust-lang/rust/pull/48477)
* [implement `--remap-path-prefix`](https://github.com/rust-lang/rust/pull/48359)
* [make TransitiveRelation thread safe](https://github.com/rust-lang/rust/pull/48587)
* [backport LLVM fixes for a `JumpThreading` / `assume` intrinsic bug](https://github.com/rust-lang/rust/pull/48583)
* [bring back `ParamEnv` deduplication](https://github.com/rust-lang/rust/pull/48576)
* [turn down xz compression level](https://github.com/rust-lang/rust-installer/pull/80)
* [include stdsimd in rust-src component](https://github.com/rust-lang/rust/pull/48736)
* [add std::path::Path::ancestors](https://github.com/rust-lang/rust/pull/48420)
* [stabilize `[T]::rotate_`{`left`, `right`}](https://github.com/rust-lang/rust/pull/48450)
* [stabilize unsafe pointer methods](https://github.com/rust-lang/rust/pull/48259)
* [stabilize `LocalKey::try_with`](https://github.com/rust-lang/rust/pull/48585)
* [specialize `Zip::nth` for `TrustedRandomAccess`](https://github.com/rust-lang/rust/pull/48635)
* [`impl Clone for ::std_unicode::char::`{`ToLowercase`, `ToUppercase`}](https://github.com/rust-lang/rust/pull/48629)
* [`impl Default + Hash for ::core::cmp::Reverse`](https://github.com/rust-lang/rust/pull/48628)
* [package lock files in published crates](https://github.com/rust-lang/cargo/pull/5093)
* [cache the query result](https://github.com/rust-lang/cargo/pull/5112)
* [support `--exclude` option for `cargo doc`](https://github.com/rust-lang/cargo/pull/5081)
* [support `+toolchain` rustup override in bash completions](https://github.com/rust-lang/cargo/pull/5111)

## New Contributors

* Andreas Streichardt
* Anthony Deschamps
* boats
* Bryan Drewery
* csmoe
* Dale Wijnand
* Dan Aloni
* Federico Poli
* hedgehog1024
* Hidehito Yabuuchi
* Jakub Adam Wieczorek
* Jimmy Brush
* moe
* Nathan Ringo
* newpavlov
* Vitali Lovich

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2325: Stable SIMD in Rust](https://github.com/rust-lang/rfcs/pull/2325).
* [RFC 2226: Hexadecimal integers with fmt::Debug, including within larger types](https://github.com/rust-lang/rfcs/pull/2226).
* [RFC 2151: Raw identifiers](https://github.com/rust-lang/rfcs/pull/2151). Add a raw identifier format `r#ident`, so crates written in future language epochs/versions can still use an older API that overlaps with new keywords.
* [RFC 2298: `?` repetition in macro rules](https://github.com/rust-lang/rfcs/pull/2298).
* [RFC 2046: label-break-value](https://github.com/rust-lang/rfcs/pull/2046). Allow a break not only out of `loop`, but of labelled blocks with no loop
* [RFC 2333: Prior art](https://github.com/rust-lang/rfcs/pull/2333). A section to the RFC template where RFC authors may discuss the experience of other programming languages.


## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Rust 2018 roadmap](https://github.com/rust-lang/rfcs/pull/2314).
* [disposition: merge] [Cargo profile dependencies](https://github.com/rust-lang/rfcs/pull/2282).
* [disposition: merge] [Allow locals and destructuring in const fn](https://github.com/rust-lang/rfcs/pull/2341).
* [disposition: merge] [Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).
* [disposition: merge] [Amend RFC 0141 Lifetime elision: Mention deduplicated lifetimes](https://github.com/rust-lang/rfcs/pull/2330).
* [disposition: merge] [Update the disambiguation handling in RFC 1946 (intra-rustdoc-links) to match impl concerns](https://github.com/rust-lang/rfcs/pull/2285).

## New RFCs

* [Standard library API for immovable types](https://github.com/rust-lang/rfcs/pull/2349).
* [Allow `loop` in constant evaluation](https://github.com/rust-lang/rfcs/pull/2344).
* [Allow panicking in constants](https://github.com/rust-lang/rfcs/pull/2345).
* [`#[derive_no_bound(..)]` and `#[derive_field_bound(..)]`](https://github.com/rust-lang/rfcs/pull/2353).
* [Add `is_sorted` to the standard library](https://github.com/rust-lang/rfcs/pull/2351).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar  4. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbgb/).
* [Mar  5. London, GB - Rust learning and hacking evening #8](https://www.meetup.com/Rust-London-User-Group/events/247286584/).
* [Mar  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxfbjb/).
* [Mar  7. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlyxfbkb/).
* [Mar  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxfbkb/).
* [Mar  7. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar  7. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxfblb/).
* [Mar  8. San Diego, US - San Diego Rust March Meetup](https://www.meetup.com/San-Diego-Rust/events/248229805/).
* [Mar 11. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbpb/).
* [Mar 12. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxfbqb/).
* [Mar 13. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content).
* [Mar 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 13. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/fmwshpyxfbtb/).
* [Mar 16. Frankfurt, DE - Rust Table of Regulars](https://www.meetup.com/Rust-Rhein-Main/events/248326240).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer at Vistaprint](https://careers.vistaprint.com/job-description/?id=23901).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
