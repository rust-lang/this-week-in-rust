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

* [video] [Talk on Pattern-defeating Quicksort, the algorithm behind `sort_unstable`](https://www.youtube.com/watch?v=jz-PBiWwNjc) 

### Observations/Thoughts

* [video] [Rust's Vision in 2022](https://www.youtube.com/watch?v=zYrudh-dsX8)

### Rust Walkthroughs
* [Integrating Rust With Android Development](https://medium.com/@otukof/integrating-rust-with-android-development-ef341c2f9cca)

### Miscellaneous
* [Modern Telecom Network Tracing](https://oxio.com/blog/modern-telecom-network-tracing)

## Crate of the Week

This week's crate is [prae](https://github.com/teenjuna/prae), a crate with macros to define types with inbuilt invariants.

Thanks to [Alex](https://users.rust-lang.org/t/crate-of-the-week/2704/1033) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

319 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-02-21..2022-02-28

* [apply noundef attribute to all scalar types which do not permit raw init](https://github.com/rust-lang/rust/pull/94157)
* [apply noundef metadata to loads of types that do not permit raw init](https://github.com/rust-lang/rust/pull/94158)
* [suggest a float literal when dividing a floating-point type by `{integer}`](https://github.com/rust-lang/rust/pull/94078)
* [suggest adding `{ .. }` around more bad const generic exprs](https://github.com/rust-lang/rust/pull/92884)
* [suggest calling `.display()` on `PathBuf` too](https://github.com/rust-lang/rust/pull/94240)
* [do not suggest using a const parameter when there are bounds on an unused type parameter](https://github.com/rust-lang/rust/pull/93400)
* [do not suggest wrapping an item if it has ambiguous un-imported methods](https://github.com/rust-lang/rust/pull/94237)
* [diagnostic: suggest parens when users want logical ops, but get closures](https://github.com/rust-lang/rust/pull/94344)
* [better error if the user tries to do assignment ... `else`](https://github.com/rust-lang/rust/pull/94211)
* [rustc_errors: let `DiagnosticBuilder::emit` return a "guarantee of emission"](https://github.com/rust-lang/rust/pull/93368)
* [consider mutations as borrows in generator drop tracking](https://github.com/rust-lang/rust/pull/94068)
* [miri: prune backtraces similar to `RUST_BACKTRACE=1` logic](https://github.com/rust-lang/miri/pull/1977)
* [miri: prune stacktraces for tag-tracking diagnostics too](https://github.com/rust-lang/miri/pull/1987)
* [fix ICE when passing block to while-loop condition](https://github.com/rust-lang/rust/pull/94248)
* [fix ICE when using `Box<T, A>` with large A](https://github.com/rust-lang/rust/pull/94414)
* [convert `newtype_index` to a proc macro](https://github.com/rust-lang/rust/pull/93878)
* [gracefully handle non-UTF-8 string slices when pretty printing](https://github.com/rust-lang/rust/pull/94156)
* [improve string literal unescaping](https://github.com/rust-lang/rust/pull/94316)
* [introduce `ChunkedBitSet` and use it for some dataflow analyses](https://github.com/rust-lang/rust/pull/93984)
* [simplify `rustc_serialize` by dropping support for decoding into JSON](https://github.com/rust-lang/rust/pull/93839)
* [only create a single expansion for each inline integration](https://github.com/rust-lang/rust/pull/94427)
* [remove in band lifetimes](https://github.com/rust-lang/rust/pull/93845)
* [codegen\_gcc: add support for `on_stack` parameters](https://github.com/rust-lang/rustc_codegen_gcc/pull/135)
* [codegen\_gcc: don't export global allocs which are not statics](https://github.com/rust-lang/rustc_codegen_gcc/pull/133)
* [codegen\_gcc: fix miscompilation when `cg_ssa` is using multiple builders at the same time](https://github.com/rust-lang/rustc_codegen_gcc/pull/131)
* [codegen\_gcc: support `-Cpanic=unwind` without unwinding](https://github.com/rust-lang/rustc_codegen_gcc/pull/132)
* [implement `LowerHex` on `Scalar` to clean up their display in rustdoc](https://github.com/rust-lang/rust/pull/94189)
* [add `slice::`{`from_ptr_range`, `from_mut_ptr_range`}](https://github.com/rust-lang/rust/pull/89793)
* [futures: `FuturesUnordered`: fix partial iteration](https://github.com/rust-lang/futures-rs/pull/2574)
* [portable-simd: bitmask conversion trait](https://github.com/rust-lang/portable-simd/pull/239)
* [cargo: implement "artifact dependencies"](https://github.com/rust-lang/cargo/pull/9992) (RFC [#3028](https://rust-lang.github.io/rfcs/3028-cargo-binary-dependencies.html))
* [cargo: add `-Z check-cfg-features` to enable compile-time checking of features](https://github.com/rust-lang/cargo/pull/10408)
* [cargo: add common profile validation](https://github.com/rust-lang/cargo/pull/10411)
* [cargo: enable propagating host rustflags to build scripts](https://github.com/rust-lang/cargo/pull/10395)
* [clippy: add `print_in_format_impl` lint](https://github.com/rust-lang/rust-clippy/pull/8253)
* [clippy: disable `new-without-default` for `#[doc(hidden)] new()` methods](https://github.com/rust-lang/rust-clippy/pull/8472)
* [clippy: false positive `redundant_closure` when using ref pattern in closure params](https://github.com/rust-lang/rust-clippy/pull/8466)
* [clippy: fix `ptr_arg`](https://github.com/rust-lang/rust-clippy/pull/8464)
* [clippy: fix some `unnecessary_filter_map` false positives](https://github.com/rust-lang/rust-clippy/pull/8479)
* [clippy: fix false positives of `large_enum_variant`](https://github.com/rust-lang/rust-clippy/pull/8453)

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
* [March 9, 2022 | Selangor, MY | **Rust Meetup** | Rust Malaysia Meetup](https://forms.gle/35pipPdsKm1VFzCa9)
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

> Due to recent events I feel the need to once again commend the reviewers and ehuss in particular for their amazing communication skills when reviewing PRs like this. I can only imagine how much work it means and how silly some of the changes proposed here might look to a seasoned cargo developer, yet you maintain a constructive, upbeat, and friendly spirit at all times. It's a style that I am aspiring when reviewing PRs myself, and is a prime example for the accessibility and friendliness of the Rust community as a whole.
>
> Thank you!

– [Sebastian Thiel commending Eric Huss on GitHub](https://github.com/rust-lang/cargo/pull/9992#issuecomment-1046606363)

Thanks to [Jacob Finkelman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1185) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
