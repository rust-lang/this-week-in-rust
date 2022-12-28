Title: This Week in Rust 475
Number: 475
Date: 2022-12-28
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
* [Board Announcement: Farewell to Shane Miller](https://foundation.rust-lang.org/news/board-announcement-farewell-to-shane-miller/)
* [Community Grantee Spotlight: Oğuz Ağcayazı](https://foundation.rust-lang.org/news/community-grantee-spotlight-o%C4%9Fuz-a%C4%9Fcayaz%C4%B1/)

### Project/Tooling Updates
* [MySQL connection pooling in Rust for Toolforge](https://blog.legoktm.com/2022/12/27/mysql-connection-pooling-in-rust-for-toolforge.html)
* [The Sequoia GnuPG Chameleon 0.1 is Released](https://sequoia-pgp.org/blog/2022/12/19/202212-chameleon-0.1/)
* [Dotenv-linter v3.3.0: Overview](https://dotenv-linter.github.io/#/whats_new/v330)
* [rust-analyzer changelog #161](https://rust-analyzer.github.io/thisweek/2022/12/26/changelog-161.html)
* [rust-gpu v0.4](https://github.com/EmbarkStudios/rust-gpu/releases/tag/v0.4.0)
* [This Week in Fyrox #8](https://fyrox.rs/blog/post/twif8/)

### Observations/Thoughts
* [cargo-semver-checks today and in 2023](https://predr.ag/blog/cargo-semver-checks-today-and-in-2023/)
* [Rust vs Common C++ Bugs](https://geo-ant.github.io/blog/2022/common-cpp-errors-vs-rust/)
* [2022 in review (Yoshua Wuyts)](https://blog.yoshuawuyts.com/look-back-2022/)
* [Rust 2023 (Yoshua Wuyts)](https://blog.yoshuawuyts.com/rust-2023/)
* [Rust in 2023 (Gijs Burghoorn)](https://gburghoorn.com/posts/rust-in-2023/)
* [Rust in 2023 (azdavis)](https://azdavis.net/posts/rust-2023/)
* [Capture The IP — Or a tiny story about how to over-engineer a simple project for the sake of learning Rust and other things about web services and binaries](https://markentier.tech/posts/2022/12/capture-the-ip/)
* [video] [Rust in 2024 (Niko Matsakis)](https://www.youtube.com/watch?v=OuSiuySr6_Q)
* [video] [Profiling Code in Rust](https://www.youtube.com/watch?v=JRMOIE_wAFk)
* [video] [Nine Rules for Creating Procedural Macros in Rust](https://www.youtube.com/watch?v=DMLBBZBlKis)

### Rust Walkthroughs
* [How we extended the River stats module with Rust using PyO3](https://boring-guy.sh/posts/river-rust/)
* [A taste of pavex, an upcoming Rust web framework](https://www.lpalmieri.com/posts/a-taste-of-pavex-rust-web-framework/)
* [Rust visitor pattern and efficient DataFusion query federation](https://www.splitgraph.com/blog/datafusion-filter-expr-visitor)
* [Embedded Rust and Embassy: Timer Ultrasonic Distance Measurement](https://apollolabsblog.hashnode.dev/embedded-rust-and-embassy-timer-ultrasonic-distance-measurement)
* [video] [Rust - IAT Hooking](https://www.youtube.com/watch?v=jHrzmflNrgY)

### Research
* [Modular Formal Verification of Rust Programs with Unsafe Blocks](https://arxiv.org/abs/2212.12976)

### Miscellaneous
* [This year in LLVM (2022)](https://www.npopov.com/2022/12/20/This-year-in-LLVM-2022.html)

## Crate of the Week

This week's crate is [scraper](https://crates.io/crates/scraper), a crate for HTML parsing and querying with CSS selectors.

Thanks to [Carlo Federico Vescovo](https://users.rust-lang.org/t/crate-of-the-week/2704/1140) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

*No calls for participation this week. Keep an eye out for more places to contribute next week!*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

344 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-12-19..2022-12-26

* [make LLD build forward-compatible with LLVM 16](https://github.com/rust-lang/rust/pull/106000)
* [add `-Zno-jump-tables`](https://github.com/rust-lang/rust/pull/105812)
* [allow `&..` to be parsed as let initializer](https://github.com/rust-lang/rust/pull/105701)
* [add `implied_bounds_entailment` lint](https://github.com/rust-lang/rust/pull/105575)
* [allow building std with cranelift](https://github.com/rust-lang/rust/pull/106051)
* [correct branch-protection ModFlagBehavior for Aarch64 on LLVM-15](https://github.com/rust-lang/rust/pull/105932)
* [ensure param-env is const before calling `eval_to_valtree`](https://github.com/rust-lang/rust/pull/105847)
* [fix arch flag on i686-apple-darwin](https://github.com/rust-lang/rust/pull/105904)
* [give a more helpful error for "`trimmed_def_paths` constructed"](https://github.com/rust-lang/rust/pull/106057)
* [give opaque types a better coherence error](https://github.com/rust-lang/rust/pull/106010)
* [mark `proc_macro_decls_static` as always used](https://github.com/rust-lang/rust/pull/105978)
* [re-enable `Fn` trait call notation error for non-tuple argument](https://github.com/rust-lang/rust/pull/105966)
* [suggest associated const on possible capitalization mistake](https://github.com/rust-lang/rust/pull/105843)
* [suggest remove last method call when type coerce with expected type](https://github.com/rust-lang/rust/pull/105872)
* [miri: data race spans](https://github.com/rust-lang/miri/pull/2646)
* [switch `#[track_caller]` back to a no-op unless feature gate is enabled](https://github.com/rust-lang/rust/pull/104741)
* [make `VecDeque::new` const](https://github.com/rust-lang/rust/pull/105127)
* [implement `From<bool>` for f32, f64](https://github.com/rust-lang/rust/pull/100390)
* [add `ptr::from_`{`ref`, `mut`}](https://github.com/rust-lang/rust/pull/104977)
* [abort immediately on bad `mem::zeroed/uninit`](https://github.com/rust-lang/rust/pull/105997)
* [cargo: fix: deduplicate dependencies by artifact target](https://github.com/rust-lang/cargo/pull/11478)
* [cargo: support vendoring with different revs from same git repo](https://github.com/rust-lang/cargo/pull/10690)
* [add readable rustdoc display for tvOS and watchOS](https://github.com/rust-lang/rust/pull/105933)
* [clippy: add `permissions_set_readonly_false` lint](https://github.com/rust-lang/rust-clippy/pull/10063)
* [clippy: add `size_of_ref` lint](https://github.com/rust-lang/rust-clippy/pull/10098)
* [clippy: avoid `match_wildcard_for_single_variants` on guarded wild matches](https://github.com/rust-lang/rust-clippy/pull/10056)
* [clippy: fix false positives in `needless_return` when using yeet](https://github.com/rust-lang/rust-clippy/pull/10109)
* [clippy: fix `manual_filter` false positive](https://github.com/rust-lang/rust-clippy/pull/10091)
* [clippy: fix incorrect suggestion in `suboptimal_flops`](https://github.com/rust-lang/rust-clippy/pull/10113)
* [clippy: improve `needless_borrow`, `redundant_clone`](https://github.com/rust-lang/rust-clippy/pull/9701)
* [rust-analyzer: add xtask for publishing release notes in Markdown on GitHub Releases from a changelog in AsciiDoc](https://github.com/rust-lang/rust-analyzer/pull/13771)
* [rust-analyzer: complete enum variants without parens when snippets are disabled](https://github.com/rust-lang/rust-analyzer/pull/13805)
* [rust-analyzer: add an option to hide adjustment hints outside of `unsafe` blocks and functions](https://github.com/rust-lang/rust-analyzer/pull/13817)
* [rust-analyzer: fix binding mode hints always adding parentheses to or-patterns](https://github.com/rust-lang/rust-analyzer/pull/13820)
* [rust-analyzer: completion: remove bound insert of type in trait](https://github.com/rust-lang/rust-analyzer/pull/13831)
* [rust-analyzer: calculate the `TargetDataLayout` correctly for the selected target](https://github.com/rust-lang/rust-analyzer/pull/13814)
* [rust-analyzer: correctly check for parentheses redundancy in `remove_parentheses` assist](https://github.com/rust-lang/rust-analyzer/pull/13764)
* [rust-analyzer: don't let mbe expr fragments match let exprs and inline consts](https://github.com/rust-lang/rust-analyzer/pull/13800)
* [rust-analyzer: handle lifetime variables in `CallableSig` query](https://github.com/rust-lang/rust-analyzer/pull/13840)
* [rust-analyzer: skip adjustment hints if the adjustment is identity (`T` → `T`)](https://github.com/rust-lang/rust-analyzer/pull/13806)
* [rust-analyzer: implement location link for type inlay hints](https://github.com/rust-lang/rust-analyzer/pull/13699)
* [rust-analyzer: inline all format arguments where possible](https://github.com/rust-lang/rust-analyzer/pull/13835)
* [docs.rs: URL-encode canonical URLs when they can include UTF8 characters](https://github.com/rust-lang/docs.rs/pull/1968)

### Rust Compiler Performance Triage

A light week with few performance changes, apart from one PR that added some
necessary extra work to rustdoc and so we observed a corresponding hit to some
doc benchmarks.

Triage done by **@pnkfelix**.
Revision range: [8a746f4a..b38a6d37](https://perf.rust-lang.org/?start=8a746f4ac3a489efb724cde813607f3b96c2df7b&end=b38a6d373cb254697411147c0e49cd2e84864258&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 2.8%  | [0.2%, 18.4%]  | 14    |
| Regressions ❌ <br /> (secondary)  | 1.3%  | [0.2%, 2.6%]   | 24    |
| Improvements ✅ <br /> (primary)   | -     | -              | 0     |
| Improvements ✅ <br /> (secondary) | -0.7% | [-1.1%, -0.3%] | 10    |
| All ❌✅ (primary)                 | 2.8%  | [0.2%, 18.4%]  | 14    |


3 Regressions, 2 Improvements, 1 Mixed; 1 of them in rollups
44 artifact comparisons made in total
 
[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-12-27.md)
 
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

* [disposition: merge] [Tracking Issue for io_error_more](https://github.com/rust-lang/rust/issues/86442)
* [disposition: merge] [More deriving on packed structs](https://github.com/rust-lang/rust/pull/104429)
* [disposition: merge] [Tracking Issue for `#![feature(const_socketaddr)]`](https://github.com/rust-lang/rust/issues/82485)
* [disposition: merge] [Add PhantomData marker to Context to make Context !Send and !Sync](https://github.com/rust-lang/rust/pull/95985)
* [disposition: merge] [Tracking Issue for abstract namespaces in Unix domain sockets](https://github.com/rust-lang/rust/issues/85410)
* [disposition: merge] [Tracking Issue for const MaybeUninit::as(_mut)_ptr (feature: const_maybe_uninit_as_ptr)](https://github.com/rust-lang/rust/issues/75251)

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

Rusty Events between 2022-12-28 - 2023-01-25 🦀

### Virtual

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

### Asia

* 2022-12-29 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**December Edition - xtask, macros and low level features**](https://www.meetup.com/rust-tlv/events/290156141/)

### Europe

* 2022-12-29 | Freiburg, DE | [Arso Collective](https://arso.xyz/)
    * [**Rust Café Freiburg**](https://tacker.fr/node/10951)
* 2023-01-12 | Enschede, NL | [Dutch Rust Meetup](https://www.meetup.com/dutch-rust-meetup/)
    * [**Rust Meetup - Subject TBA**](https://www.meetup.com/dutch-rust-meetup/events/289021643/)
* 2023-01-20 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/zmppzsyfccbbc/)
* 2023-01-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #55**](https://www.meetup.com/rust-paris/events/290100223/)


### North America

* 2022-12-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/290335198/)
* 2023-01-05 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Lightning Talks 'n' Chill (a.k.a. Show & Tell), with Pizza!**](https://www.meetup.com/utah-rust/events/dsbpxsydcqbdc/)
* 2023-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Happy Hour and Beginner Embedded Rust Hacking Session (#2!)**](https://www.meetup.com/minneapolis-rust-meetup/events/289768841/)
* 2023-01-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/rwvwzsyfccbwb/)

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

> Rust does best when we're ambitious

– [Niko Matsakis quoted by Yoshua Wuyts on his blog](https://blog.yoshuawuyts.com/rust-2023/#ambition)

llogiq is inordinately pleased with [his suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1343) and thanks Yoshua for clearing the quote!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/zxkbzb/this_week_in_rust_475/)</small>
