Title: This Week in Rust 284
Number: 284
Date: 2019-04-30
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

This week's crate is [cast-rs](https://github.com/japaric/cast), a crate with ergonomic, checked cast functions for primitive types. Thanks to [mark-i-m](https://users.rust-lang.org/t/crate-of-the-week/2704/525) for the suggestion!

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

229 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-04-22..2019-04-29

* [Introduce `hir::ExprKind::Use` and employ in for loop desugaring](https://github.com/rust-lang/rust/pull/60225)
* [Future-proof MIR for dedicated debuginfo](https://github.com/rust-lang/rust/pull/56278)
* [Add `f16c` target_feature](https://github.com/rust-lang/rust/pull/60191)
* [Fix `sync_all` on macos/ios](https://github.com/rust-lang/rust/pull/60121)
* [Implement `saturating_abs()` and `saturating_neg()` functions for signed integer types](https://github.com/rust-lang/rust/pull/60192)
* [Replace HashMap implementation with SwissTable (as an external crate)](https://github.com/rust-lang/rust/pull/58623)
* [Stabilize `Iterator::copied`](https://github.com/rust-lang/rust/pull/60333)
* [Stabilize `pointer::align_offset`](https://github.com/rust-lang/rust/pull/60303)
* [Const-stabilize `std::mem::needs_drop`](https://github.com/rust-lang/rust/pull/60364)
* [cargo: Support relative paths for registries](https://github.com/rust-lang/cargo/pull/6873)
* [Set `cfg(test)` when rustdoc is running with `--test` option](https://github.com/rust-lang/rust/pull/59940)

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

> Clippy’s Favorite Activity Is Criticizing Clippy’s Codebase

[ReductRs on twitter](https://mobile.twitter.com/reduct_rs/status/1121439213772333058)!

Llogiq is pretty self-congratulatory for picking this awesome quote.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
