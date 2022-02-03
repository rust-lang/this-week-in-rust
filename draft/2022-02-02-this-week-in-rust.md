Title: This Week in Rust 428
Number: 428
Date: 2022-02-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Project/Tooling Updates

### Newsletters

### Research

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

* [Ferrous Systems and AdaCore to join forces on Ferrocene](https://ferrous-systems.com/blog/ferrous-systems-adacore-joining-forces/)
* [AdaCore and Ferrous Systems joining forces to support Rust](https://blog.adacore.com/adacore-and-ferrous-systems-joining-forces-to-support-rust)

## Crate of the Week

This week's crate is [html5gum](https://github.com/untitaker/html5gum), a WHATWG HTML spec-compliant HTML5 tokenizer.

Thanks to [Markus Unterwaditzer](https://users.rust-lang.org/t/crate-of-the-week/2704/1012) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

381 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-01-17..2022-01-24

* [LLVM on AArch64/GlobalISel: fix incorrect handling of fp truncating stores](https://github.com/rust-lang/llvm-project/pull/127)
* [show a more informative panic message when `DefPathHash` does not exist](https://github.com/rust-lang/rust/pull/93098)
* [only suggest adding `!` to expressions that can be macro invocation](https://github.com/rust-lang/rust/pull/93061)
* [point at correct argument when async fn output type lifetime disagrees with signature](https://github.com/rust-lang/rust/pull/92183)
* [change lint message to be stronger for `&T` → `&mut T` transmute](https://github.com/rust-lang/rust/pull/92704)
* [improve string concatenation suggestion](https://github.com/rust-lang/rust/pull/92843)
* [formally implement let chains](https://github.com/rust-lang/rust/pull/88642)
* [implement `#[rustc_must_implement_one_of]` attribute](https://github.com/rust-lang/rust/pull/92164)
* [allow eq constraints on associated constants](https://github.com/rust-lang/rust/pull/87648)
* [check `const Drop` impls considering `~const` Bounds](https://github.com/rust-lang/rust/pull/93028)
* [add `~const` bound test for negative impls](https://github.com/rust-lang/rust/pull/92997)
* [fix ICEs related to `Deref<Target=[T; N]>` on newtypes](https://github.com/rust-lang/rust/pull/92640)
* [disable drop range tracking in generators](https://github.com/rust-lang/rust/pull/93165)
* [directly use ConstValue for single literals in blocks](https://github.com/rust-lang/rust/pull/92780)
* [add preliminary support for inline assembly for msp430](https://github.com/rust-lang/rust/pull/93219)
* [let qpath contain NtTy: `<$:ty as $:ty>::…`](https://github.com/rust-lang/rust/pull/91150)
* [make `Decodable` and `Decoder` infallible](https://github.com/rust-lang/rust/pull/93066)
* [remove a `Span` from `hir::ExprKind::MethodCall`](https://github.com/rust-lang/rust/pull/92787)
* [emit simpler code from `format_args`](https://github.com/rust-lang/rust/pull/91359)
* [fix CVE-2022-21658 (symbolic link timing attack in `std::fs::remove_dir_all`)](https://github.com/rust-lang/rust/pull/93112)
* [implement RFC 3151: Scoped threads](https://github.com/rust-lang/rust/pull/92555)
* [improve capacity estimation in `Vec::from_iter`](https://github.com/rust-lang/rust/pull/92138)
* [little improves in `CString::new` when creating from slice](https://github.com/rust-lang/rust/pull/92124)
* [add `MaybeUninit::`(`slice_`)`as_bytes`(`_mut`)](https://github.com/rust-lang/rust/pull/89747)
* [add `Option::is_some_with` and `Result::is_`{`ok`, `err`}`_with`](https://github.com/rust-lang/rust/pull/93051)
* [add `log2` and `log10` to `NonZeroU*`](https://github.com/rust-lang/rust/pull/92956)
* [std: implement `try_reserve` and `try_reserve_exact` on `PathBuf`](https://github.com/rust-lang/rust/pull/92513)
* [`impl Not for !`](https://github.com/rust-lang/rust/pull/91122) (did you guess that "not never" is still "never"?)
* [stabilize `arc_new_cyclic`](https://github.com/rust-lang/rust/pull/90666)
* [stabilize `vec_spare_capacity`](https://github.com/rust-lang/rust/pull/93016)
* [stabilize `-Z print-link-args` as `--print link-args`](https://github.com/rust-lang/rust/pull/91606)
* [cargo: error when setting crate type of both dylib and cdylib in library](https://github.com/rust-lang/cargo/pull/10243)
* [clippy: add `msrv` config for `map_clone`](https://github.com/rust-lang/rust-clippy/pull/8280)
* [clippy: check usages in `ptr_arg`](https://github.com/rust-lang/rust-clippy/pull/8271)
* [clippy: don't suggest an empty variant name in `enum_variant_names`](https://github.com/rust-lang/rust-clippy/pull/8329)
* [clippy: fix `needless_borrow` causing mutable borrows to be moved](https://github.com/rust-lang/rust-clippy/pull/8217)
* [clippy: `needless_lifetimes`: ignore lifetimes in explicit self types](https://github.com/rust-lang/rust-clippy/pull/8278)
* [clippy: `trait_duplication_in_bounds` checks path segments for trait items](https://github.com/rust-lang/rust-clippy/pull/8315)
* [clippy: fix `needless_question_mark` not considering async fn](https://github.com/rust-lang/rust-clippy/pull/8311)
* [clippy: fix `op_ref` false positive](https://github.com/rust-lang/rust-clippy/pull/8298)

### Rust Compiler Performance Triage

An awesome week. There was some bits of noise from PR [#91032](https://github.com/rust-lang/rust/issues/93032) that landed and then had to be backed out (and may soon land again), and we continue to wrestle with how to classify which things to include in rollup PR's. But overall there were some very real wins to the compiler's performance, and it is definitely reflected in the [total bootstrap time graph](https://perf.rust-lang.org/bootstrap.html). Great job!

Triage done by **@pnkfelix**.
Revision range: [7bc7be..c54dfe](https://perf.rust-lang.org/?start=7bc7be860f99f4a40d45b0f74e2d01b02e072357&end=c54dfee65126a0ac385d55389a316e89095a0713&absolute=false&stat=instructions%3Au)

4 Regressions, 5 Improvements, 4 Mixed; 3 of them in rollups

29 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-01-25.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Scoped threads in the standard library, take 2](https://github.com/rust-lang/rfcs/pull/3151)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered FCP this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [unreachable!("{}") works on Rust 2021](https://github.com/rust-lang/rust/issues/92137)
* [disposition: merge] [Tracking Issue for Stdin::lines forwarder method](https://github.com/rust-lang/rust/issues/87096)
* [disposition: merge] [impl Display for io::ErrorKind](https://github.com/rust-lang/rust/pull/93090)
* [disposition: merge] [Tracking Issue for feature(pin_static_ref): Pin::{static_ref,static_mut}](https://github.com/rust-lang/rust/issues/78186)
* [disposition: merge] [Tracking issue for #[cfg(target_has_atomic = ...)]](https://github.com/rust-lang/rust/issues/32976)
* [disposition: merge] [Stabilize -Z instrument-coverage as -C instrument-coverage](https://github.com/rust-lang/rust/pull/90132)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Add `ignore_if` RFC](https://github.com/rust-lang/rfcs/pull/3221)
* [update] [Add vendor-specific suffixes to v0 mangling RFC 2603](https://github.com/rust-lang/rfcs/pull/3224)
* [update] [Mark RFC 1201 (naked functions) superseded by RFC 2972 (constrained naked functions)](https://github.com/rust-lang/rfcs/pull/3223)

## Upcoming Events

Rusty Events between 1/26/2022 - 2/23/2022 🦀

### Online

* [January 26, 2021 | Vienna, AT | **The Rustic Mob** | Mob-Programming on Open Source Software](https://www.meetup.com/Mob-Programming-on-Open-Source-Software/events/283408208)
* [January 27, 2022 | Charlottesville, VA, US | **Minimal Area Bananagrams: a Tale of Needless Optimization** | Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/events/283355090/)
* [January 27, 2021 | Linz, AT | **Rust Meetup Linz - 18th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/283116945/)
* [January 27, 2022 | Nürnberg, DE | **Rust Nürnberg online #9**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/283118050/)
* [January 27, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545254)
* [January 29, 2021 | London, UK | **Rust (Remote) Hack & Learn** | Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/283335221/)
* [February 1, 2021 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283338268/)
* [February 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283011769)
* [February 1, 2022 | Los Gatos, CA, US | **Book #24 - Rust for Rustaceans - Chapter 4 - Error Handling** | Los Gatos Reading Group](https://www.meetup.com/Los-Gatos-Rust-Reading-Group/events/283494504/)
* [February 3, 2022 | Cardiff, UK | **Rust Book Study Session - Smart Pointers** | Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/283480500/)
* [February 8, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/283213217/)
* [February 9, 2022 | Los Angeles, CA, US | **Raphael Tessmer & Celeste, finding craters on a rusty planet** (Virtual) | Rust Los Angeles](https://www.meetup.com/Rust-Los-Angeles/events/283232930/)
* [February 9, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545292)
* [February 15, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/283351974/)
* [February 16, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out Night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/283260386/)
* [February 17, 2022 | München, DE | **Rust - beyond "Hello World"**| Agile Softwareentwicklung München](https://www.meetup.com/maibornwolff-software-engineering-netzwerk/events/283379985)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust : We have a race condition bug in our standard filesystem library !  
> C++ : You guys have a concurrency safe standard filesystem library ?  
> C : You guys have a standard filesystem library ?

– [redditmodsareshits on /r/cpp](https://np.reddit.com/r/cpp/comments/s8ok0h/possible_toctou_vulnerabilities_in)

Thanks to [UtherII](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1168) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
