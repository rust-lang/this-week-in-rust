Title: This Week in Rust 340
Number: 340
Date: 2020-05-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

# Crate of the Week

This week's crate is [cargo-asm](https://github.com/gnzlbg/cargo-asm), a cargo subcommand to show the resulting assembly of a function. Useful for performance work.

Thanks to [Trevor Spiteri](https://users.rust-lang.org/t/crate-of-the-week/2704/769) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

359 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-18..2020-05-25

* [update to LLVM 10](https://github.com/rust-lang/rust/pull/67759)
* [llvm: expose tiny code model to users](https://github.com/rust-lang/rust/pull/72397)
* [enable ARM TME (Transactional Memory Extensions)](https://github.com/rust-lang/rust/pull/72438)
* [implement new `asm!` syntax](https://github.com/rust-lang/rust/pull/69171) from [RFC #2850](TODO)
* [always generated object code for `#![no_builtins]`](https://github.com/rust-lang/rust/pull/72325)
* [break tokens before checking if they are 'probably equal'](https://github.com/rust-lang/rust/pull/72306)
* [emit a better diagnostic when function actually has a 'self' parameter](https://github.com/rust-lang/rust/pull/72308)
* [stabilize fn-like proc macros in expression, pattern and statement positions](https://github.com/rust-lang/rust/pull/68717)
* [use `once_cell` crate instead of custom data structure](https://github.com/rust-lang/rust/pull/72256)
* [simple NRVO](https://github.com/rust-lang/rust/pull/72205)
* [remove ReScope](https://github.com/rust-lang/rust/pull/72362)
* [exhaustively check `ty::Kind` during structural match checking](https://github.com/rust-lang/rust/pull/72153)
* [move borrow-of-packed-field unsafety check out of loop](https://github.com/rust-lang/rust/pull/72269)
* [fix `InlineAsmOperand` expresions being visited twice during liveness checking](https://github.com/rust-lang/rust/pull/72537)
* [chalk: cleanup crate structure and add features for SLG/recursive solvers](https://github.com/rust-lang/chalk/pull/459)
* [check non-`Send`/`Sync` upvars captured by generator](https://github.com/rust-lang/rust/pull/71923)
* [support coercion between `FnDef` and arg-less closure and vice versa](https://github.com/rust-lang/rust/pull/71599)
* [more lazy normalization of constants](https://github.com/rust-lang/rust/pull/71973)
* [miri: prepare Dlsym system for dynamic symbols on Windows](https://github.com/rust-lang/miri/pull/1424)
* [use `T`'s discriminant type in `mem::Discriminant<T>` instead of `u64`](https://github.com/rust-lang/rust/pull/70705)
* [fix discriminant type in generator transform](https://github.com/rust-lang/rust/pull/72502)
* [`impl From<Cow>` for `Box`, `Rc`, and `Arc`](https://github.com/rust-lang/rust/pull/71447)
* [another attempt to reduce `size_of<HashMap>`](https://github.com/rust-lang/hashbrown/pull/159)
* [set initial non-empty `Vec` size to 4 instead of 1](https://github.com/rust-lang/rust/pull/72227)
* [make `std::char` functions and constants associated to `char`](https://github.com/rust-lang/rust/pull/71854)
* [stabilize `saturating_abs` and `saturating_neg`](https://github.com/rust-lang/rust/pull/71886)
* [add `len` and `slice_from_raw_parts` to `NonNull<[T]>`](https://github.com/rust-lang/rust/pull/71940)
* [add fast-path optimization for `Ipv4Addr::fmt`](https://github.com/rust-lang/rust/pull/72399)
* [`impl Ord for proc_macro::LineColumn`](https://github.com/rust-lang/rust/pull/72446)
* [cargo: try installing exact versions before updating](https://github.com/rust-lang/cargo/pull/8022)
* [cargo: automatically update `patch`, and provide better errors if an update is not possible](https://github.com/rust-lang/cargo/pull/8248)
* [cargo: add option to strip binaries](https://github.com/rust-lang/cargo/pull/8246)
* [rustfmt: merge configs from parent directories](https://github.com/rust-lang/rustfmt/pull/4179)
* [rustfmt: umprove error message when module resolution failed](https://github.com/rust-lang/rustfmt/pull/4198)
* [rustfmt: parse comma-separated branches in macro definitions](https://github.com/rust-lang/rustfmt/pull/4173)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved last week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize AtomicN::fetch_min and AtomicN::fetch_max](https://github.com/rust-lang/rust/pull/72324)
* [disposition: merge] [Stabilize process_set_argv0 feature for Unix](https://github.com/rust-lang/rust/pull/72123)
* [disposition: merge] [impl From <Cow\> for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/71447)
* [disposition: close] [Tracking issue for non_static_type_id](https://github.com/rust-lang/rust/issues/41875)

## New RFCs
* [RFC: Reading into uninitialized buffers](https://github.com/rust-lang/rfcs/pull/2930)

# Upcoming Events

### Online
* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).
* [May 21. Turin, IT - Rust Turin Meetup](https://community.mozilla.org/events/gruppo-di-studio-di-rust/)
* [May 26. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybchbjc/)
* [May 26. Berlin, DE - Rust and Tell](https://www.meetup.com/Rust-Berlin/events/270319545/)
* [May 27. Montréal, QC, CA - Remote - RustMTL May 2020](https://www.meetup.com/Rust-Montreal/events/270635425)


### North America
* [May 25. Durham, NC, US - Triangle Rustaceans - Project Night and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybchbhc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Things that are programming patterns in C are types in Rust.

– [Kornel Lesiński on rust-users](https://users.rust-lang.org/t/how-has-learning-and-working-in-rust-influenced-how-you-think-about-writing-software/42836/3)

Thanks to [trentj](https://users.rust-lang.org/t/twir-quote-of-the-week/328/876) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gmyv8h/this_week_in_rust_339/).</small>
