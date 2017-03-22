Title: This Week in Rust 174
Number: 174
Date: 2017-03-21
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

* <img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0"><img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"> [Announcing Rust 1.16](https://blog.rust-lang.org/2017/03/16/Rust-1.16.html). <img alt="tada" class="emoji" title=":tada:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/tada.png?v=0"><img alt="balloon" class="emoji" title=":balloon:" src="https://cdn.discourse.org/business/images/emoji/emoji_one/balloon.png?v=0">
* [A gentle introduction to Rust](https://github.com/stevedonovan/gentle-intro). Series of tutorials to get you started with Rust.
* [Getting started with Piston, a game library for Rust](https://silverwingedseraph.net/programming/2017/03/13/piston-a-game-library-in-rust.html).
* [Math with distances in Rust: safety and correctness across units](https://ferrisellis.com/posts/rust-implementing-units-for-types/).
* [Rendering vector map tiles (Rust + asm.js demo)](https://pyfisch.org/blog/rendering-vector-map-tiles/).
* [ZeroMQ communication between Python and Rust](https://aimlesslygoingforward.com/blog/2017/03/18/zeromq-communication-between-python-and-rust/).
* [Announcing the tokio-io crate](https://tokio.rs/blog/tokio-io/).
* [VSCode adds support for ripgrep in latest nightly](https://github.com/Microsoft/vscode/pull/22722). ripgrep is a line oriented search tool written in Rust.
* [video] [Jeremy Soller, founder of Redox OS - interview](https://www.youtube.com/watch?v=eH5JgMlNE8o).
* [video] [Rust game demo - Box crash](https://www.youtube.com/watch?v=iEvYlKGlAPs). [Source code](https://github.com/juchiast/boxcrash).
* [This week in Rust docs 48](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-48).
* [This week in Servo 95](https://blog.servo.org/2017/03/20/twis-95/).

# Crate of the Week

This week's crate of the week is [µtest](https://github.com/japaric/utest), a testing framework for embedded software. Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the suggestion.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Hey crate authors, please start testing your code on Rust's beta branch to find regressions](https://www.reddit.com/r/rust/comments/600mwc/psa_hey_rust_users_especially_library_authors/).
* [The Underhanded Rust Contest](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [easy] [rustup: Installation failure via the script has bad error message](https://github.com/rust-lang-nursery/rustup.rs/issues/987).
* [rustup: Build with panic=abort](https://github.com/rust-lang-nursery/rustup.rs/issues/992).
* [easy] [rustup: Improve indentation of help](https://github.com/rust-lang-nursery/rustup.rs/issues/940).
* [easy] [rustup: Document the usage of CARGO_HOME and RUSTUP_HOME to install to a custom location](https://github.com/rust-lang-nursery/rustup.rs/issues/994).
* [easy] [rustup: Document the use of toolchain link](https://github.com/rust-lang-nursery/rustup.rs/issues/954).
* [easy] [rustup: "update not yet available" message should not error](https://github.com/rust-lang-nursery/rustup.rs/issues/990).
* [easy] [rustup: Replace custom download crate with reqwest](https://github.com/rust-lang-nursery/rustup.rs/issues/993).
* [bitflags: Hide @_impl implementation detail from the bitflags! rustdoc](https://github.com/rust-lang-nursery/bitflags/issues/63).
* [easy] [bitflags: Empty bitflags has unhelpful Debug representation](https://github.com/rust-lang-nursery/bitflags/issues/64).
* [easy] [bitflags: "const" items are followed by a semicolon](https://github.com/rust-lang-nursery/bitflags/issues/65).
* [easy] [bitflags: Mention Default trait in the docs](https://github.com/rust-lang-nursery/bitflags/issues/66).
* [easy] [bitflags: Move docs to the crate level](https://github.com/rust-lang-nursery/bitflags/issues/75).
* [easy] [bitflags: Add CI badges to Cargo.toml](https://github.com/rust-lang-nursery/bitflags/issues/76).
* [easy] [bitflags: Add keywords and categories to Cargo.toml](https://github.com/rust-lang-nursery/bitflags/issues/77).
* [easy] [bitflags: Add html_root_url crate attribute](https://github.com/rust-lang-nursery/bitflags/issues/78).
* [easy] [bitflags: Remove mention of stable 'assignment_ops' feature from docs](https://github.com/rust-lang-nursery/bitflags/issues/79).
* [easy] [bitflags: Add an example of what the macro-expanded API looks like](https://github.com/rust-lang-nursery/bitflags/issues/81).
* [easy] [bitflags: Implement Hex, Octal, and Binary](https://github.com/rust-lang-nursery/bitflags/issues/82).
* [easy] [byteorder: Add categories to toml file](https://github.com/BurntSushi/byteorder/issues/73).
* [easy] [byteorder: Add CI badges to toml file](https://github.com/BurntSushi/byteorder/issues/74).
* [medium] [notify-rust: Implement icons and images](https://github.com/hoodie/notify-rust/issues/13).
* [tempdir: TempDir affected by remove_dir_all unreliability on windows](https://github.com/rust-lang-nursery/tempdir/issues/15#issuecomment-286513675).
* [easy] [servo: Looking for something to work on](https://github.com/servo/servo/issues/15162).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

142 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-06..2016-03-13

* [1.16 release notes](https://github.com/rust-lang/rust/pull/39835) (merged prematurely)
* [beta backports](https://github.com/rust-lang/rust/pull/40401)
* [handle procedural macros during `save-analysis`](https://github.com/rust-lang/rust/pull/40311)
* [export attributes in `save-analysis`](https://github.com/rust-lang/rust/pull/39820)
* [rustc: exit quickly on only `--emit dep-info`](https://github.com/rust-lang/rust/pull/40336)
* [`save-analysis` now works in the presence of type errors](https://github.com/rust-lang/rust/pull/40344)
* [fix ICE on constant enums](https://github.com/rust-lang/rust/pull/40285)
* [fix incorrect span label formatting](https://github.com/rust-lang/rust/pull/40287)
* [fix const expressions with macros](https://github.com/rust-lang/rust/pull/40272)
* [fix `personality_fn` within compiler_builtins](https://github.com/rust-lang/rust/pull/40254)
* [new `syntax::ItemKind::MacroDef`, nix unnamed macros](https://github.com/rust-lang/rust/pull/40220) (breaking change)
* [update syntax for `pub(restricted)`](https://github.com/rust-lang/rust/pull/40340)
* [spans for individual path segments](https://github.com/rust-lang/rust/pull/40369)
* [fix obligation cause body mismatch](https://github.com/rust-lang/rust/pull/40404)
* [No more StorageLive for the `!` type](https://github.com/rust-lang/rust/pull/40372)
* [no more MIR pass plugins](https://github.com/rust-lang/rust/pull/40239) (even if there ever was one)
* [MIR inlining](https://github.com/rust-lang/rust/pull/39648)
* placement-in for [`HashMap`](https://github.com/rust-lang/rust/pull/40390), [`VecDeque`](https://github.com/rust-lang/rust/pull/40389)
* [reduce adaptive `HashMap` size overhead](https://github.com/rust-lang/rust/pull/40237)
* [`OsString::shrink_to_fit`](https://github.com/rust-lang/rust/pull/40410)
* [fix rustdoc display of associated consts](https://github.com/rust-lang/rust/pull/40419)
* [improve rustdoc sidebar style](https://github.com/rust-lang/rust/pull/40265)
* [decimate the old `configure` script further](https://github.com/rust-lang/rust/pull/39770)
* [rustbuild: fix compiler docs](https://github.com/rust-lang/rust/pull/40448)
* [rustbuild now copies instead of links](https://github.com/rust-lang/rust/pull/39518)
* [crates.io now uses Diesel](https://github.com/rust-lang/crates.io/pull/609)

## New Contributors

* David Roundy
* Dawid Ciężarkiewicz
* Petr Zemek
* portal
* projektir
* Russell Mackenzie
* ScottAbbey
* z1mvader

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 1860: Include the `ManuallyDrop` wrapper in `core::mem`](https://github.com/rust-lang/rfcs/pull/1860).
* [RFC 1884: Add unstable sort to libcore](https://github.com/rust-lang/rfcs/pull/1884).
* [RFC 1869: Write to standard error with `eprint!` and `eprintln!`](https://github.com/rust-lang/rfcs/pull/1869).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: postpone] [Revisiting specialization: Complementary traits](https://github.com/rust-lang/rfcs/pull/1658).
* [disposition: close] [Extend the `Hasher` trait with `fn delimit` to support one-shot hashing](https://github.com/rust-lang/rfcs/pull/1666).
* [disposition: postpone] [Disjointness based on associated types](https://github.com/rust-lang/rfcs/pull/1672).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).
* [disposition: merge] [Add a `compile_error!` macro to libstd](https://github.com/rust-lang/rfcs/pull/1695). `compile_error!` will unconditionally cause compilation to fail with the given error message when encountered.
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: postpone] [Add a `#[safe("Reason")]` to annotate why unsafe blocks are actually safe](https://github.com/rust-lang/rfcs/pull/1910).
* [disposition: postpone] [Polymorphic Numeric Constants](https://github.com/rust-lang/rfcs/pull/1945).

## New RFCs

* [Provide a more explicit way to "clone" a standard reference counted pointer](https://github.com/rust-lang/rfcs/pull/1954).
* [Introduce Cargo schema versioning](https://github.com/rust-lang/rfcs/pull/1953).
* [Expand and stabilize `impl Trait`](https://github.com/rust-lang/rfcs/pull/1951).

## Style RFCs

[Style RFCs](https://github.com/rust-lang-nursery/fmt-rfcs) are part of the process for deciding on style guidelines for the Rust community and defaults for [Rustfmt](https://github.com/rust-lang-nursery/rustfmt). The process is similar to the RFC process, but we try to reach rough consensus on issues (including a final comment period) before progressing to PRs. Just like the RFC process, all users are welcome to comment and submit RFCs. If you want to help decide what Rust code should look like, come get involved!

Issues in final comment period:

* [Ordering of types of groups within a module](https://github.com/rust-lang-nursery/fmt-rfcs/issues/71).
* [where clauses](https://github.com/rust-lang-nursery/fmt-rfcs/issues/38)

Other significant issues:

* [expressions (tracking issue)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/16)

# Upcoming Events

* [Mar 22. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238181558/).
* [Mar 22. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 22. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 23. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Mar 29. Neues Rust Meetup in Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/26).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 29. South Florida Rust Meetup: Intro to Ownership and Borrowing Part 3](https://www.meetup.com/South-Florida-Rust-Meetup/events/238110251/).
* [Mar 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [Apr  5. Rust User Group Cologne - Crate Polishing](http://rust.cologne/2017/04/05/crate-polishing.html).
* [Apr  5. Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/238104881/).
* [Apr  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr  5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr  6. Rust DC Learn + Try: tokio](https://www.meetup.com/RustDC/events/238221152/).
* [Apr  6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developers at Cornell Tech New York](https://cornell.wd1.myworkdayjobs.com/en-US/CornellCareerPage/job/New-York-City-Cornell-Tech/Developer-or-Senior-Developer-in-Residence-Cornell-Tech--New-York--NY_WDR-00010241).
* [Rust engineer at a startup in San Francisco](https://users.rust-lang.org/t/jobs-in-rust-development/3628/4).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> #rustlang is a very strange place
> sans null deref nor data race
> it has its own styles
> but once it compiles
> it will not blow up in your face

— [llogiq on Twitter](https://twitter.com/llogiq/status/837266557680168961). Check out his [Twitter feed](https://twitter.com/llogiq) for more #rustlang limericks!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
