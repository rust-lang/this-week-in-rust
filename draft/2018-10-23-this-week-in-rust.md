Title: This Week in Rust 257
Number: 257
Date: 2018-10-23
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

* [Writing an OS in Rust: Hardware interrupts](https://os.phil-opp.com/hardware-interrupts/).
* [Towards fearless SIMD](https://raphlinus.github.io/rust/simd/2018/10/19/fearless-simd.html).
* [Shifgrethor I: Garbage collection as a Rust library](https://boats.gitlab.io/blog/post/shifgrethor-i/).
* [Update on the October 15, 2018 incident on crates.io](https://blog.rust-lang.org/2018/10/19/Update-on-crates.io-incident.html).
* [docs.rs is now part of the rust-lang-nursery organization](https://github.com/rust-lang-nursery/docs.rs).
* [Is Rust functional](https://www.fpcomplete.com/blog/2018/10/is-rust-functional)?
* [Multithreading Rust and WebAssembly](https://rustwasm.github.io/2018/10/24/multithreading-rust-and-wasm.html).
* [Rust has higher kinded types already... sort of](https://joshlf.com/post/2018/10/18/rust-higher-kinded-types-already/).
* [Auth Web Microservice with rust using Actix-Web](https://hgill.io/posts/auth-microservice-rust-actix-web-diesel-complete-tutorial-part-1/).

# Crate of the Week

This week's crate is [Noria](https://crates.io/crates/noria),  a new streaming data-flow system designed to act as a fast storage backend for read-heavy web applications. Thanks to [Stevensonmt](https://users.rust-lang.org/t/crate-of-the-week/2704/464) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [toml: External enum serialization](https://github.com/alexcrichton/toml-rs/pull/267).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

124 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2018-10-08..2018-10-15

* [rustc: allow targets to specify SIMD args are by-val](https://github.com/rust-lang/rust/pull/55024)
* [stabilize tool lints](https://github.com/rust-lang/rust/pull/54870)
* [user annotations in patterns](https://github.com/rust-lang/rust/pull/54757)
* [`impl Eq+Hash for TyLayout`](https://github.com/rust-lang/rust/pull/54936)
* [prepare miri engine for enforcing validity invariant during execution](https://github.com/rust-lang/rust/pull/54762)
* [miri engine: fix run-time validation](https://github.com/rust-lang/rust/pull/54955)
* [fix handling of `#[must_use]` on unit and uninhabited types](https://github.com/rust-lang/rust/pull/54920)
* [`#[must_use]` for associated functions is supposed to actually work](https://github.com/rust-lang/rust/pull/55003)
* [the `#[panic_handler]` attribute can be applied to non-functions](https://github.com/rust-lang/rust/pull/54997)
* [NLL is missing struct field suggestion](https://github.com/rust-lang/rust/pull/54831)
* [add chalk rules related to associated type defs](https://github.com/rust-lang/rust/pull/54909)
* [better Diagnostic for Trait Object Capture](https://github.com/rust-lang/rust/pull/54848)
* [structured suggestions for unused-lifetimes lint](https://github.com/rust-lang/rust/pull/54686)
* [mir-inlining: don't inline virtual calls](https://github.com/rust-lang/rust/pull/55046)
* [use `MaybeUninit` in liballoc](https://github.com/rust-lang/rust/pull/54924)
* [stabilize the `Option::replace` method](https://github.com/rust-lang/rust/pull/54904)
* [std: implement Thread-local storage for wasm32-unknown-unknown](https://github.com/rust-lang/rust/pull/54951)
* [std: synchronize global allocator on wasm32](https://github.com/rust-lang/rust/pull/54950)
* [rustdoc: fix mobile docs](https://github.com/rust-lang/rust/pull/54869)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2476: Clippy 1.0](https://github.com/rust-lang/rfcs/pull/2476).
* [RFC 2514: Union initialization and Drop](https://github.com/rust-lang/rfcs/pull/2514).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Re-Rebalancing Coherence](https://github.com/rust-lang/rfcs/pull/2451).
* [disposition: merge] [Allow non-ASCII identifiers](https://github.com/rust-lang/rfcs/pull/2457).
* [disposition: merge] [Meta-RFC: Future possibilities](https://github.com/rust-lang/rfcs/pull/2561).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Report const eval error inside the query](https://github.com/rust-lang/rust/pull/53821).
* [disposition: merge] [Unchecked thread spawning](https://github.com/rust-lang/rust/pull/55043).
* [disposition: merge] [Implement FromStr for PathBuf](https://github.com/rust-lang/rust/pull/55148).
* [disposition: close] [Regression from stable: pointer to usize conversion no longer compiles](https://github.com/rust-lang/rust/issues/54709).

## New RFCs

* [SIMD vectors in FFI](https://github.com/rust-lang/rfcs/pull/2574).
* [flat_map as an alias for and_then](https://github.com/rust-lang/rfcs/pull/2572).
* [Linked list cursors](https://github.com/rust-lang/rfcs/pull/2570).

# Upcoming Events

### Online

* [Oct 31. Rust Community Team Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov  5. Rust Community Content Subteam Meeting in Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Nov  7. Rust Events Team Meeting in Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Africa

* [Oct 27. Nairobi, KE - HACK & LEARN: Hacktoberfest Edition](https://www.meetup.com/Rust-Nairobi/events/255546089).
* [Nov  6. Johannesburg, SA - Monthly Meetup of the Johannesburg Rustaceans](https://www.meetup.com/Johannesburg-Rust-Meetup/events/cpblrnyxpbjb/).

### Europe

* [Oct 27. St. Petersburg, RU - Неформальная встреча Rust-разработчиков](https://www.meetup.com/Rust-%D0%B2-%D0%9F%D0%B8%D1%82%D0%B5%D1%80%D0%B5/events/nhpkmpyxnbkc).
* [Oct 30. Paris, FR - Rust Paris meetup #43](https://www.meetup.com/Rust-Paris/events/255604978).
* [Oct 31. Prague, CZ - Prague Containers Meetup - The way of Rust](https://www.meetup.com/Prague-Containers-Meetup/events/251325363/).
* [Oct 31. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyxnbpc/).
* [Oct 31. Milan, IT - Rust Language Milano - Rust Exercises](https://www.meetup.com/rust-language-milano/events/255737296/).
* [Nov  7. Stuttgart, DE - Rust in der Industrie & Automatisierung](https://www.meetup.com/slowtec/events/255390000/).
* [Nov  7. Cologne, DE - Rust Cologne](https://www.meetup.com/RustCologne/events/vnwndpyxpbkb/).

### North America

* [Oct 28. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxnblc/).
* [Oct 30. Dallas, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/zfgwzmyxnbnc/).
* [Oct 31. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/xttphqyxnbpc/).
* [Nov  4. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyxpbgb/).
* [Nov  7. Atlanta, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/cbcmbqyxpbkb/).
* [Nov  7. Indianapolis, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyxpbkb/).
* [Nov  8. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209655/).
* [Nov  8. Arlington, US - Rust DC - Mid-month Rustful](https://www.meetup.com/RustDC/events/254871472).
* [Nov  8. Columbus, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyxpblb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [Rust Developer at 1aim, Berlin](https://1aim.com/careers).
* [Rust Engineer at Commure, Inc. (San Francisco, Boston, Montreal)](https://www.reddit.com/r/rust/comments/92e67g/commure_healthcare_software_startup_hiring_rust/).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> There actually are NOT very many places where the C code’s behavior conflicts with Rust’s borrowing rules. This is both somewhat surprising, because there’s no way this code was written with Rust’s borrowing semantics in mind, and also entirely sensible, since Rust’s borrowing semantics are often quite close to how you actually want your code to behave anyway.

– SimonHeath [porting C to Rust](https://wiki.alopex.li/PortingCToRust)

Thanks to [Pascal Hertleif](https://users.rust-lang.org/t/twir-quote-of-the-week/328/565) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
