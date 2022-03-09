Title: This Week in Rust 433
Number: 433
Date: 2022-03-09
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
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W09](https://www.fornjot.app/blog/weekly-dev-log/2022-w09/)

### Research

### Observations/Thoughts

### Rust Walkthroughs

* [Fuzzing unsafe code in a Rust crate](https://medium.com/@adetaylor/fuzzing-unsafe-code-in-a-rust-crate-dcf3ec04d79a)

### Miscellaneous

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

A relatively noisy week in performance measurements, particularly on the
`externs` incremental benchmark. Based on the timing of the first noise, this
seems to be due to [#93839], which makes me suspect this is related to PGO or
inlining decisions of some kind. [#94373] might help.

Overall a relatively unchanged to slightly good week, with no outright regressions and most
changes relatively small.

[#93839]: https://github.com/rust-lang/rust/pull/93839
[#94373]: https://github.com/rust-lang/rust/pull/94373

Triage done by **@simulacrum**.
Revision range: [1204400a..f0c4da4](https://perf.rust-lang.org/?start=1204400ab8da9830f6f77a5e40e7ad3ea459676a&end=f0c4da49983aa699f715caf681e3154b445fb60b&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-01.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Scoped threads in the standard library, take 2](https://github.com/rust-lang/rfcs/pull/3151)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Add vendor-specific suffixes to v0 mangling RFC 2603](https://github.com/rust-lang/rfcs/pull/3224)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Rename unix::net::SocketAddr::from_path to from_pathname and stabilize it](https://github.com/rust-lang/rust/pull/94356)
* [disposition: merge] [Tracking issue for Vec::retain_mut and VecDeque::retain_mut](https://github.com/rust-lang/rust/issues/90829)
* [disposition: merge] [Stabilize const_fn_fn_ptr_basics, const_fn_trait_bound, and const_impl_trait](https://github.com/rust-lang/rust/pull/93827)
* [disposition: merge] [Tracking Issue for const_intrinsic_copy](https://github.com/rust-lang/rust/issues/80697)
* [disposition: merge] [Implement `Write for Cursor<[u8; N]>`, plus `A: Allocator` cursor support](https://github.com/rust-lang/rust/pull/92663)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No new or updated RFCs were submitted this week.*

## Upcoming Events

Rusty Events between 2022-03-02 - 2022-03-30 🦀

### Virtual

* 2022-03-02 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/de-DE/opentechschool-berlin/events/283633083/)
* 2022-03-02 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Zach Mitchell - Optimizing a Physics Simulation from 8 hours to 10 minutes**](https://www.meetup.com/indyrs/events/283899260)
* 2022-03-02 | Vienna, AT | [Mob-Programming on Open Source Software](https://www.meetup.com/Mob-Programming-on-Open-Source-Software)
    * [**The Rustic Mob**](https://www.meetup.com/Mob-Programming-on-Open-Source-Software/events/284228300)
* 2022-03-03 | Cardiff, UK | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Book Study Session - Object Oriented Programming & Patterns and Matching**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/284196124/)
* 2022-03-03 | Würzburg, DE | [Rust Würzburg Meetup Group](https://www.meetup.com/rust-wurzburg-meetup-group/)
    * [**Guest Speaker | Herbert Wolverson | Rust gamedev in 2022**](https://www.meetup.com/rust-wurzburg-meetup-group/events/283765814)
* 2022-03-03 | [Scylla](https://www.scylladb.com/)
    * [**Build Low-Latency Applications in Rust on ScyllaDB**](https://lp.scylladb.com/rust-virtual-workshop-registration.html)
* 2022-03-07 | Valence, FR | [Ardèch’Drôm Dev](https://www.meetup.com/Ardech-Drom-Dev/)
    * [**Coding-dojo - Rust**](https://www.meetup.com/Ardech-Drom-Dev/events/283624590)
* 2022-03-08 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/284064891/)
* 2022-03-08 | Rostock, DE | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**5. Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/283819113)
* 2022-03-08 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/Rust-Saar/)
    * [**Meetup: 19u16**](https://www.meetup.com/Rust-Saar/events/284154283)
* 2022-03-08 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/283221922/)
* 2022-03-09 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/283985719/)
* 2022-03-09 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich Remote(?) #10**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-03-09 | Selangor, MY | [Rust Malaysia](https://t.me/golangmalaysia)
    * [**Rust Meetup**](https://forms.gle/35pipPdsKm1VFzCa9)
* 2022-03-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/284068315)
* 2022-03-10 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Bluetoothing with Rust using BlueR**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284152560/)
* 2022-03-15 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/de-DE/opentechschool-berlin/events/284205132/)
* 2022-03-15 | Dublin, IE | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Rust Dublin March Meetup**](https://www.meetup.com/Rust-Dublin/events/283613905)
* 2022-03-15 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/283374540/)
* 2022-03-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out Night**](https://www.meetup.com/Vancouver-Rust/events/283910183/)

### Europe

* 2022-03-15 | Sofia, BG | [Rust Meetup Sofia](https://www.meetup.com/rust-meetup-sofia/)
    * [**Rust Meetup Sofia - 1st Edition**](https://www.meetup.com/rust-meetup-sofia/events/284141594)

### North America

* 2022-03-14 | Atlanta, GA, US | [Atlanta Rustaceans](https://twitter.com/atl_rustaceans/)
    * [**_New_ Atlanta Rust Meetup**](https://twitter.com/atl_rustaceans/status/148958647136758989)
* 2022-03-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/284215958/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**LoanPASS**

* [Full Stack Engineer, Rust + Typescript (Remote US)](https://loanpass.io/careerPage.html)

** Quickwit **

* [Senior Software Engineer, Rust & distributed systems - Rust (Remote)](https://quickwit.io/jobs/distributed-software-engineer)
* [Software search engineering intern - Rust (Remote)](https://quickwit.io/jobs/oss-software-search-engineering-intern)

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

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
