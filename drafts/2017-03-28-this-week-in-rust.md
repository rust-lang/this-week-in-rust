Title: This Week in Rust 175
Number: 175
Date: 2017-03-28
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

This week's Crate of this Week is [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions) which replaces the standard ones to make them shiny. Thanks to [willi_kappler](https://users.rust-lang.org/users/willi_kappler) for the suggestion!

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

120 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-20..2016-03-27

* [yet another sort optimization](https://github.com/rust-lang/rust/pull/40807)
* [even faster `unstable_sort`](https://github.com/rust-lang/rust/pull/40601) (integrates [pdqsort](https://github.com/stjepang/pdqsort) into std, note the "unstable" here is about sort order)
* [replace `FromStr` with `TryFrom`](https://github.com/rust-lang/rust/pull/40281) (yay for the more general solution)
* [implement `Error` for `!`](https://github.com/rust-lang/rust/pull/40566)
* [`format!(..)` changes padding logic](https://github.com/rust-lang/rust/pull/40241) (⚠ breaking change! ⚠)
* [fix invalid `Debug` display for associated constants](https://github.com/rust-lang/rust/pull/39628)
* [fix macro derive ICE](https://github.com/rust-lang/rust/pull/40664)
* [macros: better quoting for `TokenStream`s](https://github.com/rust-lang/rust/pull/40532)
* [forbid conflicts between 1.0 and 2.0 macros](https://github.com/rust-lang/rust/pull/40509)
* [allow `use`d 2.0 macros to shadow global macros](https://github.com/rust-lang/rust/pull/40501)
* [on-demand privacy checks](https://github.com/rust-lang/rust/pull/40771) & [associated item retrieval](https://github.com/rust-lang/rust/pull/40668)
* [warn instead of err on `'static` lifetime bounds](https://github.com/rust-lang/rust/pull/40734) (just use it directly, will you?)
* [stabilize `pub(restricted)`](https://github.com/rust-lang/rust/pull/40566)
* [more helpful error on incorrect `pub(restricted)`](https://github.com/rust-lang/rust/pull/40627)
* [simplify hash table drops](https://github.com/rust-lang/rust/pull/40739)
* [implement `?` in `catch` expressions](https://github.com/rust-lang/rust/pull/40229)
* [remove unused adt-def insertion](https://github.com/rust-lang/rust/pull/40696) (yay for cleaning up)
* [revert an unfortunate interaction between reachability & type inference](https://github.com/rust-lang/rust/pull/40636) (never-types make my head hurt, too)
* [propagate expected type hints through struct literals](https://github.com/rust-lang/rust/pull/40398)
* [trait object type parsing refactored, fixed](https://github.com/rust-lang/rust/pull/40043)
* [HIR now has a `HirId` to use instead of `ast::NodeId`](https://github.com/rust-lang/rust/pull/40518) (plugin-breaking)
* [MIR: constant function pointers are now values instead of items](https://github.com/rust-lang/rust/pull/40602)
* [use MIR to translate shims](https://github.com/rust-lang/rust/pull/39628) (removes a lot of the old `trans` code)
* [rustc now uses the liblog crate from crates.io](https://github.com/rust-lang/rust/pull/40347) (The first crate from crates.io in rustc! 🎉)
* [split out `rls-data` crate to be used both by `--save-analysis` and the RLS](https://github.com/rust-lang/rust/pull/40554) (another crate on crates.io)
* [`rustc --emit=mir`](https://github.com/rust-lang/rust/pull/39891)
* [Correctly get source for metatdata-only crate type](https://github.com/rust-lang/rust/pull/40542)
* [add missing LLVM 4.0 debuginfo to globals](https://github.com/rust-lang/rust/pull/40581)
* [always create unwind tables under Windows](https://github.com/rust-lang/rust/pull/40549)
* LLVM on ARM: [fix to codegen](https://github.com/rust-lang/rust/pull/40779), [fix `computeKnownBits` off-by-one error](https://github.com/rust-lang/llvm/pull/67)
* [rustdoc now uses natural sort order for item names](https://github.com/rust-lang/rust/pull/40567)
* [cargo invalidates caches on metadata change](https://github.com/rust-lang/cargo/pull/3857)
* [`cargo test` now reports the name of failing tests](https://github.com/rust-lang/cargo/pull/3848)
* [cargo no longer overflows the stack on cyclic dependencies](https://github.com/rust-lang/cargo/pull/3848)
* [crates.io now uses](https://github.com/rust-lang/crates.io/pull/640) [Diesel](https://diesel.rs) for `/crates/new`
* [crates.io: Schema simplification](https://github.com/rust-lang/crates.io/pull/641)
* [crates.io: updated dependencies](https://github.com/rust-lang/crates.io/pull/642)
* [crates.io now builds with clippy](https://github.com/rust-lang/crates.io/pull/618)

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
* [Ranges](https://github.com/rust-lang-nursery/fmt-rfcs/issues/60)
* [Conventions for blank lines](https://github.com/rust-lang-nursery/fmt-rfcs/issues/57)

Other significant issues:

* [expressions (tracking issue)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/16)

Good first issues:

We're happy to mentor these, please reach out to us in #rust-style if you'd like to get invovled

* [simple expressions](https://github.com/rust-lang-nursery/fmt-rfcs/issues/68)
* [assignment and assignment operators](https://github.com/rust-lang-nursery/fmt-rfcs/issues/67)


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
* [Apr  6. Rust Detroit - Letting the type system catch errors for you](https://www.meetup.com/rust-detroit/events/238662757).
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

> \#rustlang is a very strange place
> sans null deref nor data race
> it has its own styles
> but once it compiles
> it will not blow up in your face

— [llogiq on Twitter](https://twitter.com/llogiq/status/837266557680168961). Check out his [Twitter feed](https://twitter.com/llogiq) for more #rustlang limericks!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
