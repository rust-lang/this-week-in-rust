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
* [This week in Rust and WebAssembly 4](https://rustwasm.github.io/2018/07/10/this-week-in-rust-wasm-004.html).
* [The Embedded WG newsletter 7](https://internals.rust-lang.org/t/the-embedded-working-group-newsletter-7/7959).
* [2018 Edition - end of week post (2018-07-13)](https://internals.rust-lang.org/t/2018-edition-end-of-week-post-2018-07-13/7943).
* [Possible stabilizations for 2018 Edition Preview 2](https://internals.rust-lang.org/t/possible-stabilizations-for-2018-edition-preview-2/7983).

# Crate of the Week

This week's crate is [cargo-geiger](https://github.com/anderejd/cargo-geiger), which detects usage of unsafe Rust in your project and its dependencies.

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

158 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-07-02..2018-07-09

* [Stable release 1.27.1](https://github.com/rust-lang/rust/pull/52134).
* [Add `#[repr(transparent)]` to some libcore types](https://github.com/rust-lang/rust/pull/51395).
* [Stabilize rc_downcast](https://github.com/rust-lang/rust/pull/52103).
* [Add lint warning for inner function marked as `#[test]`](https://github.com/rust-lang/rust/pull/51450).
* [rust: add initial changes to support powerpc64le musl](https://github.com/rust-lang/rust/pull/51619).
* [Initialize LLVM's AMDGPU target machine, if available](https://github.com/rust-lang/rust/pull/51548).
* [Implement always-fallible TryFrom for usize/isize conversions that are infallible on some platforms](https://github.com/rust-lang/rust/pull/51564).
* [Haiku: several smaller fixes to build and run rust on Haiku](https://github.com/rust-lang/rust/pull/51757).
* [Add `read_exact_at` and `write_all_at` methods to `FileExt` on unix](https://github.com/rust-lang/rust/pull/51809).
* [Add the `alloc::prelude` module](https://github.com/rust-lang/rust/pull/52159).
* [Ship clippy in manifests](https://github.com/rust-lang/rust/pull/52131).
* [Don't suggest `let` bindings if they don't help with borrows](https://github.com/rust-lang/rust/pull/52106).
* [Get rid of `TyImplTraitExistential`](https://github.com/rust-lang/rust/pull/51979).
* [rename rustc's lld to rust-lld](https://github.com/rust-lang/rust/pull/51936).
* [Add outlives annotations to `BTreeMap`](https://github.com/rust-lang/rust/pull/51914).
* [Performance improvement of Vec's swap_remove](https://github.com/rust-lang/rust/pull/52166).
* [Add a punch card to weird expressions test](https://github.com/rust-lang/rust/pull/52073).
* [Add linux musl powerpc (32-bit) support](https://github.com/rust-lang/libc/pull/1031).
* [Implementation of tool lints](https://github.com/rust-lang/rust/pull/52018).
* [Enable Atomic*.{load,store} for ARMv6-M / MSP430](https://github.com/rust-lang/rust/pull/51953).
* [Make causal tracking lazy](https://github.com/rust-lang/rust/pull/51889).
* [Move self trait predicate to items](https://github.com/rust-lang/rust/pull/51895).
* [Mostly fix metadata_only backend and extract some code out of rustc_codegen_llvm](https://github.com/rust-lang/rust/pull/51590).
* [Deprecate `std::env::home_dir` and fix incorrect documentation](https://github.com/rust-lang/rust/pull/51656).

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

> actix-web has removed all unsound use of unsafe in its codebase. It’s down to less than 15 occurences of unsafe from 100+.

– [u/_ar7 celebrating this commendable achievement](https://www.reddit.com/r/rust/comments/8wlkbe/actixweb_has_removed_all_unsound_use_of_unsafe_in/).

Thanks to [Jules Kerssemakers](https://users.rust-lang.org/t/twir-quote-of-the-week/328/542) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
