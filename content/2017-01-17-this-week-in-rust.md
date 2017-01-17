Title: This Week in Rust 165
Number: 165
Date: 2017-01-17
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

* [Announcing Tokio 0.1](https://tokio.rs/blog/tokio-0-1/).
* [Forensic tool development with Rust](http://getreu.net/public/downloads/doc/forensic-tool-development-with-rust/Forensic-Tool%20Development%20with%20Rust.html) [(pdf)](http://getreu.net/public/downloads/doc/forensic-tool-development-with-rust/Forensic-Tool%20Development%20with%20Rust.pdf).
* [PEG parser combinators using operator overloading without macros](https://github.com/J-F-Liu/pom/blob/master/doc/article.md).
* [Rust tidbits: Box is special](https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special/).
* [Rust tidbits: What is a lang item](https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item/)?
* [Let’s stop ascribing meaning to Code Points](https://manishearth.github.io/blog/2017/01/14/stop-ascribing-meaning-to-unicode-code-points/).
* [Steed: Let’s build a standard library (std) free of C code / dependencies for Linux](https://users.rust-lang.org/t/lets-build-a-standard-library-std-free-of-c-code-dependencies-for-linux/8930).
* [Remacs: Porting Emacs to Rust](http://www.wilfred.me.uk/blog/2017/01/11/announcing-remacs-porting-emacs-to-rust/).
* [How do I satisfy the borrow checker](https://m-decoster.github.io//2017/01/16/fighting-borrowchk/).
* [Rust on Teensy part 1: Bootup to LED](https://branan.github.io/teensy/2017/01/12/bootup.html). PJRC Teensy is a USB-based microcontroller development system.
* [How to make something public within a crate, but private outside it](http://stackoverflow.com/a/41667202).
* [Debugging Rust's new Custom Derive system](https://quodlibetor.github.io/posts/debugging-rusts-new-custom-derive-system/).
* [Announcing Rust Contributors](http://words.steveklabnik.com/announcing-rust-contributors).
* [Announcing Language team shepherds](https://internals.rust-lang.org/t/language-team-shepherds/4595). Language team is creating a group of _shepherds_, comprising of members of the community with an increased level of trust and responsibility. They will guide discussion on RFCs, mentor RFC authors, and collaborate on improving designs and expositions.
* [Rust and the limits of swarm design](http://esr.ibiblio.org/?p=7303). Follow-up to - [Rust severely disappoints me](http://esr.ibiblio.org/?p=7294).
* [video playlist] [Videos from Rust Belt Rust 2016 are now available](https://www.youtube.com/playlist?list=PLgC1L0fKd7UmdG82JOEE0uzXci1XY61xU).

## Other Weeklies from Rust Community

* [This week in Rust docs 39](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-39).
* [This week in Servo 88](https://blog.servo.org/2017/01/16/twis-88/).
* [This week in Ruma 2017-01-15](https://www.ruma.io/news/this-week-in-ruma-2017-01-15/).
* [These weeks in Ruru 6](http://this-week-in-ruru.org/2017/01/10/these-weeks-in-ruru-6/).
* [What's coming up in imag 22](https://beyermatthias.de/blog/2017/01/17/whats-coming-up-in-imag-22/)

# Crate of the Week

This week's Crate of the Week is [alacritty](https://github.com/jwilm/alacritty), an OpenGL-propelled Terminal application. Really fast, nice looking. Missing scrollback. Thanks to Vikrant for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Rust::from(lang)](https://github.com/mgattozzi/rust-from-lang) is a project to help people transition from other languages to Rust with articles that show how to do something in one language and then how to do it in Rust and comparing the two. You can help by writing examples or request for articles on problems you need help with.
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

* Behnam Esfahbod
* Benjamin Saunders
* Ben Wiederhake
* Bjorn Tipling
* Christopher Armstrong
* Craig Macomber
* Djzin
* Jeff Waugh
* Tyler Julian

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [Allow uncallable method impls to be omitted](https://github.com/rust-lang/rfcs/pull/1699).
* [Const-dependent type system (also known as, Π-types and value-types)](https://github.com/rust-lang/rfcs/pull/1657).
* [Extend `Cell` to work with non-`Copy` types](https://github.com/rust-lang/rfcs/pull/1651).
* [Macros by example 2.0. A replacement for `macro_rules!`](https://github.com/rust-lang/rfcs/pull/1584).
* [Add syntax for expressing tuples as a head and tail pair, similar to a Lisp cons cell](https://github.com/rust-lang/rfcs/pull/1582).
* [Allow coercing non-capturing closures to function pointers](https://github.com/rust-lang/rfcs/pull/1558).

## Closed RFCs

Following proposals were rejected by [the team](https://www.rust-lang.org/team.html) after their 'final comment period' elapsed.

* [Abort by default v2](https://github.com/rust-lang/rfcs/pull/1765). Specify abort-by-default in `Cargo.toml` when the user does `cargo new --bin`, as well as various other refinements to the panick strategy system.

## New RFCs

*No new RFCs were proposed this week.*

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

* [1/18. Rust Cologne: Ruby meets Rust](https://www.meetup.com/RustCologne/events/235877954/).
* [1/18. Rust LA Monthly Meetup - Hack Night](https://www.meetup.com/Rust-Los-Angeles/events/236735645/).
* [1/18. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/18. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/19. Rust Paris: Rust meetup #35](https://www.meetup.com/Rust-Paris/events/236727277/).
* [1/19. GPU enhanced terminals, Counting Votes, and Converting C to Rust](https://www.meetup.com/Rust-Bay-Area/events/236668916/).
* [1/20. Rust Rhein-Main: Rust Table of Regulars Darmstadt](https://www.meetup.com/de-DE/Rust-Rhein-Main/events/236456912/?eventId=236456912).
* [1/24. Mozilla Meetup Switzerland: Rust January Meetup @ Coredump.ch](https://www.meetup.com/de-DE/Mozilla-Meetup-Switzerland/events/236277734/?eventId=236277734).
* [1/25. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [1/25. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [1/25. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/236658932/).
* [1/26. Rust Stockholm: REST in Rust and Rust Hack Night](https://www.meetup.com/ruststhlm/events/236791788/).
* [1/26. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [1/28. Rust MX - Rust Meetup in Mexico City](https://www.meetup.com/Rust-MX/events/236642131/).
* [2/1. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [2/1. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [2/2. Stockholm Google Developer Group - Rust Talk](https://www.meetup.com/Stockholm-Google-Developer-Group/events/236959999/).
* [2/4 - 2/5: FOSDEM 2017 Belgium - Meeting for Rustaceans](https://fosdem.org/2017/schedule/event/rust_bof/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior backend developer at OneSignal](https://angel.co/onesignal/jobs/128684-senior-backend-developer).
* [Rust backend developer at 1aim.com](https://news.ycombinator.com/item?id=13302210).
* [Rust systems programmer at Hadean](https://news.ycombinator.com/item?id=13301893).
* [Embedded software engineer at ATS](http://advancedtelematic.com/en/careers/embedded-software-engineer.html)
* [Rust engineer at MaidSafe](https://maidsafe.net/careers.html#rust_engineer)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I really hate the phrase "fighting". Calling it a fight doesn't do justice to the conversations you have with the borrow checker when you use Rust every day. You don't fight with the borrow checker, because there isn't a fight to win. It's far more elegant, more precise. It's fencing; you fence with the borrow checker, with ripostes and parries and well-aimed thrusts. And sometimes, you get to the end and you realize you lose anyway because the thing you were trying to do was fundamentally wrong. And it's okay, because it's just fencing, and you're a little wiser, a little better-honed, a little more practiced for your next bout.

— [kaosjester on Hacker News](https://news.ycombinator.com/item?id=13413021).

Thanks to [Manishearth](https://users.rust-lang.org/users/manishearth) for the [suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/346).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
