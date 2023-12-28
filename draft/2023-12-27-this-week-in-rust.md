Title: This Week in Rust 527
Number: 527
Date: 2023-12-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Announcing `async fn` and return-position `impl Trait` in traits](https://blog.rust-lang.org/2023/12/21/async-fn-rpit-in-traits.html)
* [Rustc Trait System Refactor Initiative Update: A call for testing](https://blog.rust-lang.org/inside-rust/2023/12/22/trait-system-refactor-initiative.html)

### Foundation
* [Improving Supply Chain Security for Rust Through Artifact Signing](https://foundation.rust-lang.org/news/2023-12-21-improving-supply-chain-security/)

### Newsletters

### Project/Tooling Updates
* [Introducing Loco: The Rails of Rust](https://www.shuttle.rs/blog/2023/12/20/loco-rust-rails)

### Observations/Thoughts
* [My path to becoming a Rustacean](https://thedataquarry.com/posts/path-to-becoming-a-rustacean/)
* [Memory Safety is a Red Herring](https://steveklabnik.com/writing/memory-safety-is-a-red-herring)
* [The Most Common Rust Compiler Errors as Encountered in RustRover: Part 2](https://blog.jetbrains.com/rust/2023/12/20/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-2/)
* [Video] [Rust 1.74.1 & Rust News](https://youtu.be/_UItLy_nLf8)

- [My reference was dropped, why is the compiler complaining about multiple borrows?](https://ntietz.com/blog/my-reference-was-dropped-why-is-the-compiler-complaining-about-multiple-borrows/)

* [Can CppRef be ergonomic?](https://medium.com/@adetaylor/can-cppref-t-be-ergonomic-c9cb1365bda1)

### Rust Walkthroughs
* [The dark side of inlining and monomorphization](https://nickb.dev/blog/the-dark-side-of-inlining-and-monomorphization/)
* [The Heart of a Language Server](https://rust-analyzer.github.io/blog/2023/12/26/the-heart-of-a-language-server.html)
* [Rust: Multi threading](https://priver.dev/blog/rust/multi-threading/)
* [series] [Meilisearch Expands Search Power with Arroy's Filtered Disk ANN](https://blog.kerollmops.com/meilisearch-expands-search-power-with-arroy-s-filtered-disk-ann)
* [Encrypted Portals between Macs – built in Rust and Swift (sorta similar to ngrok and tailscale)](https://github.com/build-trust/ockam/blob/develop/examples/app/portals/README.md)

### Research

### Miscellaneous
* [An anonymous survey about mutable statics](https://www.surveyhero.com/c/static-mut)
* [Web development in Rust](https://rust.code-maven.com/web)
* [Rocket: Web-based Hello World! with tests](https://rust.code-maven.com/rocket-hello-world)

## Crate of the Week

This week's crate is [rouille](https://crates.io/crates/rouille), a small synchronous web framework.

Thanks to [Peter Puetz](https://users.rust-lang.org/t/crate-of-the-week/2704/1275 for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website either through a PR to TWiR or on the [Rust-lang forums].[link TBD]

## Updates from the Rust Project

268 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-20..2023-12-27

* [add support for `for await` loops](https://github.com/rust-lang/rust/pull/118847)
* [add illumos aarch64 target for rust](https://github.com/rust-lang/rust/pull/112936)
* [add support for hexagon-unknown-none-elf as target](https://github.com/rust-lang/rust/pull/117601)
* [-Znext-solver: adapt overflow rules to avoid breakage](https://github.com/rust-lang/rust/pull/119071)
* [`rustc_codegen_ssa`: Don't drop `IncorrectCguReuseType` , make `rustc_expected_cgu_reuse` attr work](https://github.com/rust-lang/rust/pull/118973)
* [`subtype_predicate`: remove unnecessary probe](https://github.com/rust-lang/rust/pull/119107)
* [add check for possible `CStr` literals in pre-2021](https://github.com/rust-lang/rust/pull/118691)
* [add method to get instance instantiation arguments](https://github.com/rust-lang/rust/pull/119141)
* [add missing feature gate for sanitizer CFI cfgs](https://github.com/rust-lang/rust/pull/119235)
* [avoid redundant Option for `cross_crate_inlinable`](https://github.com/rust-lang/rust/pull/119225)
* [coverage: check for `async fn` explicitly, without needing a heuristic](https://github.com/rust-lang/rust/pull/119155)
* [do not allow ABI mismatches inside `repr(C)` types](https://github.com/rust-lang/rust/pull/119037)
* [do not fetch HIR in `inferred_outlives_of`](https://github.com/rust-lang/rust/pull/119261)
* [emit better suggestions for `&T == T` and `T == &T`](https://github.com/rust-lang/rust/pull/118431)
* [emits error if has bound regions](https://github.com/rust-lang/rust/pull/119215)
* [encode `CoroutineKind` directly](https://github.com/rust-lang/rust/pull/119173)
* [exhaustiveness: improve complexity on some wide matches](https://github.com/rust-lang/rust/pull/118796)
* [exhaustiveness: keep the original `thir::Pat` around](https://github.com/rust-lang/rust/pull/119233)
* [exhaustiveness: reveal empty opaques in depth](https://github.com/rust-lang/rust/pull/119218)
* [exhaustiveness: reveal opaque types properly](https://github.com/rust-lang/rust/pull/116821)
* [fallback `default` to `None` during ast-lowering for lifetime binder](https://github.com/rust-lang/rust/pull/119042)
* [fix ICE when using raw ptr in a pattern](https://github.com/rust-lang/rust/pull/119274)
* [fix crash due to `CrateItem::kind()` not handling constructors](https://github.com/rust-lang/rust/pull/119135)
* [give temporaries in if let guards correct scopes](https://github.com/rust-lang/rust/pull/119122)
* [make `soft_unstable` show up in future breakage reports](https://github.com/rust-lang/rust/pull/116274)
* [make closures carry their own ClosureKind](https://github.com/rust-lang/rust/pull/119258)
* [mark `ty::Const::Error` when meet unsupport ty for const generic params](https://github.com/rust-lang/rust/pull/117176)
* [pass `DeadItem` and lint as consistent group in dead-code](https://github.com/rust-lang/rust/pull/119297)
* [remove `DiagCtxt` API duplication](https://github.com/rust-lang/rust/pull/119146)
* [remove metadata decoding `DefPathHash` cache](https://github.com/rust-lang/rust/pull/119265)
* [resolve: eagerly feed closure visibilities](https://github.com/rust-lang/rust/pull/119136)
* [resolve: feed visibilities for unresolved trait impl items](https://github.com/rust-lang/rust/pull/119134)
* [resolve: stop feeding visibilities for import list stems](https://github.com/rust-lang/rust/pull/119168)
* [rework `-Zverbose`](https://github.com/rust-lang/rust/pull/119129)
* [simple modification of `non_lifetime_binders`'s diagnostic information to adapt to type binders](https://github.com/rust-lang/rust/pull/119154)
* [skip duplicate stable crate ID encoding into metadata](https://github.com/rust-lang/rust/pull/119238)
* [split coroutine desugaring kind from source](https://github.com/rust-lang/rust/pull/119198)
* [subtree sync for `rustc_codegen_cranelift`](https://github.com/rust-lang/rust/pull/119278)
* [suggest `=` to `==` in more cases, even in the face of reference mismatch](https://github.com/rust-lang/rust/pull/119328)
* [add function ABI and type layout to StableMIR](https://github.com/rust-lang/rust/pull/119094)
* [separate MIR lints from validation](https://github.com/rust-lang/rust/pull/119077)
* [miri: implement and test `simd_masked_load` and `simd_masked_store`](https://github.com/rust-lang/miri/pull/3237)
* [improve coding efficiency for `RawDefId`](https://github.com/rust-lang/rust/pull/119226)
* [use `Vec` for region constraints instead of `BTreeMap`](https://github.com/rust-lang/rust/pull/118824)
* [stabilize `file_create_new`](https://github.com/rust-lang/rust/pull/119153)
* [stabilize `ip_in_core` feature](https://github.com/rust-lang/rust/pull/119276)
* [add more niches to rawvec](https://github.com/rust-lang/rust/pull/106790)
* [add `IntoAsyncIterator`](https://github.com/rust-lang/rust/pull/119222)
* [add `hint::assert_unchecked`](https://github.com/rust-lang/rust/pull/119133)
* [cargo: extend the build directive syntax with `cargo:`:](https://github.com/rust-lang/cargo/pull/12201)
* [cargo: hold the mutate exclusive lock when vendoring](https://github.com/rust-lang/cargo/pull/12509)
* [cargo: refactor: centralize git checkouts and db paths](https://github.com/rust-lang/cargo/pull/13187)
* [cargo: refactor: custom error types for `cargo-util-schemas`](https://github.com/rust-lang/cargo/pull/13186)
* [cargo: rework `--check-cfg` generation comment](https://github.com/rust-lang/cargo/pull/13195)
* [rustdoc: Add `is_object_safe` information for traits in JSON output](https://github.com/rust-lang/rust/pull/119246)
* [rustdoc: fix display of warning block if it is first element of the top doc block](https://github.com/rust-lang/rust/pull/119283)
* [clippy: `question_mark`: also trigger on `return` statements](https://github.com/rust-lang/rust-clippy/pull/11994)
* [clippy: check whether out of bound when access a known length array with a constant index](https://github.com/rust-lang/rust-clippy/pull/11998)
* [clippy: do not consider `async { (impl IntoFuture).await }` as redundant](https://github.com/rust-lang/rust-clippy/pull/11967)
* [clippy: extend `UNNECESSARY_TO_OWNED` to handle `split`](https://github.com/rust-lang/rust-clippy/pull/11871)
* [clippy: move `uninhabited_references` to `nursery`](https://github.com/rust-lang/rust-clippy/pull/11997)
* [clippy: new lints `iter_filter_is_some` and `iter_filter_is_ok`](https://github.com/rust-lang/rust-clippy/pull/12004)
* [clippy: stop `bool_comparison`'s suggestion from consuming parentheses](https://github.com/rust-lang/rust-clippy/pull/11991)
* [rust-analyzer: complete exported macros in `#[macro_use($0)]`](https://github.com/rust-lang/rust-analyzer/pull/16137)
* [rust-analyzer: implement a rust-analyzer span backed proc-macro server mode](https://github.com/rust-lang/rust-analyzer/pull/16088)
* [rust-analyzer: auto remove unnecessary braces after remove unused imports](https://github.com/rust-lang/rust-analyzer/pull/16066)
* [rust-analyzer: correctly set and mark the proc-macro spans](https://github.com/rust-lang/rust-analyzer/pull/16175)
* [rust-analyzer: fix completions analysis not caching all nodes in Semantics](https://github.com/rust-lang/rust-analyzer/pull/16184)
* [rust-analyzer: fix span marking for builtin fn macros](https://github.com/rust-lang/rust-analyzer/pull/16178)
* [rust-analyzer: fully remove dummy spans](https://github.com/rust-lang/rust-analyzer/pull/16167)
* [rust-analyzer: remove wrong comma after remove unnecessary braces](https://github.com/rust-lang/rust-analyzer/pull/16185)

### Rust Compiler Performance Triage

Some of the recent noise swings have been appearing again this week, but luckily
less often than before. There were a few actual regressions, but most of them
were localized to a single stress test. On the other hand, there were a few very
nice wins across the board, especially for check and incremental builds, primarily
thanks to [#118824](https://github.com/rust-lang/rust/pull/118824) and
[#119265](https://github.com/rust-lang/rust/pull/119265).

Triage done by **@kobzol**.
Revision range: [bf9229a2e366b4c311f059014a4aa08af16de5d8..1ab783112ab4e4807304dbd249b39771246013ef](https://perf.rust-lang.org/?start=bf9229a2e366b4c311f059014a4aa08af16de5d8&end=1ab783112ab4e4807304dbd249b39771246013ef&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.6%, 0.6%]   | 1     |
| Regressions ❌ <br /> (secondary)  | 2.5%  | [0.3%, 4.2%]   | 10    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-3.3%, -0.1%] | 180   |
| Improvements ✅ <br /> (secondary) | -1.2% | [-5.5%, -0.2%] | 109   |
| All ❌✅ (primary)                 | -0.8% | [-3.3%, 0.6%]  | 181   |

5 Regressions, 7 Improvements, 2 Mixed; 1 of them in rollups
58 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/73d96e7ca26ef9ddfc1c32c7701e1f1159512c49/triage/2023-12-26.md)

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
* [disposition: merge] [Use version-sorting for all sorting](https://github.com/rust-lang/rust/pull/115046)
* [disposition: merge] [Tracking issue for exclusive range patterns](https://github.com/rust-lang/rust/issues/37854)
* [disposition: merge] [rustdoc: clean up source sidebar hide button](https://github.com/rust-lang/rust/pull/119066)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Allow type inference for const or static](https://github.com/rust-lang/rfcs/pull/3546)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-27 - 2024-01-24 🦀

### Virtual

* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 2024-01-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtygccbmb/)
* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)

### Europe

* 2023-12-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs and plushies**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)
* 2023-12-28 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 3: Holiday Edition**](https://www.meetup.com/rust-vienna/events/297826979/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)

### North America

* 2023-12-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 2024-01-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch**](https://www.meetup.com/bostonrust/events/297633937/)
* 2024-01-08 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 2024-01-09 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 2024-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs
<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> Rust can be rather more verbose than C; there are a lot of invariants that have to be expressed in the code. But that is countered by the need for far less error-handling code; it turns out to be a wash, with the size of the two implementations being about the same.

– [Alice Ryhl at the Linux Plumbers Conference as quoted by Jonathan Corbet, LWN](https://lwn.net/Articles/953116)

Thanks to [Ivan Fraixedes](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1498) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
