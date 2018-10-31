Title: This Week in Rust 258
Number: 258
Date: 2018-10-30
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

* 🎈🎉 [Announcing Rust 1.30](https://blog.rust-lang.org/2018/10/25/Rust-1.30.0.html). 🎉🎈
* [Parsing logs 230x faster with Rust](https://andre.arko.net/2018/10/25/parsing-logs-230x-faster-with-rust/).
* [Shifgrethor III: Rooting](https://boats.gitlab.io/blog/post/shifgrethor-iii/).
* [A release checklist for Rust programs](https://dev.to/sharkdp/my-release-checklist-for-rust-programs-1m33).
* [2d graphics in Rust discussion - A look at GPU memory management](https://nical.github.io/posts/rust-2d-graphics-02.html).
* [Announcing Gotham 0.3](https://gotham.rs/blog/release/2018/10/29/gotham-0.3.html).
* [Finding and fixing memory leaks in a Hyper application](https://blog.1aim.com/2018/10/finding-and-fixing-memory-leaks-in-a-hyper-application-or-how-i-learned-to-stop-worrying-and-love-the-allocator/).
* [A brief introduction to Serverless applications in Rust](https://medium.com/@softprops/serverless-rust-318732a3596).
* [The case for macros](https://llogiq.github.io/2018/10/25/macros.html).
* [Jim Blandy interview on mastering moves and borrows](https://corecursive.com/016-moves-and-borrowing-in-rust-with-jim-blandy/).
* [The Embedded WG newsletter 14](https://rust-embedded.github.io/blog/2018-10-28-newsletter-14/).

# Crate of the Week

This week's crate is [dutree](https://github.com/nachoparker/dutree), a command line tool that produces a colorful tree view of your disk usage. Thanks to [gilescope](https://users.rust-lang.org/t/crate-of-the-week/2704/466) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help test Rust 2018](https://blog.rust-lang.org/2018/10/30/help-test-rust-2018.html).
* [Cargo: Resolvers error messages should include the version requirements](https://github.com/rust-lang/cargo/issues/6199).
* [Cargo: Intern more kinds of things](https://github.com/rust-lang/cargo/issues/6207).
* [Rutie: Current dll linking has timeouts on Windows](https://github.com/danielpclark/rutie/issues/48).
* [railroad: Implement a "Auto-Stack" that overflows a Sequence into a Stack](https://github.com/lukaslueg/railroad/issues/5).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

131 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-10-22..2018-10-29

* [compile the libstd we distribute with `-Ccodegen-unit=1`](https://github.com/rust-lang/rust/pull/55264)
* [implement by-value object safety](https://github.com/rust-lang/rust/pull/54183)
* [report const eval error inside the query](https://github.com/rust-lang/rust/pull/53821)
* [path suggestions in Rust 2018 should point out the change in semantics](https://github.com/rust-lang/rust/pull/55185)
* [suggest appropriate syntax on missing lifetime specifier in return type](https://github.com/rust-lang/rust/pull/55173)
* [Macro diagnostics tweaks](https://github.com/rust-lang/rust/pull/55292)
* [list allowed tokens after macro fragments](https://github.com/rust-lang/rust/pull/55301)
* [make unused-parens suggestions heed what the user actually wrote](https://github.com/rust-lang/rust/pull/55138)
* [fix suggestion on renamed import conflict](https://github.com/rust-lang/rust/pull/55113)
* [suggest to remove prefix `b` in cfg attribute lint string](https://github.com/rust-lang/rust/pull/54929)
* [lint reasons](https://github.com/rust-lang/rust/pull/54683) (RFC #[2383](https://rust-lang.github.io/rfcs/2383-lint-reasons.html), part 1)
* [point at macro definition when no rules expect token](https://github.com/rust-lang/rust/pull/55298)
* [fix an ICE in the min_const_fn analysis](https://github.com/rust-lang/rust/pull/55412)
* [avoid unnecessary allocations in `float_lit` and `integer_lit`](https://github.com/rust-lang/rust/pull/55384)
* [add a "cheap" mode for `compute_missing_ctors`](https://github.com/rust-lang/rust/pull/55167)
* [use `SmallVec` for the queue in `coerce_unsized`](https://github.com/rust-lang/rust/pull/55383)
* [shrink `Statement`](https://github.com/rust-lang/rust/pull/55346)
* [introduce type-op for user-type ascription in NLL](https://github.com/rust-lang/rust/pull/55323)
* [NLL: cast causes failure to promote to static](https://github.com/rust-lang/rust/pull/55385)
* [rustc: tweak filenames encoded into metadata](https://github.com/rust-lang/rust/pull/54626)
* [unimplement ExactSizeIterator for MIR traversing iterators](https://github.com/rust-lang/rust/pull/55271)
* [miri engine: stacked Borrows NG](https://github.com/rust-lang/rust/pull/55270)
* [validity: assert that unions are non-empty](https://github.com/rust-lang/rust/pull/55379)
* [allow extern statics with an extern type](https://github.com/rust-lang/rust/pull/55257)
* [add `extern crate` items to extern prelude](https://github.com/rust-lang/rust/pull/54658)
* [rewrite the `UnconditionalRecursion` lint to use MIR](https://github.com/rust-lang/rust/pull/54490)
* [`#[inline]` a bunch of trivial methods of `NonNull`](https://github.com/rust-lang/rust/pull/55426)
* [add `ManuallyDrop::take`](https://github.com/rust-lang/rust/pull/55421)
* [add `MaybeUninit::new`](https://github.com/rust-lang/rust/pull/55244)
* [add line numbers option to rustdoc](https://github.com/rust-lang/rust/pull/54921)
* [fix rustdoc ICE when checking blanket impls](https://github.com/rust-lang/rust/pull/55258)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2451: Re-Rebalancing Coherence](https://github.com/rust-lang/rfcs/pull/2451).
* [RFC 2457: Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Meta-RFC: Future possibilities](https://github.com/rust-lang/rfcs/pull/2561).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Minor standard library constification](https://github.com/rust-lang/rust/pull/55278).

## New RFCs

* [Generic integers](https://github.com/rust-lang/rfcs/pull/2581).
* [Pointer metadata & VTable](https://github.com/rust-lang/rfcs/pull/2580).
* [Second-generation binary operator traits with specialization](https://github.com/rust-lang/rfcs/pull/2578).

# Upcoming Events

### Online

* [Nov  5. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov  7. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Nov 14. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Africa

* [Nov  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxpbjb/).

### Europe

* [Nov  6. Rome, IT - Rust Rome Meetup](https://www.meetup.com/it-IT/Rust-Roma/events/255940927/).
* [Nov  7. Stuttgart, DE - Rust in der Industrie & Automatisierung](https://www.meetup.com/slowtec/events/255390000/).
* [Nov  7. Cologne, DE - Rust Cologne](https://www.meetup.com/RustCologne/events/vnwndpyxpbkb/).
* [Nov 14. Helsinki, FI - Helsinki Rust meetup](https://www.meetup.com/Finland-Rust-Meetup/events/255855675/).
* [Nov 14. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxpbsb/).
* [Nov 15. Cambridge, GB - Cambridge Rust Meetup](https://www.meetup.com/Cambridge-Rust-Meetup/events/pzwshpyxpbtb/).

### North America

* [Nov  4. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbgb/).
* [Nov  6. Santa Monica, US - Rust Los Angeles Meetup](https://www.meetup.com/Rust-Los-Angeles/events/255934998).
* [Nov  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxpbkb/).
* [Nov  7. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxpbkb/).
* [Nov  8. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209655/).
* [Nov  8. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxpblb/).
* [Nov  8. Boston, US - Rust/Scala meetup at SPLASH conf](https://www.meetup.com/BostonRust/events/255445951/).
* [Nov  8. Arlington, US - Rust DC—Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov 11. Mountain View,US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbpb/).
* [Nov 12. Seattle, US - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/pkggvpyxpbqb/).
* [Nov 14. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxpbsb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> &T means it’s borrowed, and T means it’s owned, and you can’t take ownership of a thing you’ve borrowed — Rust doesn’t support stealing! 😉

– kornel [on rust-users](https://users.rust-lang.org/t/vec-t-to-vec-t/21736/2)

Thanks to [Cerberuser](https://users.rust-lang.org/t/twir-quote-of-the-week/328/576) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
