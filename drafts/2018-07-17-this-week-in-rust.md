Title: This Week in Rust 243
Number: 243
Date: 2018-07-17
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

* [Clippy is now available as a rustup component](https://internals.rust-lang.org/t/clippy-is-available-as-a-rustup-component/7967).
* [Auditing popular crates: how a one-line unsafe has nearly ruined everything](https://www.reddit.com/r/rust/comments/8zpp5f/auditing_popular_crates_how_a_oneline_unsafe_has/).
* [Announcing State Of Rust](https://internals.rust-lang.org/t/announcing-state-of-rust/7937).
* [The tale of a bug in Arc: Synchronization and data races](https://www.ralfj.de/blog/2018/07/13/arc-synchronization.html).
* [Running Rust on a Drone Flight Controller](https://www.joshmcguigan.com/blog/betafpv-drone-flight-controller-hello-rust/).
* [Writing a GPU-accelerated path tracer in Rust - part 2](https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-2/).
* [Programming Servo: a 'script' event-loop](https://medium.com/programming-servo/programming-servo-the-script-event-loop-be687b985b3e).
* [Compatibility with dependencies](https://github.com/teiesti/compdep/blob/master/compdep.pdf).
* [This week in Rust and WebAssembly 4](https://rustwasm.github.io/2018/07/10/this-week-in-rust-wasm-004.html).
* [The Embedded WG newsletter 7](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-7/7959).
* [2018 Edition - end of week post (2018-07-13)](https://internals.rust-lang.org/t/2018-edition-end-of-week-post-2018-07-13/7943).
* [Possible stabilizations for 2018 Edition Preview 2](https://internals.rust-lang.org/t/possible-stabilizations-for-2018-edition-preview-2/7983).

# Crate of the Week

This week's crate is [paste](https://github.com/jkcclemens/paste), a self-hosted pastebin made of a lot of Rust and a little Ruby. Thanks to [Kyle Clemens](https://users.rust-lang.org/u/jkcclemens) for both crate and suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Help port musl's libm to Rust, for math support in WASM/core/no_std code](https://mobile.twitter.com/japaricious/status/1017934106318032901).
* [image-png: Unbounded memory consumption on malformed inputs](https://github.com/PistonDevelopers/image-png/issues/80).
* [Get started with these beginner-friendly issues](https://www.rustaceans.org/findwork/starters).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

172 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-09..2018-07-16

* [add the `amdgpu-kernel` ABI](https://github.com/rust-lang/rust/pull/52032)
* [infinite loop detection for const evaluation](https://github.com/rust-lang/rust/pull/51702)
* [chalk lowering rule: WellFormed-TraitRef](https://github.com/rust-lang/rust/pull/50250)
* [fix ICE when using a pointer cast as array size](https://github.com/rust-lang/rust/pull/52314)
* [implement `#[alloc_error_handler]`](https://github.com/rust-lang/rust/pull/52191)
* [improve error message shown for unsafe operations](https://github.com/rust-lang/rust/pull/52207)
* [remove most of `PartialEq` and `Hash` impls from AST and HIR structures](https://github.com/rust-lang/rust/pull/51829)
* [deny bare trait objects in the rest of rust](https://github.com/rust-lang/rust/pull/52302)
* [reach the body of functions returning `impl Trait` but don't treat it as public](https://github.com/rust-lang/rust/pull/52348)
* [NLL: suggest `ref mut` and `&mut self`](https://github.com/rust-lang/rust/pull/52242)
* [resolve: functions introducing procedural macros reserve a slot in the macro namespace as well](https://github.com/rust-lang/rust/pull/52383)
* [proc_macro: fix crate root detection](https://github.com/rust-lang/rust/pull/52328)
* [correct suggestion for println](https://github.com/rust-lang/rust/pull/51614)
* [do not attempt to recompile codegen backend(s) with --keep-stage](https://github.com/rust-lang/rust/pull/52360)
* [openbsd fix](https://github.com/rust-lang/libc/pull/1040)
* [rustc: stabilize the `proc_macro` feature](https://github.com/rust-lang/rust/pull/52081)
* [ensure StorageDead is created even if variable initialization fails](https://github.com/rust-lang/rust/pull/52046)
* [rustc_codegen_llvm: replace the first argument early in FnType::new_vtable](https://github.com/rust-lang/rust/pull/52089)
* [change RangeInclusive to a three-field struct](https://github.com/rust-lang/rust/pull/51622)
* [add ExactChunks::remainder and ExactChunks::into_remainder](https://github.com/rust-lang/rust/pull/51339)
* [implement `Option::replace` in the core library](https://github.com/rust-lang/rust/pull/52003)
* [add `#[repr(transparent)]` to `Atomic*` types](https://github.com/rust-lang/rust/pull/52149)
* [remove sync::Once::call_once 'static bound](https://github.com/rust-lang/rust/pull/52239)
* [improve Debug display for a few types](https://github.com/rust-lang/cargo/pull/5712)
* [cargo: most sorts can be unstable](https://github.com/rust-lang/cargo/pull/5732)
* [implement default-run option to set default binary for cargo run](https://github.com/rust-lang/cargo/pull/5710)
* [rustdoc: don't panic when the cross-re-export handler sees a proc-macro](https://github.com/rust-lang/rust/pull/52361)

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

* [disposition: merge] [Associated type bounds of form `MyTrait<AssociatedType: Bounds>`](https://github.com/rust-lang/rfcs/pull/2289).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [resolve: Modularize crate-local `#[macro_export] macro_rules`](https://github.com/rust-lang/rust/pull/52234).
* [disposition: merge] [Change behavior of `?` as a macro separator and Kleene op in 2018 edition](https://github.com/rust-lang/rust/issues/51934).
* [disposition: merge] [Deprecation of `str::slice_unchecked(_mut)`](https://github.com/rust-lang/rust/pull/51807).
* [disposition: merge] [Tracking issue for `ToOwned::clone_into` (`toowned_clone_into`)](https://github.com/rust-lang/rust/issues/41263).
* [disposition: close] [NLL lets borrowck observe drop order for `let (a, b);`](https://github.com/rust-lang/rust/issues/51036).
* [disposition: close] [adds Default impl for Instant](https://github.com/rust-lang/rust/pull/50800).

## New RFCs

* [if- and while-let-chains, take 2](https://github.com/rust-lang/rfcs/pull/2497).
* [Pattern API](https://github.com/rust-lang/rfcs/pull/2500).
* [Hygiene opt-out (escaping) for declarative macros 2.0](https://github.com/rust-lang/rfcs/pull/2498).

# Upcoming Events

### Online

* [Jul 25. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Jul 31. Rust Community Content Subteam Meeting at #rust-content on irc.mozilla.org](irc://irc.mozilla.org/rust-content).
* [Aug  1. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Aug  1. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Asia-Pacific

* [Jul 23. Sydney, AU - Rust Sydney Meetup 14](https://www.meetup.com/Rust-Sydney/events/251749825/).

### Europe

* [Jul 20. Barcelona, ES - BcnRust 1st meetup with Ashley Williams & Steve Klabnik](https://www.meetup.com/BcnRust/events/251237895/).
* [Jul 24. Rome, IT - Rust learning and hacking evening #10](https://www.meetup.com/Rust-Roma/events/252627092/).
* [Jul 25. Wrocław, PL - Rust Wroclaw Meetup #3](https://www.meetup.com/Rust-Wroclaw/events/252190812/).
* [Jul 25. Berlin, DE - OpenTechSchool - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/xkdlvpyxkbhc/).
* [Jul 25. Milan, IT - Rust Language Milano - Rust Exercises](https://www.meetup.com/rust-language-milano/events/252893336/).
* [Aug  1. Cologne, DE - Rust Cologne](https://www.meetup.com/RustCologne/events/252432033).

### North America

* [Jul 22. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbdc/).
* [Jun 23. Durham, US - Triangle Rustaceans - Project Night & Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpyxkbfc/).
* [Jul 24. Denver, US - Rust Boulder/Denver - Rust Denver July Meetup](https://www.meetup.com/Rust-Boulder-Denver/events/252275279/).
* [Jul 25. Chicago, US - Rust Meetup July 2018](https://www.meetup.com/Chicago-Rust-Meetup/events/251961097/).
* [Jul 25. New York, NY US - Rust NYC Meetup](https://www.meetup.com/Rust-NYC/events/252181812/)
* [Jul 25. Vancouver, CA - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/dqldspyxkbhc/).
* [Jul 29. Mountain View, US - Open Table / Icebreaker: what projects are you working on](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxkbmc/).
* [Jul 31. Dallas, US - Last Tuesday Meetup](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxkbpc/).
* [Aug  1. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxlbcb/).
* [Aug  1. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmyxlbcb/).
* **[Aug 17. Portland, US - RustConf 2018](http://rustconf.com/).**

### South America

* [Jul 21. São Paulo, BR - Rust at The Developers Conference - TDC2018](http://www.thedevelopersconference.com.br/tdc/2018/saopaulo/trilha-rust).
* [Jul 28. Florianópolis, BR - 2º Encontro Rust Floripa](https://www.meetup.com/rustfloripa/events/xvglrpyxkbkb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Software Engineer at IOHK (Remote work available)](https://iohk.recruiterbox.com/jobs/fk0177c?source=linkedin).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> References are not pointers, but temporary locks on data

– [Kornel](https://users.rust-lang.org/u/kornel) [on rust-users](https://users.rust-lang.org/t/cannot-move-out-of-borrowed-content-take-2/18700/7).

Thanks to [Squirrel](https://users.rust-lang.org/u/gilescope) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
