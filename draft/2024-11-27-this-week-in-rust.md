Title: This Week in Rust 575
Number: 575
Date: 2024-11-27
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
* [Rust 2024 call for testing](https://blog.rust-lang.org/2024/11/27/Rust-2024-public-testing.html)

### Foundation

### Newsletters
* [The Embedded Rustacean Issue #33](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-33)

### Project/Tooling Updates
* [Announcing Nio](https://nurmohammed840.github.io/posts/announcing-nio/)
* [Announcing rust-query](https://blog.lucasholten.com/rust-query-announcement/)
* ["pigg" (the Raspberry Pi GPIO GUI) 0.5.0 release](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.5.0) of the project for Remote control and viewing of Raspberry Pi GPIO hardware, now with Pi Pico W support via rust and embassy

### Observations/Thoughts
* [A Novel Idea About `Functor` In Rust?](https://wolfgirl.dev/blog/2024-11-24-a-novel-idea-about-functor-in-rust/)
* [Optimizing a Rust GPU matmul kernel](https://rust-gpu.github.io/blog/optimizing-matmul/)
* [Map Keys and Lifetimes](https://blinsay.com/blog/compound-keys/)
* [Rustlantis: Randomized Differential Testing of the Rust Compiler](https://plf.inf.ethz.ch/research/oopsla24-rustlantis.html)
* [Undroppable Types](https://jack.wrenn.fyi/blog/undroppable/)
* [40-year-old hacker group prefers Rust](https://blog.rust.careers/post/40y_old_hacking_group_rust_veilid/)
* [Cursed Linear Types In Rust](https://geo-ant.github.io/blog/2024/rust-linear-types-use-once/)
* [Unsafe for work](https://oida.dev/unsafe-for-work/)

* [Aligning with rust's Guiding Principle: my path to finding my 'Why'](https://flakm.com/posts/rust_guiding_principle/)

### Rust Walkthroughs
* [Building Thread-safe Async Primitives in 150 lines of Rust](https://amit.prasad.me/blog/async-oneshot)
* [Securely sending DHT22 sensor data from an ESP32 board to PostgreSQL](https://c410-f3r.github.io/thoughts/securely-sending-dht22-sensor-data-from-an-esp32-board-to-postgresql/)
* [Writing a mark-and-sweep tracing GC in Rust](https://elric.pl/blog/tracing-gc/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [postcard](https://docs.rs/postcard), a battle-tested, well-documented `#[no_std]` compatible serializer/deserializer geared towards use in embedded devices.

Thanks to [Reto Trappitsch](https://users.rust-lang.org/t/crate-of-the-week/2704/1377) for the suggestion!

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
### [RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)

### [Rust](https://github.com/rust-lang/rust/labels/call-for-testing)

### [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing)

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Rust in Paris 2025**](https://docs.google.com/forms/d/e/1FAIpQLSdamzdbUi3EIGBrmEw0-Na4myXP0088kvxVmVT4YU-1BEiyCg/viewform) | Deadline: 30 November 2024 | Paris, FR | 15 March 2025

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

* [FOSDEM 2025 Rust devroom](https://rust-fosdem.github.io) | Deadline: 1 December 2024 | Brussels, BE | 1 February 2025

## Updates from the Rust Project

405 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-11-19..2024-11-26

* [`lints_that_dont_need_to_run`: never skip future-compat-reported lints](https://github.com/rust-lang/rust/pull/133108)
* [aarch64 softfloat target: always pass floats in int registers](https://github.com/rust-lang/rust/pull/133102)
* [account for `wasm32v1-none` when exporting TLS symbols](https://github.com/rust-lang/rust/pull/133244)
* [add `visit` methods to ast nodes that already have `walk`s on ast visitors](https://github.com/rust-lang/rust/pull/133188)
* [add specific diagnostic for using `macro_rules` macro as attribute/derive](https://github.com/rust-lang/rust/pull/132949)
* [add visits to nodes that already have `flat_maps` in `ast::MutVisitor`](https://github.com/rust-lang/rust/pull/133153)
* [allow disabling ASan instrumentation for globals](https://github.com/rust-lang/rust/pull/127483)
* [bail in effects in old solver if self ty is ty var](https://github.com/rust-lang/rust/pull/133323)
* [btree: don't leak value if destructor of key panics](https://github.com/rust-lang/rust/pull/132597)
* [constify the `Deref`/`DerefMut` traits, too](https://github.com/rust-lang/rust/pull/133260)
* [continue `ParamEnv` to `TypingEnv` transition](https://github.com/rust-lang/rust/pull/133212)
* [correct the tier listing of `wasm32-wasip2`](https://github.com/rust-lang/rust/pull/133213)
* [default-enable `llvm_tools_enabled` when no `config.toml` is present](https://github.com/rust-lang/rust/pull/133207)
* [distinguish overflow and unimplemented in `Step::steps_between`](https://github.com/rust-lang/rust/pull/130867)
* [don't allow `-Zunstable-options` to take a value](https://github.com/rust-lang/rust/pull/133159)
* [drop debug info instead of panicking if we exceed LLVM's capability to represent it](https://github.com/rust-lang/rust/pull/133194)
* [emscripten: link with `-sWASM_BIGINT`](https://github.com/rust-lang/rust/pull/131736)
* [fix LLVM target triple for `x86_64-win7-windows-msvc`](https://github.com/rust-lang/rust/pull/133239)
* [fix asm goto with outputs and move it to a separate feature gate](https://github.com/rust-lang/rust/pull/131523)
* [fix closure arg extraction in `extract_callable_info`, generalize it to async closures](https://github.com/rust-lang/rust/pull/132489)
* [implement `~const Destruct` effect goal in the new solver](https://github.com/rust-lang/rust/pull/132329)
* [implement `~const Fn` trait goal in the new solver](https://github.com/rust-lang/rust/pull/133216)
* [implement `~const` item bounds in RPIT](https://github.com/rust-lang/rust/pull/133218)
* [implement the unsafe-fields RFC](https://github.com/rust-lang/rust/pull/132915)
* [make `PointerLike` opt-in instead of built-in](https://github.com/rust-lang/rust/pull/133226)
* [make asm label blocks safe context](https://github.com/rust-lang/rust/pull/131544)
* [merge `-Zhir-stats` into `-Zinput-stats`](https://github.com/rust-lang/rust/pull/133023)
* [point at `const` definition when used instead of a binding in a `let` statement](https://github.com/rust-lang/rust/pull/132708)
* [pretty print async fn sugar in opaques and trait bounds](https://github.com/rust-lang/rust/pull/132911)
* [reduce false positives of tail-expr-drop-order from consumed values (attempt #2)](https://github.com/rust-lang/rust/pull/131326)
* [refactor `where` predicates, and reserve for attributes support](https://github.com/rust-lang/rust/pull/132894)
* [remove `is_trivially_const_drop`](https://github.com/rust-lang/rust/pull/133371)
* [remove legacy bitcode for iOS](https://github.com/rust-lang/rust/pull/133297)
* [report the `unexpected_cfgs` lint in external macros](https://github.com/rust-lang/rust/pull/132577)
* [rustc: fail fast when compiling a source file larger than 4 GiB](https://github.com/rust-lang/rust/pull/132791)
* [show `abi_unsupported_vector_types` lint in future breakage reports](https://github.com/rust-lang/rust/pull/133374)
* [stop being so bail-y in candidate assembly](https://github.com/rust-lang/rust/pull/132090)
* [store resolution for self and crate root module segments](https://github.com/rust-lang/rust/pull/132207)
* [unstable feature usage metrics](https://github.com/rust-lang/rust/pull/130236)
* [use `ConstArgKind::Path` for all single-segment paths, not just params under `min_generic_const_args`](https://github.com/rust-lang/rust/pull/131081)
* [use `confstr(_CS_DARWIN_USER_TEMP_DIR, ...)` as a `TMPDIR` fallback on Darwin](https://github.com/rust-lang/rust/pull/131505)
* [use arc4random of libc for RTEMS target](https://github.com/rust-lang/rust/pull/133313)
* [use attributes for `dangling_pointers_from_temporaries` lint](https://github.com/rust-lang/rust/pull/132732)
* [interpret: do not ICE when a promoted fails with OOM](https://github.com/rust-lang/rust/pull/133164)
* [miri: added epoll and eventfd for Android](https://github.com/rust-lang/miri/pull/4016)
* [miri: eventfd: comment tweaks](https://github.com/rust-lang/miri/pull/4047)
* [miri: fill out windows io error mapping table](https://github.com/rust-lang/miri/pull/4046)
* [miri: follow-up on #4052, making a miri evaluation context fn for `strerror_r`](https://github.com/rust-lang/miri/pull/4054)
* [miri: implement square root without relying on host floats](https://github.com/rust-lang/miri/pull/4026)
* [miri: refactor `AnonSocket::read/write` for blocking socketpair](https://github.com/rust-lang/miri/pull/4037)
* [miri: simplify thread blocking tests](https://github.com/rust-lang/miri/pull/4059)
* [miri: sysconf adding few more constants](https://github.com/rust-lang/miri/pull/4053)
* [miri: sysconf interception fix for solarish systems](https://github.com/rust-lang/miri/pull/4052)
* [miri: trophy case: add `RwLock::downgrade` bug](https://github.com/rust-lang/miri/pull/4042)
* [miri: use `PathBuf` APIs to correctly do some path manipulation cross-platform](https://github.com/rust-lang/miri/pull/4061)
* [resolve tweaks](https://github.com/rust-lang/rust/pull/132761)
* [finish `Reveal` removal](https://github.com/rust-lang/rust/pull/133242)
* [stabilize the 2024 edition](https://github.com/rust-lang/rust/pull/133349)
* [stabilize `Ipv6Addr::is_unique_local` and `Ipv6Addr::is_unicast_link_local`](https://github.com/rust-lang/rust/pull/129238)
* [stabilize `const_float_methods`](https://github.com/rust-lang/rust/pull/133389)
* [stabilize `const_pin_2`](https://github.com/rust-lang/rust/pull/131904)
* [minimally constify `Add`](https://github.com/rust-lang/rust/pull/133237)
* [mark `<[T; N]>::as_mut_slice` with the `const` specifier](https://github.com/rust-lang/rust/pull/133332)
* [mark `get_mut` and `set_position` in `std::io::Cursor` as const](https://github.com/rust-lang/rust/pull/130800)
* [reduce integer `Display` implementation size](https://github.com/rust-lang/rust/pull/133247)
* [std: allow after-main use of synchronization primitives](https://github.com/rust-lang/rust/pull/132730)
* [implement `OsString::truncate`](https://github.com/rust-lang/rust/pull/133264)
* [add `AsyncFn*` to the prelude in all editions](https://github.com/rust-lang/rust/pull/132611)
* [add `BorrowedBuf::into_filled{,_mut}` methods to allow returning buffer with original lifetime](https://github.com/rust-lang/rust/pull/132533)
* [add `std::thread::add_spawn_hook`](https://github.com/rust-lang/rust/pull/125405)
* [add `vec_deque::Iter::as_slices` and friends](https://github.com/rust-lang/rust/pull/123947)
* [support `each_ref` and `each_mut` in `[T; N]` in constant expressions](https://github.com/rust-lang/rust/pull/133288)
* [support input/output in vector registers of s390x inline assembly (under `asm_experimental_reg` feature)](https://github.com/rust-lang/rust/pull/131664)
* [support s390x z13 vector ABI](https://github.com/rust-lang/rust/pull/131586)
* [uefi: process: add args support](https://github.com/rust-lang/rust/pull/129838)
* [hashbrown: release v0.15.2](https://github.com/rust-lang/hashbrown/pull/587)
* [cargo: `test(rustflags)`: Verify -Cmetadata directly, not through -Cextra-filename](https://github.com/rust-lang/cargo/pull/14846)
* [cargo: allow registries to omit empty/default fields in JSON](https://github.com/rust-lang/cargo/pull/14838)
* [cargo: check build target supports std when building with -Zbuild-std=std](https://github.com/rust-lang/cargo/pull/14183)
* [cargo: docs for optional registry JSON fields](https://github.com/rust-lang/cargo/pull/14839)
* [cargo: feat: stabilize Edition 2024](https://github.com/rust-lang/cargo/pull/14828)
* [cargo: improve error handling when PathSource is relative](https://github.com/rust-lang/cargo/pull/14854)
* [cargo: test: address test output nondeterminism](https://github.com/rust-lang/cargo/pull/14855)
* [cargo: test: switch from `'exec_with_output'` to 'run'](https://github.com/rust-lang/cargo/pull/14848)
* [rustdoc: do not call `to_string,` it's already impl Display](https://github.com/rust-lang/rust/pull/133398)
* [bindgen: add `raw_ref_macros` feature](https://github.com/rust-lang/rust-bindgen/pull/2988)
* [clippy: add new lint `doc_include_without_cfg`](https://github.com/rust-lang/rust-clippy/pull/13625)
* [clippy: add note about caveat for `cfg(doc)`](https://github.com/rust-lang/rust-clippy/pull/13724)
* [clippy: don't consider lifetimes in bounded types unused (fix `extra_unused_lifetimes` FP)](https://github.com/rust-lang/rust-clippy/pull/13583)
* [clippy: sync and Release automation](https://github.com/rust-lang/rust-clippy/pull/13694)
* [clippy: use a better message for `unnecessary_map_or` lint](https://github.com/rust-lang/rust-clippy/pull/13708)
* [rust-analyzer: convert `add_braces` to SyntaxFactory SyntaxEditor abstraction](https://github.com/rust-lang/rust-analyzer/pull/18485)
* [rust-analyzer: use snippet placeholders for generated match arms](https://github.com/rust-lang/rust-analyzer/pull/18459)
* [rust-analyzer: fix a stack overflow when computing the sizedness of a `struct` that includes itself as the tail field](https://github.com/rust-lang/rust-analyzer/pull/18559)
* [rust-analyzer: improve selection handling for the `merge_match_arms` assist](https://github.com/rust-lang/rust-analyzer/pull/18529)
* [rust-analyzer: migrate `reorder_impl_items` Assist to Use `SyntaxFactory`](https://github.com/rust-lang/rust-analyzer/pull/18521)

### Rust Compiler Performance Triage

This week saw more regressions than improvements, mostly due to three PRs that performed internal
refactorings that are necessary for further development and modification of the compiler.

Triage done by **@kobzol**.
Revision range: [7d40450b..7db7489f](https://perf.rust-lang.org/?start=7d40450b2df92bdc9dec414b30cf5f7a5979a92e&end=7db7489f9bc274cb60c4956bfa56de0185eb1b9b&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.1%, 3.6%]   | 57    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.0%, 2.7%]   | 100   |
| Improvements ✅ <br /> (primary)   | -0.5% | [-1.5%, -0.2%] | 11    |
| Improvements ✅ <br /> (secondary) | -0.4% | [-0.5%, -0.3%] | 7     |
| All ❌✅ (primary)                 | 0.4%  | [-1.5%, 3.6%]  | 68    |


4 Regressions, 2 Improvements, 3 Mixed; 3 of them in rollups
40 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/556db980efa8c8553fe92ce64f04db372b0c7d61/triage/2024-11-26.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [disposition: merge] [[RFC] Explicit ABI in extern](https://github.com/rust-lang/rfcs/pull/3722)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Tracking Issue for `sub_ptr` (feature `ptr_sub_ptr`)](https://github.com/rust-lang/rust/issues/95892)
* [disposition: merge] [Stabilize unsigned and float variants of ｀num_midpoint｀ feature](https://github.com/rust-lang/rust/pull/131784)
* [disposition: merge] [Tracking issue for Vec::extract_if and LinkedList::extract_if](https://github.com/rust-lang/rust/issues/43244)
* [disposition: merge] [Stabilize noop_waker](https://github.com/rust-lang/rust/pull/133089)
* [disposition: merge] [Stabilize const_maybe_uninit_write](https://github.com/rust-lang/rust/pull/131713)
* [disposition: merge] [Tracking Issue for Path::file_prefix](https://github.com/rust-lang/rust/issues/86319)
* [disposition: merge] [Stabilize ｀#[diagnostic::do_not_recommend]｀](https://github.com/rust-lang/rust/pull/132056)
* [disposition: merge] [Make missing_abi lint warn-by-default.](https://github.com/rust-lang/rust/pull/132397)
* [disposition: merge] [Tracking Issue for ptr::fn_addr_eq](https://github.com/rust-lang/rust/issues/129322)
* [disposition: merge] [Fix and undeprecate home_dir()](https://github.com/rust-lang/rust/pull/132515)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [crates.io: Remove version deletions from the "crate deletions" RFC](https://github.com/rust-lang/rfcs/pull/3731)
* [new] [RFC: Add a semantically non-blocking lint level](https://github.com/rust-lang/rfcs/pull/3730)

## Upcoming Events

Rusty Events between 2024-11-27 - 2024-12-25 🦀

### Virtual
* 2024-11-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898099/)
* 2024-11-28 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-11-29 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304477903/)
* 2024-12-02 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/)
    * [**Advent of Code - Kick Off!**](https://www.meetup.com/women-in-rust/events/304668776/)
* 2024-12-03 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/302007374/)
* 2024-12-03 | Virtual (San Francisco, CA, US) | [Blockchain Center SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust in Web3: Developer Series**](https://www.meetup.com/blockchaincentersf/events/304510595/)
* 2024-12-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031652)
* 2024-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/05/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633275/)
* 2024-12-05 | Virtual (Miami, FL) | [Miami Java User Group Events](https://www.meetup.com/miami-java-user-group)
    * [**Introduction to Rust for Java Developers**](https://www.meetup.com/miami-java-user-group/events/304717903/)
* 2024-12-06 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304369723/)
* 2024-12-07 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2024-12-08 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Reading JSON files in Rust - קריאת קבצי ג'ייסון בראסט**](https://www.meetup.com/rust-tlv/events/304685546/)
* 2024-12-10 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/299346988/)
* 2024-12-11 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/304047666/)
* 2024-12-12 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898129/)
* 2024-12-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820276/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**December Meetup**](https://www.meetup.com/Seattle-Rust-Meetup/)
* 2024-12-13 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/304730560/)
* 2024-12-17 | Virtual (San Francisco, CA, US) | [Blockchain Center SF](https://www.meetup.com/blockchaincentersf/)
    * [**Rust in Web3: Developer Series**](https://www.meetup.com/blockchaincentersf/events/kwnzntygcqbwb/)
* 2024-12-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346972/)
* 2024-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633276/)
* 2024-12-19 | Virtual (Mexico City, DF, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Posada 2024**](https://www.meetup.com/rust-mx/events/304639403/)
* 2024-12-20 | Virtual (Jersey City, NJ, US)| [Jersey City Classy and Curious Coders Club Cooperative](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/)
    * [**Rust Coding / Game Dev Fridays Open Mob Session!**](https://www.meetup.com/jersey-city-classy-curious-coders-club-cooperative/events/gvxrntygcqbbc/)
* 2024-12-24 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fkmcntygcqbgc/)

### Africa
* 2024-12-10 | Johannesburg, ZA | [Johannesburg Rust Meetup](https://www.meetup.com/johannesburg-rust-meetup/events/)
    * [**Hello World... again**](https://www.meetup.com/johannesburg-rust-meetup/events/304649358/)

### Asia
* 2024-11-28 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**RustTechX Summit 2024 BOSCH**](https://hasgeek.com/rustbangalore/rusttechx-summit-2024-bosch/)
* 2024-11-30 | Tokyo, JP | [Rust Tokyo](https://rust.tokyo/)
    * [**Rust.Tokyo 2024**](https://rust.tokyo/lineup)
* 2024-12-03 | Ra'anana, IL | [Abra R&D Meetups](https://www.meetup.com/abra-rnd-solutions/)
    * [**Rust in the Linux Kernel**](https://www.meetup.com/abra-rnd-solutions/events/304302596/)

### Europe
* 2024-11-27 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund)
    * [**Rust Dortmund**](https://www.meetup.com/rust-dortmund/events/304290556)
* 2024-11-28 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Lind Capital**](https://www.meetup.com/rust-aarhus/events/304005322/)
* 2024-11-28 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #10**](https://www.meetup.com/rust-meetup-augsburg/events/304002691/)
* 2024-11-28 | Berlin, DE | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299421381/)
* 2024-11-28 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust meetup #53 sponsored by Microsoft**](https://www.meetup.com/copenhagen-rust-community/events/304608747/)
* 2024-11-28 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #5**](https://www.meetup.com/rust-gdansk/events/304462668/)
* 2024-11-28 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn with Mainmatter & Otto**](https://www.meetup.com/rust-meetup-hamburg/events/303898286/)
* 2024-11-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester November Code Night**](https://www.meetup.com/rust-manchester/events/304556866/)
* 2024-11-28 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Prague (November 2024)**](https://www.meetup.com/rust-prague/events/304002733/)
* 2024-11-30 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #7**](https://www.meetup.com/stockholm-rust/events/304722627/)
* 2024-12-03 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/events/)
    * [**Rust Hack Night #11: Advent of Code**](https://www.meetup.com/copenhagen-rust-community/events/304427710/)
* 2024-12-04 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Aprenem junts Rust / Learn Rust Together**](https://lu.ma/pypwr0m7)
* 2024-12-04 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in December: Advent of Code**](https://www.meetup.com/rustcologne/events/304760521/)
* 2024-12-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/303123399/)
* 2024-12-04 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #73**](https://www.meetup.com/rust-paris/events/304685955/)
* 2024-12-05 | Gdansk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/events/)
    * [**Rust Gdansk Meetup #6**](https://www.meetup.com/rust-gdansk/events/304773705/)
* 2024-12-05 | Zlin, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**Rust Moravia Meetup (December 2024)**](https://www.meetup.com/rust-moravia/events/304075150/)
* 2024-12-06 | Moscow, RU | [RustCon RU](https://rustcon.ru/)
    * [**RustCon Russia**](https://rustcon.ru/)
* 2024-12-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/wrdkmtygcqbpb/)
* 2024-12-12 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/events/)
    * [**Rust Meetup @ JetBrains**](https://www.meetup.com/rust-amsterdam-group/events/304514267/)
* 2024-12-17 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Types, Traits und Best Practices**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/302425056/)

### North America
* 2024-11-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygcpbkc/)
* 2024-12-05 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**December Monthly Social**](https://www.meetup.com/rust-montreal/events/304778367/)
* 2024-12-05 | St. Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Rust Strings**](https://www.meetup.com/stl-rust/events/302371466/)
* 2024-12-10 | Ann Arbor, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ann Arbor**](https://www.meetup.com/detroitrust/events/cvdcntygcqbnb/)
* 2024-12-12 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbqb/)
* 2024-12-12 | Hybrid: In-Person and Virtual (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/)
    * [**December Meetup**](https://www.meetup.com/join-srug/events/304806455/)
* 2024-12-16 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/events/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/304530508/)
* 2024-12-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638256/)
* 2024-12-23 | Ferndale, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Rust Community Meetup - Ferndale**](https://www.meetup.com/detroitrust/events/dmgjntygcqbfc/)

### Oceania
* 2024-12-04 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**2024 🦀 Encore ✨ Talks**](https://www.meetup.com/rust-sydney/events/304625921/)
* 2024-12-08 | Canberra, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**CRUG Xmas party**](https://www.meetup.com/rust-canberra/events/304282046/)

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

> Will never stop being positively surprised by clippy
>
> ```text
> error: hypothenuse can be computed more accurately:
>    --> src/main.rs:835:5
>     |
> 835 |     (width * width + height * height).sqrt() / diag
>     |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: consider using `width.hypot(height)`
>     |
> help: for further information, visit https://rust-lang.github.io/rust-clippy/master/index.html#imprecise_flops
> ```

- [Manos Pitsidianakis (and rust-clippy) on Mastodon](https://chaos.social/@epilys/113538172289599584)

llogiq is quite self-appreciative regarding [his suggestion](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1633).

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
