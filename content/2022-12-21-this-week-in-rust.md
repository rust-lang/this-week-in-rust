Title: This Week in Rust 474
Number: 474
Date: 2022-12-21
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

### Official
* [Announcing Rust 1.66.0](https://blog.rust-lang.org/2022/12/15/Rust-1.66.0.html)

### Newsletters
* [This Month in Rust GameDev #40 - November 2022](https://gamedev.rs/news/040/)
* [Announcing Rust Magazine](https://rustmagazine.github.io/announcing/)

### Project/Tooling Updates
* [IntelliJ Rust Changelog #185](https://intellij-rust.github.io/2022/12/19/changelog-185.html)
* [rust-analyzer changelog #160](https://rust-analyzer.github.io/thisweek/2022/12/19/changelog-160.html)
* [Cranelift Progress in 2022](https://bytecodealliance.org/articles/cranelift-progress-2022)
* [Native Reflection in Rust](https://jack.wrenn.fyi/blog/deflect/)
* [Announcing Diesel-Async 0.2.0](https://www.reddit.com/r/rust/comments/zn9ut0/announcing_dieselasync_020/)
* [Easily verify your Rust in CI with Kani and GitHub Actions](https://model-checking.github.io/kani-verifier-blog/2022/12/21/easily-verify-your-rust-in-ci-with-kani.html)
* [Fornjot (code-first CAD in Rust) - Weekly Release](https://www.fornjot.app/blog/weekly-release/2022-w51/)
* [Slint 0.3.3 Release and weekly update](https://slint-ui.com/thisweek/2022-12-19.html)

### Observations/Thoughts
* [My Year With Rust: The Good, The Bad, The Ugly | BreakBuildGames](https://breakbuild.dev/blog/my-year-with-rust/)
* [An adventure with optimization, Rust and Z3](https://ochagavia.nl/blog/an-adventure-with-optimization-rust-and-z3/)
* [Faster CI builds for Rust with pre-baked builder images and sccache](https://vadosware.io/post/faster-ci-builds-for-rust-with-builder-images-and-sccache/)
* [UI development in Rust](https://saona-raimundo.github.io/2022/12/21/UI-development-in-Rust.html)
* [Docs as Code: Mermaid inline diagrams](https://frehberg.com/2022/12/docs-as-code-mermaid-inline-diagrams/)
* [Rust needs `#[throws]`, not ubiquitous handwritten `Ok()`](https://diziet.dreamwidth.org/13657.html)
* [video] [Possibility of OCI Container Runtime with Rust at KubeDay Japan](https://youtu.be/hdF45WGzi7g)

### Rust Walkthroughs
* [How to build a Rust API with the builder pattern](https://blog.logrocket.com/build-rust-api-builder-pattern/)
* [Building a Command-Line Application in Rust](https://www.joshfinnie.com/blog/a-command-line-application-in-rust/)
* [Writing SQL User-Defined Functions in Rust](https://mariadb.org/writing-user-defined-functions-in-rust/)
* [New Rust course by Android: Comprehensive Rust 🦀](https://google.github.io/comprehensive-rust/)

### Miscellaneous
* [Our year in Rust - in a rhyme!](https://tweedegolf.nl/en/blog/81/our-year-in-rust)
* [Rust for the Polyglot Programmer (2022 edition) - a rather different introductory text, now revised](https://diziet.dreamwidth.org/13884.html)
* [audio] [Episode 108: Jane Losare-Lusby on Rust! (Part 3)](https://adspthepodcast.com/2022/12/16/Episode-108.html)
* [DE] [Programmiersprache Rust 1.66 erweitert Enumerations und entfernt Abhängigkeiten](https://www.heise.de/news/Programmiersprache-Rust-1-66-erweitert-Enumerations-und-entfernt-Abhaengigkeiten-7398888.html)
* [video] [Re-writing an Express.js chat app (web-sockets) in Rust](https://www.youtube.com/watch?v=-N8AKKCE9L8)
* [video] [Story time: Rust compiler errors](https://www.youtube.com/watch?v=Iv3UuGdB0TM)
* [video] [Let's write a TUI Password Manager in Rust - Part 2](https://www.youtube.com/watch?v=Y917bFaH_IU)
* [video] [Decrusting the serde crate](https://www.youtube.com/watch?v=BI_bHCGRgMY)

## Crate of the Week

This week's crate is [dhat](https://docs.rs/dhat), a crate providing DHAT-like allocation profiling capabilities.

Thanks to [Kornel](https://users.rust-lang.org/t/crate-of-the-week/2704/1139) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Add `clap` command to list available vaults](https://github.com/build-trust/ockam/issues/3935)
* [Ockam - Add optional --identity argument to clap commands that use CloudOpts](https://github.com/build-trust/ockam/issues/3904)
* [Ockam - Add optional --identity argument to clap command secure-channel-listener create and modify its handler](https://github.com/build-trust/ockam/issues/3907)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

368 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-12-12..2022-12-19

* [fix `-Z print-type-sizes` for generators with discriminant field ordered first](https://github.com/rust-lang/rust/pull/105623)
* [account for item-local in inner scope for E0425](https://github.com/rust-lang/rust/pull/104864)
* [add `0..=isize::MAX` range metadata to size loads from vtables](https://github.com/rust-lang/rust/pull/105446)
* [add batch flag to remote-test-server](https://github.com/rust-lang/rust/pull/105145)
* [address some `EarlyBinder` nits](https://github.com/rust-lang/rust/pull/103600)
* [adjust log line in `fuchsia-test-runner.py`](https://github.com/rust-lang/rust/pull/105663)
* [allow `impl ~const Trait` opaque types](https://github.com/rust-lang/rust/pull/105725)
* [allow unsafe through inline const](https://github.com/rust-lang/rust/pull/105147)
* [always check alignment during CTFE](https://github.com/rust-lang/rust/pull/104616)
* [always evaluate vecs of subdiagnostics eagerly](https://github.com/rust-lang/rust/pull/105233)
* [always use `anonymize_bound_vars`](https://github.com/rust-lang/rust/pull/105717)
* [auto traits in `dyn Trait + Auto` are suggestable](https://github.com/rust-lang/rust/pull/105627)
* [bail in `collect_trait_impl_trait_tys` if signatures reference errors](https://github.com/rust-lang/rust/pull/105711)
* [change pattern borrowing suggestions to be verbose and remove invalid suggestion](https://github.com/rust-lang/rust/pull/105476)
* [check AArch64 branch-protection earlier in the pipeline](https://github.com/rust-lang/rust/pull/105421)
* [combine `ty::Projection` and `ty::Opaque` into `ty::Alias`](https://github.com/rust-lang/rust/pull/104986)
* [custom MIR: Many more improvements](https://github.com/rust-lang/rust/pull/105356)
* [fast-path some binder relations](https://github.com/rust-lang/rust/pull/105350)
* [find the right lower bound region in the scenario of partial order relations](https://github.com/rust-lang/rust/pull/104765)
* [fix transmutes between pointers in different address spaces (e.g. fn ptrs on AVR)](https://github.com/rust-lang/rust/pull/105578)
* [guard ProjectionTy creation against passing the wrong number of substs](https://github.com/rust-lang/rust/pull/105657)
* [help rust-analyzer normalize query return types](https://github.com/rust-lang/rust/pull/105493)
* [highlight conflicting param-env candidates, again](https://github.com/rust-lang/rust/pull/105285)
* [illegal sized bounds: only suggest mutability change if needed](https://github.com/rust-lang/rust/pull/105491)
* [implement DerefMut for PathBuf](https://github.com/rust-lang/rust/pull/105018)
* [make some diagnostics not depend on the source of what they reference being available](https://github.com/rust-lang/rust/pull/105500)
* [normalize receiver substs and erase the regions](https://github.com/rust-lang/rust/pull/105561)
* [point at method chains on `E0271` errors](https://github.com/rust-lang/rust/pull/105674)
* [point out the type of associated types in every method call of iterator chains](https://github.com/rust-lang/rust/pull/105332)
* [print argument name in arg mismatch if possible](https://github.com/rust-lang/rust/pull/105842)
* [properly handle postfix inc/dec in standalone and subexpr scenarios](https://github.com/rust-lang/rust/pull/104875)
* [simpler diagnostic when passing arg to closure and missing borrow](https://github.com/rust-lang/rust/pull/102813)
* [stabilize `default_alloc_error_handler`](https://github.com/rust-lang/rust/pull/102318)
* [start improving monomorphization items stats](https://github.com/rust-lang/rust/pull/105481)
* [suggest `collect`ing into `Vec<_>`](https://github.com/rust-lang/rust/pull/105523)
* [suggest a `T: Send` bound for `&mut T` upvars in `Send` generators](https://github.com/rust-lang/rust/pull/105839)
* [suggest constraining type parameter with `Clone`](https://github.com/rust-lang/rust/pull/105679)
* [suggest dereferencing receiver arguments properly](https://github.com/rust-lang/rust/pull/105595)
* [support `#[track_caller]` on async closures](https://github.com/rust-lang/rust/pull/105464)
* [use a more efficient `Once` on platforms without threads](https://github.com/rust-lang/rust/pull/105698)
* [futures: impl FusedStream for Buffered](https://github.com/rust-lang/futures-rs/pull/2676)
* [cargo: artifact deps should works when target field specified coexists with `optional = true`](https://github.com/rust-lang/cargo/pull/11434)
* [cargo: show `--help` if there is no man page for subcommand](https://github.com/rust-lang/cargo/pull/11473)
* [cargo: stabilize terminal-width](https://github.com/rust-lang/cargo/pull/11494)
* [cargo: use workspace lockfile when running `cargo package` and `cargo publish`](https://github.com/rust-lang/cargo/pull/11477)
* [rustdoc: don't add "Read more" link if there is no extra content](https://github.com/rust-lang/rust/pull/105780)
* [clippy: fix `new_return_no_self` with recursive bounds](https://github.com/rust-lang/rust-clippy/pull/10086)
* [clippy: fix logic in `IncrementVisitor`](https://github.com/rust-lang/rust-clippy/pull/10094)
* [clippy: fix overflow ICE in `large_stack/const_arrays`](https://github.com/rust-lang/rust-clippy/pull/10103)
* [clippy: fix: not suggest `seek_to_start_instead_of_rewind` when expr is used](https://github.com/rust-lang/rust-clippy/pull/10096)
* [clippy: extend `useless_conversion` to identify more cases of useless `into_iter()` calls](https://github.com/rust-lang/rust-clippy/pull/10020)
* [clippy: `manual_is_ascii_check ` check](https://github.com/rust-lang/rust-clippy/pull/10053)
* [clippy: move `manual_clamp` to nursery](https://github.com/rust-lang/rust-clippy/pull/10101)
* [clippy: `null` fn lints](https://github.com/rust-lang/rust-clippy/pull/10099)
* [rust-analyzer: add a command to clear flycheck diagnostics](https://github.com/rust-lang/rust-analyzer/pull/13792)
* [rust-analyzer: add command for manually running flychecks](https://github.com/rust-lang/rust-analyzer/pull/13785)
* [rust-analyzer: add parentheses for binding mode hints when they attach to an Or-pattern](https://github.com/rust-lang/rust-analyzer/pull/13783)
* [rust-analyzer: deduplicate inserted parentheses in binding mode hints](https://github.com/rust-lang/rust-analyzer/pull/13784)
* [rust-analyzer: parse half-open `..= X` patterns](https://github.com/rust-lang/rust-analyzer/pull/13769)
* [rust-analyzer: fix wrong config patching logic for addCallParenthesis](https://github.com/rust-lang/rust-analyzer/pull/13766)
* [rust-analyzer: add a check for `if` token in patterns parser](https://github.com/rust-lang/rust-analyzer/pull/13777)
* [rust-analyzer: fix "parser seems stuck" panic when parsing colossal files](https://github.com/rust-lang/rust-analyzer/pull/13794)
* [rust-analyzer: resolve all inference vars in `InferenceResult::assoc_resolutions`](https://github.com/rust-lang/rust-analyzer/pull/13774)
* [rust-analyzer: use the correct edition when formatting code in path dependencies](https://github.com/rust-lang/rust-analyzer/pull/13795)

### Rust Compiler Performance Triage

Relatively quiet week for performance, with most regressions and improvements being pretty small.

Triage done by **@simulacrum**.
Revision range: [109ccc..8a746f4](https://perf.rust-lang.org/?start=109cccbe4f345c0f0785ce860788580c3e2a29f5&end=8a746f4ac3a489efb724cde813607f3b96c2df7b&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.9%  | [0.2%, 2.8%]   | 28    |
| Regressions ❌ <br /> (secondary)  | 0.7%  | [0.2%, 1.3%]   | 39    |
| Improvements ✅ <br /> (primary)   | -1.0% | [-1.1%, -1.0%] | 2     |
| Improvements ✅ <br /> (secondary) | -1.7% | [-4.3%, -0.2%] | 24    |
| All ❌✅ (primary)                 | 0.8%  | [-1.1%, 2.8%]  | 30    |


3 Regressions, 4 Improvements, 2 Mixed; 3 of them in rollups
53 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-12-20.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: `c"…"` string literals](https://github.com/rust-lang/rfcs/pull/3348)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Don't normalize in AstConv](https://github.com/rust-lang/rust/pull/101947)
* [disposition: merge] [Change `bindings_with_variant_name` to deny-by-default ](https://github.com/rust-lang/rust/pull/104154)

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

Rusty Events between 2022-12-21 - 2023-01-18 🦀

### Virtual

* 2022-12-21 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Show & Tell: Tableturf**](https://www.meetup.com/vancouver-rust/events/tqvhxsydcqbcc/)
* 2022-12-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcqbkc/)
* 2023-01-03 | Virtual (Beijing, CN) | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/mbmxvsyfccbfb/)
* 2023-01-03 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/289581074/)
* 2023-01-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfccbfb/)
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

### Asia

* 2022-12-29 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**December Edition - xtask, macros and low level features**](https://www.meetup.com/rust-tlv/events/290156141/)

### Europe

* 2022-12-29 | Freiburg, DE | [Arso Collective](https://arso.xyz/)
    * [**Rust Café Freiburg**](https://tacker.fr/node/10951)


### North America

* 2022-12-27 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**Atx Rustaceans Meetup**](https://www.meetup.com/atx-rustaceans/events/290064553/)
* 2023-01-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Interesting Title and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcqbdc/)

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

> In the depths of a computer's core,  
> Where bits and bytes are stored,  
> Lies a tool that's often ignored  
> But without it, things would be floored.
>
> It's the rust borrow checker,  
> A guardian of memory,  
> Ensuring that data is in the right place  
> And never causing miseries.
>
> With each line of code it carefully scans,  
> Checking for underflows and overflows,  
> Preventing errors, saving the day,  
> And keeping the program in a flow.
>
> So let's give a nod to this silent hero,  
> Whose work may go unnoticed, but is never zero,  
> It's the rust borrow checker,  
> A vital part of the machine,  
> Ensuring our programs run clean.

– [ChatGPT prompted by Vivek Yadav](https://twitter.com/vivek_verse/status/1602232115982393344)

[llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1340) is quite self-appreciative for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/zsrzuq/this_week_in_rust_474/)</small>
