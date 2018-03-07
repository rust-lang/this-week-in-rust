Title: This Week in Rust 225
Number: 225
Date: 2018-03-13
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

* [Rust CLI Survey – Help the CLI WG determine which problems to tackle](https://goo.gl/forms/lqUTazC78j26ISu53).
* [Help write the Embedded Rust Book - an official guide to using Rust on microcontrollers](https://github.com/rust-lang-nursery/embedded-wg/issues/56).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).
* [Cargo: Abort crate resolution if too many candidates have been tried](https://github.com/rust-lang/cargo/issues/4066).
* [Cargo: Command to update Cargo.lock to minimal versions](https://github.com/rust-lang/cargo/issues/4100).

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

* Alexander Ronald Altman
* debris
* flip1995
* Jewoo Lee
* Lukas Lueg
* Tobias Stolzmann

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2314: Rust 2018 roadmap](https://github.com/rust-lang/rfcs/pull/2314).
* [RFC 2282: Cargo profile dependencies](https://github.com/rust-lang/rfcs/pull/2282).
* [RFC 2250: Finalize syntax of `impl Trait` and `dyn Trait` with multiple bounds](https://github.com/rust-lang/rfcs/pull/2250).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Add Euclidean modulo & division functionality for integers](https://github.com/rust-lang/rfcs/pull/2169).
* [disposition: merge] [Inherent traits](https://github.com/rust-lang/rfcs/pull/2309).
* [disposition: merge] [Allow locals and destructuring in const fn](https://github.com/rust-lang/rfcs/pull/2341).
* [disposition: merge] [Update the disambiguation handling in RFC 1946 (intra-rustdoc-links) to match impl concerns](https://github.com/rust-lang/rfcs/pull/2285).
* [disposition: postpone] [Function Structs](https://github.com/rust-lang/rfcs/pull/2276).

## New RFCs

* [Ghost busting](https://github.com/rust-lang/rfcs/pull/2357).

# Upcoming Events

The community team is trying to improve outreach to meetup organisers. Please fill out their [call for contact info](https://docs.google.com/forms/d/e/1FAIpQLSf52YXGhqBaHtCXtVna4iHYMK7IQaTqUW6V-ztsZC8C2TBInQ/viewform) if you are running or used to run a meetup.

* [Mar  8. Washington, DC, US - Mid-month Rustful](https://www.meetup.com/RustDC/events/247478996/).
* [Mar  8. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/czcwhlyxfblb/).
* [Mar  8. San Diego, US - San Diego Rust March Meetup](https://www.meetup.com/San-Diego-Rust/events/248229805/).
* [Mar 11. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbpb/).
* [Mar 12. Cologne, DE - Rust User Group Cologne - Is that a spectrum, or memory access timings](https://www.meetup.com/RustCologne/events/247875532/)?
* [Mar 12. Seattle, US - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/hztzcpyxfbqb/).
* [Mar 13. Rome, IT - Rust Roma - Rust learning and hacking evening #7](https://www.meetup.com/Rust-Roma/events/248449368/).
* [Mar 13. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-content).
* [Mar 13. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 13. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 13. Rust Events Team Meeting](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Mar 14. Tokyo, JP - Rust Meetup in Tokyo](https://www.facebook.com/events/355711738265206).
* [Mar 15. Hamburg, DE - Rust Hack & Learn with a Sprinkle of Web Assembly](https://www.meetup.com/Rust-Meetup-Hamburg/events/248310938/).
* [Mar 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/fmwshpyxfbtb/).
* [Mar 16. Frankfurt, DE - Rust Table of Regulars](https://www.meetup.com/Rust-Rhein-Main/events/248326240).
* [Mar 17. Chennai, IN - Monthly Meetup - March](https://www.meetup.com/mad-rs/events/248475319/).
* [Mar 18. Bangalore, IN - Rust for newbies (Part 5 of 12)](https://www.meetup.com/rustox/events/247982987/).
* [Mar 18. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxfbxb/).
* [Mar 19. London, GB - LDN Talks: March 2018](https://www.meetup.com/Rust-London-User-Group/events/247681377/).
* [Mar 21. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/247387953/).
* [Mar 21. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/ckwdlpyxfbcc/).
* [Mar 21. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 21. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 22. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Research Engineer at Cornell University](https://cornell.wd1.myworkdayjobs.com/CornellCareerPage/job/New-York-City-Cornell-Tech/Research-Engineer_WDR-00013499-1).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
