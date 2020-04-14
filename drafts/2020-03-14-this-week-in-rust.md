Title: This Week in Rust 334
Number: 334
Date: 2020-04-14
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
* [How often does rust change](https://words.steveklabnik.com/how-often-does-rust-change).
* [Swift: Google's bet on differentiable programming](https://tryolabs.com/blog/2020/04/02/swift-googles-bet-on-differentiable-programming/).
* [The differences between Ok-wrapping, try blocks, and function level try](https://yaah.dev/try-blocks).
* [Library-ification and analyzing Rust](https://smallcultfollowing.com/babysteps/blog/2020/04/09/libraryification/).
* [April Lang Team Design Meetings](https://blog.rust-lang.org/inside-rust/2020/04/10/lang-team-design-meetings.html)
* [Downloading all the crates on crates.io](https://www.pietroalbini.org/blog/downloading-crates-io/).
* [New sysinfo release: processes disk usage](https://blog.guillaume-gomez.fr/articles/2020-04-09+New+sysinfo+release%3A+processes+disk+usage).
# Crate of the Week

This week's crate is [explaine.rs](https://github.com/jrvidal/explaine.rs), an interactive Rust syntax playground.

Thanks to [Vlad Frolov](https://users.rust-lang.org/t/crate-of-the-week/2704/747) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

443 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-03-30..2020-04-06

* [handle unterminated raw strings with no `#`s properly](https://github.com/rust-lang/rust/pull/70681)
* [parse: recover on `const fn()` / `async fn()`](https://github.com/rust-lang/rust/pull/70421)
* [improve error messages for raw strings](https://github.com/rust-lang/rust/pull/70522)
* [remove unused discriminant reads from MIR bodies](https://github.com/rust-lang/rust/pull/70595)
* [track the finalizing node in the specialization graph](https://github.com/rust-lang/rust/pull/70535)
* [use smaller span for suggestion restricting lifetime](https://github.com/rust-lang/rust/pull/70827)
* [fix performance regression in debuginfo `file_metadata`](https://github.com/rust-lang/rust/pull/70803)
* [enable layout debugging for `impl Trait` type aliases](https://github.com/rust-lang/rust/pull/70815)
* [polonius: update facts to remove the rest (🤞) of the move errors false positives](https://github.com/rust-lang/polonius/pull/147)
* [chalk: use fallback debug impls instead of `unimplemented`](https://github.com/rust-lang/chalk/pull/366)
* [chalk: goal builder](https://github.com/rust-lang/chalk/pull/361)
* [chalk: intern `Vec<ProgramClause<I>>`](https://github.com/rust-lang/chalk/pull/370)
* [typeck/type_of: let wfcheck handle generics in opaque types' substs](https://github.com/rust-lang/rust/pull/70272)
* [miri: make backtrace function names and spans match up](https://github.com/rust-lang/rust/pull/70590)
* [miri terminator handling: only do progress sanity check for 'Call' terminator](https://github.com/rust-lang/rust/pull/70771)
* [fix double-free and undefined behaviour in `libstd::syn::unix::Thread::new`](https://github.com/rust-lang/rust/pull/70597)
* [std: fix over-aligned allocations on wasm32-wasi](https://github.com/rust-lang/rust/pull/70585)
* [add `-Z dump-mir-dataflow` flag for dumping dataflow results visualization](https://github.com/rust-lang/rust/pull/70511)
* [stabilize `float::to_int_unchecked`](https://github.com/rust-lang/rust/pull/70487)
* [avoid creating unnecessary reference in Windows `Env` iterator](https://github.com/rust-lang/rust/pull/70479)
* [implement `Hash` for `Infallible`](https://github.com/rust-lang/rust/pull/70281)
* [optimize `strip_prefix` and `strip_suffix` with `str` patterns](https://github.com/rust-lang/rust/pull/69784)
* [add shims for `RwLock::`{`try_read`, `try_write`}](https://github.com/rust-lang/miri/pull/1157)
* [query-ify `Instance::resolve`](https://github.com/rust-lang/rust/pull/67797)
* [stdarch: support `crc32` even if on arm32](https://github.com/rust-lang/stdarch/pull/834)
* [add `slice::fill`](https://github.com/rust-lang/rust/pull/70752)
* [expand `vec![]` to `Vec::new()`](https://github.com/rust-lang/rust/pull/70632)
* [detailed panic messages for `Vec` functions](https://github.com/rust-lang/rust/pull/70573)
* [fix some aliasing issues in `Vec`](https://github.com/rust-lang/rust/pull/70558)
* [add `fn make_contiguous` to `VecDeque`](https://github.com/rust-lang/rust/pull/69425)
* [`BTreeMap`/`BTreeSet`: implement `drain_filter`](https://github.com/rust-lang/rust/pull/68770)
* [keep track of position when deleting from a `BTreeMap`](https://github.com/rust-lang/rust/pull/70795)
* [use `ManuallyDrop` instead of `forget` inside collections](https://github.com/rust-lang/rust/pull/70766)
* [match options directly in the `Fuse` implementation](https://github.com/rust-lang/rust/pull/70750)
* [place TLS initializers with relocations in .tdata](https://github.com/rust-lang/rust/pull/70720)
* [futures: reduce box allocation in bilock](https://github.com/rust-lang/futures-rs/pull/2104)
* [futures: impl `Extend` for `SelectAll`](https://github.com/rust-lang/futures-rs/pull/2107)
* [hashbrown: micro optimize `repeat` function](https://github.com/rust-lang/hashbrown/pull/150)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Unsafe blocks in unsafe fn](https://github.com/rust-lang/rfcs/pull/2585).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for PathBuf capacity methods](https://github.com/rust-lang/rust/issues/58234).
* [disposition: merge] [Remove -Z no-landing-pads flag](https://github.com/rust-lang/rust/pull/70175).
* [disposition: merge] [Move LLVM bitcode destination](https://github.com/rust-lang/rust/pull/70458).

## New RFCs

* [result-missing-methods-for-err](https://github.com/rust-lang/rfcs/pull/2897).

# Upcoming Events

### Online

* [Apr 10. Samara, RU - Rust Users Team Samara - Online meetup](https://samara-it-community.timepad.ru/event/1293744/).
* [Apr 14. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/prbtdrybcgbsb/).
* [Apr 23. Turin, IT - Rust Turin online meetup](http://www.toolboxoffice.it/eventi/rust-meetup-15/).

### Europe

* [Apr 15. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gztznrybcgbvb/).

### North America

* [Apr 15. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybcgbtb/).
* [Apr 22. Ann Arbor, MI, US - Ann Arbor Rust Meetup - Monthly Gathering](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/zdfscrybcgbdc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer (US & Canada) at 1Password](https://1password.com/jobs/rust-developer/).
* [Infrastructure Engineer at Aleph Alpha, Heidelberg, Germany](https://aleph-alpha.de/sw_engineer.html?language=de).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> In many cases, it is possible to completely rearchitect the underlying code while leaving the public API as-is, and without introducing new bugs. I've literally never had such a liberating experience with refactoring until Rust.
>
> In other words, I have never been so productive in any other language. Dynamic languages like JavaScript and Python are the least productive *by far*. Code runs, tests pass, put it into production and... uncaught exception! Time to rollback and redo that whole dance **AGAIN**. With Rust, we take care of all of that crap while actually writing the code the first time. No more surprise 3am wake up calls. *That* is productivity.

– [Jay Oster on rust-users](https://users.rust-lang.org/t/rust-language-efficacy-and-productivity/39352/10)

Thanks to [Louis Cloete](https://users.rust-lang.org/t/twir-quote-of-the-week/328/846) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*

<small>[Discuss on r/rust]().</small>
