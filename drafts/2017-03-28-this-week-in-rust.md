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

* [Rust is the most loved language in Stack Overflow Developer Survey, again](https://stackoverflow.com/insights/survey/2017/#most-loved-dreaded-and-wanted).
* [WebRenderer landed in Firefox Nightly](https://www.reddit.com/r/rust/comments/618p54/webrenderer_landed_in_firefox_nightly_here_is_how/). [WebRender](https://github.com/servo/webrender) is an experimental render backend written in Rust.
* [Handling exceptions](https://os.phil-opp.com/handling-exceptions.html). Part of the series [Writing an OS in Rust](http://os.phil-opp.com/).
* [Writing an audio plugin in Rust](https://www.seventeencups.net/writing-an-audio-plugin-in-rust/).
* [Wouldn't it be neat if you could write C++ inline in Rust](https://mystor.github.io/wouldnt-it-be-neat-p1.html)?
* [Rust and RPC - OkCupid Hackweek 2017](https://tech.okcupid.com/rust-and-rpc-okcupid-hackweek-2017/).
* [Unification in Chalk, part 1](http://smallcultfollowing.com/babysteps/blog/2017/03/25/unification-in-chalk-part-1/). Chalk is a PROLOG-ish interpreter written in Rust, intended eventually for use in the compiler.
* [Polymorphism in Rust: Enum vs Trait + Struct](http://keepcalmandlearnrust.com/2017/03/polymorphism-in-rust-enum-vs-trait-struct/).
* [Using Rust in Windows](http://www.jonathanturner.org/2017/03/rust-in-windows.html).
* [How to implement a trait for `&str` and `&[&str]`](https://scribbles.pascalhertleif.de/impl-a-trait-for-str-slices-and-slices-of-strs.html).
* [Rust support has landed in upstream vim](https://github.com/vim/vim/commit/3c2881dc1195f53ebafc387378399ddd6cb677a7).
* [Redox 0.1.3 released with better VirtualBox integration](https://github.com/redox-os/redox/releases/tag/0.1.3).
* [Speakers for RustFest 2017 announced](http://2017.rustfest.eu/talks/). 30 April at Kyiv, Ukraine.
* [This week in Rust docs 49](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-49).

# Crate of the Week

We don't have a Crate of this Week for lack of suggestions. Sorry.

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

117 pull requests were [merged in the last week][merged].

[merged]: https://github.com/issues?page=6&q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2016-03-13..2016-03-20

* [1.17 library stabilizations](https://github.com/rust-lang/rust/pull/40538)
* [`0e+10` is now a valid Rust float literal](https://github.com/rust-lang/rust/pull/40589)
* [Rust works again on pre 1.12 macOS](https://github.com/rust-lang/rust/pull/40482)
* [pass attributes to procedural macros as `TokenStream`](https://github.com/rust-lang/rust/pull/40346) (macro plugin-breaking)
* [fix `include!(_)` regression](https://github.com/rust-lang/rust/pull/40583)
* [add `catch { }` to AST](https://github.com/rust-lang/rust/pull/39921) (plugin-breaking)
* [avoid alignment-related undefined behavior on operand-pair store](https://github.com/rust-lang/rust/pull/40385)
* [`TryFrom<Err=_>` is now `TryFrom<Error=_>`, also subsumes `FromStr`](https://github.com/rust-lang/rust/pull/40281)
* [new `Utf8Error::error_len`](https://github.com/rust-lang/rust/pull/40212)
* [remove `Default` impl for `Box<Path>`](https://github.com/rust-lang/rust/pull/40539)
* [remove a few stray `From<Box<_>>` impls](https://github.com/rust-lang/rust/pull/40009)
* [Fix race condition in `fs::create_dir_all(..)`](https://github.com/rust-lang/rust/pull/39799)
* [LLVM: fix inliner funclet unwind memoization](https://github.com/rust-lang/llvm/pull/66) (unbreaks MSVC optimizer)
* [`cargo -vv` now caps lints at `warn` level](https://github.com/rust-lang/cargo/pull/3827)
* [`cargo` enables default features of dependencies overriding others without those features](https://github.com/rust-lang/cargo/pull/3843)
* [`cargo` now assumes the `--cap-lints` feature is available](https://github.com/rust-lang/cargo/pull/3839)
* [rustdoc displays `const` items](https://github.com/rust-lang/rust/pull/40564)
* [docs are now required again](https://github.com/rust-lang/rust/pull/40526)

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

Other significant issues:

* [expressions (tracking issue)](https://github.com/rust-lang-nursery/fmt-rfcs/issues/16)

# Upcoming Events

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
* [Apr  5. OpenTechSchool Berlin: Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/238613284/).
* [Apr  6. Rust DC Learn + Try: tokio](https://www.meetup.com/RustDC/events/238221152/).
* [Apr  6. Rust Detroit - Letting the type system catch errors for you](https://www.meetup.com/rust-detroit/events/238662757).
* [Apr  6. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Apr 10. Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/238404173/).
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

* [Rust developers at Cornell Tech New York](https://cornell.wd1.myworkdayjobs.com/en-US/CornellCareerPage/job/New-York-City-Cornell-Tech/Developer-or-Senior-Developer-in-Residence-Cornell-Tech--New-York--NY_WDR-00010241).
* [Rust engineer at a startup in San Francisco](https://users.rust-lang.org/t/jobs-in-rust-development/3628/4).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I had many questions during the example implementations but "where do I find that" was none of them. [...] Thanks, docs team, you are doing great work!

— [Florian Gilcher in a blog post](http://asquera.de/blog/2017-02-27/rust-training/).

Thanks to [Jules Kerssemakers for the suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/369).

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [brson](https://github.com/brson).*
