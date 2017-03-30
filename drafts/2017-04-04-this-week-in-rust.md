Title: This Week in Rust 176
Number: 176
Date: 2017-04-04
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

* [Async, Futures, AMQP, pick three](https://www.clever-cloud.com/blog/engineering/2017/03/28/lapin-new-rust-amqp-library/).

# Crate of the Week

This week's Crate of this Week is [pretty_assertions](https://github.com/colin-kiegel/rust-pretty-assertions) which replaces the standard ones to make them shiny. Thanks to [willi_kappler](https://users.rust-lang.org/users/willi_kappler) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Want to join the Rust docs team](http://words.steveklabnik.com/want-to-join-the-rust-docs-team)?
* [The Underhanded Rust Contest](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [parenchyma: CUDA maintainer](https://github.com/lychee-eng/parenchyma/issues/22). Parenchyma is an extensible HPC-Framework for CUDA, OpenCL and native CPU.
* [rustup: `target add` and `component add` should succeed if target/component is already installed](https://github.com/rust-lang-nursery/rustup.rs/issues/1009).
* [flate2-rs: Add a pure-Rust backend](https://github.com/alexcrichton/flate2-rs/issues/67). flate2 provides FLATE, Gzip, and Zlib bindings for Rust.
* [tempdir: Add keywords and categories to Cargo.toml](https://github.com/rust-lang-nursery/tempdir/issues/25).
* [tempdir: Add CI for Windows and Mac](https://github.com/rust-lang-nursery/tempdir/issues/24).
* [easy] [rustup: Installation failure via the script has bad error message](https://github.com/rust-lang-nursery/rustup.rs/issues/987).
* [rustup: Build with panic=abort](https://github.com/rust-lang-nursery/rustup.rs/issues/992).
* [easy] [rustup: Improve indentation of help](https://github.com/rust-lang-nursery/rustup.rs/issues/940).
* [easy] [rustup: Document the usage of CARGO_HOME and RUSTUP_HOME to install to a custom location](https://github.com/rust-lang-nursery/rustup.rs/issues/994).
* [easy] [rustup: Document the use of toolchain link](https://github.com/rust-lang-nursery/rustup.rs/issues/954).
* [easy] [rustup: "update not yet available" message should not error](https://github.com/rust-lang-nursery/rustup.rs/issues/990).
* [easy] [bitflags: Move docs to the crate level](https://github.com/rust-lang-nursery/bitflags/issues/75).
* [easy] [bitflags: Add keywords and categories to Cargo.toml](https://github.com/rust-lang-nursery/bitflags/issues/77).
* [easy] [bitflags: Add html_root_url crate attribute](https://github.com/rust-lang-nursery/bitflags/issues/78).
* [easy] [bitflags: Remove mention of stable 'assignment_ops' feature from docs](https://github.com/rust-lang-nursery/bitflags/issues/79).
* [easy] [bitflags: Add an example of what the macro-expanded API looks like](https://github.com/rust-lang-nursery/bitflags/issues/81).

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

* Adam Ransom
* Cldfire
* Irfan Hudda
* mandeep
* Manuel
* omtcyfz
* Sam Whited

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Add a `compile_error!` macro to libstd](https://github.com/rust-lang/rfcs/pull/1695). `compile_error!` will unconditionally cause compilation to fail with the given error message when encountered.

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: postpone] [Polymorphic Numeric Constants](https://github.com/rust-lang/rfcs/pull/1945).
* [disposition: merge] [extend `?` to operate over other types](https://github.com/rust-lang/rfcs/pull/1859).
* [disposition: merge] [Remove static bound from type_id](https://github.com/rust-lang/rfcs/pull/1849).
* [disposition: merge] [Extend entry API to work on borrowed keys](https://github.com/rust-lang/rfcs/pull/1769).
* [disposition: merge] [Deprecate anonymous parameters](https://github.com/rust-lang/rfcs/pull/1685).

## New RFCs

* [Add functions to the language which take a value and an inclusive range, and will "clamp" the input to the range](https://github.com/rust-lang/rfcs/pull/1961).
* [Support profile-specific overrides for cargo features and dependencies](https://github.com/rust-lang/rfcs/pull/1956).

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

* [Mar 29. Neues Rust Meetup in Dresden](https://forum.rustplatz.de/t/neues-rust-meetup-in-dresden/156/26).
* [Mar 29. GNOME+Rust Hackfest 2017, Mexico City](https://wiki.gnome.org/Hackfests/Rust2017).
* [Mar 29. South Florida Rust Meetup: Intro to Ownership and Borrowing Part 3](https://www.meetup.com/South-Florida-Rust-Meetup/events/238110251/).
* [Mar 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Mar 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Mar 31. Underhanded Rust Contest Submission Deadline](https://underhanded.rs/blog/2016/12/15/underhanded-rust.en-US.html).
* [Apr  4. Clever Cloud talks Rust in Paris](https://www.meetup.com/fr-FR/Rust-Paris/events/238791322/)
* [Apr  5. Rust User Group Cologne - Crate Polishing](http://rust.cologne/2017/04/05/crate-polishing.html).
* [Apr  5. Rust Atlanta Meetup](https://www.meetup.com/Rust-ATL/events/238104881/).
* [Apr  5. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr  5. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr  5. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238613284/).
* [Apr  6. Rust DC Learn + Try: tokio](https://www.meetup.com/RustDC/events/238221152/).
* [Apr  6. Rust Detroit - Letting the type system catch errors for you](https://www.meetup.com/rust-detroit/events/238662757).
* [Apr  6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/238404173/).
* [Apr 11. Toronto Rust Meetup](https://www.meetup.com/Rust-Toronto/events/238780453/).
* [Apr 12. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Apr 12. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Apr 13. Rust Meetup Hamburg - Hack & Learn Tokio Edition](https://www.meetup.com/Rust-Meetup-Hamburg/events/237984043/).
* [Apr 13. Columbus Rust Society](https://www.meetup.com/columbus-rs/events/238502945/).
* **[Apr 30. RustFest 2017 - Kyiv, Ukraine](http://2017.rustfest.eu/).**

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*No jobs listed for this week.*

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I had many questions during the example implementations but "where do I find that" was none of them. [...] Thanks, docs team, you are doing great work!

— [Florian Gilcher in a blog post](http://asquera.de/blog/2017-02-27/rust-training/).

Thanks to [Jules Kerssemakers for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/369).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
