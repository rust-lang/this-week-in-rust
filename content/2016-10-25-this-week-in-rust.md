Title: This Week in Rust 153
Number: 153
Date: 2016-10-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## Blog Posts

* [Fixing Python performance with Rust](https://blog.sentry.io/2016/10/19/fixing-python-performance-with-rust.html).
* [Hello Teensy](https://github.com/SimonSapin/teensy-clock). Building a wall-clock with Rust and Teensy microcontroller.
* [Saved by the compiler: Parallelizing a loop with Rust and rayon](http://blog.faraday.io/saved-by-the-compiler-parallelizing-a-loop-with-rust-and-rayon/). Eric Kidd shares his story of how Rust compiler just saved him from a nasty threading bug.
* [Semi-hosting on ARM with Rust](http://embed.rs/articles/2016/semi-hosting-rust/).
* [Rewriting tinyhttpd in Rust, Part One](http://silverwingedseraph.net/rewriting-tinyhttpd-in-rust-part-one). Tiny HTTPd is a web-server written in C.
* [Rust tips and tricks](https://thesquareplanet.com/blog/rust-tips-and-tricks/).
* [Notes on building a medium-sized library in rust](http://quietmisdreavus.net/code/2016/10/22/dispatches-from-egg-mode/).
* [Complete me!](https://blog.clap.rs/complete-me/) Generating Bash/ZSH completion scripts in Clap.
* [Supporting blanket impls in specialization](http://smallcultfollowing.com/babysteps/blog/2016/10/24/supporting-blanket-impls-in-specialization/).
* [How to test a DOM API for Servo](http://hellomalisa.me/2016-10-24/how-to-test-a-servo-dom-api.html).

## News & Project Updates

* 🎈🎉 [Announcing Rust 1.12.1](https://blog.rust-lang.org/2016/10/20/Rust-1.12.1.html). 🎉🎈
* [Introducing Rust Language Server, source release](https://internals.rust-lang.org/t/introducing-rust-language-server-source-release/4209). RLS provides a service that runs in the background, providing IDEs, editors, and other tools with information about Rust programs.
* [Facebook is writing a Mercurial server in Rust](https://groups.google.com/forum/#!topic/mozilla.dev.version-control/nh4fITFlEMk).
* [Announcing Cage: Develop and deploy complex Docker applications](http://blog.faraday.io/announcing-cage-develop-and-deploy-complex-docker-applications/).
* [imag v0.2.0 released](http://beyermatthias.de/blog/2016/10/23/imag-0-2-0/).

## Other Weeklies from Rust Community

* [This summer in Redox](https://www.redox-os.org/news/this-summer-in-redox-15/). Redox is an operating-system written in Rust.
* [These weeks In Servo 81](https://blog.servo.org/2016/10/17/twis-81/). Servo is a prototype web browser engine written in Rust.
* [This week in Rust Docs 27](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-27). Updates from the Rust documentation team.
* [This week in Tock Embedded OS #7](http://www.tockos.org/blog/2016/talking-tock-7/). Tock is a safe, multitasking operating system for low-power, low-memory microcontrollers.
* [This week in Ruru 3](http://this-week-in-ruru.org/2016/10/24/this-weeks-in-ruru-3/). Ruru lets you write native Ruby extensions in Rust.
* [This week in TiKV 2016-10-23](http://weekly.pingcap.com/2016/10/24/tidb-weekly/#notable-changes-to-tikv). TiKV is a distributed Key-Value database.
* [What's coming up in imag 18](http://beyermatthias.de/blog/2016/10/21/what-s-coming-up-in-imag-18/). imag is a text based personal information management suite.

# New Crates

* [EdgeDNS](https://github.com/jedisct1/edgedns). A high performance DNS cache designed for Content Delivery Networks, with built-in security mechanisms to protect origins, clients and itself.
* [Pinky](https://github.com/koute/pinky). An NES emulator written in Rust.
* [combine](https://github.com/Marwes/combine). A parser combinator library for Rust.
* [TensorFlow Rust](https://github.com/google/tensorflow-rust/). Rust language bindings for TensorFlow.

# Crate of the Week

*No crate was selected for CotW.*

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Unused import warning should have identifier in first line](https://github.com/rust-lang/rust/issues/37376).
* [easy] [rust: Provide a better error message when the target sysroot is not installed](https://github.com/rust-lang/rust/issues/37131).
* [hard] [rust: Optimize emscripten targets with emcc](https://github.com/rust-lang/rust/issues/36899).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

126 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-10-17..2016-10-24

* [macros 1.1: future proofing and cleanup](https://github.com/rust-lang/rust/pull/37198).
* [Enable line number debuginfo in releases](https://github.com/rust-lang/rust/pull/37280).
* [Cargo: Use `CommandExt::exec` for `cargo run` on Unix](https://github.com/rust-lang/cargo/pull/2818).
* [libc: Add setresuid/setresgid for linux](https://github.com/rust-lang/libc/pull/434).
* [Implement `From<Cow<str>> for String` and `From<Cow<[T]>> for Vec<T>`](https://github.com/rust-lang/rust/pull/37326).
* [libc: Add support for Fuchsia](https://github.com/rust-lang/libc/pull/432).
* [Use a faster `deflate` setting](https://github.com/rust-lang/rust/pull/37298).
* [libc: Add UNIX 98 pty functions](https://github.com/rust-lang/libc/pull/431).
* [LLVM: Add triple for Fuchsia](https://github.com/rust-lang/rust/pull/37261).
* [macros: Future proof `#[no_link]`](https://github.com/rust-lang/rust/pull/37247).
* [ICH: Use 128-bit Blake2b hash instead of 64-bit SipHash for incr. comp. fingerprints](https://github.com/rust-lang/rust/pull/37233).
* [Expand `.zip()` specialization to `.map()` and `.cloned()`](https://github.com/rust-lang/rust/pull/37230).
* [Mark enums with non-zero discriminant as non-zero](https://github.com/rust-lang/rust/pull/37224).
* [impl `Debug` for `ReadDir`](https://github.com/rust-lang/rust/pull/37221).
* [macros: improve `$crate`](https://github.com/rust-lang/rust/pull/37213).
* [include LLVM version in `--version --verbose`](https://github.com/rust-lang/rust/pull/37200).
* [Lint against lowercase static mut](https://github.com/rust-lang/rust/pull/37162).
* [`#[may_dangle]` attribute](https://github.com/rust-lang/rust/pull/37117).
* [Optimize `Substs::super_fold_with`](https://github.com/rust-lang/rust/pull/37108).
* [Inline `read_{un,}signed_leb128` and `opaque::Decoder` functions](https://github.com/rust-lang/rust/pull/37083).

## New Contributors

* Duncan
* loggerhead
* Ryan Senior
* Vangelis Katsikaros
* Артём Павлов [Artyom Pavlov]

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1721: Enable customizing the linkage of a platform's C runtime](https://github.com/rust-lang/rfcs/pull/1721).
* [RFC 1717: Use `#[link(kind)]` to fix imports from native libs on Windows](https://github.com/rust-lang/rfcs/pull/1717).
* [RFC 1682: Propose a shorthand syntax for constructing struct-like values with _named_ fields](https://github.com/rust-lang/rfcs/pull/1682).
* [RFC 1624: Let a `loop { ... }` expression return a value via `break my_value;`](https://github.com/rust-lang/rfcs/pull/1624).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [A process for establishing the Rust roadmap](https://github.com/rust-lang/rfcs/pull/1728).
* [Windows subsystem support](https://github.com/rust-lang/rfcs/pull/1665).
* [Add Cortex-M targets to the compiler + binary releases of `core`](https://github.com/rust-lang/rfcs/pull/1645).

## New RFCs

* [Roadmap for 2017](https://github.com/rust-lang/rfcs/pull/1774).
* [A type representing an owned C-compatible wide string](https://github.com/rust-lang/rfcs/pull/1773).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

PRs:

* [Customising Rustfmt](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

[Issue 17](https://github.com/rust-lang-nursery/fmt-rfcs/issues/17) (comments) is ready for a PR, we'd love someone to help out with that, if you're interested ping someone in #rust-style.

*No FCP issues this week.*

New P-high issues:

* [Struct and union declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/30).
* [Enums and variants](https://github.com/rust-lang-nursery/fmt-rfcs/issues/31).
* [`type` aliases](https://github.com/rust-lang-nursery/fmt-rfcs/issues/32).

# Upcoming Events

* [10/26. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [10/26. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [10/27. Rust Belt Rust Conference - Pittsburgh](http://www.rust-belt-rust.com/).
* [10/31. Rust Paris](https://www.meetup.com/Rust-Paris/events/234528214/).
* [11/2. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [11/2. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [11/3. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [11/5. Servo / Rust Hackathon](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/234883249/?eventId=234883249).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# fn work(on: RustProject) -> Money

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*No quote was selected for QotW.*

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
