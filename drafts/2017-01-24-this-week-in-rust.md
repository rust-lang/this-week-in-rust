Title: This Week in Rust 166
Number: 166
Date: 2017-01-24
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

* [Announcing Rust Language Server alpha release](http://www.jonathanturner.org/2017/01/rls-alpha-release.html). RLS provides a service that runs in the background, providing IDEs, editors, and other tools with information about Rust programs.
* [Hyper 0.10 is released which no longer depends on OpenSSL](http://seanmonstar.com/post/156128815358/a-hyper-update), and next release [will bring in non-blocking IO support](https://github.com/hyperium/hyper/commit/2d2d5574a698e74e5102d39b9a9ab750860d92d1).
* [Parallelizing Enjarify in Go and Rust](https://medium.com/@robertgrosse/parallelizing-enjarify-in-go-and-rust-21055d64af7e). Comparing Go and Rust performance in parallelism ([source code](https://github.com/google/enjarify/tree/go)).
* [Mitigating underhandedness with Clippy](https://manishearth.github.io/blog/2017/01/21/mitigating-underhandedness-clippy/).
* [Coroutines and Rust](https://users.rust-lang.org/t/coroutines-and-rust/9058).
* [Text analysis in Rust: Tokenization](http://nitschinger.at/Text-Analysis-in-Rust-Tokenization/).
* [Assigning blame to unsafe code](http://smallcultfollowing.com/babysteps/blog/2017/01/22/assigning-blame-to-unsafe-code/).
* [Defining a ‘handshake’ protocol between two traits](https://withoutboats.github.io/blog/rust/patterns/traits/2017/01/21/handshake-patterns.html).
* [Short intro to C++ for Rust developers: Ownership and borrowing](https://nercury.github.io/c++/intro/2017/01/22/cpp-for-rust-devs.html).
* [Using the Borrow Checker to make Tic-Tac-Toe safer](https://lukaskalbertodt.github.io/2017/01/20/abusing-borrowck-to-make-tic-tac-toe-safer.html).
* [servo/rust-bindgen is not the canonical repository for `bindgen` crate](https://www.reddit.com/r/rust/comments/5pr3t9/heads_up_yamakakyrustbindgen_and_servorustbindgen/).
* [Categories and CI badges are now available on crates.io](http://www.integer32.com/2017/01/20/categories-and-ci-badges.html).

## Other Weeklies from Rust Community

* [This week in Rust docs 40](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-40).
* [This week in Remacs 2017-01-19](http://www.wilfred.me.uk/blog/2017/01/19/this-week-in-remacs/).
* [video] [Ferris makes Emulators 17](https://www.youtube.com/watch?v=0Lq3pj8qxk4): Envelopes and play control.

# Crate of the Week

This week's Crate of the Week is [alacritty](https://github.com/jwilm/alacritty), an OpenGL-propelled Terminal application. Really fast, nice looking. Missing scrollback. Thanks to Vikrant for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [easy] [rust: Removal of the lang feature gate tests whitelist](https://github.com/rust-lang/rust/issues/39059).
* [rust: Make Rust on wasm + emscripten a reliable, 1st class Rust target](https://github.com/rust-lang/rust/issues/38805).
* [easy] [rust: Rvalue static promotion](https://github.com/rust-lang/rust/issues/38865).
* [easy] [Diesel: Refactorings using macros in type position](https://github.com/diesel-rs/diesel/issues/521).
* [easy] [Diesel: Deny missing docs](https://github.com/diesel-rs/diesel/issues/563).
* [android-rs-glue: Add more arguments and use clap to parse the arguments](https://github.com/tomaka/android-rs-glue/issues/115).
* [tokei: Add package repositories](https://github.com/Aaronepower/tokei/issues/92).
* [RustCrypto/hashes: Missing hash functions](https://github.com/RustCrypto/hashes/issues/1).
* [RustCrypto/block-ciphers: Missing block ciphers](https://github.com/RustCrypto/block-ciphers/issues/1).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

119 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-01-09..2017-01-16

* [jemalloc now x86(_64)-only](https://github.com/rust-lang/rust/pull/38675)
* [actual ranges for `BTree::range(_)`](https://github.com/rust-lang/rust/pull/38610)
* [better ergonomics for iterators yielding `Result`s](https://github.com/rust-lang/rust/pull/38580)
* [`use _::{self, ..}` now only imports `self` once](https://github.com/rust-lang/rust/pull/38313) (breaking change)
* [faster UTF-8 validation](https://github.com/rust-lang/rust/pull/37926)
* [enable attributes and `cfg` on struct fields](https://github.com/rust-lang/rust/pull/38814)
* [allow lint attributes on non-item nodes](https://github.com/rust-lang/rust/pull/38806)
* [MIR constant promote `fn` arguments correctly](https://github.com/rust-lang/rust/pull/38989)
* [use little, nbot native endian for Blake2 hashing](https://github.com/rust-lang/rust/pull/38960)
* [more complete `save-analysis`](https://github.com/rust-lang/rust/pull/38937)
* [unions don't get drop glue](https://github.com/rust-lang/rust/pull/38934)
* [`impl Display for char::`{`Escape`, `To*Case`}](https://github.com/rust-lang/rust/pull/38909)
* [cache predecessors for incremental compilation](https://github.com/rust-lang/rust/pull/39020)
* [`cargo test --doc` now correctly handles dev-dependencies](https://github.com/rust-lang/cargo/pull/3490)
* [allow specifying numerical debuginfo level](https://github.com/rust-lang/cargo/pull/3534)
* [`cargo build --all`](https://github.com/rust-lang/cargo/pull/3511), [`cargo doc --all`](https://github.com/rust-lang/cargo/pull/3515)

## New Contributors

* Colm Seale
* Constantin
* Eijebong
* gralpli
* Jack Vickeridge
* Jacob Wahlgren
* Josh
* krdln
* Lin Clark
* Martin Hafskjold Thoresen
* Matthew Dawson
* Richard S. Imaoka
* Stephen E. Baker
* theduke

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1651: Extend `Cell` to work with non-`Copy` types](https://github.com/rust-lang/rfcs/pull/1651).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

* [Const-dependent type system (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1657).
* [Add syntax for expressing tuples as a head and tail pair, similar to a Lisp cons cell](https://github.com/rust-lang/rfcs/pull/1582).

## New RFCs

* [Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).
* [A portability lint](https://github.com/rust-lang/rfcs/pull/1868).
* [Improve the `assert_eq` failure message formatting to increase legibility](https://github.com/rust-lang/rfcs/pull/1866).
* [Add official Gitter and Slack channels to compliment our official IRC channels](https://github.com/rust-lang/rfcs/pull/1865).
* [Add `extern type` declarations for declaring types from external libraries which have an unknown size/layout](https://github.com/rust-lang/rfcs/pull/1861).
* [Include the `ManuallyDrop` wrapper in `core::mem`](https://github.com/rust-lang/rfcs/pull/1860).
* [Extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [Add built-in trait `Move` which all existing types will implement. Types which do not implement it cannot move after they have been borrowed](https://github.com/rust-lang/rfcs/pull/1858).
* [Add metadata to diagnostic messages' json output](https://github.com/rust-lang/rfcs/pull/1855).
* [Stabilize drop order](https://github.com/rust-lang/rfcs/pull/1857).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Ready for PR:

There's [a lot of them](https://github.com/rust-lang-nursery/fmt-rfcs/issues?q=is%3Aopen+is%3Aissue+label%3Aready-for-PR) right now, contributions here would be very welcome. If you want advice or help getting started, please ping nrc, or any other member of the style team, in #rust-style.

Issues in final comment period:

* [Whitespace in associated type syntax](https://github.com/rust-lang-nursery/fmt-rfcs/issues/51).
* [Against braces always demanding rightward drift](https://github.com/rust-lang-nursery/fmt-rfcs/issues/50).
* [Disable trailing comma by default](https://github.com/rust-lang-nursery/fmt-rfcs/issues/42).
* [Conventions for Cargo.toml files (FCP)](https://github.com/rust-lang-nursery/fmt-rfcs/pull/41).
* [function declarations](https://github.com/rust-lang-nursery/fmt-rfcs/issues/39).
* [Customisation of Rustfmt should be allowed](https://github.com/rust-lang-nursery/fmt-rfcs/pull/33).

# Upcoming Events

* [1/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/25. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658932/).
* [1/26. Rust Stockholm: REST in Rust and Rust Hack Night](https://www.meetup.com/ruststhlm/events/236791788/).
* [1/26. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [1/28. Rust MX - Rust Meetup in Mexico City](https://www.meetup.com/Rust-MX/events/236642131/).
* [2/1. Rust User Group Cologne - Macros 1.1](http://rust.cologne/2017/02/01/proc-macros.html).
* [2/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/2. Stockholm Google Developer Group - Rust Talk](https://www.meetup.com/Stockholm-Google-Developer-Group/events/236959999/).
* [2/4 - 2/5: FOSDEM 2017 Belgium - Meeting for Rustaceans](https://fosdem.org/2017/schedule/event/rust_bof/).
* [2/8. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/8. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/9. Rust Boulder/Denver - Redox OS with Denver Open Source OS](https://www.meetup.com/Rust-Boulder-Denver/events/237016107/).
* [2/9. Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/236907254/).
* [2/9. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior backend developer at OneSignal](https://angel.co/onesignal/jobs/128684-senior-backend-developer).
* [Rust backend developer at 1aim.com](https://news.ycombinator.com/item?id=13302210).
* [Rust systems programmer at Hadean](https://news.ycombinator.com/item?id=13301893).
* [Rust engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Yeah, it's like learning to dance when your partner [borrow checker] already knows all the steps. When you're just getting started, you step on their toes a lot, but over time you get the motions down. Eventually, you can start to anticipate their movements and start to appreciate the music as part of the dance, instead of just concentrating on getting your feet in the right place.

— [QuietMisdreavus on reddit](https://www.reddit.com/r/rust/comments/5okn5y/this_week_in_rust_165/dcl0vv4/).

Thanks to [matthieum for the suggestion](https://www.reddit.com/r/rust/comments/5okn5y/this_week_in_rust_165/dclejnt/).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
