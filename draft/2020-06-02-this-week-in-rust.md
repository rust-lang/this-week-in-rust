Title: This Week in Rust 341
Number: 341
Date: 2020-06-02
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

* [RustConf 2020 Registration is Open](https://rustconf.com/)
* [2020 Contributor Survey](https://blog.rust-lang.org/inside-rust/2020/05/27/contributor-survey.html)
* [A retrospective on the 2018 rust-lang.org redesign](https://blog.rust-lang.org/inside-rust/2020/05/26/website-retrospective.html)
* [Rust Disassembly: Part 1](https://giordi91.github.io/post/disassemlbyrust1/)
* [Fuzzing Sequoia-PGP](https://blog.hackeriet.no/fuzzing-sequoia/)
* [Auto-Vectorization for Newer Instruction Sets in Rust](https://www.nickwilcox.com/blog/autovec2/)
* [Current State of Embedded Rust for Flight Controllers](https://gist.github.com/tstellanova/81c963f556522447dd007a0c3a84ebc3)
* [3D boids swimming along in perfect harmony; Implementing the boids flocking algorithm in Rust](https://www.reddit.com/r/rust/comments/gsldbi/3d_boids_swimming_along_in_perfect_harmony/)
* [Coverage Marks](https://ferrous-systems.com/blog/coverage-marks/)
* [Ringbahn: a safe, ergonomic API for io-uring in Rust](https://boats.gitlab.io/blog/post/ringbahn/)
* [rust-analyzer changelog #26](https://rust-analyzer.github.io/thisweek/2020/05/25/changelog-26.html)
* [Why I'm enjoying learning Rust as a Java programmer](https://opensource.com/article/20/5/rust-java)
* [Contributing to Rust](https://blog.elinvynia.com/posts/2020-05-26-contributing-to-rust.html)
* [Get a Look on Key Rust Crates for WebAssembly](https://blog.knoldus.com/get-a-look-on-key-rust-crates-for-webassembly/)
* [video] [A Rust & Wasm tutorial on building Bitcoin infrastructure. Rust beginner-friendly!](https://www.youtube.com/watch?v=qaykNPHJcyw)
* [video] [Crust of Rust: Iterators](https://www.youtube.com/watch?v=yozQ9C69pNs&feature=emb_logo)
* [video] [Rust and Tell Berlin - May 2020](https://www.youtube.com/watch?v=rpilJV-eIVw&feature=emb_logo)

# Crate of the Week

This week's crate is [cargo-asm](https://github.com/gnzlbg/cargo-asm), a cargo subcommand to show the resulting assembly of a function. Useful for performance work.

Thanks to [Jay Oster](https://users.rust-lang.org/t/crate-of-the-week/2704/772) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation


Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ruma-events: Add predefined push notification rules](https://github.com/ruma/ruma-events/issues/105)
* [ruma-events: Add helpers to construct the fallback for rich replies](https://github.com/ruma/ruma-events/issues/81)
* [ruma-events: Create a distinct type for PushCondition::RoomMemberCount::is](https://github.com/ruma/ruma-events/issues/104)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

359 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-18..2020-05-25

* [update to LLVM 10](https://github.com/rust-lang/rust/pull/67759)
* [llvm: expose tiny code model to users](https://github.com/rust-lang/rust/pull/72397)
* [enable ARM TME (Transactional Memory Extensions)](https://github.com/rust-lang/rust/pull/72438)
* [implement new `asm!` syntax](https://github.com/rust-lang/rust/pull/69171) from [RFC #2873](https://github.com/rust-lang/rfcs/pull/2873)
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

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [impl `AsRef<[T]` for `vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [disposition: merge] [Add raw_ref macros](https://github.com/rust-lang/rust/pull/72279)
* [disposition: merge] [Tracking issue for `std::io::{BufReader, BufWriter}::capacity`](https://github.com/rust-lang/rust/issues/68833)

## New RFCs

* [add lang-team Major Change Proposals as a "pre-RFC" step](https://github.com/rust-lang/rfcs/pull/2936)
* [Unsafe statics](https://github.com/rust-lang/rfcs/pull/2937)
* [Request for creating pipes with fd other than 0,1,2](https://github.com/rust-lang/rfcs/pull/2939)

# Upcoming Events

### Online
* [June 3. Johannesburg, ZA - Remote - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/270827463/)
* [June 8. Auckland, NZ - Remote - Rust AKL](https://www.meetup.com/rust-akl/events/266876685/)
* [June 9. Seattle, WA - Remote - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcjbmb/)
* [June 11. San Diego, CA - Remote - San Diego Rust Meetup](https://www.meetup.com/San-Diego-Rust/events/270938860/)

### North America
* [June 3. Indianapolis, IN, US - Indy.rs Meetup](https://www.meetup.com/indyrs/events/dtqwprybcjbfb/)
* [June 11. Columbus, OH, US - Columbus Rust Society Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcjbpb/)
* [June 11. Lehi, UT, US - Utah Rust - Lightning Talks](https://www.meetup.com/utah-rust/events/269263282/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Rust - IOTA Foundation - Remote](https://iota.bamboohr.com/jobs/view.php?id=105)
* [Senior Rust Engineer - Polymath - Remote](https://polymath.bamboohr.com/jobs/view.php?id=96)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Things that are programming patterns in C are types in Rust.

– [Kornel Lesiński on rust-users](https://users.rust-lang.org/t/how-has-learning-and-working-in-rust-influenced-how-you-think-about-writing-software/42836/3)

Thanks to [trentj](https://users.rust-lang.org/t/twir-quote-of-the-week/328/876) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/grs1ql/this_week_in_rust_340/).</small>
