Title: This Week in Rust 280
Number: 280
Date: 2019-04-02
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

This week's crate is [sonic](https://github.com/valeriansaliou/sonic), a fast, lightweight & schema-less search backend. Thanks to [Vikrant](https://users.rust-lang.org/t/crate-of-the-week/2704/509) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Call for proposal - RustLab 2019, June 28-29, Florence, IT](https://www.rustlab.it/page/1398227/call-for-proposal).
* [Tools Team: tell us your sorrows](https://internals.rust-lang.org/t/tools-team-tell-us-your-sorrows/9657).
* [grpc-rs: Add support for well-known types](https://github.com/pingcap/grpc-rs/issues/276).
* [TiKV: Flush logs on fatal errors](https://github.com/tikv/tikv/issues/4328).
* [TiKV: Disable test suites that contain no tests](https://github.com/tikv/tikv/issues/4391).
* [GNOME Google Summer of Code](https://twitter.com/sdroege_/status/1109135842793148419): couple of Rust-related ideas lined up for this year's GSOC. From low-level libraries like Gstreamer, and Librsvg, to system services, to desktop applications. More details can be found by searching [this wiki page](https://wiki.gnome.org/Outreach/SummerOfCode/2019/Ideas).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

251 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-03-25..2019-04-01

* [Add intial support for `wasm32-unknown-wasi`](https://github.com/rust-lang/libc/pull/1307)
* [Allow closure to unsafe fn coercion](https://github.com/rust-lang/rust/pull/59580)
* [Include bounds in generic re-ordering diagnostic](https://github.com/rust-lang/rust/pull/59572)
* [Optimize indentation in the pretty printer](https://github.com/rust-lang/rust/pull/59507)
* [Use `SmallVec` in `TokenStreamBuilder`](https://github.com/rust-lang/rust/pull/59476)
* [Make ASCII case conversions more than 4× faster](https://github.com/rust-lang/rust/pull/59283)
* [Adjust `MaybeUninit` API to discussions](https://github.com/rust-lang/rust/pull/59284)
* [Introduce `proc_macro::Span::source_text`](https://github.com/rust-lang/rust/pull/55780)
* [Add `Default` to `std::alloc::System`](https://github.com/rust-lang/rust/pull/59451)
* [Add `FromStr` impl for `NonZero` types](https://github.com/rust-lang/rust/pull/58717)
* [Implement `AsRawFd` for stdio locks](https://github.com/rust-lang/rust/pull/59512)
* [Simplify `checked_duration_since`](https://github.com/rust-lang/rust/pull/59374)
* [Stabilize `refcell_replace_swap`](https://github.com/rust-lang/rust/pull/59581)
* [stabilize `ptr::hash`](https://github.com/rust-lang/rust/pull/59603)
* [Stabilize {`f32`, `f64`}`::copysign()`](https://github.com/rust-lang/rust/pull/59503)
* [cargo metadata: Don't show `null` deps](https://github.com/rust-lang/cargo/pull/6534)
* [cargo install: Be more restrictive about cli flags](https://github.com/rust-lang/cargo/pull/6801)
* [rustdoc: Fix infinite recursion](https://github.com/rust-lang/rust/pull/59539)
* [rustdoc: collapse blanket impls in the same way as normal impls](https://github.com/rust-lang/rust/pull/59534)

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

* [disposition: merge] [Roadmap for 2019](https://github.com/rust-lang/rfcs/pull/2657).
* [disposition: merge] [Transparent Unions and Enums](https://github.com/rust-lang/rfcs/pull/2645).
* [disposition: close] [`mut (x, y, ..)` and `mut [x, y, ..]` pattern shorthand](https://github.com/rust-lang/rfcs/pull/2401).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [More restrictive 2 phase borrows - take 2](https://github.com/rust-lang/rust/pull/58739).
* [disposition: merge] [Introduce RefCell::try_borrow_unguarded](https://github.com/rust-lang/rust/pull/59211).
* [disposition: merge] [Tracking issue for {f32, f64}::copysign](https://github.com/rust-lang/rust/issues/58046).

## New RFCs

* [RFC for a match based surface syntax to get pointer-to-field](https://github.com/rust-lang/rfcs/pull/2666).

# Upcoming Events

### Africa

* [Mar 30. Nairobi, KE - Rust Nairobi Meetup: Projects Showcase](https://www.meetup.com/Rust-Nairobi/events/259650701/).
* [Apr  3. Johannesburg, ZA - Johannesburg meetup - using Rust in production](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzgbfb/).

### Asia Pacific

* [Mar 29. Noida, IN - Fearless Concurrency in Rust - Knoldus Meetup](https://www.meetup.com/Reactive-Application-Programmers-in-Delhi-NCR/events/259722745/).
* [Mar 30. Taipei, TW - Rust Taiwan Meetup](https://www.facebook.com/events/431401857668601/).
* [Apr 20. Beijing, CN - RustCon Asia](https://rustcon.asia/).

### Europe

* [Mar 28. Copenhagen, DK - Copenhagen Rust Hack Night #14](https://cph.rs/).
* [Mar 28. Toulouse, FR - Rust Toulouse meetup](https://www.meetup.com/fr-FR/Toulouse-Rust-Meetup/events/259589986/).
* [Mar 31. St. Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/whmxrqyzfbpc).
* [Apr  3. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzgbfb/).
* [Apr  4. Zagreb, HR - Rust Meetup 201904: Persistent data in Rust](https://www.meetup.com/Zagreb-Rust-Meetup/events/259597646/).
* [Apr 04. Wroclaw, PL - Rust Wroclaw Meetup](https://www.meetup.com/Rust-Wroclaw/events/259511136/).
* [Apr 13. Kyiv, UA - PeerLab Kyiv #NativeDev: Rust 1.34 Release in Depth](https://www.meetup.com/PeerLab-Native-Developers/events/260050471/).
* [Jun 28-29. Firenze, IT - RustLab](https://www.rustlab.it/).

### North America

* [Apr  3. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/246726699/).
* [Apr  3. Atlanta, US - Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/lgtvsqyzgbfb/).
* [Apr  3. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/gqbksqyzgbfb/).
* [Apr  9. Seattle, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzgbmb/).
* [Apr 11. Arlington, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/259782531).
* [Apr 11. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzgbpb/).

### South America

* [Mar 29. Montevideo, UY - Rust Latam @ Montevideo, Uruguay](https://rustlatam.org/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [WebAssembly / Cranelift Intern (Summer 2019) at Mozilla, San Francisco, US](https://careers.mozilla.org/position/gh/1501382/).
* [Senior Software Engineer at Metaswitch, North London, UK](https://www.metaswitch.com/careers-blog/senior-software-engineer-enfield).
* [Lead Engineer (Cryptography) at Cosmian, Paris, FR](https://cosmian.com/wp-content/uploads/2019/03/Rust-CPP-lead-engineer.pdf).
* [Database Engineer Developer at Parity, Berlin, DE](https://www.parity.io/jobs/#berlin-database-engine-developer).
* [Rust Software Consultant at Knoldus, Noida, IN](https://www.knoldus.com/careers/rust-software-consultant.knol).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Thanks for walking through the process.
>
> Quite the mental exercise, some people do Sudoku, others solve borrow puzzles!

– [Gambhiro on rust-users](https://users.rust-lang.org/t/solved-channel-in-a-loop-in-a-thread-borrowed-value-does-not-live-long-enough/26733/9)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/633) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
