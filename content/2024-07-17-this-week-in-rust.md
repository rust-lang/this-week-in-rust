Title: This Week in Rust 556
Number: 556
Date: 2024-07-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Newsletters
* [This Month in Rust OSDev: June 2024](https://rust-osdev.com/this-month/2024-06/)

### Project/Tooling Updates
* [Zed: Linux when? Linux now.](https://zed.dev/blog/zed-on-linux)
* [r3bl_terminal_async v0.5.4 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#r3bl_terminal_async)
* [r3bl_test_fixtures v0.0.1 released](https://github.com/r3bl-org/r3bl-open-core/blob/main/CHANGELOG.md#r3bl_test_fixtures)
* [📦 Cratery, a lightweight private cargo registry with batteries included, built for organisations, is now open-source](https://github.com/cenotelie/cratery)

### Observations/Thoughts
* [The missing parts in Cargo](https://weihanglo.tw/posts/2024/the-missing-parts-in-cargo/)
* [How HappyLock Works](https://www.botahamec.dev/blog/how-happylock-works.html)
* [Implementing a generic range parser in Rust](https://blog.veeso.dev/blog/en/implementing-a-generic-range-parser-in-rust/)
* [video] [My favorite Rust design pattern](https://www.youtube.com/watch?v=qrf52BVaZM8)
* [audio] [Fusion Engineering with Jakub Valtar](https://corrode.dev/podcast/s02e06-fusion-engineering/)
* [audio] [On the Road: RustNL & Oxidize](https://jamesmunns.com/podcast/017-on-the-road/)

### Rust Walkthroughs
* [Global Registration](https://donsz.nl/blog/global-registration/)
* [Gray-Scott with Rust](https://grayscott-with-rust-grasland-5e6591fc7054976525da4f6c87122ea76c.pages.in2p3.fr/)
* [How to make Rust Desktop App with Egui and ChatGPT](https://www.onlycoiners.com/user/steadylearner/blog/how-to-make-rust-desktop-app-with-egui-and-chatgpt)
* [Writing eBPF Tracepoint Program with Rust Aya: Tips and Example](https://yuki-nakamura.com/2024/07/06/writing-ebpf-tracepoint-program-with-rust-aya-tips-and-example/)
* [Surprises with Rust's `as` (and Python division)](https://annahope.me/blog/rust-as/)
* [Build with Naz : Box and Pin exploration in Rust](https://developerlife.com/2024/07/16/pin-box-dynamic-duo)
* [How to organize large Rust codebases](https://kerkour.com/rust-how-to-organize-large-workspaces)
* [Playing guitar tablatures in Rust](https://agourlay.github.io/ruxguitar-tablature-player/)

### Research
* [An Empirical Study of Rust-for-Linux: The Success, Dissatisfaction, and Compromise](https://www.usenix.org/conference/atc24/presentation/li-hongyu)
* [Bringing Rust to Safety-Critical Systems in Space](https://arxiv.org/abs/2405.18135)

### Miscellaneous
* [audio] [RustShip: Rust on AWS Lambda with Luciano Mammino](https://ieni.dev/2024/07/%EF%B8%8F-rust-on-aws-lambda-with-luciano-mammino-rustship-8/)

## Crate of the Week

This week's crate is [cargo-wizard](https://github.com/Kobzol/cargo-wizard), a cargo subcommand that applies profile and config templates to your Cargo project to configure it for maximum performance, fast compile times or minimal binary size.

Thanks to [Jakub Beránek](https://users.rust-lang.org/t/crate-of-the-week/2704/1322) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
* *No calls for testing were issued this week.*

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

*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

*No Calls for papers or presentations were submitted this week.*

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

385 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-09..2024-07-16

* [add AMX target-features and `x86_amx_intrinsics` feature flag](https://github.com/rust-lang/rust/pull/126639)
* [add `constness` to `TraitDef`](https://github.com/rust-lang/rust/pull/127200)
* [add classify and related methods for f16 and f128](https://github.com/rust-lang/rust/pull/127020)
* [add lint for inline asm labels that look like binary](https://github.com/rust-lang/rust/pull/126922)
* [add suggestions for possible missing `fn`, struct`, or enum` keywords](https://github.com/rust-lang/rust/pull/127419)
* [added the `xop` target-feature and the `xop_target_feature` feature gate](https://github.com/rust-lang/rust/pull/127209)
* [allows `#[diagnostic::do_not_recommend]` to supress trait impls in suggestions as well](https://github.com/rust-lang/rust/pull/127598)
* [automatically taint when reporting errors from ItemCtxt](https://github.com/rust-lang/rust/pull/127358)
* [avoid "no field" error and ICE on recovered ADT variant](https://github.com/rust-lang/rust/pull/127575)
* [avoid follow-up errors and ICEs after missing lifetime errors on data structures](https://github.com/rust-lang/rust/pull/127311)
* [check `is_ident` before `parse_ident`](https://github.com/rust-lang/rust/pull/127601)
* [clear `inner_attr_ranges` regularly](https://github.com/rust-lang/rust/pull/127477)
* [consolidate region error reporting in `rustc_infer`](https://github.com/rust-lang/rust/pull/127500)
* [coverage: restrict `ExpressionUsed` simplification to `Code` mappings](https://github.com/rust-lang/rust/pull/127758)
* [ensure floats are returned losslessly by the Rust ABI on 32-bit x86](https://github.com/rust-lang/rust/pull/123351)
* [fire `unsafe_code` lint on unsafe extern blocks](https://github.com/rust-lang/rust/pull/127535)
* [fix `DebugParser`](https://github.com/rust-lang/rust/pull/127273)
* [fix import suggestion ice](https://github.com/rust-lang/rust/pull/127310)
* [fix incorrect NDEBUG handling in LLVM bindings](https://github.com/rust-lang/rust/pull/127654)
* [fix interleaved output in the default panic hook when multiple threads panic simultaneously](https://github.com/rust-lang/rust/pull/127397)
* [fix regression in the MIR lowering of or-patterns](https://github.com/rust-lang/rust/pull/127028)
* [gate the type length limit check behind a nightly flag](https://github.com/rust-lang/rust/pull/127670)
* [generalize `fn allocator` for Rc/Arc](https://github.com/rust-lang/rust/pull/124980)
* [generalize search graph to enable fuzzing](https://github.com/rust-lang/rust/pull/127627)
* [guard against calling `libc::exit` multiple times on Linux](https://github.com/rust-lang/rust/pull/126606)
* [implement simple, unstable lint to suggest turning closure-of-async-block into async-closure](https://github.com/rust-lang/rust/pull/127097)
* [instantiate higher ranked goals in candidate selection again](https://github.com/rust-lang/rust/pull/127568)
* [make `visit_clobber`'s impl safe](https://github.com/rust-lang/rust/pull/127560)
* [make parse error suggestions verbose and fix spans](https://github.com/rust-lang/rust/pull/127407)
* [make sure that labels are defined after the primary span in diagnostics](https://github.com/rust-lang/rust/pull/127591)
* [mark `builtin_syntax` as internal](https://github.com/rust-lang/rust/pull/127622)
* [migration lint for `expr2024` for the edition 2024](https://github.com/rust-lang/rust/pull/125627)
* [more suggestion for converting `Option<&Vec<T>>` to `Option<&[T]>`](https://github.com/rust-lang/rust/pull/127596)
* [more trait error reworking](https://github.com/rust-lang/rust/pull/127495)
* [only track mentioned places for jump threading](https://github.com/rust-lang/rust/pull/127087)
* [suggest borrowing on fn argument that is `impl AsRef`](https://github.com/rust-lang/rust/pull/124599)
* [suggest using `map_or` when `Option<&T>::unwrap_or where T: Deref` fails](https://github.com/rust-lang/rust/pull/127629)
* [suggest using precise capturing for hidden type that captures region](https://github.com/rust-lang/rust/pull/127619)
* [use verbose style when suggesting changing `const` with `let`](https://github.com/rust-lang/rust/pull/127382)
* [miri: TB: reserved + Protected + IM + lazy is a horrible combination that should not exist](https://github.com/rust-lang/miri/pull/3742)
* [miri: implement support for multiple TLS destructors on macOS](https://github.com/rust-lang/miri/pull/3739)
* [miri: implement the `os_unfair_lock` functions on macOS](https://github.com/rust-lang/miri/pull/3745)
* [fix `Parser::look_ahead`](https://github.com/rust-lang/rust/pull/127636)
* [stabilize `const_cstr_from_ptr (CStr::from_ptr, CStr::count_bytes)`](https://github.com/rust-lang/rust/pull/127433)
* [stabilize `io_slice_advance`](https://github.com/rust-lang/rust/pull/127661)
* [stabilize const unchecked conversion from u32 to char](https://github.com/rust-lang/rust/pull/126958)
* [add `f16` and `f128` as simd types in LLVM](https://github.com/rust-lang/rust/pull/127487)
* [add cache for `allocate_str`](https://github.com/rust-lang/rust/pull/127638)
* [`offset_from`: always allow pointers to point to the same address](https://github.com/rust-lang/rust/pull/124921)
* [std: `#![deny(unsafe_op_in_unsafe_fn)]` in platform-independent code](https://github.com/rust-lang/rust/pull/127744)
* [std: removes logarithms family function edge cases handling for solaris](https://github.com/rust-lang/rust/pull/127719)
* [impl FusedIterator and a size hint for the error sources iter](https://github.com/rust-lang/rust/pull/127091)
* [use `pidfd_spawn` for faster process spawning when a PidFd is requested](https://github.com/rust-lang/rust/pull/126827)
* [make os/windows and pal/windows default to `#![deny(unsafe_op_in_unsafe_fn)]`](https://github.com/rust-lang/rust/pull/127750)
* [windows: add experimental support for linking std-required system DLLs using raw-dylib](https://github.com/rust-lang/rust/pull/127370)
* [windows: remove some unnecessary type aliases](https://github.com/rust-lang/rust/pull/127712)
* [exposing STARTUPINFOW.wShowWindow in CommandExt trait](https://github.com/rust-lang/rust/pull/126690)
* [cargo: `docs(ref)`: Note MSRV for features in the docs](https://github.com/rust-lang/cargo/pull/14224)
* [cargo: add `cargo_test` to test-support prelude](https://github.com/rust-lang/cargo/pull/14243)
* [cargo: overrides: Don't warn on duplicate packages from using '..'](https://github.com/rust-lang/cargo/pull/14234)
* [cargo: source: Don't warn about unreferenced duplicate packages](https://github.com/rust-lang/cargo/pull/14239)
* [cargo: test: Redact elapsed time in the minutes time frame](https://github.com/rust-lang/cargo/pull/14233)
* [cargo: test: Reduce over-prescription to the caller](https://github.com/rust-lang/cargo/pull/14217)
* [cargo: add workflow to publish Cargo automatically](https://github.com/rust-lang/cargo/pull/14202)
* [cargo: fix: ensure dep/feature activates the dependency on 2024](https://github.com/rust-lang/cargo/pull/14221)
* [cargo: fix: rename to `rustdoc::broken_intra_doc_links`](https://github.com/rust-lang/cargo/pull/14215)
* [cargo: refactor: move `get_source_id` out of registry](https://github.com/rust-lang/cargo/pull/14218)
* [clippy: `unwrap_or_default`: skip warning when calling inside of suggested method's implementation](https://github.com/rust-lang/rust-clippy/pull/13090)
* [clippy: add more doc-valid-idents](https://github.com/rust-lang/rust-clippy/pull/13093)
* [clippy: fix `manual_unwrap_or` false positive](https://github.com/rust-lang/rust-clippy/pull/13061)
* [clippy: fix `needless_option_as_deref` false-positive on `struct` literals](https://github.com/rust-lang/rust-clippy/pull/13102)
* [clippy: fix and rename `overflow_check_conditional`](https://github.com/rust-lang/rust-clippy/pull/12944)
* [clippy: fix guidance of `float_cmp` and `float_cmp_const` to not incorrectly recommend `f__::EPSILON` as the error margin](https://github.com/rust-lang/rust-clippy/pull/13079)
* [clippy: make `or_fun_call` recursive](https://github.com/rust-lang/rust-clippy/pull/13085)
* [clippy: fix `doc_lazy_continuation` lints](https://github.com/rust-lang/cc-rs/pull/1153)
* [rust-analyzer: add `f16` and `f128` support](https://github.com/rust-lang/rust-analyzer/pull/17572)
* [rust-analyzer: encode ident rawness and literal kind separately in `tt::Leaf`](https://github.com/rust-lang/rust-analyzer/pull/17559)
* [rust-analyzer: add incorrect case diagnostics for `enum` variant fields and all variables/params](https://github.com/rust-lang/rust-analyzer/pull/17588)
* [rust-analyzer: add inlay hints for generic parameters](https://github.com/rust-lang/rust-analyzer/pull/17544)
* [rust-analyzer: do not add new `enum` if it already exists](https://github.com/rust-lang/rust-analyzer/pull/17571)
* [rust-analyzer: fix incorrect encoding of literals in the proc-macro-api on version 4](https://github.com/rust-lang/rust-analyzer/pull/17601)
* [rust-analyzer: implement symbol interning infra](https://github.com/rust-lang/rust-analyzer/pull/17584)
* [rust-analyzer: trigger VSCode to rename after extract variable assist is applied](https://github.com/rust-lang/rust-analyzer/pull/17587)
* [rustfmt: impl `StyleEditionDefault` trait for all configs](https://github.com/rust-lang/rustfmt/pull/5937)
* [rustfmt: return RewriteResult for `rewrite_block` and `rewrite_closure`](https://github.com/rust-lang/rustfmt/pull/6235)

### Rust Compiler Performance Triage

Fairly quiet week with the only pure regressions being small and coming from correctness fixes. The biggest single change came from turning off the `-Zenforce-type-length-limit` check which had positive impacts across many different benchmarks since the compiler is doing strictly less work.

Triage done by **@rylev**.
Revision range: [a2d58197..5572759b](https://perf.rust-lang.org/?start=a2d58197a766085856504328948c89a33a6a36e8&end=5572759b8d7012fa34eba47f4885c76fa06d9251&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.3%  | [0.2%, 0.7%]    | 12    |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.2%, 0.9%]    | 45    |
| Improvements ✅ <br /> (primary)   | -0.7% | [-1.5%, -0.2%]  | 37    |
| Improvements ✅ <br /> (secondary) | -3.3% | [-13.5%, -0.4%] | 21    |
| All ❌✅ (primary)                 | -0.4% | [-1.5%, 0.7%]   | 49    |


2 Regressions, 3 Improvements, 2 Mixed; 1 of them in rollups
56 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/d86903679ac12804e7b15d9007e2539c0b541dc6/triage/2024-07-16.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Match ergonomics 2024](https://github.com/rust-lang/rfcs/pull/3627)
* [Return type notation (RTN)](https://github.com/rust-lang/rfcs/pull/3654)
* [#[derive(SmartPointer)]](https://github.com/rust-lang/rfcs/pull/3621)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
*No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Fix supertrait associated type unsoundness](https://github.com/rust-lang/rust/pull/126090)
* [disposition: merge] [Graciously handle `Drop` impls introducing more generic parameters than the ADT](https://github.com/rust-lang/rust/pull/127220)
* [disposition: merge] [Reorder trait bound modifiers *after* `for<...>` binder in trait bounds](https://github.com/rust-lang/rust/pull/127054)
* [disposition: merge] [size_of_val_raw: for length 0 this is safe to call](https://github.com/rust-lang/rust/pull/126152)
* [disposition: merge] [Implement lint against ambiguous negative literals](https://github.com/rust-lang/rust/pull/121364)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC for project goals](https://github.com/rust-lang/rfcs/pull/3672)

## Upcoming Events

Rusty Events between 2024-07-17 - 2024-08-14 🦀

### Virtual
* 2024-07-17| Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/302139632/)
* [**Rust for Rustaceans Book Club: Chapter 10: Concurrency (and Parallelism)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-07-17 | Hybrid - Virtual and In-person (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298488824/)
* 2024-07-18 | Virtual (IL) | [Rust in Israel](https://www.meetup.com/rust-in-israel/)
    * [**Threads Rust (Virtual) - תהליכונים בראסט - מפגש בזום**](https://www.meetup.com/rust-in-israel/events/302219468/)
* 2024-07-18 | Virtual (Rotterdam, NL)| [Bevy Game Development](https://www.meetup.com/bevy-game-development/)
    * [**Bevy Meetup #5**](https://www.meetup.com/bevy-game-development/events/301711262/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/301062840/)
* 2024-07-24 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn: Exploring Rust API Use Cases**](https://www.meetup.com/women-in-rust/events/301730780/)
* 2024-07-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897865/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-27 | Virtual | [Leptos Monthly Meetup](https://lu.ma/user/leptos)
    * [**Leptos Monthly Meetup: Pavex with Luca Palmieri**](https://lu.ma/3ouqapsr)
* 2024-07-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/301585665/)
* 2024-07-31 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Command Line Tools: Implementing wc in Rust (English, Virtual)**](https://www.meetup.com/code-mavens/events/302151487/)
* 2024-08-01 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633265/)
* 2024-08-06 | Virtual | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Lunch & Learn! (Virtual)**](https://www.meetup.com/women-in-rust/events/300994574/)
* 2024-08-06 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191718/)
* 2024-08-06 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Web development in Rust using Rocket - part 2 (English)**](https://www.meetup.com/code-mavens/events/301736709/)
* 2024-08-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328027/)
* 2024-08-08 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897918/)
* 2024-08-08 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300787793/)
* 2024-08-13 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346978/)

### Africa
* 2024-08-02 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)

### Asia
* 2024-07-20 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**July 2024 Rustacean meetup 🤝 C4GT**](https://hasgeek.com/rustbangalore/july-2024-rustacean-meetup-c4gt/)

### Europe
* 2024-07-17 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/302024746/)
* 2024-07-18 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Rust Bern Meetup #3 2024**](https://www.meetup.com/rust-bern/events/301952761/)
* 2024-07-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester July Code Night**](https://www.meetup.com/rust-manchester/events/301461206/)
* 2024-07-23 | Hybrid - Virtual and In-Person (München/Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-07-25 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/de-DE/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #8**](https://www.meetup.com/rust-meetup-augsburg/events/301642385/)
* 2024-07-25 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288967/)
* 2024-07-27 | Hybrid - Virtual and In-Person (Kyiv, UA) | [UA Rust](https://uarust.com/)
    * [**UARust Conference 2024**](https://uarust.com/)
* 2024-07-30 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #9**](https://www.meetup.com/rust-basel/events/301459503/)
* 2024-08-14 | Reading, UK | [Reading Rust Workshop](https://rustworkshop.co/meetup/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/302153005/)

### North America
* 2024-07-17 | Hybrid - Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631734/)
* 2024-07-18 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers : holding pattern**](https://www.meetup.com/music-city-rust-developers/events/301411794/)
* 2024-07-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/301883176/)
* 2024-07-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Back Bay Rust Lunch, July 21**](https://www.meetup.com/bostonrust/events/301550076/)
* 2024-07-24 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygckbgc/)
* 2024-07-25 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302066981/)
* 2024-07-29 | Cambridge, MA, US| [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch, July 29**](https://www.meetup.com/bostonrust/events/301550289/)
* 2024-08-01 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/)
    * [**Lifetimes**](https://www.meetup.com/stl-rust/events/301697569/)
* 2024-08-08 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/302067008/)

# Oceania
* 2024-08-01 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**August Meetup**](https://www.meetup.com/rust-brisbane/events/302244260/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1dvlhl6/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> I have a dream. A dream that Cargo has its own release cadence, so it is free from the strict stability curse and can then ship major version releases.

– [Weihang Lo on their blog](https://weihanglo.tw/posts/2024/the-missing-parts-in-cargo/)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1591) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1e63esr/this_week_in_rust_556/)</small>
