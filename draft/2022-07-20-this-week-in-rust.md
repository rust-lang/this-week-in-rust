Title: This Week in Rust 452
Number: 452
Date: 2022-07-20
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [bnum](https://crates.io/crates/bnum), a library of arbitrarily sized fixed-size numerals.

Thanks to [Isaac Holt](https://users.rust-lang.org/t/crate-of-the-week/2704/1079) for the self-suggestion.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

416 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-07-11..2022-07-18

* [add Nintendo Switch as tier 3 target](https://github.com/rust-lang/rust/pull/88991)
* [implement `for<>` lifetime binder for closures](https://github.com/rust-lang/rust/pull/98705)
* [allow destructuring opaque types in their defining scopes](https://github.com/rust-lang/rust/pull/98582)
* [allow unions with mutable references and tuples of allowed types](https://github.com/rust-lang/rust/pull/97995)
* [always create elided lifetime parameters for functions](https://github.com/rust-lang/rust/pull/97720)
* [do not error during method probe on `Sized` predicates for types that aren't the method receiver](https://github.com/rust-lang/rust/pull/99146)
* [add Output = expected type trait obligation for known binary operators](https://github.com/rust-lang/rust/pull/96482)
* [fix drop-tracking ICE when a struct containing a field with a significant drop is used across an await](https://github.com/rust-lang/rust/pull/98754)
* [fix ICE in `named_arguments_used_positionally` lint](https://github.com/rust-lang/rust/pull/99263)
* [fix spans for asm diagnostics](https://github.com/rust-lang/rust/pull/99192)
* [emit warning when named arguments are used positionally in format](https://github.com/rust-lang/rust/pull/98580)
* [better error message for `generic_const_exprs` inference failure](https://github.com/rust-lang/rust/pull/99222)
* [lower let-else in MIR](https://github.com/rust-lang/rust/pull/98574)
* [miri: optimizing Stacked Borrows (part 2): Shrink Item](https://github.com/rust-lang/miri/pull/2315)
* [use ICF (identical code folding) for building rustc](https://github.com/rust-lang/rust/pull/99062)
* [utilize PGO for windows x64 rustc dist builds](https://github.com/rust-lang/rust/pull/96978)
* [`replace_bound_vars` fast path: check predicates, don't check consts](https://github.com/rust-lang/rust/pull/99232)
* [borrow `Vec<T, A>` as `\[T\]`](https://github.com/rust-lang/rust/pull/99317)
* [final derive output improvements](https://github.com/rust-lang/rust/pull/99046)
* [fix last `let_chains` blocker](https://github.com/rust-lang/rust/pull/98633)
* [stabilize `let_chains` in Rust 1.64](https://github.com/rust-lang/rust/pull/94927)
* [stabilize `core::ffi::CStr`, `alloc::ffi::CString`, and friends](https://github.com/rust-lang/rust/pull/99277)
* [stabilize `core::ffi:c_*` and rexport in `std::ffi`](https://github.com/rust-lang/rust/pull/98315)
* [stabilize `future_poll_fn`](https://github.com/rust-lang/rust/pull/99306)
* [document and stabilize `process_set_process_group`](https://github.com/rust-lang/rust/pull/99088)
* [rearrange `slice::split_mut` to remove bounds check](https://github.com/rust-lang/rust/pull/99223)
* [add provider API to error trait](https://github.com/rust-lang/rust/pull/98072)
* [add new unstable API `downcast` to `std::io::Error`](https://github.com/rust-lang/rust/pull/98387)
* [add `#[must_use]` to `Box::from_raw`](https://github.com/rust-lang/rust/pull/99270)
* [implement `fmt::Write` for `OsString`](https://github.com/rust-lang/rust/pull/97915)
* [`UnsafeCell` blocks niches inside its nested type from being available outside](https://github.com/rust-lang/rust/pull/99011)
* [hashbrown: fix double-drop in `RawTable::clone_from`](https://github.com/rust-lang/hashbrown/pull/348)
* [cargo: allow '.' in workspace.default-members in non-virtual workspaces](https://github.com/rust-lang/cargo/pull/10784)
* [cargo: fix nested workspace resolution](https://github.com/rust-lang/cargo/pull/10846)
* [cargo: normalize path for `cargo vendor` output](https://github.com/rust-lang/cargo/pull/10668)
* [cargo: stabilize `--crate-type` flag for `cargo rustc`](https://github.com/rust-lang/cargo/pull/10838)
* [cargo: stabilize `-Zmultitarget`](https://github.com/rust-lang/cargo/pull/10766)
* [rustdoc: avoid inlining items with duplicate `(type, name)`](https://github.com/rust-lang/rust/pull/99344)
* [rustfmt: fix/comments inside trait generics gets duplicated](https://github.com/rust-lang/rustfmt/pull/5446)
* [rustfmt: remove useless conditional compilation - 2](https://github.com/rust-lang/rustfmt/pull/5449)
* [rustfmt: add `skip_macro_invocations` option](https://github.com/rust-lang/rustfmt/pull/5347)
* [clippy: add `repeated_where_clause_or_trait_bound` lint](https://github.com/rust-lang/rust-clippy/pull/8703)
* [clippy: add `std_instead_of_core`, `std_instead_of_alloc`, `alloc_instead_of_core`](https://github.com/rust-lang/rust-clippy/pull/9103)
* [clippy: add new lint `obfuscated_if_else`](https://github.com/rust-lang/rust-clippy/pull/9148)
* [clippy: fix `mismatching_type_param_order` false positive](https://github.com/rust-lang/rust-clippy/pull/9146)
* [clippy: fix for `branches_sharing_code`](https://github.com/rust-lang/rust-clippy/pull/9138)
* [clippy: improve `while_let_on_iterator` suggestion inside an `FnOnce` closure](https://github.com/rust-lang/rust-clippy/pull/9134)
* [clippy: move `format_push_string` to `restriction`](https://github.com/rust-lang/rust-clippy/pull/9161)
* [clippy: `box_collection`: raise warn for all std collections](https://github.com/rust-lang/rust-clippy/pull/9170)
* [clippy: change applicability type to `MaybeIncorrect` in `explicit_counter_loop`](https://github.com/rust-lang/rust-clippy/pull/9149)
* [clippy: `unused_self`: respect `avoid-breaking-exported-api`](https://github.com/rust-lang/rust-clippy/pull/9199)
* [clippy: `match_like_matches_macro` does not trigger when one arm contains conta…](https://github.com/rust-lang/rust-clippy/pull/9178)
* [rust-analyzer: add simple support for completion item details](https://github.com/rust-lang/rust-analyzer/pull/12807)
* [rust-analyzer: add `str_ref_to_string` fix](https://github.com/rust-lang/rust-analyzer/pull/12696)
* [rust-analyzer: automatically instaciate trivially instaciable structs in "Generate new" and "Fill struct fields"](https://github.com/rust-lang/rust-analyzer/pull/12539)
* [rust-analyzer: fix extract variable assist for subexpression in mutable borrow](https://github.com/rust-lang/rust-analyzer/pull/12788)
* [rust-analyzer: support negative, `char` & `bool` const generics](https://github.com/rust-lang/rust-analyzer/pull/12778)
* [rust-analyzer: go to implementation of trait methods](https://github.com/rust-lang/rust-analyzer/pull/12549)
* [rust-analyzer: `super::` completion at crate root and module depth aware](https://github.com/rust-lang/rust-analyzer/pull/12735)
* [rust-analyzer: don't show qualified path completions for private items](https://github.com/rust-lang/rust-analyzer/pull/12766)
* [rust-analyzer: fix VSCode status bar tooltip not showing the error messages](https://github.com/rust-lang/rust-analyzer/pull/12754)
* [rust-analyzer: fix imports being inserted before doc comments in inline modules](https://github.com/rust-lang/rust-analyzer/pull/12765)
* [rust-analyzer: fix unresolved proc macro diagnostics pointing to macro expansions](https://github.com/rust-lang/rust-analyzer/pull/12691)
* [rust-analyzer: stack overflows and wrong type inference of associated type shorthands](https://github.com/rust-lang/rust-analyzer/pull/12781)
* [rust-analyzer: support generics in extract_function assist](https://github.com/rust-lang/rust-analyzer/pull/12556)
* [rustup: revert "Set RUSTC and RUSTDOC env for child processes run through the proxy"](https://github.com/rust-lang/rustup/pull/3034)
* [rustup: improved warning message for System-Rust-override](https://github.com/rust-lang/rustup/pull/3038)
* [rustup: correctly propagate subshell failures in rustup-init](https://github.com/rust-lang/rustup/pull/3012)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### Call for Testing

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Pre-Stabilization RFCs go here -->

<!-- RFC and FCP sections go here -->

## Upcoming Events

Rusty Events between 2022-07-20 - 2022-08-17 🦀

### Virtual

* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-14 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust July 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/287024976/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-20 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Discussion - Building a Multithreaded Web Server**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121101/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-26 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydckbjc/)
* 2022-07-27 | Cardiff, UK | [Rust and C++ Cardiff ](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Using Rust to read the Little Schemer**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/287121637/)
* 2022-07-29 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/Minneapolis-Rust-Meetup/)
    * [**Beginner Rust Open "Office Hours"**](https://www.meetup.com/Minneapolis-Rust-Meetup/events/286993342/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)
* 2022-08-02 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydclbdb/)
* 2022-08-03 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)

### Europe

* 2022-07-20 | Wrocław, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw/)
    * [**Rust Warsaw #5**](https://www.meetup.com/rust-warsaw/events/287093615/)
* 2022-07-21 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #27**](https://www.meetup.com/rust-wroclaw/events/287023750/)
* 2022-06-22 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developers Amsterdam Group**](https://www.meetup.com/rust-amsterdam-group/events/287019877/)

### North America

* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-13 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/287009519/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)

### Oceania

* 2022-07-19 | Phillip, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**Canberra July Meetup**](https://www.meetup.com/rust-canberra/events/286884699/)
* 2022-07-28 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**July Meetup**](https://www.meetup.com/rust-brisbane/events/286889804/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has removed the jobs posting section. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The long compile times where all responsibility is taken away from you is infinitely more effective than submission patterns in BDSM, where the graceful rustc takes over and all you have to do is wait until they tell you that you're a good person and that everything is alright!

– [/u/whyvitamins on /r/rust](https://www.reddit.com/r/rust/comments/w0oyo5/comment/igfs4fw)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1268) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
