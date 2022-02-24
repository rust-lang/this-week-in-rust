Title: This Week in Rust 432
Number: 432
Date: 2022-03-02
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

### Foundation

### Project/Tooling Updates

### Research

### Observations/Thoughts

### Rust Walkthroughs
* [Integrating Rust With Android Development](https://medium.com/@otukof/integrating-rust-with-android-development-ef341c2f9cca)

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-nextest](https://nexte.st), a new fast test runner for Rust.

Thanks to [Christopher Durham](https://users.rust-lang.org/t/crate-of-the-week/2704/1026) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

284 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-02-14..2022-02-21

* [upgrade to LLVM 14](https://github.com/rust-lang/rust/pull/93577)
* [compiler-builtins: handle Win64 builtins ABI change in LLVM 14](https://github.com/rust-lang/compiler-builtins/pull/455)
* [add MemTagSanitizer Support](https://github.com/rust-lang/rust/pull/91675)
* [put crate metadata first in the rlib](https://github.com/rust-lang/rust/pull/93816)
* [debuginfo: support fat pointers to unsized tuples](https://github.com/rust-lang/rust/pull/94050)
* [extend uninhabited enum variant branch elimination to also affect fallthrough](https://github.com/rust-lang/rust/pull/93387)
* [fix inconsistent symbol mangling with -Zverbose](https://github.com/rust-lang/rust/pull/94037)
* [fix pretty printing of enums without variants](https://github.com/rust-lang/rust/pull/94093)
* [improve `unused_unsafe` lint](https://github.com/rust-lang/rust/pull/93678)
* [add more info and suggestions to use of `#[test]` on invalid items](https://github.com/rust-lang/rust/pull/92959)
* [do not suggest "is a function" for free variables](https://github.com/rust-lang/rust/pull/93996)
* [suggest `impl Trait` return type when incorrectly using a generic return type](https://github.com/rust-lang/rust/pull/89892)
* [suggest copying trait associated type bounds on lifetime error](https://github.com/rust-lang/rust/pull/92683)
* [suggest deriving required supertraits](https://github.com/rust-lang/rust/pull/93693)
* [suggest using raw strings when invalid escapes appear in literals](https://github.com/rust-lang/rust/pull/93999)
* [correctly mark the span of captured arguments in `format_args!()`](https://github.com/rust-lang/rust/pull/94030)
* [deny mixing bin crate type with lib crate types](https://github.com/rust-lang/rust/pull/92933)
* [normalize obligation and expected trait_refs in confirm_poly_trait_refs](https://github.com/rust-lang/rust/pull/94108)
* [only mark projection as ambiguous if GAT substs are constrained](https://github.com/rust-lang/rust/pull/93892)
* [generator drop tracking: improve break and continue handling](https://github.com/rust-lang/rust/pull/93752)
* [overhaul interning](https://github.com/rust-lang/rust/pull/93148)
* [support pretty printing of invalid constants](https://github.com/rust-lang/rust/pull/94020)
* [add mentions to `Copy` types being valid for `union` fields](https://github.com/rust-lang/rust/pull/94031)
* [stabilize `#[cfg(panic = "...")]`](https://github.com/rust-lang/rust/pull/93658)
* [do not ICE when inlining a function with un-satisfiable bounds](https://github.com/rust-lang/rust/pull/93024)
* [fix ICE when using `Box<T, A>` with pointer sized A](https://github.com/rust-lang/rust/pull/94043)
* [fix ScalarInt to char conversion](https://github.com/rust-lang/rust/pull/94086)
* [make `[u8]::cmp` implementation branchless](https://github.com/rust-lang/rust/pull/93962)
* [add a `try_collect()` helper method to `Iterator`](https://github.com/rust-lang/rust/pull/94041)
* [add basic platform support to `library/`{`panic_`}`unwind` for m68k](https://github.com/rust-lang/rust/pull/93949)
* [add a stack-`pin!`-ning macro to `core::pin`](https://github.com/rust-lang/rust/pull/93176)
* [stabilize pin_static_ref](https://github.com/rust-lang/rust/pull/93580)
* [core: implement ASCII trim functions on byte slices](https://github.com/rust-lang/rust/pull/93686)
* [asm: allow the use of r8-r14 as clobbers on Thumb1](https://github.com/rust-lang/rust/pull/93877)
* [futures: fix `Sync` impl of `BiLockGuard`](https://github.com/rust-lang/futures-rs/pull/2570)
* [`cargo new` should not add ignore rule on Cargo.lock inside subdirs](https://github.com/rust-lang/cargo/pull/10379)
* [rustdoc: buffer JSON output](https://github.com/rust-lang/rust/pull/93954)
* [rustdoc: avoid duplicating macros in sidebar](https://github.com/rust-lang/rust/pull/94002)
* [rustdoc: collect traits in scope for lang items](https://github.com/rust-lang/rust/pull/93766)
* [rustdoc: resolve intra-doc links when checking HTML](https://github.com/rust-lang/rust/pull/93605)
* [rustdoc: add `--scrape-tests` option to scrape functions marked `#[test]`](https://github.com/rust-lang/rust/pull/93497)
* [clippy: move `transmute_undefined_repr` back to nursery](https://github.com/rust-lang/rust/pull/94014)
* [clippy: don't lint Default::default if it is the udpate syntax base](https://github.com/rust-lang/rust-clippy/pull/8433)
* [clippy: don't lint `match` expressions with `cfg`ed arms](https://github.com/rust-lang/rust-clippy/pull/8443)
* [clippy: don't lint `needless_borrow` in method receiver positions](https://github.com/rust-lang/rust-clippy/pull/8441)
* [clippy: fix `await_holding_lock` not linting `parking_lot` Mutex/RwLock](https://github.com/rust-lang/rust-clippy/pull/8419)
* [clippy: improve `redundant_slicing` lint](https://github.com/rust-lang/rust-clippy/pull/8218)
* [clippy: lint enum-to-int casts with `cast_possible_truncation`](https://github.com/rust-lang/rust-clippy/pull/8381)
* [clippy: some more fixes for `transmute_undefined_repr`](https://github.com/rust-lang/rust-clippy/pull/8440)
* [clippy: new lint: `recursive_format_impl`](https://github.com/rust-lang/rust-clippy/pull/8188)
* [clippy: trigger  `ptr_as_ptr` inside macros](https://github.com/rust-lang/rust-clippy/pull/8442)

### Rust Compiler Performance Triage

Only one outright regression this week. We had some very cool work from cjgillot
to prevent queries from doing expensive clones, by just forcing them all to be
`Copy`! Also, nnethercote's overhaul of interning yielded massive improvements
across many crates. Also, a slew of benchmarks were unexpectedly improved
[quite a lot][] by some changes to way we invoke the linker when building LLVM itself.

[quite a lot]: https://perf.rust-lang.org/compare.html?start=8d163e66211c529465868a22686f46c5956342a4&end=6655109f58b7d0f4cae7e04eab476e389c9b9a0f

Triage done by **@pnkfelix**.
Revision range: [775e4807..a240ccd8](https://perf.rust-lang.org/?start=775e480722c7aba6ff4ff3ccec8c1f4639ae7889&end=a240ccd81c74c105b6f5fe84c46f8d36edb7e306&absolute=false&stat=instructions%3Au)

2 Regressions, 2 Improvements, 4 Mixed; 0 of them in rollups
47 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-02-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [RFC: Add a `process_group` method to UNIX `CommandExt`](https://github.com/rust-lang/rfcs/pull/3228)
* [disposition: postpone] [Let Cargo put data into platform-specific directories](https://github.com/rust-lang/rfcs/pull/1615)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Tracking Issue for JoinHandle::is_running](https://github.com/rust-lang/rust/issues/90470)
* [disposition: merge] [Make regular stdio lock() return 'static handles](https://github.com/rust-lang/rust/pull/93965)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: add try_all and try_any to Iterator](https://github.com/rust-lang/rfcs/pull/3233)

## Upcoming Events

Rusty Events between 2/23/2022 - 3/23/2022 🦀

### Online

* [February 23, 2022 | Graz, AT | **Async Programming with Tokio** | Rust Graz Meetup](https://www.meetup.com/Graz-Rust-Meetup/events/284131759)
* [February 23, 2022 | México City, MX | **Platica Febrero 2022** | Rust MX](https://www.meetup.com/Rust-MX/events/283662630)
* [February 24, 2022 | Linz, AT | **Rust Meetup Linz - 19th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/283377693/)
* [March 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283638736)
* [March 2, 2022 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283633083/)
* [March 2, 2022 | Indianapolis, IN, US | **Zach Mitchell - Optimizing a Physics Simulation from 8 hours to 10 minutes** | Indy Rust](https://www.meetup.com/indyrs/events/283899260)
* [March 3, 2022 | Würzburg, DE | **Guest Speaker | Herbert Wolverson | Rust gamedev in 2022** | Rust Würzburg Meetup Group](https://www.meetup.com/rust-wurzburg-meetup-group/events/283765814)
* [March 7, 2022 | Valence, FR | **Coding-dojo - Rust** | Ardèch’Drôm Dev](https://www.meetup.com/Ardech-Drom-Dev/events/283624590)
* [March 8, 2022 | Dallas, TX, US | **Second Tuesday**| Dallas Rust](https://www.meetup.com/Dallas-Rust/events/284064891/)
* [March 8, 2022 | Rostock, DE | **5. Rust Meetup Rostock** | Altow Academy](https://www.meetup.com/altow-academy/events/283819113)
* [March 8, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/283221922/)
* [March 9, 2022 | Boulder, CO, US | **Monthly Meetup** | Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/events/283985719/)
* [March 9, 2022 | München, DE | **Rust Munich Remote(?) #10** | Rust Munich](https://www.meetup.com/rust-munich/events/283790509)
* [March 9, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/284068315)
* [March 15, 2022 | Dublin, IE | **Rust Dublin March Meetup** - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/283613905)
* [March 15, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/283374540/)
* [March 16, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out Night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/283910183/)

### Europe

* [February 28, 2022 | London, UK | **Rust London Code Dojo: Rust with Front-End Web Assembly** | Rust London User Group](https://www.meetup.com/Rust-London-User-Group/events/283852309/)

### North America

* [March 14, 2022 | Atlanta, GA, US | **_New_ Atlanta Rust Meetup** | Atlanta Rustaceans](https://twitter.com/atl_rustaceans/status/1489586471367589893)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> There's a big difference between solving a problem and making a problem go away

– [Patrick Doyle on rust-users](https://users.rust-lang.org/t/what-does-impl-a-myserver-static-mean/71755)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1184) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
