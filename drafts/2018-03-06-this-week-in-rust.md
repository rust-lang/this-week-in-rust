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

* 🎈🎉 [Announcing Rust 1.24.1](https://blog.rust-lang.org/2018/03/01/Rust-1.24.1.html). 🎉🎈
* [Announcing the CLI working group](https://internals.rust-lang.org/t/announcing-the-cli-working-group/6872).
* [Come join the Rust and WebAssembly working group](http://fitzgeraldnick.com/2018/02/27/wasm-domain-working-group.html).
* [Why Rust has macros](https://kasma1990.gitlab.io/2018/03/04/why-rust-has-macros/).
* [Writing a microservice in Rust](http://www.goldsborough.me/rust/web/tutorial/2018/01/20/17-01-11-writing_a_microservice_in_rust/).
* [Futures 0.2 is nearing release](https://aturon.github.io/2018/02/27/futures-0-2-RC/).
* [Writing your first compiler: Making a Brainf*ck to C compiler in Rust](https://medium.com/@CanHasCommunism/making-a-brainf-ck-to-c-compiler-in-rust-10f0c01a282d).
* [Stopping a Rust worker](https://matklad.github.io/2018/03/02/stopping-a-rust-worker.html).
* [Serializing awkward data with Serde](http://zork.net/~st/jottings/Serializing_awkward_data_with_serde.html).
* [An introduction to writing platform agnostic drivers in Rust using the MCP3008 as an example](http://pramode.in/2018/02/24/an-introduction-to-writing-embedded-hal-based-drivers-in-rust/).
* [Opportunistic mutations for the mutagen - a Rust mutation testing framework](https://llogiq.github.io/2018/03/03/opportune.html).
* [This week in Rust docs 95](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-95).
* [podcast] [New Rustacean: Rust 1.24](http://www.newrustacean.com/show_notes/news/rust_1_24/). Performance wins, incremental compilation, and the Rust 2018 Roadmap and Epoch.
* [podcast] [Rusty Spike Podcast - episode 21](https://rusty-spike.blubrry.net/2018/03/01/episode-21-feb-28-2018/). SIMD, WebAssembly for performance, the embedded working group, the Rust+WebAssembly working group, and the return of the Servo newsletter.

# Crate of the Week

This week's crate is [fselect](https://github.com/jhspetersson/fselect), a crate to find files by SQL-like queries. Thanks to [Jhspetersson](https://users.rust-lang.org/u/jhspetersson) for the suggestion.

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

127 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-02-19..2018-02-26

* [fix exponential projection complexity on nested types](https://github.com/rust-lang/rust/pull/48296)
* [avoid ICE in arg mistmatch error for tuple variants](https://github.com/rust-lang/rust/pull/48246)
* [fix parsing of extern paths in types and poly-traits](https://github.com/rust-lang/rust/pull/48441)
* [make ".e0" not parse as 0.0](https://github.com/rust-lang/rust/pull/48235)
* [use sparse bitsets instead of dense ones for NLL results](https://github.com/rust-lang/rust/pull/48245)
* [reset default binding mode when we pass through a `&` pattern](https://github.com/rust-lang/rust/pull/48448)
* [add nonstandard_style alias for bad_style](https://github.com/rust-lang/rust/pull/48386)
* [fix span of visibility](https://github.com/rust-lang/rust/pull/47799)
* [overhaul improper_ctypes output](https://github.com/rust-lang/rust/pull/48221)
* [inform user where to give a type annotation](https://github.com/rust-lang/rust/pull/48198)
* [handle custom diagnostic for `&str + String`](https://github.com/rust-lang/rust/pull/48392)
* [fix nested impl trait lifetimes](https://github.com/rust-lang/rust/pull/48072)
* [error on nested impl Trait and path projections from impl Trait](https://github.com/rust-lang/rust/pull/48084)
* [remove "static item recursion checking" in favor of relying on cycle checks in the query engine](https://github.com/rust-lang/rust/pull/47987)
* [improve tuple struct field access hygiene](https://github.com/rust-lang/rust/pull/48083)
* [macros: improve struct constructor field hygiene, fix span bug](https://github.com/rust-lang/rust/pull/48082)
* [do not run the default panic hook inside procedural macros](https://github.com/rust-lang/rust/pull/47933)
* [introduce UnpackedKind](https://github.com/rust-lang/rust/pull/48452)
* [do not run MIR type checker twice](https://github.com/rust-lang/rust/pull/48061)
* [MIR: gather move at SwitchInt, Assert terminators](https://github.com/rust-lang/rust/pull/48232)
* [allow two-phase borrows of &mut self in ops](https://github.com/rust-lang/rust/pull/48197)
* [incr.comp.: Don't keep RefCells in on-disk-cache borrowed in order to allow for recursive invocations](https://github.com/rust-lang/rust/pull/48185)
* [rustc_trans: rewrite mips64 ABI code](https://github.com/rust-lang/rust/pull/47964)
* [termination trait in tests](https://github.com/rust-lang/rust/pull/48143)
* [detect wrong number of args when type-checking a closure](https://github.com/rust-lang/rust/pull/48123)
* [add Iterator::try_for_each](https://github.com/rust-lang/rust/pull/48157)
* [add Iterator::flatten](https://github.com/rust-lang/rust/pull/48115)
* [add Condvar APIs not susceptible to spurious wake](https://github.com/rust-lang/rust/pull/47970)
* [stabilize 'entry_and_modify' feature](https://github.com/rust-lang/rust/pull/48166)
* [stabilize Box::leak](https://github.com/rust-lang/rust/pull/48110)
* [Fix borrow checker unsoundness with unions](https://github.com/rust-lang/rust/pull/47689)
* [Derive std::cmp::Reverse as Copy or Clone](https://github.com/rust-lang/rust/pull/47379)
* [When encountering invalid token after `unsafe`, mention `{`](https://github.com/rust-lang/rust/pull/48356)
* [Implement implied shortcut links for intra-rustdoc-links](https://github.com/rust-lang/rust/pull/48335)
* [rename rdrnd target feature to rdrand](https://github.com/rust-lang/rust/pull/48369)
* [book: Second edition is now the definitive edition](https://github.com/rust-lang/book/pull/1180)
* [rustc explain](https://github.com/rust-lang/rust/pull/48337)
* [rustdoc: generate documentation for auto-trait impls](https://github.com/rust-lang/rust/pull/47833)
* [rustdoc: don't crash when an external trait's docs needs to import another trait](https://github.com/rust-lang/rust/pull/48415)
* [fix rustdoc test ICE](https://github.com/rust-lang/rust/pull/48382)
* [make cargo-the-binary version the same as the Rust version](https://github.com/rust-lang/cargo/pull/5083)
* [cargo: warn Windows 7 users about old TLS](https://github.com/rust-lang/cargo/pull/5069)
* [cargo: display path to custom commands with `--list -v`](https://github.com/rust-lang/cargo/pull/5041)

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
