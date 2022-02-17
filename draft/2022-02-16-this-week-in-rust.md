Title: This Week in Rust 430
Number: 430
Date: 2022-02-16
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
* [Rust Survey 2021 Results](https://blog.rust-lang.org/2022/02/15/Rust-Survey-2021.html)
* [CTCFT 2022-02-21 Agenda](https://blog.rust-lang.org/inside-rust/2022/02/11/CTCFT-february.html)
* [Crates.io Index Snapshot Branches Moving](https://blog.rust-lang.org/2022/02/14/crates-io-snapshot-branches.html)

### Foundation

### Project/Tooling Updates
* [This week in Fluvio #22: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0022/)
* [rsadsb: v0.5.0 Release](https://rsadsb.github.io/v0.5.0.html)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W06](https://www.fornjot.app/blog/weekly-dev-log/2022-w06/)
* [Rust Analyzer Changelog #116](https://rust-analyzer.github.io/thisweek/2022/02/14/changelog-116.html)
* [GCC Rust Weekly Status Report 39](https://thephilbert.io/2022/02/14/gcc-rust-weekly-status-report-39/)
* [cargo-mutants 0.2.0: A new mutation testing tool for Rust](https://github.com/sourcefrog/cargo-mutants#readme)
* [SixtyFPS becomes Slint](https://slint-ui.com/blog/sixtyfps-becomes-slint.html)
* [cargo binstall 0.6: with QuickInstall support](https://github.com/ryankurte/cargo-binstall/releases/tag/v0.6.2)

### Newsletters
* [This Month in Rust GameDev #30 - January 2022](https://gamedev.rs/news/030/)

### Research
* [Learning and Programming Challenges of Rust: A Mixed-Methods Study](https://songlh.github.io/paper/survey.pdf)

### Observations/Thoughts
* [Mocking Time in Async Rust](https://www.ditto.live/blog/mocking-time-in-async-rust)
* [Plugins in Rust: Getting our Hands Dirty](https://nullderef.com/blog/plugin-impl/)
* [Async IO fundamentals](https://www.ncameron.org/blog/async-io-fundamentals/)
* [Async read and write traits](https://www.ncameron.org/blog/async-read-and-write-traits/)
* [Dare to ask for more #rust2024](http://smallcultfollowing.com/babysteps//blog/2022/02/09/dare-to-ask-for-more-rust2024/)
* [PostgreSQL Aggregates with Rust](https://hoverbear.org/blog/postgresql-aggregates-with-rust/)
* [My Rust Frontend Experiences](https://blog.urth.org/2022/02/08/my-rust-frontend-experiences/)
* [ZH] [为什么我们要Associated Type？Why do we need Associated Type?](https://fengliang.io/RustWHY/engineering_features/why_associated_type.html)

### Rust Walkthroughs
* [rsadsb: Raspberry Pi in-car ADS-B Display - with Rust!](https://rsadsb.github.io/rasp-pi-display.html)
* [Computing image filters with wgpu-rs](https://blog.redwarp.app/image-filters/)
* [Let’s Build a WebSockets Project With Rust and Yew 0.19](https://fsjohnny.medium.com/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f)
* [How to build a custom blockchain implementation in Rust using Substrate](https://blog.logrocket.com/how-to-build-a-custom-blockchain-implementation-in-rust-using-substrate/)
* [Rust on iOS and Mac Catalyst: A Simple, Updated Guide](https://nadim.computer/posts/2022-02-11-maccatalyst.html)
* [What Is Rust's Hole Purpose?](http://blog.pnkfx.org/blog/2022/02/09/what-is-rusts-hole-purpose/)
* [A Rust match made in hell](https://fasterthanli.me/articles/a-rust-match-made-in-hell)
* [Futures Concurrency III](https://blog.yoshuawuyts.com/futures-concurrency-3/)
* [More Enum Types](https://blog.yoshuawuyts.com/more-enum-types/)
* [Frontend Rust Without Node](https://blog.urth.org/2022/02/14/frontend-rust-without-node/)
* [Implementing a custom async runtime in a game engine](https://domwillia.ms/devlog7)
* [Video] [Implementing the NTFS filesystem in Rust](https://fosdem.org/2022/schedule/event/misc_ntfs_rust/)
* [video] [Rust Project: Deserialization with Serde](https://www.youtube.com/watch?v=A38SoLTB1Y4)
* [video] [5 Tips for Rust Beginners](https://www.youtube.com/watch?v=zdT3bUljGQw)


* [Assertion reporting](https://hole.tuziwo.info/assertion-reporting.html)

* [video] [Developing Embedded Applications in Rust](https://www.youtube.com/watch?v=pgG95y0hBDc)

### Miscellaneous
* [Rust core team alumni](https://brson.github.io/2022/02/09/rust-core-team-alumni)
* [Crane Support for Alternative Registries and Git Dependencies](https://ipetkov.dev/blog/crane-support-for-alt-registries-and-git-deps/)
* [Sustainability with Rust](https://aws.amazon.com/blogs/opensource/sustainability-with-rust/)
* [Rotating log files and reassembling them for inspection](https://www.openanalytics.eu/blog/2022/02/15/log-files-in-rust/)
* [DE] [Grafische User-Interfaces: SixtyFPS-Toolkit aus Berlin als Slint überarbeitet](https://www.heise.de/news/UI-Toolkit-aus-Berlin-SixtyFPS-heisst-nun-Slint-6443665.html)

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

A week with a number of correctness-related regressions, and a few small
cleanups yielding good performance results. Overall an improvement, particularly
for incremental benchmarks.

Triage done by **@simulacrum**.
Revision range: [1ea48517..775e480](https://perf.rust-lang.org/?start=1ea4851715893ee3f365a8ef09d47165e9a7864f&end=775e480722c7aba6ff4ff3ccec8c1f4639ae7889&absolute=false&stat=instructions%3Au)

4 Regressions, 2 Improvements, 5 Mixed; 1 of them in rollups
27 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-02-08.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered final comment period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Remove the everybody loops pass](https://github.com/rust-lang/rust/pull/93913)
* [disposition: merge] [Stabilize `#[cfg(panic = "...")]`](https://github.com/rust-lang/rust/pull/93658)
* [disposition: merge] [Tracking Issue for bool_to_option](https://github.com/rust-lang/rust/issues/80967)
* [disposition: merge] [Check if enum from foreign crate has any non exhaustive variants when attempting a cast](https://github.com/rust-lang/rust/pull/92744)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [add raw-code-data](https://github.com/rust-lang/rfcs/pull/3232)
* [new] [Cargo authenticateing users without sending secrets over the network](https://github.com/rust-lang/rfcs/pull/3231)

## Upcoming Events

Rusty Events between 2/16/2022 - 3/16/2022 🦀

### Online

* [February 16, 2022 | Vancouver, BC, CA | **Rust Study/Hack/Hang-out Night** | Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/283260386/)
* [February 17, 2022 | Charlottesville, VA, US | **Live coding talk: Implementing parser combinators in Rust** | Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/events/283819886/)
* [February 17, 2022 | München, DE | **Rust - beyond "Hello World"**| Agile Softwareentwicklung München](https://www.meetup.com/maibornwolff-software-engineering-netzwerk/events/283379985)
* [February 17, 2022 | Nürnberg, DE | **Rust Nürnberg online #10**| Rust Nuremberg](https://www.meetup.com/rust-noris/events/283545751/)
* [February 17, 2022 | Stuttgart, DE | **Rust-Meetup** | Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/events/282545308)
* [February 17, 2022 | Würzburg, DE | **Meet and chat about Rust** | Rust Würzburg Meetup Group](https://www.meetup.com/rust-wurzburg-meetup-group/events/283609518)
* [February 18, 2022 | Tehran, IR | **Rust Iran Meetup #5 - Type Driven Design** | Rust Iran Meetup](https://rust-meetup.ir/2022/02/18/fifth-meetup.html)
* [February 20, 2022 | Kyiv, UA | **Ukrainian Rusty Dinner** | Rust Ukraine](https://dou.ua/calendar/40897/)
* [February 22, 2022 | Dallas, TX, US | **Last Tuesday Meetup** | Dallas Rust](https://www.meetup.com/Dallas-Rust/events/283669162/)
* [February 22, 2022 | Dublin, IE | **Rust Dublin February Meetup** - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/283613610)
* [February 23, 2022 | México City, MX | **Platica Febrero 2022** | Rust MX](https://www.meetup.com/Rust-MX/events/283662630)
* [February 24, 2022 | Linz, AT | **Rust Meetup Linz - 19th Edition** | Rust Linz](https://www.meetup.com/Rust-Linz/events/283377693/)
* [March 1, 2022 | Buffalo, NY, US | **First Tuesdays: Buffalo Rust User Group** | Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/events/283638736)
* [March 2, 2022 | Berlin, DE | **Rust Hack and Learn** | OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/events/283633083/)
* [March 3, 2022 | Würzburg, DE | **Guest Speaker | Herbert Wolverson | Rust gamedev in 2022** | Rust Würzburg Meetup Group](https://www.meetup.com/rust-wurzburg-meetup-group/events/283765814)
* [March 7, 2022 | Valence, FR | **Coding-dojo - Rust** | Ardèch’Drôm Dev](https://www.meetup.com/Ardech-Drom-Dev/events/283624590)
* [March 8, 2022 | Rostock, DE | **5. Rust Meetup Rostock** | Altow Academy](https://www.meetup.com/altow-academy/events/283819113)
* [March 8, 2022 | Seattle, WA, US | **Monthly meetup** | Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/283221922/)
* [March 9, 2022 | München, DE | **Rust Munich Remote(?) #10** | Rust Munich](https://www.meetup.com/rust-munich/events/283790509)
* [March 15, 2022 | Dublin, IE | **Rust Dublin March Meetup** - Rust Dublin](https://www.meetup.com/Rust-Dublin/events/283613905)
* [March 15, 2022 | Washington, DC, US| **Mid-month Rustful** | Rust DC](https://www.meetup.com/RustDC/events/283374540/)

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

**Kollider**

* [Senior Frontend Engineer - Rust (Remote)](https://careers.kollider.xyz/senior-frontend-engineer/en)
* [Junior Backend Engineer - Rust (Remote)](https://careers.kollider.xyz/junior-backend-engineer/en)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

**LoanPASS**

* [Full Stack Engineer, Rust + Typescript (Remote US)](https://loanpass.io/careerPage.html)

# Quote of the Week

> I still get excited about programming languages. But these days, it's not so much because of what they let me do, but rather what they don't let me do.

– [Amos blogging about mistakes Rust doesn't catch](https://fasterthanli.me/articles/some-mistakes-rust-doesnt-catch)

Thanks to [Rob Donnelly](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1181) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
