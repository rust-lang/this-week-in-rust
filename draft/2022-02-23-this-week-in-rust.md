Title: This Week in Rust 431
Number: 431
Date: 2022-02-23
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

* [Zellij 0.25.0: floating panes, Tmux mode and more!](https://zellij.dev/news/floating-panes-tmux-mode/)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W07](https://www.fornjot.app/blog/weekly-dev-log/2022-w07/)
- [Hurl 1.6.0, a tool for running and testing HTTP requests with plain text](https://hurl.dev/blog/2022/02/11/announcing-hurl-1.6.0.html)
* [Slint (former SixtyFPS, UI crate) Project Update](https://slint-ui.com/thisweek/2022-02-21.html)
* [Type inference in `ogma` using graphs](https://www.kurtlawrence.info/blog/type-inference-in-ogma-using-graphs)

### Newsletters

### Research

### Observations/Thoughts

### Rust Walkthroughs
* [Compiler Adventures, part 2: Constant Propagation](https://medium.com/@predrag.gruevski/compiler-adventures-part-2-constant-propagation-c6f2e67d9881)

* [video] [Rust Project: Custom Deserialization with Serde](https://www.youtube.com/watch?v=5D1hAy3UhTY)

* [Checking Tailwind Class Names at Compile Time with Rust](https://blog.urth.org/2022/02/21/checking-tailwind-class-names-at-compile-time-with-rust/)

### Miscellaneous

## Crate of the Week

This week's crate is [assay](https://lib.rs/crates/assay), a test macro that puts each test in its own process and filesystem.

Thanks to [Harsh Shandilya](https://users.rust-lang.org/t/crate-of-the-week/2704/1023) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

321 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-02-07..2022-02-14

* [support custom options for LLVM build](https://github.com/rust-lang/rust/pull/93756)
* [store rlink data in opaque binary format on disk](https://github.com/rust-lang/rust/pull/93681)
* [fix incorrect register conflict detection in `asm!`](https://github.com/rust-lang/rust/pull/93868)
* [fix regression from lazy opaque types](https://github.com/rust-lang/rust/pull/93783)
* [make `span_extend_to_prev_str()` more robust](https://github.com/rust-lang/rust/pull/91607)
* [better suggestions when user tries to collect into an unsized `[_]`](https://github.com/rust-lang/rust/pull/91443)
* [do not suggest char literal for zero-length strings](https://github.com/rust-lang/rust/pull/92715)
* [improve opaque type higher-ranked region error message under NLL](https://github.com/rust-lang/rust/pull/92306)
* [point at type when a `static` `#[global_allocator]` doesn't `impl GlobalAlloc`](https://github.com/rust-lang/rust/pull/91950)
* [make `find_similar_impl_candidates` even fuzzier](https://github.com/rust-lang/rust/pull/93298)
* [implement `tainted_by_errors` in MIR borrowck, use it to skip CTFE](https://github.com/rust-lang/rust/pull/93691)
* [more informative error message for E0015](https://github.com/rust-lang/rust/pull/90532)
* [miri: implement `const_allocate` intrinsic](https://github.com/rust-lang/miri/pull/1973)
* [miri: implement `const_deallocate` as a no-op](https://github.com/rust-lang/miri/pull/1974)
* [stabilise `is_aarch64_feature_detected!` under `simd_aarch64` feature](https://github.com/rust-lang/rust/pull/90271)
* [stabilise `inherent_ascii_escape`](https://github.com/rust-lang/rust/pull/93886)
* [stabilize `cfg_target_has_atomic`](https://github.com/rust-lang/rust/pull/93824)
* [stabilize `int_abs_diff`](https://github.com/rust-lang/rust/pull/93735)
* [fix `HashMap` not displaying correctly in VS debugger](https://github.com/rust-lang/rust/pull/93626)
* [add `From<u8>` for `ExitCode`](https://github.com/rust-lang/rust/pull/93445)
* [add `str::`{`floor`, `ceil`}`_char_boundary` methods](https://github.com/rust-lang/rust/pull/86497)
* [`std::path::absolute`](https://github.com/rust-lang/rust/pull/91673)
* [implement `AsFd` for `&T` and `&mut T`](https://github.com/rust-lang/rust/pull/93888)
* [make `Instant::`{`duration_since`, `elapsed`, `sub`} saturating and remove workarounds](https://github.com/rust-lang/rust/pull/89926)
* [clippy: fix `transmute_undefined_repr` with single field `#[repr(C)]` structs](https://github.com/rust-lang/rust-clippy/pull/8425)
* [rustfmt: fix incorrect string indentation in macro defs with `format_strings`](https://github.com/rust-lang/rustfmt/pull/5201)
* [rustfmt: leverage itemized blocks to support formatting markdown block quotes](https://github.com/rust-lang/rustfmt/pull/5160)

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

**LoanPASS**

* [Full Stack Engineer, Rust + Typescript (Remote US)](https://loanpass.io/careerPage.html)
**Timescale**

* [Sr. Rust/C/C++ Engineer - Core Database (Remote)](https://www.timescale.com/careers/5920911002?gh_jid=5920911002)

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I still get excited about programming languages. But these days, it's not so much because of what they let me do, but rather what they don't let me do.

– [Amos blogging about mistakes Rust doesn't catch](https://fasterthanli.me/articles/some-mistakes-rust-doesnt-catch)

Thanks to [Rob Donnelly](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1181) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
