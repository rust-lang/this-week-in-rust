Title: This Week in Rust 283
Number: 283
Date: 2019-04-23
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

* [Rust's 2019 roadmap](https://blog.rust-lang.org/2019/04/23/roadmap.html).
* [Amethyst receives Mozilla grant for WASM development](https://www.amethyst.rs/blog/moss-grant-announce/).
* [6 useful Rust macros that you might not have seen before](https://medium.com/@benmcdonald_11671/6-useful-rust-macros-that-you-might-not-have-seen-before-59d1386f7bc5).
* [Learning parser combinators with Rust](https://bodil.lol/parser-combinators/).
* [What not to do in Rust](https://blog.sentry.io/2018/04/05/you-cant-rust-that).
* [Generic returns in Rust](https://blog.jcoglan.com/2019/04/22/generic-returns-in-rust/).
* [Futures 0.1 compatibility layer](https://rust-lang-nursery.github.io/futures-rs/blog/2019/04/18/compatibility-layer.html).
* [What I learned from my failed attempt of writing baremetal android in Rust](https://onatm.dev/2019/04/22/what-i-learned-from-my-failed-attempt-of-writing-baremetal-android-in-rust/).
* [Building a plugin system using Rust and Wasmer](https://wiredforge.com/blog/wasmer-plugin-pt-1) ([part 2](https://wiredforge.com/blog/wasmer-plugin-pt-2), [part 3](https://wiredforge.com/blog/wasmer-plugin-pt-3/index.html)).
* [Using DMA to transfer data with embedded Rust](https://flowdsp.io/blog/stm32f3-02-dac-dma/).
* [AiC: Adventures in consensus](http://smallcultfollowing.com/babysteps/blog/2019/04/19/aic-adventures-in-consensus/).
* [rustup 1.18.0 released](https://github.com/rust-lang/rustup.rs/blob/master/CHANGELOG.md#1180---2019-04-22).
* [RustFest 2019 - next stop: Barcelona](https://blog.rustfest.eu/next-stop-barcelona).
* [Videos from Rust Latam 2019](https://www.youtube.com/playlist?list=PL85XCvVPmGQjuWUNeFCgl8X2EOC_aAq5N).

# Crate of the Week

This week's crate is [color-backtrace](https://github.com/athre0z/color-backtrace), a crate to give panic backtraces more information (and some color, too). Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/518) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* The [CFP for "Everything in Rust" in COSCUP](https://blog.coscup.org/2019/04/2019-cfp-open.html#rust) is open now. Rust Taiwan Community is looking for speakers. COSCUP is one of the biggest open source conferences in Asia and takes place in Taipei, Taiwan.
* The [CFP for Colorado Gold Rust](https://cfp.cogoldrust.com/events/cogoldrust-2019) is open now. The organizers are also looking for volunteers to help people draft talk proposals. If you can help out [send them an email](mailto:coloradogoldrust@gmail.com) or DM them on Twitter at [@COGoldRust](https://twitter.com/cogoldrust).
* [Evolution Island: Amethyst showcase game looking for collaborators](https://www.reddit.com/r/rust/comments/bf65l3/evolution_island_amethyst_showcase_game_looking/).
* [good first issue] [futures-jsonrpc: Handler track request/reply](https://github.com/vlopes11/futures-jsonrpc/issues/2).
* [Out-of-band crate evaluation for 2019-04-19: uuid](https://internals.rust-lang.org/t/out-of-band-crate-evaluation-for-2019-04-19-uuid/9848).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

221 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-15..2019-04-22

* [Implement event filtering for self-profiler](https://github.com/rust-lang/rust/pull/59915)
* [Continue evaluating after missing main](https://github.com/rust-lang/rust/pull/59903)
* [Point at try `?` on errors affecting the err match arm of the desugared code](https://github.com/rust-lang/rust/pull/60064)
* [Make const parameters enforce no variance constraints](https://github.com/rust-lang/rust/pull/60058)
* [save-analysis: Use serde instead of libserialize to dump JSON data](https://github.com/rust-lang/rust/pull/60053)
* [Fix ICE on const evaluation of const method](https://github.com/rust-lang/rust/pull/60048)
* [Specialize `nth_back()` for `Bytes`, `Fuse` and `Enumerate`](https://github.com/rust-lang/rust/pull/60023)
* [Fix the max value of `usize` on 16-bit platforms](https://github.com/rust-lang/rust/pull/60013)
* [Fix `LinkedList` invalidating mutable references](https://github.com/rust-lang/rust/pull/60072)
* [Allow multiple args to `dbg!(..)`](https://github.com/rust-lang/rust/pull/59826)
* [Add `must_use` annotations to `Result::is_ok` and `is_err`](https://github.com/rust-lang/rust/pull/59648)
* [chalk: Remove coinductive_traits from `ProgramEnvironment`](https://github.com/rust-lang/chalk/pull/213)
* [chalk: Simplify crate structure](https://github.com/rust-lang/chalk/pull/215)
* [cargo: Treat HTTP/2 stream errors as spurious network errors](https://github.com/rust-lang/cargo/pull/6861)
* [cargo: Validate registry token before operations that require it](https://github.com/rust-lang/cargo/pull/6854)
* [cargo: Pass `OsStr`/`OsString` args through to the process spawned by cargo run](https://github.com/rust-lang/cargo/pull/6849)
* [rustdoc: Remove `default` keyword from re-exported trait methods](https://github.com/rust-lang/rust/pull/59978)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Attributes in formal function parameter position](https://github.com/rust-lang/rfcs/pull/2565).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [[Stabilization] Future APIs](https://github.com/rust-lang/rust/issues/59725).
* [disposition: merge] [Tracking issue for vectored IO support](https://github.com/rust-lang/rust/issues/58452).
* [disposition: merge] [Tracking issue for Iterator::copied](https://github.com/rust-lang/rust/issues/57127).
* [disposition: merge] [Implement `iter::Sum` and `iter::Product` for `Option`](https://github.com/rust-lang/rust/pull/58975).

## New RFCs

* [Add `f16b` floating-point type for native support of `bfloat16`](https://github.com/rust-lang/rfcs/pull/2690).
* [Allow floating-point operations to provide extra precision than specified, as an optimization](https://github.com/rust-lang/rfcs/pull/2686).
* [Introduce "compiler-team contributors"](https://github.com/rust-lang/rfcs/pull/2689).

# Upcoming Events

### Africa

* [May  2. Johannesburg, ZA - Johannesburg meetup - Everybody Borrows](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzhbcb/).

### Asia Pacific

* [Apr 20. Beijing, CN - RustCon Asia](https://rustcon.asia/).
* [Apr 20. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/400895290642737/).
* [Apr 24. Tokyo, JP - Tokyo Rust Meetup](https://rust.connpass.com/event/125666/).
* [May  6. Auckland, NZ - Rust AKL - Rust Debugging Techniques + Lightening Talks](https://www.meetup.com/rust-akl/events/259480601/).
* [May  8. Kuala Lumpur, MY - Rust Meetup May 2019](https://docs.google.com/forms/d/e/1FAIpQLScUHpCLPMF8I1QxA_WnIz9bipalrNsUckSyLMysGGNB5y0Lyw/viewform).

### Europe

* [Apr 25. Paris, FR - Rust Paris meetup #44](https://www.meetup.com/Rust-Paris/events/260443108/).
* [Apr 25. Brno, CZ - Rust Brno meetup](https://rust-brno.github.io/).
* [Apr 26. Stuttgart, DE - Rust Meetup #2](https://gettogether.community/rust-stuttgart/)
* [Apr 26. Berlin, DE - Oxidize Berlin Conference](https://oxidizeconf.com/).
* [Apr 30. London, UK - Rust London User Group - LDN Talks](https://www.meetup.com/Rust-London-User-Group/events/260565918/).
* [Apr 30. Vienna, AT - Rust Meetup](https://www.meetup.com/Rust-Vienna/events/260693863/).
* [May  1. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzhbcb/).
* [May  2. Munich, DE - Rust Munich - Rust libp2p](https://www.meetup.com/rust-munich/events/259984522/).
* [May  6. Budapest, HU - Rust Hungary Meetup](https://www.meetup.com/Rust-Hungary-Meetup/events/260651034/).
* [May  9. Wrocław, PL - Rust Wroclaw Meetup #10](https://www.meetup.com/Rust-Wroclaw/events/260858425/).
* [May  9. Berlin, DE - Rust+GNOME 2019 Hackfest#5](https://wiki.gnome.org/Hackfests/Rust2019).

### North America

* [Apr 25. San Francisco, US - WebAssembly SF - Let's talk about Rust and a microkernel @ Cloudflare](https://www.meetup.com/wasmsf/events/260288977/).
* [Apr 30. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyzgbnc/).
* [May  1. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/hjrwvqyzhbcb/).
* [May  1. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/lgtvsqyzhbcb/).
* [May  9. San Diego, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/260763786/).
* [May  9. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/260559957).
* [May  9. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzhbmb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://twitter.com/ParityTech/status/1120303295606788097).
* [Senior Firware Engineer (Rust/C) at Helium, San Francisco, US](https://angel.co/helium-2/jobs/541447-senior-firware-engineer-rust-c).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/bgvxxn/this_week_in_rust_283/).</small>
