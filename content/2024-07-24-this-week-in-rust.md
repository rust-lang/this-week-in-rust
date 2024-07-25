Title: This Week in Rust 557
Number: 557
Date: 2024-07-24
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

### Foundation
* [Rust Foundation Fellowship Grants Program 2024](https://foundation.rust-lang.org/grants/fellowships/)

### Newsletters
* [thisweekinbevy - Bevy Jam 5, Radiance Cascades, and Calculators in many ui kits](https://thisweekinbevy.com/issue/2024-07-22-bevy-jam-5-radiance-cascades-and-calculators-in-many-ui-kits)

### Project/Tooling Updates
* [Slint 1.7 Released with New Widgets, Multi-Window Support, and Live-Preview Redesign](https://slint.dev/blog/slint-1.7-released)
* [Diesel Async 0.5](https://blog.weiznich.de/blog/diesel-async-0-5/)
* [iroh 0.21.0 - Fix, clean & polish](https://iroh.computer/blog/iroh-0-21-fix-clean-polish)
* [gitoxide [June 2024]](https://github.com/Byron/gitoxide/discussions/1459)
* [Iced v0.12 Tutorial - Asynchronous actions with Commands](https://leafheap.com/articles/iced-v0-12-tutorial-asynchronous-actions-with-commands)
* [Query.rs - A search engine for Rust](https://query.rs/)

### Observations/Thoughts
* [without.boats - Pin](https://without.boats/blog/pin/)
* [without.boats - Pinned places](https://without.boats/blog/pinned-places/)
* [RocksDB: Not A Good Choice for a High-Performance Streaming Platform](https://www.feldera.com/blog/rocksdb-not-a-good-choice-for-high-performance-streaming/)
* [A type system for RCL: Implementing a typechecker in Rust](https://ruudvanasseldonk.com/2024/implementing-a-typechecker-for-rcl-in-rust)
* [Beating the compiler](https://www.mattkeeter.com/blog/2024-07-12-interpreter/)
* [Deconstructing the Role-Playing Video Game](https://olano.dev/blog/deconstructing-the-role-playing-videogame/)
* [the spatula](https://www.thespatula.io/rust/rust_websocket/)
* [Named Arguments In Rust, If You Want Them](https://rtpg.co/2024/07/22/rust-named-arguments/)
* [WebAssembly on the server: Compiling Rust to WASM and executing it from Go](https://blog.arcjet.com/webassembly-on-the-server-compiling-rust-to-wasm-and-executing-it-from-go/)
* [Async Rust: The new billion-dollar mistake?](https://kerkour.com/rust-async-billion-dollar-mistake)
* [Nine Rust Cargo.toml Wats and Wat Nots: Master Cargo.toml formatting rules and avoid frustration](https://towardsdatascience.com/nine-rust-cargo-toml-wats-and-wat-nots-1e5e02e41648)

### Rust Walkthroughs
* [Plugins With Rust and WASI Preview 2](https://benw.is/posts/plugins-with-rust-and-wasi)
* [Build your own SQLite, Part 1: Listing tables](https://blog.sylver.dev/build-your-own-sqlite-part-1-listing-tables)

### Miscellaneous
* [Building Search In Rust](https://filtra.io/rust-meilisearch-jul-24)

* [Oxidize 2024 talks are now available on YouTube](https://www.youtube.com/playlist?list=PLilpJp3WAOveS7dwg0YNvSTfkRt9hDHrS)

## Crate of the Week

This week's crate is [diatomic-waker](https://crates.io/crates/diatomic-waker), a spinlock-less library for async task wakeup.

Thanks to [Ddystopia](https://users.rust-lang.org/t/crate-of-the-week/2704/1323) for the suggestion!

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

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

402 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-07-16..2024-07-23

* [promote the `wasm32-wasip2` target to Tier 2](https://github.com/rust-lang/rust/pull/126967)
* [CFI: support provided methods on traits](https://github.com/rust-lang/rust/pull/127295)
* [MIR building: Stop using `unpack!` for `BlockAnd<()>`](https://github.com/rust-lang/rust/pull/127472)
* [`C-cmse-nonsecure-call`: improved error messages](https://github.com/rust-lang/rust/pull/127814)
* [`macro_metavar_expr_concat` Add support for literals](https://github.com/rust-lang/rust/pull/127542)
* [`macro_metavar_expr_concat` Allow `concat` in repetitions](https://github.com/rust-lang/rust/pull/127720)
* [add Process support for UEFI](https://github.com/rust-lang/rust/pull/123196)
* [add missing `try_new_uninit_slice_in` and `try_new_zeroed_slice_in`](https://github.com/rust-lang/rust/pull/127415)
* [change `binary_asm_labels` to only fire on x86 and `x86_64`](https://github.com/rust-lang/rust/pull/127935)
* [cleanup dll/exe filename calculations in `run_make_support`](https://github.com/rust-lang/rust/pull/127960)
* [conditionally build `wasm-component-ld`](https://github.com/rust-lang/rust/pull/127866)
* [deal with invalid UTF-8 from `gai_strerror`](https://github.com/rust-lang/rust/pull/127583)
* [delegation: support coercion for target expression](https://github.com/rust-lang/rust/pull/126699)
* [deny keyword lifetimes pre-expansion](https://github.com/rust-lang/rust/pull/126762)
* [don't allow unsafe statics outside of extern blocks](https://github.com/rust-lang/rust/pull/127943)
* [don't output incremental test artifacts into working directory](https://github.com/rust-lang/rust/pull/128038)
* [don't use implicit features in `Cargo.toml` in `compiler/`](https://github.com/rust-lang/rust/pull/127769)
* [fix ICE in suggestion caused by `⩵` being recovered as `==`](https://github.com/rust-lang/rust/pull/127835)
* [fix a bunch of sites that were walking instead of visiting, making it impossible for visitor impls to look at these values](https://github.com/rust-lang/rust/pull/127817)
* [fix ambiguous cases of multiple & in elided self lifetimes](https://github.com/rust-lang/rust/pull/117967)
* [fix and enforce `unsafe_op_in_unsafe_fn` in compiler](https://github.com/rust-lang/rust/pull/127730)
* [fix associated item removal suggestion](https://github.com/rust-lang/rust/pull/127878)
* [fix precise capturing suggestion for hidden regions when we have APITs](https://github.com/rust-lang/rust/pull/127664)
* [fix the issue of invalid suggestion for a reference of iterator](https://github.com/rust-lang/rust/pull/127669)
* [fixes panic error `index out of bounds` in conflicting error](https://github.com/rust-lang/rust/pull/127948)
* [forbid borrows and unsized types from being used as the type of a const generic under `adt_const_params`](https://github.com/rust-lang/rust/pull/127722)
* [interpret: add sanity check in dyn upcast to double-check what codegen does](https://github.com/rust-lang/rust/pull/127856)
* [invert infer `error_reporting` mod struture](https://github.com/rust-lang/rust/pull/127501)
* [just totally fully deny late-bound consts](https://github.com/rust-lang/rust/pull/128020)
* [lazy type aliases: diagnostics: Detect bivariant ty params that are only used recursively](https://github.com/rust-lang/rust/pull/127976)
* [maintain the given order on step execution](https://github.com/rust-lang/rust/pull/127602)
* [make ErrorGuaranteed discoverable outside types, consts, and lifetimes](https://github.com/rust-lang/rust/pull/127808)
* [make `pub_use_of_private_extern_crate` show up in cargo's future breakage reports](https://github.com/rust-lang/rust/pull/127656)
* [match lowering: Use an iterator to find `expand_until`](https://github.com/rust-lang/rust/pull/127707)
* [accurate `use` rename suggestion span](https://github.com/rust-lang/rust/pull/127886)
* [more accurate span for anonymous argument suggestion](https://github.com/rust-lang/rust/pull/127889)
* [more accurate span for type parameter suggestion](https://github.com/rust-lang/rust/pull/127888)
* [use more accurate span for `addr_of!` suggestion](https://github.com/rust-lang/rust/pull/127929)
* [more accurate suggestion for `-> Box<dyn Trait>` or `-> impl Trait`](https://github.com/rust-lang/rust/pull/127987)
* [parser: suggest placing the return type after function parameters](https://github.com/rust-lang/rust/pull/127350)
* [safely enforce thread name requirements](https://github.com/rust-lang/rust/pull/127918)
* [solve a error `.clone()` suggestion when moving a mutable reference](https://github.com/rust-lang/rust/pull/127579)
* [suggest a borrow when using dbg](https://github.com/rust-lang/rust/pull/120990)
* [tweak suggestions when using incorrect type of `enum` literal](https://github.com/rust-lang/rust/pull/127891)
* [use ordinal number in argument error](https://github.com/rust-lang/rust/pull/125042)
* [when finding item gated behind a `cfg` flag, point at it](https://github.com/rust-lang/rust/pull/127662)
* [miri: add `O_NOFOLLOW` flag support](https://github.com/rust-lang/miri/pull/3744)
* [miri: add `pread` and `pwrite` shims](https://github.com/rust-lang/miri/pull/3743)
* [remove unnecessary impl sorting in queries and metadata](https://github.com/rust-lang/rust/pull/120812)
* [some parser improvements](https://github.com/rust-lang/rust/pull/127806)
* [fix least significant digits of f128 associated constants](https://github.com/rust-lang/rust/pull/127047)
* [std: use `read_unaligned` for reads from DWARF](https://github.com/rust-lang/rust/pull/127792)
* [`impl Send + Sync` and override `count` for the `CStr::bytes` iterator](https://github.com/rust-lang/rust/pull/127444)
* [`ptr::metadata`: avoid references to extern types](https://github.com/rust-lang/rust/pull/127859)
* [add `isqrt` to `NonZero<uN>`](https://github.com/rust-lang/rust/pull/126199)
* [use ThreadId instead of TLS-address in `ReentrantLock`](https://github.com/rust-lang/rust/pull/124881)
* [use Option's discriminant as its size hint](https://github.com/rust-lang/rust/pull/127748)
* [use futex.rs for Windows thread parking](https://github.com/rust-lang/rust/pull/127807)
* [windows: use futex implementation for `Once`](https://github.com/rust-lang/rust/pull/125942)
* [windows: prevent double reference in generic futex](https://github.com/rust-lang/rust/pull/127813)
* [start using `#[diagnostic::do_not_recommend]` in the standard library](https://github.com/rust-lang/rust/pull/128008)
* [skip fast path for dec2flt when `optimize_for_size`](https://github.com/rust-lang/rust/pull/126271)
* [cargo toml: Improve error on missing package and workspace](https://github.com/rust-lang/cargo/pull/14261)
* [cargo: add `TomlPackage::new`, `Default` for `TomlWorkspace`](https://github.com/rust-lang/cargo/pull/14271)
* [cargo: fix passing of links-overrides with target-applies-to-host and an implicit target](https://github.com/rust-lang/cargo/pull/14205)
* [rustdoc: click target for sidebar items flush left](https://github.com/rust-lang/rust/pull/127229)
* [rustdoc: fix `current` class on sidebar modnav](https://github.com/rust-lang/rust/pull/127932)
* [rustdoc: short descriptions cause word-breaks in tables](https://github.com/rust-lang/rust/pull/128023)
* [add cross-crate precise capturing support to rustdoc](https://github.com/rust-lang/rust/pull/127658)
* [rustfmt: impl `rewrite_result` for ArmWrapper](https://github.com/rust-lang/rustfmt/pull/6239)
* [rustfmt: return RewriteResult for `rewrite_path` & `rewrite_struct_***`](https://github.com/rust-lang/rustfmt/pull/6236)
* [clippy: `pathbuf_init_then_push`: Checks for calls to `push` immediately a…](https://github.com/rust-lang/rust-clippy/pull/11700)
* [clippy: add lint for recreation of an entire `struct`](https://github.com/rust-lang/rust-clippy/pull/12772)
* [clippy: create lint passes using `Conf`](https://github.com/rust-lang/rust-clippy/pull/13088)
* [clippy: fix `excessive_precision` suggestions on floats written in scientific notation](https://github.com/rust-lang/rust-clippy/pull/13096)
* [clippy: fix wrong suggestion for `single_element_loop` where parens were missing](https://github.com/rust-lang/rust-clippy/pull/13117)
* [clippy: lint `zero_repeat_side_effects` only if array length is a literal zero](https://github.com/rust-lang/rust-clippy/pull/13116)
* [rust-analyzer: add missing dyn parse special cases in 2015 edition](https://github.com/rust-lang/rust-analyzer/pull/17646)
* [rust-analyzer: allow macro expansions into `RestPat` in tuple args work as ellipsis like plain `RestPat`](https://github.com/rust-lang/rust-analyzer/pull/17586)
* [rust-analyzer: avoid ref when using format! in compiler](https://github.com/rust-lang/rust-analyzer/pull/17641)
* [rust-analyzer: add inlay hint support for block expr with lifetime label](https://github.com/rust-lang/rust-analyzer/pull/17635)
* [rust-analyzer: edition aware parser](https://github.com/rust-lang/rust-analyzer/pull/17620)
* [rust-analyzer: go-to-def and find-references on control-flow keywords](https://github.com/rust-lang/rust-analyzer/pull/17542)
* [rust-analyzer: feature: teach rust-analyzer to discover `linked_projects`](https://github.com/rust-lang/rust-analyzer/pull/17246)
* [rust-analyzer: fix incorrect generic parameter hint defaults](https://github.com/rust-lang/rust-analyzer/pull/17616)
* [rust-analyzer: fix path resolution for child mods of those expanded by `include!`](https://github.com/rust-lang/rust-analyzer/pull/17650)
* [rust-analyzer: allow flyimport to import primitive shadowing modules](https://github.com/rust-lang/rust-analyzer/pull/17656)
* [rust-analyzer: don't call `macro_arg` directly in `ExpandDatabase::syntax_context`](https://github.com/rust-lang/rust-analyzer/pull/17611)
* [rust-analyzer: fix more path resolution for included submodules](https://github.com/rust-lang/rust-analyzer/pull/17660)
* [rust-analyzer: handle synonymous imports with different renaming in 'merge imports'](https://github.com/rust-lang/rust-analyzer/pull/17622)
* [rust-analyzer: panic in debug profile for tuple deconstruct with arity mismatch](https://github.com/rust-lang/rust-analyzer/pull/17649)
* [rust-analyzer: remove incorrect never! invocations](https://github.com/rust-lang/rust-analyzer/pull/17668)
* [rust-analyzer: more `find_path` improvements](https://github.com/rust-lang/rust-analyzer/pull/17655)
* [rust-analyzer: more symbol usage](https://github.com/rust-lang/rust-analyzer/pull/17604)
* [rust-analyzer: parse contextual dyn keyword properly in edition 2015](https://github.com/rust-lang/rust-analyzer/pull/17640)
* [rust-analyzer: reduce memory usage of salsa slots by 8 bytes](https://github.com/rust-lang/rust-analyzer/pull/17638)
* [rust-analyzer: prefer standard library paths over shorter extern deps re-exports](https://github.com/rust-lang/rust-analyzer/pull/17653)
* [rust-analyzer: set `RUSTC_TOOLCHAIN` for runnables](https://github.com/rust-lang/rust-analyzer/pull/17605)
* [rust-analyzer: some more small salsa memory improvements](https://github.com/rust-lang/rust-analyzer/pull/17639)
* [rust-analyzer: support `rustc_skip_during_method_dispatch`](https://github.com/rust-lang/rust-analyzer/pull/17618)
* [rust-analyzer: switch token trees to use Symbols](https://github.com/rust-lang/rust-analyzer/pull/17603)

### Rust Compiler Performance Triage

Light week, with few changes on any dimension (including memory), though
overall a slight regression.

Triage done by **@simulacrum**.
Revision range: [5572759b..9629b90b](https://perf.rust-lang.org/?start=5572759b8d7012fa34eba47f4885c76fa06d9251&end=9629b90b3f7dd8f5626ec9d3b42556f39f09e214&absolute=false&stat=instructions%3Au)

1 Regression, 1 Improvement, 1 Mixed; 2 of them in rollups
34 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-07-21.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: not specified] [RFC for project goals](https://github.com/rust-lang/rfcs/pull/3672)
* [disposition: merge] [Promote aarch64-apple-darwin to Tier 1](https://github.com/rust-lang/rfcs/pull/3671)
* [disposition: merge] [add float semantics RFC](https://github.com/rust-lang/rfcs/pull/3514)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for nested field access in offset_of](https://github.com/rust-lang/rust/issues/120140)
* [disposition: merge] [[rustdoc] Add copy code feature](https://github.com/rust-lang/rust/pull/125779)
* [disposition: merge] [Stabilize `const {integer}::from_str_radix` i.e. `const_int_from_str`](https://github.com/rust-lang/rust/pull/124941)
* [disposition: merge] [Scoped thread implicit join doesn't wait for thread locals to be dropped](https://github.com/rust-lang/rust/issues/116237)

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

Rusty Events between 2024-07-24 - 2024-08-21 🦀

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
* 2024-08-08 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Source Code Reading: The thousands crate (English)**](https://www.meetup.com/code-mavens/events/302391142/)
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

> Rust doesn't give you good errors, it gives you control over errors.

– [cameronm1024 on r/rust](https://www.reddit.com/r/rust/comments/1e978l7/ive_used_and_loved_rust_for_10_years_here_are_the/lecp79z/)

Despite a woeful dearth of suggestions, llogiq is content with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1ebhawb/this_week_in_rust_557/)</small>
