Title: This Week in Rust 476
Number: 476
Date: 2023-01-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation
* [A Q4 Recap & 2022 Reflection from Rebecca Rumbul](https://foundation.rust-lang.org/news/a-q4-recap-2022-reflection-from-rebecca-rumbul-rust-foundation-executive-director-ceo/)

### Project/Tooling Updates
* [rust-analyzer changelog #162](https://rust-analyzer.github.io/thisweek/2023/01/02/changelog-162.html)
* [fltk-rs in 2022](https://moalyousef.github.io/blog/Fltkrs2022.html)
* [shuttle - Release v0.8.0](https://github.com/shuttle-hq/shuttle/releases/tag/v0.8.0)
* [This Week in Fyrox](https://fyrox.rs/blog/post/twif9/)
* [gitoxide - The year in retrospective, and what's to come](https://github.com/Byron/gitoxide/discussions/681)
* [The AeroRust community - 3 years birthday (and the roadmap for 2023)](https://aerorust.org/blog/aerorust-3-years-birthday)
* [SeaQuery 0.28.0 - A dynamic query builder for SeaORM](https://www.sea-ql.org/blog/2022-12-30-whats-new-in-seaquery-0.28.0/)
* [Databend 2022 Recap](https://databend.rs/blog/2022-12-31-databend-2022-recap)

### Observations/Thoughts
* [State Machines III: Type States](https://blog.yoshuawuyts.com/state-machines-3/)
* [Rust — vim — code completion](https://medium.com/rust-sections/rust-vim-code-completion-b6a36a177340)
* [How to Test](https://matklad.github.io/2021/05/31/how-to-test.html)
* [Embedded Rust and Embassy: DMA Controllers](https://apollolabsblog.hashnode.dev/embedded-rust-and-embassy-dma-controllers)
* [Parsing TFTP in Rust](https://tuckersiemens.com/posts/parsing-tftp-in-rust/)
* [Rustdoc JSON in 2022](https://adotinthevoid.github.io/posts/2022/12/rustdoc-json-in-2022/)
* [From PHP to Rust: Migrating a REST API between these two languages. (Part I)](https://blog.equationlabs.io/from-php-to-rust-migrating-a-rest-api-between-these-two-languages-part-i)
* [React + Rust + Wasm: Play an Animated 3D Model](https://guptanikhil.medium.com/react-rust-wasm-play-an-animated-3d-model-ab0dc3fbb903)
* [Open Source Grindset Explained (with a Rust example)](https://blog.orhun.dev/open-source-grindset)

### Rust Walkthroughs
* [Building a Simple DB in Rust - Part 1](https://johns.codes/blog/build-a-db/part01)
* [Microservices with Rust and WASM using Fermyon](https://medium.com/@shyamsundarb/microservices-with-rust-and-wasm-using-fermyon-30245673cdbb)
* [Compiling Brainfuck code - Part 4: A Static Compiler](https://rodrigodd.github.io/2022/12/31/bf_compiler-part4.html)
* [Rusty Circuit Breaker 🦀](https://blog.softwheel.io/how-to-implement-circuit-breaker-in-rust/)
* [Zero-dependency random number generation in Rust](https://blog.orhun.dev/zero-deps-random-in-rust)

### Miscellaneous
* [Rust 101: an open-source university course](https://tweedegolf.nl/en/blog/80/rust-101-open-source-university-course)
* [video] [If Rust Compiles, It WORKS (Testing not needed 📚)](https://www.youtube.com/watch?v=JIvKgSyvtxI)
* [video] [Introduction to Axum](https://www.youtube.com/playlist?list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS)
* [video] [Ergonomic APIs for hard problems - Raph Levien](https://www.youtube.com/watch?v=Phk0C-kLlho)

## Crate of the Week

This week's crate is [Sniffnet](https://github.com/GyulyVGC/sniffnet), a cross-platform GUI application to analyze your network traffic.

Thanks to [Gyuly Vgc](https://users.rust-lang.org/t/crate-of-the-week/2704/1143) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

*No calls for participation this week. Keep an eye out for more places to contribute next week!*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

291 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-12-26..2023-01-02

* [CFI: monomorphize transparent ADTs before typeid](https://github.com/rust-lang/rust/pull/106232)
* [account for `match` expr in single line](https://github.com/rust-lang/rust/pull/105347)
* [account for macros in const generics](https://github.com/rust-lang/rust/pull/105515)
* [account for multiple multiline spans with empty padding](https://github.com/rust-lang/rust/pull/106190)
* [adjust message on non-unwinding panic](https://github.com/rust-lang/rust/pull/105998)
* [allow trait method paths to satisfy `const Fn` bounds](https://github.com/rust-lang/rust/pull/106210)
* [always suggest as `MachineApplicable` in `recover_intersection_pat`](https://github.com/rust-lang/rust/pull/106066)
* [detect diff markers in the parser](https://github.com/rust-lang/rust/pull/106242)
* [detect when method call on LHS might be shadowed](https://github.com/rust-lang/rust/pull/106150)
* [dont use `--merge-base` during bootstrap formatting subcommand](https://github.com/rust-lang/rust/pull/106310)
* [emit fewer errors on invalid `#[repr(transparent)]` on `enum`](https://github.com/rust-lang/rust/pull/106201)
* [encode spans relative to the enclosing item -- enable on nightly](https://github.com/rust-lang/rust/pull/84762)
* [error parsing lifetime following by Sized and message + between them](https://github.com/rust-lang/rust/pull/103020)
* [fix confusing diagnostic when attempting to implementing trait for tuple](https://github.com/rust-lang/rust/pull/106358)
* [format only modified files](https://github.com/rust-lang/rust/pull/105702)
* [on unsized locals with explicit types suggest `&`](https://github.com/rust-lang/rust/pull/106223)
* [only deduplicate stack traces for good path bugs](https://github.com/rust-lang/rust/pull/106317)
* [give the correct track-caller location with MIR inlining](https://github.com/rust-lang/rust/pull/106139)
* [implement allow-by-default `multiple_supertrait_upcastable` lint](https://github.com/rust-lang/rust/pull/105484)
* [improve heuristics whether `format_args` string is a source literal](https://github.com/rust-lang/rust/pull/106195)
* [make trait/impl `where` clause mismatch on region error a bit more actionable](https://github.com/rust-lang/rust/pull/106208)
* [merge multiple mutable borrows of immutable binding errors](https://github.com/rust-lang/rust/pull/106284)
* [partially fix `explicit_outlives_requirements` lint in macros](https://github.com/rust-lang/rust/pull/106064)
* [properly calculate best failure in macro matching](https://github.com/rust-lang/rust/pull/105570)
* [provide a better error and a suggestion for `Fn` traits with lifetime params](https://github.com/rust-lang/rust/pull/104531)
* [provide local extern function arg names](https://github.com/rust-lang/rust/pull/105965)
* [recover `fn` keyword as `Fn` trait in bounds](https://github.com/rust-lang/rust/pull/106176)
* [remove unreasonable help message for auto trait](https://github.com/rust-lang/rust/pull/105817)
* [silence knock-down errors on `[type error]` bindings](https://github.com/rust-lang/rust/pull/106199)
* [suggest `Pin::as_mut` when encountering borrow error](https://github.com/rust-lang/rust/pull/106095)
* [suggest `impl Iterator` when possible for `_` return type](https://github.com/rust-lang/rust/pull/106172)
* [suggest rewriting a malformed hex literal if we expect a float](https://github.com/rust-lang/rust/pull/105852)
* [suppress errors due to TypeError not coercing with inference variables](https://github.com/rust-lang/rust/pull/106302)
* [trim more paths in obligation types](https://github.com/rust-lang/rust/pull/106202)
* [miri: cargo-miri: use rustc to determine the output filename](https://github.com/rust-lang/miri/pull/2741)
* [miri: handle unknown targets more gracefully](https://github.com/rust-lang/miri/pull/2742)
* [miri: simplify path joining code a bit](https://github.com/rust-lang/miri/pull/2743)
* [miri: support using a JSON target file](https://github.com/rust-lang/miri/pull/2744)
* [miri: tweaks to retag diagnostic handling](https://github.com/rust-lang/miri/pull/2746)
* [use some more `const_eval_select` in pointer methods for compile times](https://github.com/rust-lang/rust/pull/106275)
* [more inference-friendly API for lazy](https://github.com/rust-lang/rust/pull/103718)
* [more verbose `Debug` implementation of `std::process:Command`](https://github.com/rust-lang/rust/pull/97176)
* [add `#[inline]` markers to `once_cell` methods](https://github.com/rust-lang/rust/pull/105651)
* [unify id-based thread parking implementations](https://github.com/rust-lang/rust/pull/105903)
* [`available_parallelism: `gracefully handle zero value `cfs_period_us`](https://github.com/rust-lang/rust/pull/104493)
* [catch panics/unwinding in destruction of thread-locals](https://github.com/rust-lang/rust/pull/105426)
* [cargo: asymmetric tokens](https://github.com/rust-lang/cargo/pull/10771)
* [cargo: reasons for rebuilding](https://github.com/rust-lang/cargo/pull/11407)
* [clippy: fix false negative in `needless_return`](https://github.com/rust-lang/rust-clippy/pull/10110)
* [clippy: fix `match_single_binding` suggestion introducing an extra semicolon](https://github.com/rust-lang/rust-clippy/pull/10060)
* [clippy: move `mutex_atomic` to `restriction`](https://github.com/rust-lang/rust-clippy/pull/10115)
* [rust-analyzer: derive `Hash`](https://github.com/rust-lang/rust-analyzer/pull/13861)
* [rust-analyzer: enum variant discriminants hints](https://github.com/rust-lang/rust-analyzer/pull/13832)
* [rust-analyzer: diagnose private assoc item accesses](https://github.com/rust-lang/rust-analyzer/pull/13875)
* [rust-analyzer: diagnose private field accesses](https://github.com/rust-lang/rust-analyzer/pull/13870)
* [rust-analyzer: implement yeeting](https://github.com/rust-lang/rust-analyzer/pull/13857)
* [rust-analyzer: fall back to inaccessible associated functions and constants if no visible resolutions are found](https://github.com/rust-lang/rust-analyzer/pull/13867)
* [rust-analyzer: improve exit point highlighting for `for` and `while` loops in tail position](https://github.com/rust-lang/rust-analyzer/pull/13869)
* [rust-analyzer: merge multiple intersecting ranges](https://github.com/rust-lang/rust-analyzer/pull/13871)
* [rust-analyzer: prefix prelude items whose name collides in current scope](https://github.com/rust-lang/rust-analyzer/pull/13877)
* [rust-analyzer: type check unstable `try{}` blocks](https://github.com/rust-lang/rust-analyzer/pull/13856)
* [rust-analyzer: support multi-character punct tokens in MBE](https://github.com/rust-lang/rust-analyzer/pull/13854)
* [rust-analyzer: write down adjustments introduced by binary operators](https://github.com/rust-lang/rust-analyzer/pull/13882)

### Rust Compiler Performance Triage

Fairly busy week with some massive performance improvements at the expense of some significant albeit smaller regressions. The main wins came in a long-standing PR from @cjgillot to enable encoding spans in metadata relative to their enclosing item. This causes more work in full compilation which causes some regressions up to 5% but can lead to very large wins in incremental compilation scenarios (up to ~70%). For example, the clap crate compiles 68% faster after a small 1 line change than it did previously.

Triage done by **@rylev**.
Revision range: [b38a6d..b43596](https://perf.rust-lang.org/?start=b38a6d373cb254697411147c0e49cd2e84864258&end=b435960c4cfd3975651c7051be56d7f5d6c201ab&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.6%  | [0.3%, 4.6%]    | 97    |
| Regressions ❌ <br /> (secondary)  | 1.8%  | [0.2%, 7.6%]    | 60    |
| Improvements ✅ <br /> (primary)   | -9.7% | [-68.7%, -0.2%] | 53    |
| Improvements ✅ <br /> (secondary) | -1.7% | [-15.3%, -0.1%] | 62    |
| All ❌✅ (primary)                 | -2.4% | [-68.7%, 4.6%]  | 150   |

1 Regressions, 1 Improvements, 4 Mixed; 1 of them in rollups
47 artifact comparisons made in total


[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-01-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Only include stable lints in `rustdoc::all` group](https://github.com/rust-lang/rust/pull/106316)
* [disposition: merge] [Don't derive Debug for `OnceWith` & `RepeatWith`](https://github.com/rust-lang/rust/pull/104163)
* [disposition: merge] [PhantomData layout guarantees](https://github.com/rust-lang/rust/pull/104081)
* [disposition: merge] [Add O(1) `Vec -> VecDeque` conversion guarantee](https://github.com/rust-lang/rust/pull/105128)
* [disposition: merge] [Stabilize `::{core,std}::pin::pin!`](https://github.com/rust-lang/rust/pull/103800)
* [disposition: merge] [Stabilize `main_separator_str`](https://github.com/rust-lang/rust/pull/103104)
* [disposition: merge] [Loosen the bound on the Debug implementation of Weak.](https://github.com/rust-lang/rust/pull/90291)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-01-04 - 2023-02-01 🦀

### Virtual

* 2023-01-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfccbgb/)
* 2023-01-04 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsyfccbgb/)
* 2023-01-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Part 2: Exploring USB with Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/290122605/)
* 2023-01-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfccbnb/)
* 2023-01-11 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/) 
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/290277662/)
* 2023-01-12 | Virtual (San Francisco, CA, US; Stockholm, SE; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) | [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/)
    * [**Crack code interview problems in Rust - Ep. 1**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290071417/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/290071415/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290071420/)
* 2023-01-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfccbqb/)
* 2023-01-14 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://www.google.com/url?q=https%3A%2F%2Fdiscord.gg%2FyNtPTb2&sa=D&ust=1666661760000000&usg=AOvVaw13uHY9m-8bJJwgeP58VS8l)
* 2023-01-16 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/)
    * [**Primeros pasos con Rust - Qué es y Configuración el entorno de desarrollo**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224512/) | [**São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224516/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224515/)
* 2023-01-17 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581080/)
* 2023-01-17 | Virtual (San Francisco, CA, US; São Paulo, BR, New York, NY, US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust - Creación del primer programa de Rust**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224517/) | [***São Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224521/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
* 2023-01-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/289015967/)
* 2023-01-18 | Virtual (San Francisco, CA, US; São Paulo, BR; New York, NY US) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/) and [Microsoft Reactor São Paulo](https://www.meetup.com/microsoft-reactor-sao-paulo/) and [Microsoft Reactor New York](https://www.meetup.com/microsoft-reactor-new-york/events/290224518/)
    * [**Primeros pasos con Rust: QA y horas de comunidad**](https://www.meetup.com/microsoft-reactor-san-francisco/events/290224523/) | [**Sao Paulo Mirror**](https://www.meetup.com/microsoft-reactor-sao-paulo/events/290224522/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/290224525/)
* 2023-01-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/tqvhxsyfccbxb/)
* 2023-01-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust Lightning Talks!**](https://www.meetup.com/charlottesville-rust-meetup/events/290122935/)
* 2023-01-31 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfccbpc/)
* 2023-02-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfcdbcb/)

### Asia

* 2023-01-15 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Property-Based Testing in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/290667325/)

### Europe

* 2023-01-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)
* 2023-01-20 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfccbbc/)
* 2023-01-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #55**](https://www.meetup.com/rust-paris/events/290100223/)


### North America

* 2023-01-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Lightning Talks 'n' Chill (a.k.a. Show & Tell), with Pizza!**](https://www.meetup.com/utah-rust/events/dsbpxsydcqbdc/)
* 2023-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Happy Hour and Beginner Embedded Rust Hacking Session (#2!)**](https://www.meetup.com/minneapolis-rust-meetup/events/289768841/)
* 2023-01-11 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/290597764/)
* 2023-01-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/rwvwzsyfccbwb/)
* 2023-01-26 | Copenhagen, DK | [Copenhagen Rust group](https://www.meetup.com/copenhagen-rust-meetup-group/)
    * [**Rust Hack Night #32**](https://www.meetup.com/copenhagen-rust-meetup-group/events/290037532/)
* 2023-01-26 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Building a Rust Playground with WASM and Lane and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsyfccbjc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/zpd1qo/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> You haven’t “fooled” rustc, you are using unsafe code. Unsafe code means that all you can do is fool yourself.

– [Frank Steffahn on rust-users](https://users.rust-lang.org/t/aint-it-funny/86661/3)

Thanks to [Quine Dot](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1348) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/103cr02/this_week_in_rust_476/)</small>
