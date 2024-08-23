Title: This Week in Rust 560
Number: 560
Date: 2024-08-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X (formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

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
* [Announcing Rust 1.80.1](https://blog.rust-lang.org/2024/08/08/Rust-1.80.1.html)
* [Rust Project goals for 2024](https://blog.rust-lang.org/2024/08/12/Project-goals.html)

### Newsletters
* [This Month in Rust OSDev: July 2024](https://rust-osdev.com/this-month/2024-07/)

### Project/Tooling Updates
* [Bevy's Fourth Birthday](https://bevyengine.org/news/bevys-fourth-birthday/)
* [Intoducing Tonbo](https://tonbo.io/blog/introducing-tonbo)
* [This Month in Xilem, July 2024](https://linebender.org/blog/tmix-07/)
* [Rust GPU Transitions to Community Ownership](https://rust-gpu.github.io/blog/transition-announcement/)
* [rust-analyzer changelog #246](https://rust-analyzer.github.io/thisweek/2024/08/12/changelog-246.html)
* [Ringboard: a new clipboard manager for Linux](https://alexsaveau.dev/blog/projects/performance/clipboard/ringboard/ringboard)
* [derive_more 1.0.0 - Finally a stable release](https://github.com/JelteF/derive_more/releases/tag/v1.0.0)
* [Aura 4.0: Port from Haskell to Rust complete](https://github.com/fosskers/aura/releases/tag/v4.0.0) ([Migration Guide](https://fosskers.github.io/aura/migration.html), [Why Rust?](https://fosskers.github.io/aura/faq.html#why-did-you-rewrite-aura-in-rust))
* [`r3bl_terminal_async` v0.5.6 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v056-2024-08-13)
* [`r3bl_tui` v0.5.7 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#v057-2024-08-13)

### Observations/Thoughts
* [I compared 14 hashing algorithms on Rust using Criterion](https://blog.goose.love/posts/rosetta-hashing/)
* [Rust's Mutex, Atomics and UnsafeCell – Spooky Action at a Distance?](https://leon.schuermann.io/blog/2024-08-07_rust-mutex-atomics-unsafecell_spooky-action-at-a-distance.html)
* [How I Created 175 Fonts Using Rust](https://chevyray.dev/blog/creating-175-fonts/)
* [The weird of function-local types in Rust](https://elastio.github.io/bon/blog/the-weird-of-function-local-types-in-rust)
* [Fear Not the Association of Types: a walkthrough of associated types](https://gavinleroy.com/writings/i-heart-assoc-types.html)
* [Programming vs Software Engineering (Rust vs Go)](https://kerkour.com/programming-vs-software-engineering-rust-vs-go)
* [video] [The Best Games from Bevy Jam 5](https://www.youtube.com/watch?v=_H87sL5ieOc)
* [audio] [I Was Wrong About Rust Build Times](https://sdr-podcast.com/episodes/wrong-build-times/)
* [audio] [Rust in Production Podcast Season 2 Finale](https://corrode.dev/podcast/s02e08-season-finale/)
* [audio] [Exploring Fiberplane's 3-Year Rust Journey, with Benno van den Berg](https://rustacean-station.org/episode/benno-van-den-berg/)

### Rust Walkthroughs
* [GraphQL based Admin Dashboard with Loco and Seaography](https://www.sea-ql.org/blog/2024-08-08-graphql-admin-dashboard-with-loco-seaography/)

### Research
* [Don’t Write, but Return: Replacing Output Parameters with Algebraic Data Types in C-to-Rust Translation](https://dl.acm.org/doi/10.1145/3656406)
* [The Hitchhiker’s Guide to Building an Encrypted Filesystem in Rust](https://medium.com/@xorio42/list/828492b94c23)

### Miscellaneous
* [RP2350 Launch Blog](https://thejpster.org.uk/blog/blog-2024-08-08/)
* [July 2024 Rust Jobs Report](https://filtra.io/rust-jul-24)
* [video] [John Nunley discusses Rust, Open source, smol-rs and Async Programming](https://www.youtube.com/watch?v=EnWbnJXkOsg)
* [Audio] [Supercharging Python Tooling – An Interview with Charlie Marsh](https://timclicks.dev/podcast/supercharging-python-tooling-an-interview-with-charlie-marsh/)
* [Audio] [Systems Software for Linux with Rust – Interview with Zeeshan Ali Khan](https://timclicks.dev/podcast/systems-software-for-linux-with-rust-interview-with-zeeshan-ali-khan/)

## Crate of the Week

This week's crate is [rencfs](https://github.com/radumarias/rencfs), a FUSE-based encrypted filesystem for Linux.

Thanks to [Radu Marias](https://users.rust-lang.org/t/crate-of-the-week/2704/1329) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)
  - [Testing Steps](https://github.com/rust-lang/cargo/issues/13873)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)
* *No calls for testing were issued this week.*

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)
* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

* [Diesel - Add support for currently unsupported array functions](https://github.com/diesel-rs/diesel/issues/4153)
* [Diesel - Provide a r2d2::CustomizeConnection for testing](https://github.com/diesel-rs/diesel/issues/4152)
* [Mysqlclient-sys - Add support for mysql-client 9.0](https://github.com/sgrif/mysqlclient-sys/issues/53)
* [rencfs-desktop - Use SurrealDB](https://github.com/radumarias/rencfs-desktop/issues/3)
* [rencfs-desktop - Implement daemon](https://github.com/radumarias/rencfs-desktop/issues/4)
* [rencfs-desktop - Implement functionality](https://github.com/radumarias/rencfs-desktop/issues/9)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

395 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-08-06..2024-08-13

* [promote aarch64-apple-darwin to Tier 1](https://github.com/rust-lang/rust/pull/128592)
* [enable zstd for debug compression](https://github.com/rust-lang/rust/pull/125642)
* [use stable sort to sort multipart diagnostics](https://github.com/rust-lang/rust/pull/128852)
* [`nontemporal_store`: make sure that the intrinsic is truly just a hint](https://github.com/rust-lang/rust/pull/128149)
* [`rustc_codegen_ssa`: Set architecture for object crate for 32-bit SPARC](https://github.com/rust-lang/rust/pull/128772)
* [`rustc_lint`: make `let-underscore-lock` translatable](https://github.com/rust-lang/rust/pull/128369)
* [`unused_parens`: do not lint against parens around &raw](https://github.com/rust-lang/rust/pull/128782)
* [add -Zmetrics-dir=PATH to save diagnostic metadata to disk](https://github.com/rust-lang/rust/pull/128702)
* [add `builder-config` into tarball sources](https://github.com/rust-lang/rust/pull/128822)
* [add implied target features to `target_feature` attribute](https://github.com/rust-lang/rust/pull/128221)
* [add range attribute to scalar function results and arguments](https://github.com/rust-lang/rust/pull/128371)
* [cache supertrait outlives of impl header for soundness check](https://github.com/rust-lang/rust/pull/128746)
* [change generate-copyright to generate HTML, with cargo dependencies included](https://github.com/rust-lang/rust/pull/128353)
* [codegen: better centralize function declaration attribute computation](https://github.com/rust-lang/rust/pull/128679)
* [consider `cfg_attr` checked by `CheckAttrVisitor`](https://github.com/rust-lang/rust/pull/128718)
* [const vector passed through to codegen](https://github.com/rust-lang/rust/pull/128537)
* [diagnostics: do not warn when a lifetime bound infers itself](https://github.com/rust-lang/rust/pull/128908)
* [differentiate between methods and associated functions in diagnostics](https://github.com/rust-lang/rust/pull/128910)
* [disable verbose bootstrap command failure logging by default](https://github.com/rust-lang/rust/pull/128874)
* [disallow setting some built-in cfg via the command-line](https://github.com/rust-lang/rust/pull/126158)
* [don't ICE when getting an input file name's stem fails](https://github.com/rust-lang/rust/pull/128710)
* [don't arbitrarily choose one upper bound for hidden captured region error message](https://github.com/rust-lang/rust/pull/128753)
* [don't implement `AsyncFn` for `FnDef`/`FnPtr` that wouldnt implement `Fn`](https://github.com/rust-lang/rust/pull/128791)
* [don't inline tainted MIR bodies](https://github.com/rust-lang/rust/pull/128616)
* [emit an error for invalid use of the `#[no_sanitize]` attribute](https://github.com/rust-lang/rust/pull/128552)
* [ensure let stmt compound assignment removal suggestion respect codepoint boundaries](https://github.com/rust-lang/rust/pull/128865)
* [fallback to string formatting if source is not available for lint](https://github.com/rust-lang/rust/pull/128517)
* [fix ICE Caused by Incorrectly Delaying E0107](https://github.com/rust-lang/rust/pull/128377)
* [fix `ElaborateBoxDerefs` on debug varinfo](https://github.com/rust-lang/rust/pull/128572)
* [ensure `Guard`'s `drop` method is removed at `opt-level=s` for `…`](https://github.com/rust-lang/rust/pull/128862)
* [improve `Ord` viulation help](https://github.com/rust-lang/rust/pull/128273)
* [interpret: refactor function call handling to be better-abstracted](https://github.com/rust-lang/rust/pull/128687)
* [link `std` statically in `rustc_driver`](https://github.com/rust-lang/rust/pull/122362)
* [linker: remove the "`--whole-archive` in test mode" backcompat hack](https://github.com/rust-lang/rust/pull/128400)
* [more information for fully-qualified suggestion when there are multiple impls](https://github.com/rust-lang/rust/pull/128527)
* [normalize `struct` tail properly for `dyn` ptr-to-ptr casting in new solver](https://github.com/rust-lang/rust/pull/128712)
* [on trait bound mismatch, detect multiple crate versions in dep tree](https://github.com/rust-lang/rust/pull/124944)
* [only suggest `#[allow]` for `--warn` and `--deny` lint level flags](https://github.com/rust-lang/rust/pull/128826)
* [only walk ribs to collect possibly shadowed params if we are adding params in our new rib](https://github.com/rust-lang/rust/pull/128550)
* [pass the right `ParamEnv` to `might_permit_raw_init_strict`](https://github.com/rust-lang/rust/pull/128720)
* [skip over args when determining if async-closure's inner coroutine consumes its upvars](https://github.com/rust-lang/rust/pull/128520)
* [miri-script: use --remap-path-prefix to print errors relative to the right root](https://github.com/rust-lang/miri/pull/3798)
* [miri: make vtable addresses not globally unique](https://github.com/rust-lang/rust/pull/128742)
* [miri: `throw_unsup_format` for alignment greater than 2^29](https://github.com/rust-lang/miri/pull/3795)
* [miri: allow all code to call `getuid()`](https://github.com/rust-lang/miri/pull/3794)
* [miri: don't panic on `miri_print_borrow_state()` under `-Zmiri-disable-stacked-borrows`](https://github.com/rust-lang/miri/pull/3797)
* [miri: josh: wait until the socket is ready](https://github.com/rust-lang/miri/pull/3799)
* [stabilize `min_exhaustive_patterns`](https://github.com/rust-lang/rust/pull/122792)
* [std: do not overwrite style in `get_backtrace_style`](https://github.com/rust-lang/rust/pull/128632)
* [`std::thread: set_name` implementation proposal for vxWorks](https://github.com/rust-lang/rust/pull/128751)
* [introduce PinCoerceUnsized trait to core](https://github.com/rust-lang/rust/pull/125048)
* [core: optimise Debug impl for `ascii::Char`](https://github.com/rust-lang/rust/pull/120314)
* [add `f16` and `f128` math functions](https://github.com/rust-lang/rust/pull/128417)
* [impl `Default` for collection iterators that don't already have it](https://github.com/rust-lang/rust/pull/128261)
* [implement `BufReader::peek`](https://github.com/rust-lang/rust/pull/128406)
* [apply "polymorphization at home" to `RawVec`](https://github.com/rust-lang/rust/pull/126793)
* [make `LocalWaker::will_wake` consistent with `Waker::will_wake`](https://github.com/rust-lang/rust/pull/128882)
* [configure `f16` and `f128` support for WebAssembly](https://github.com/rust-lang/compiler-builtins/pull/665)
* [cargo: trim-paths: rustdoc supports trim-paths for diagnostics](https://github.com/rust-lang/cargo/pull/14389)
* [cargo: vendor: strip excluded build targets](https://github.com/rust-lang/cargo/pull/14367)
* [cargo: don't specify the depedency name in the `cargo add` inferred name test](https://github.com/rust-lang/cargo/pull/14357)
* [cargo: add `info` cargo subcommand](https://github.com/rust-lang/cargo/pull/14141)
* [cargo: fix renamed disallowed cfg lint name](https://github.com/rust-lang/cargo/pull/14352)
* [cargo: fix: `cargo package` failed on bare commit git repo](https://github.com/rust-lang/cargo/pull/14359)
* [cargo: infer registry](https://github.com/rust-lang/cargo/pull/14340)
* [cargo: use context instead of `with_context`](https://github.com/rust-lang/cargo/pull/14377)
* [cargo: use longhand gitoxide path-spec patterns](https://github.com/rust-lang/cargo/pull/14380)
* [rustdoc-search: account for numeric disambiguators on impls](https://github.com/rust-lang/rust/pull/128693)
* [rustdoc: do not run doctests with invalid langstrings](https://github.com/rust-lang/rust/pull/128838)
* [rustdoc: Stop showing impl items for negative impls](https://github.com/rust-lang/rust/pull/128923)
* [rustdoc: strip unreachable modules](https://github.com/rust-lang/rust/pull/128834)
* [fix rustdoc missing handling of remap-path-prefix option](https://github.com/rust-lang/rust/pull/128736)
* [clippy: `macro_metavars_in_unsafe`: recognize metavariables in tail expressions](https://github.com/rust-lang/rust-clippy/pull/13220)
* [clippy: `single_match`: fix checking of explicitly matched enums](https://github.com/rust-lang/rust-clippy/pull/11441)
* [clippy: add lint for `unused_result_ok`](https://github.com/rust-lang/rust-clippy/pull/12150)
* [clippy: add path prefixes back when compiling `clippy_dev` and `lintcheck`](https://github.com/rust-lang/rust-clippy/pull/13232)
* [clippy: add settings menu on clippy lints page](https://github.com/rust-lang/rust-clippy/pull/13187)
* [clippy: don't walk the HIR tree when checking for a const context](https://github.com/rust-lang/rust-clippy/pull/13231)
* [clippy: fix case where `doc_markdown` is triggered on words ending with "ified"](https://github.com/rust-lang/rust-clippy/pull/13163)
* [clippy: lintcheck: disable doc links](https://github.com/rust-lang/rust-clippy/pull/13234)
* [clippy: lintcheck: key lints on line start rather than byte start/end](https://github.com/rust-lang/rust-clippy/pull/13250)
* [clippy: remove more `snippet_opt` calls](https://github.com/rust-lang/rust-clippy/pull/13255)
* [clippy: respect allow `inconsistent_struct_constructor` on the `struct` definition](https://github.com/rust-lang/rust-clippy/pull/13211)
* [rust-analyzer: allow rust-project.json to be hidden](https://github.com/rust-lang/rust-analyzer/pull/17818)
* [rust-analyzer: implement TAIT and fix ATPIT a bit](https://github.com/rust-lang/rust-analyzer/pull/17845)
* [rust-analyzer: build and run build scripts in lsif command](https://github.com/rust-lang/rust-analyzer/pull/17864)
* [rust-analyzer: correctly support `#[rustc_deprecated_safe_2024]`](https://github.com/rust-lang/rust-analyzer/pull/17859)
* [rust-analyzer: fix `find_path` not respecting non-std preference config correctly](https://github.com/rust-lang/rust-analyzer/pull/17844)
* [rust-analyzer: fix unconfigured diagnostic being attached to the wrong file for modules](https://github.com/rust-lang/rust-analyzer/pull/17823)
* [rust-analyzer: fix missing non-exhaustive `let` diagnostics inside async or unsafe block](https://github.com/rust-lang/rust-analyzer/pull/17865)
* [rust-analyzer: fix native diagnostics not working](https://github.com/rust-lang/rust-analyzer/pull/17824)
* [rust-analyzer: fix panic while rendering function type hint with impl trait arg](https://github.com/rust-lang/rust-analyzer/pull/17832)
* [rust-analyzer: resolve included files to their calling modules in IDE layer](https://github.com/rust-lang/rust-analyzer/pull/17863)
* [rust-analyzer: remove trailing excess comma in "Convert to named struct" assist](https://github.com/rust-lang/rust-analyzer/pull/17867)
* [rust-analyzer: tyck for non-ADT types when searching refs for `Self` kw](https://github.com/rust-lang/rust-analyzer/pull/17813)
* [rust-analyzer: include vendored crates in StaticIndex](https://github.com/rust-lang/rust-analyzer/pull/17809)
* [rust-analyzer: only keep lib/ in publish-libs](https://github.com/rust-lang/rust-analyzer/pull/17862)
* [rust-analyzer: reuse recursion limit as expansion depth limit](https://github.com/rust-lang/rust-analyzer/pull/17833)

### Rust Compiler Performance Triage

A big week for compiler performance brought on mostly by statically linking the std library into `rustc_driver` instead of dynamic linking. This overshadows all other improvements and regressions that were seen this week.

Triage done by **@rylev**.
Revision range: [8c7e0e16..9cb1998e](https://perf.rust-lang.org/?start=8c7e0e160831866bc1a40691a39455aac21271c0&end=9cb1998ea15e179482504e07cad8fa121e169a32&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -     | -              | 0     |
| Regressions ❌ <br /> (secondary)  | -     | -              | 0     |
| Improvements ✅ <br /> (primary)   | -1.3% | [-2.9%, -0.2%] | 217   |
| Improvements ✅ <br /> (secondary) | -1.4% | [-4.9%, -0.2%] | 196   |
| All ❌✅ (primary)                 | -1.3% | [-2.9%, -0.2%] | 217   |


2 Regressions, 2 Improvements, 2 Mixed; 1 of them in rollups
35 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/3f0fc031a10c9decbfac4d1753452da2b10a03e1/triage/2024-08-13.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for `iter::repeat_n()` (`feature(iter_repeat_n)`)](https://github.com/rust-lang/rust/issues/104434)
* [disposition: merge] [Tracking Issue for CharIndices::offset function](https://github.com/rust-lang/rust/issues/83871)
* [disposition: merge] [Stabilize `unsafe_attributes`](https://github.com/rust-lang/rust/pull/128771)
* [disposition: merge] [Stabilize opaque type precise capturing (RFC 3617)](https://github.com/rust-lang/rust/pull/127672)
* [disposition: merge] [CloneToUninit impls](https://github.com/rust-lang/rust/pull/126877)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2024-08-14 - 2024-09-11 🦀

### Virtual
* 2024-08-15 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633266/)
* 2024-08-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346968/)
* 2024-08-21 | Hybrid - Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-22 | Virtual | [Conf42: Online Tech Events](https://www.meetup.com/conf42/)
    * [**Conf42 Rustlang 2024**](https://www.meetup.com/conf42/events/297266825/)
* 2024-08-22 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897938/)
* 2024-08-22 | Virtual (Karlsruhe, DE) | [Karlsruhe Functional Programmers Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA): various topics, from C++ to Rust**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/293937801)
* 2024-08-27 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Fearless Concurrency with Rust**](https://www.eventbrite.com/e/fearless-concurrency-with-rust-tickets-934569591807)
* 2024-08-27 | Virtual (Bordeaux, FR) | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Live coding - A distance #1**](https://www.meetup.com/bordeaux-rust/events/302570681/)
* 2024-08-27 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585668/)
* 2024-08-27 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Declarative macros in Rust (Virtual) - מקרוים בראסט**](https://www.meetup.com/rust-in-israel/events/302327956/)
* 2024-08-28 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-29 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633267/)
* 2024-08-29 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
* 2024-09-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007365/)
* 2024-09-04 | Virtual (Indianapolis, IN, US) | [Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328029/)
* 2024-09-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820268/)
* 2024-09-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346981/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA)| [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)

### Africa
* 2024-09-06 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587/)

### Asia
* 2024-08-24 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**August 2024 Rustacean meetup**](https://hasgeek.com/rustbangalore/august-2024-rustacean-meetup/)

### Europe
* 2024-08-14 | Köln/Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, August**](https://www.meetup.com/rustcologne/events/302674635/)
* 2024-08-14 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302153005/)
* 2024-08-20 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Cargo plugins - what the cargo ecosystem has to offer**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425034/)
* 2024-08-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/301522950/)
* 2024-08-21 | Cabridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/302574953/)
* 2024-08-21 | Nürnberg, DE | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Walk'n'Talk around Wöhrder See (+ Burritos)**](https://www.meetup.com/rust-noris/events/302080495/)
* 2024-08-22 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Talks August**](https://www.meetup.com/rust-manchester/events/302419276/)
* 2024-08-26 | Mainz, DE | [Fachschaft Mathematik+Informatik der JGU Mainz](https://rheinneckar.events/@fsmathe_informatik_mainz@rheinmain.social)
    * [**Ferienkurs Rust**](https://rheinneckar.events/events/3f76f860-75c1-4f3a-810f-03fc0cecb691/)
* 2024-08-27 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Exploring Slint: A Rust-based UI Toolkit – Mob Programming Session**](https://www.meetup.com/rust-trondheim/events/300991355/)
* 2024-08-28 | Frankfurt (Main), DE | [Rust Rhein Main](https://www.meetup.com/rust-rhein-main)
    * [**Rust Frankfurt WebAssembly**](https://www.meetup.com/rust-rhein-main/events/302738445/)
* 2024-08-29 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421378/)
* 2024-09-11 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302833564/)


### North America
* 2024-08-19 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup: "State of Rust GPU Programming" & Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301428949/)
* 2024-08-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**Rust NYC: Doing the Bare Minimum with Isograph (talk)**](https://www.meetup.com/rust-nyc/events/302480064/)
* 2024-08-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/301614081/)
* 2024-08-21 | Virtual and In-Person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631735/)
* 2024-08-26 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Boulder Rust Meetup: Learnings and Hangs!**](https://www.meetup.com/boulder-rust-meetup/events/302579817/)
* 2024-08-28 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygclblc/)
* 2024-08-29 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : placeholder**](https://www.meetup.com/music-city-rust-developers/events/301420110/)
* 2024-09-05 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302723843/)
* 2024-09-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/hdzdmtygcmbhb/)
* 2024-09-10 - 2024-09-13 | Hybrid: Virtual and In-Person (Montreal, QC, CA)| [Rust Conf](https://rustconf.com/)
    * [**Rust Conf 2024**](https://foundation.rust-lang.org/events/rustconf-2024/)

# Oceania
* 2024-08-22 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Dot IX: Diagram Generator + Deep Learning from Scratch in Rust**](https://www.meetup.com/rust-akl/events/302431924/)
* 2024-08-27 | Canberra, ACT, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/301887261/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ecdzp2/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> We are living through Rust finding its more "complete" form while still being an actually useful production language.

– [Catherine West on /r/rust](https://old.reddit.com/r/rust/comments/1eq357a/ive_been_working_with_rust_for_a_couple_years_now/lhu5ag2/)

Given a woeful lack of suggestions, llogiq is reasonably pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1esluoq/this_week_in_rust_560/)</small>
