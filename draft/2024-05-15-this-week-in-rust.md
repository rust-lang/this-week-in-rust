Title: This Week in Rust 547
Number: 547
Date: 2024-05-15
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
* [May 2024 Leadership Council Update](https://blog.rust-lang.org/inside-rust/2024/05/14/leadership-council-update.html)

### Foundation

### Newsletters
* [ClearCoat, new examples, and game updates](https://thisweekinbevy.com/issue/2024-05-13-clearcoat-new-examples-and-game-updates)

### Project/Tooling Updates
* [Rust for embedded Linux kernels](https://lwn.net/Articles/970216/)
* [kira - release v0.9.0](https://github.com/tesselode/kira/releases/tag/v0.9.0)
* [Cushy v0.3: New widgets, offscreen capture, Plotters and Tokio integrations, and more](https://ecton.dev/cushy-v0-3/)
* [bbolt-rs v1.3.8](https://github.com/ambaxter/bbolt-rs/blob/v1.3.8/docs/announcement.md)
* [Maelstrom: A Hermetic, Clustered Test Runner for Rust (and It’s Fast)](https://www.reddit.com/r/rust/comments/1chrshl/maelstrom_a_hermetic_clustered_test_runner_for/)

### Observations/Thoughts
* [Long-term Rust Project Maintenance](https://corrode.dev/blog/long-term-rust-maintenance/)
* [Methods Should Be Object Safe](https://nora.codes/post/methods-should-be-object-safe/)
* [References are like jumps](https://without.boats/blog/references-are-like-jumps/)
* [Rust 1.78: Performance Impact of the 128-bit Memory Alignment Fix](https://codspeed.io/blog/rust-1-78-performance-impact-of-the-128-bit-memory-alignment-fix)
* [HowTo: Egui with webworkers](https://voelklmichael.github.io/Blog/2024/05/12/egui-wasm-threads.html)
* [Using build.rs to integrate rust applications with system libraries like a pro](https://neosmart.net/blog/using-build-rs-to-integrate-rust-applications-with-system-libraries-like-a-pro/)
* [Rust actors + ArcMutex: handle with care](https://dgroshev.com/blog/rust-actors-mutex/)
* [Rust through the ages](https://www.ncameron.org/blog/rust-through-the-ages/)
* [Mixing rayon and tokio for fun and (hair) loss](https://blog.dureuill.net/articles/dont-mix-rayon-tokio/)
* [Long-running backend async tasks in tauri v2](https://sneakycrow.dev/blog/2024-05-12-running-async-tasks-in-tauri-v2)
* [Blazingly Fast Linked Lists](https://dygalo.dev/blog/blazingly-fast-linked-lists/)

### Rust Walkthroughs
* [Let's build a Load Balancer in Rust - Part 1](https://marcobacis.com/blog/load-balancer-rust-1/)

### Research

### Miscellaneous
* [April 2024 Rust Jobs Report](https://filtra.io/rust-apr-24)
* [VS Code Extensions and WebAssembly](https://code.visualstudio.com/blogs/2024/05/08/wasm)

## Crate of the Week

This week's crate is [stated-scope-guard](https://crates.io/crates/stated-scope-guard), a library supporting a more flexible RAII pattern for stated resouce management.

Thanks to [Evian Zhang](https://users.rust-lang.org/t/crate-of-the-week/2704/1309) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
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

* [greptimedb - Add more tests for Copy From](https://github.com/GreptimeTeam/greptimedb/issues/3265)
* [greptimedb - Checksum for manifests](https://github.com/GreptimeTeam/greptimedb/issues/3004)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [EuroRust 2024](https://www.papercall.io/eurorust-2024) | Closes 2024-06-03 | Vienna, Austria & online | Event date: 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/) | Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22
* [Rust Argentina June 2024](https://sessionize.com/rust-argentina-june/) | Closes 2024-05-31 | Buenos Aires, AR | Event date: 2024-06-04

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

329 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-07..2024-05-14

* [analyse visitor: build proof tree in probe](https://github.com/rust-lang/rust/pull/124936)
* [consolidate obligation cause codes for where clauses](https://github.com/rust-lang/rust/pull/124988)
* [correct the const stabilization of `last_chunk` for slices](https://github.com/rust-lang/rust/pull/124836)
* [coverage: branch coverage support for let-else and if-let](https://github.com/rust-lang/rust/pull/124223)
* [coverage: further simplify extraction of mapping info from MIR](https://github.com/rust-lang/rust/pull/124615)
* [display walltime benchmarks with subnanosecond precision](https://github.com/rust-lang/rust/pull/124774)
* [do not ICE on `AnonConst`s in `diagnostic_hir_wf_check`](https://github.com/rust-lang/rust/pull/124219)
* [do not ICE on foreign malformed `diagnostic::on_unimplemented`](https://github.com/rust-lang/rust/pull/124683)
* [do not add leading asterisk in the `PartialEq`](https://github.com/rust-lang/rust/pull/124157)
* [don't ICE when we cannot eval a const to a valtree in the new solver](https://github.com/rust-lang/rust/pull/124846)
* [don't call `env::set_var` in `rustc_driver::install_ice_hook`](https://github.com/rust-lang/rust/pull/125063)
* [fix error messages for `break` inside coroutines](https://github.com/rust-lang/rust/pull/124777)
* [fix ICE while casting a type with error](https://github.com/rust-lang/rust/pull/124997)
* [fix `MemCategorization` and `ExprUse` visitors for new solver (this time it's better)](https://github.com/rust-lang/rust/pull/124902)
* [fix insufficient logic when searching for the underlying allocation](https://github.com/rust-lang/rust/pull/124761)
* [fix more ICEs in `diagnostic::on_unimplemented`](https://github.com/rust-lang/rust/pull/124875)
* [fix parse error message for meta items](https://github.com/rust-lang/rust/pull/124778)
* [handle Deref expressions in `invalid_reference_casting`](https://github.com/rust-lang/rust/pull/124978)
* [handle field projections like slice indexing in `invalid_reference_casting`](https://github.com/rust-lang/rust/pull/124908)
* [ignore empty `RUSTC_WRAPPER` in bootstrap](https://github.com/rust-lang/rust/pull/124903)
* [ignore generics args in attribute paths](https://github.com/rust-lang/rust/pull/124318)
* [implement `as_chunks` with `split_at_unchecked`](https://github.com/rust-lang/rust/pull/124793)
* [implement lldb formatter for "clang encoded" enums (LLDB 18.1+) (V3)](https://github.com/rust-lang/rust/pull/124781)
* [improve `rustc_parse::Parser`'s debuggability](https://github.com/rust-lang/rust/pull/124779)
* [make `#![feature]` suggestion `MaybeIncorrect`](https://github.com/rust-lang/rust/pull/124926)
* [make `Ty::builtin_deref` just return a `Ty`](https://github.com/rust-lang/rust/pull/124957)
* [make sure we consume a generic arg when checking mistyped turbofish](https://github.com/rust-lang/rust/pull/124930)
* [make sure we don't deny macro vars w keyword names](https://github.com/rust-lang/rust/pull/124869)
* [match ergonomics 2024: let `&` patterns eat `&mut`](https://github.com/rust-lang/rust/pull/124567)
* [match ergonomics 2024: migration lint](https://github.com/rust-lang/rust/pull/124639)
* [pretty-print let-else with added parenthesization when needed](https://github.com/rust-lang/rust/pull/125051)
* [remove braces when fixing a nested use tree into a single item](https://github.com/rust-lang/rust/pull/123344)
* [rename `Generics::params` to `Generics::own_params`](https://github.com/rust-lang/rust/pull/124953)
* [simplify `use crate::rustc_foo::bar` occurrences](https://github.com/rust-lang/rust/pull/124876)
* [split out `ty::AliasTerm` from `ty::AliasTy`](https://github.com/rust-lang/rust/pull/125076)
* [uplift `TraitRef` into `rustc_type_ir`](https://github.com/rust-lang/rust/pull/124982)
* [uplift various `*Predicate` types into `rustc_type_ir`](https://github.com/rust-lang/rust/pull/125001)
* [use fewer origins when creating type variables](https://github.com/rust-lang/rust/pull/124955)
* [never patterns: lower never patterns to `Unreachable` in MIR](https://github.com/rust-lang/rust/pull/123332)
* [avoid `alloca`s in codegen for simple `mir::Aggregate` statements](https://github.com/rust-lang/rust/pull/123886)
* [interpret/miri: better errors on failing `offset_from`](https://github.com/rust-lang/rust/pull/124923)
* [miri: `io::Error` handling: keep around the full `io::Error` for longer so we can give better errors](https://github.com/rust-lang/miri/pull/3589)
* [miri: a bit of intrinsics organization](https://github.com/rust-lang/miri/pull/3601)
* [miri: allow test targets to be set via CLI args](https://github.com/rust-lang/miri/pull/3588)
* [miri: intrinsics: just panic when they get used incorrectly](https://github.com/rust-lang/miri/pull/3604)
* [miri: make `MIRI_TEST_TARGET` and `RUSTC_BLESS` entirely an internal thing](https://github.com/rust-lang/miri/pull/3590)
* [miri: return non-null pointer from `malloc(0)`](https://github.com/rust-lang/miri/pull/3580)
* [miri: support `f*_algebraic`](https://github.com/rust-lang/miri/pull/3596)
* [miri: use non-null pointer for size 0 posix memalign](https://github.com/rust-lang/miri/pull/3600)
* [codegen: memmove/memset cannot be non-temporal](https://github.com/rust-lang/rust/pull/124932)
* [codegen-cranelift: translate MIR to clif ir in parallel with parallel rustc](https://github.com/rust-lang/rustc_codegen_cranelift/pull/1489)
* [stabilize `byte_slice_trim_ascii` for `&[u8]`/`&str`](https://github.com/rust-lang/rust/pull/124928)
* [stabilize `seek_seek_relative`](https://github.com/rust-lang/rust/pull/123817)
* [relax allocator requirements on some Rc/Arc APIs](https://github.com/rust-lang/rust/pull/124981)
* [`f16::is_sign_{positive,negative}` were feature-gated on f128](https://github.com/rust-lang/rust/pull/124828)
* [`io::Write::write_fmt`: panic if the formatter fails when the stream does not fail](https://github.com/rust-lang/rust/pull/125012)
* [`std::net: Socket::new_raw` now set to `SO_NOSIGPIPE` on freebsd](https://github.com/rust-lang/rust/pull/124470)
* [`std::rand`: adding solaris/illumos for getrandom support](https://github.com/rust-lang/rust/pull/124766)
* [cargo: add local-only build scripts example in check-cfg docs](https://github.com/rust-lang/cargo/pull/13884)
* [cargo: fix: build only the specified artifact library when multiple types are available](https://github.com/rust-lang/cargo/pull/13842)
* [rustdoc: dedup search form HTML](https://github.com/rust-lang/rust/pull/124738)
* [rustdoc: use stability, instead of features, to decide what to show](https://github.com/rust-lang/rust/pull/124864)
* [clippy: `significant_drop_in_scrutinee`: Fix false positives due to false drops of place expressions](https://github.com/rust-lang/rust-clippy/pull/12764)
* [clippy: add new lint `doc_lazy_continuation`](https://github.com/rust-lang/rust-clippy/pull/12770)
* [clippy: add new lint that disallow renaming parameters in trait functions](https://github.com/rust-lang/rust-clippy/pull/11540)
* [clippy: fix false positive because lack of consideration of mutable caller](https://github.com/rust-lang/rust-clippy/pull/12650)
* [clippy: fix: merge multiple suggestions into a single multi-span suggestion in `needless_late_init`](https://github.com/rust-lang/rust-clippy/pull/12777)
* [clippy: fix: use `hir_with_context` to produce correct snippets for `assigning_clones`](https://github.com/rust-lang/rust-clippy/pull/12783)
* [clippy: handle `rustc_on_unimplemented` in `duplicated_attributes`](https://github.com/rust-lang/rust-clippy/pull/12620)
* [clippy: ignore `_to_string` lints in code `from_expansion`](https://github.com/rust-lang/rust-clippy/pull/12780)
* [clippy: lint direct priority conflicts in `[workspace.lints]`](https://github.com/rust-lang/rust-clippy/pull/12730)
* [clippy: make `from_str_radix_10` skip constant context](https://github.com/rust-lang/rust-clippy/pull/12787)
* [clippy: new lint: `macro_metavars_in_unsafe`](https://github.com/rust-lang/rust-clippy/pull/12107)
* [rust-analyzer: fix OOM caused by term search](https://github.com/rust-lang/rust-analyzer/pull/17203)
* [rust-analyzer: fix `source_range` for `INT_NUMBER` in completion](https://github.com/rust-lang/rust-analyzer/pull/17192)
* [rust-analyzer: fix: improve confusing literal hovers](https://github.com/rust-lang/rust-analyzer/pull/17220)
* [rust-analyzer: fix: keep parentheses when the precedence of inner expr is lower than the outer one](https://github.com/rust-lang/rust-analyzer/pull/17187)
* [rust-analyzer: fix: report all LSP protocol errors with `invalid_data`](https://github.com/rust-lang/rust-analyzer/pull/17207)
* [rust-analyzer: fix: report both IO errors and `main_loop` errors](https://github.com/rust-lang/rust-analyzer/pull/17208)
* [rust-analyzer: implement unsafe attribute parsing](https://github.com/rust-lang/rust-analyzer/pull/17195)
* [rust-analyzer: use the repository field to link to the repository](https://github.com/rust-lang/rust-analyzer/pull/17188)

### Rust Compiler Performance Triage

A pretty quiet week with only a few PRs being flagged for analysis.
More improvements than regressions this week, and also several nice
binary size reductions caused by generating less LLVM IR.

Triage done by **@kobzol**.
Revision range: [69f53f5e..9105c57b](https://perf.rust-lang.org/?start=69f53f5e5583381267298ac182eb02c7f1b5c1cd&end=9105c57b7f6623310e33f3ee7e48a3114e5190a7&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.2%, 0.9%]   | 8     |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.2%, 2.4%]   | 18    |
| Improvements ✅ <br /> (primary)   | -1.1% | [-2.3%, -0.2%] | 51    |
| Improvements ✅ <br /> (secondary) | -0.6% | [-1.4%, -0.3%] | 19    |
| All ❌✅ (primary)                 | -0.9% | [-2.3%, 0.9%]  | 59    |


1 Regression, 0 Improvements, 3 Mixed; 0 of them in rollups
75 artifact comparisons made in total

[Full report here](https://github.com/Kobzol/rustc-perf/blob/0ab8cfe4bdc3044f8e610349d90c1708675b1ccf/triage/2024-05-14.md)

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
* [disposition: merge] [Tracking Issue for `IpvNAddr::{BITS, to_bits, from_bits}` (`ip_bits`)](https://github.com/rust-lang/rust/issues/113744)
* [disposition: merge] [Add `size_of` and `size_of_val` and `align_of` and `align_of_val` to the prelude](https://github.com/rust-lang/rust/pull/123168)
* [disposition: merge] [offset: allow zero-byte offset on arbitrary pointers](https://github.com/rust-lang/rust/pull/117329)
* [disposition: merge] [Add `-` (stdin) support in rustdoc](https://github.com/rust-lang/rust/pull/124611)
* [disposition: merge] [Warn (or error) when `Self` ctor from outer item is referenced in inner nested item](https://github.com/rust-lang/rust/pull/124187)
* [disposition: merge] [Bump `elided_lifetimes`_in_associated_constant to deny](https://github.com/rust-lang/rust/pull/124211)

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Decide on validity for metadata of wide pointer/reference with slice tail](https://github.com/rust-lang/unsafe-code-guidelines/issues/510)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [[RFC] externally definable statics](https://github.com/rust-lang/rfcs/pull/3635)
* [new] [Scoped `impl Trait for Type`](https://github.com/rust-lang/rfcs/pull/3634)
* [new] [[RFC] `core::marker::Freeze` in bounds](https://github.com/rust-lang/rfcs/pull/3633)
* [new] [[RFC] externally implementable functions](https://github.com/rust-lang/rfcs/pull/3632)
* [new] [RFC for doc_cfg, doc_cfg_auto, doc_cfg_hide and doc_cfg_show features](https://github.com/rust-lang/rfcs/pull/3631)

## Upcoming Events

Rusty Events between 2024-05-15 - 2024-06-12 🦀

### Virtual

* 2024-05-15 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events/)
    * [**#RustSemineri - 7**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-7-0a97e784)
* 2024-05-15 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 6 - Testing**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/300819214/)
* 2024-05-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**NativeLink**](https://www.meetup.com/vancouver-rust/events/298542331/)
* 2024-05-16 | Virtual (Ankara, TR) | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**#RustSemineri - 8**](https://kommunity.com/turkiye-rust-community/events/rustsemineri-8-ddfe6b15)
* 2024-05-16 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298312423/)
* 2024-05-17 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Rust at Full Speed: Harnessing Concurrency with Confidence**](https://www.eventbrite.com/e/rust-at-full-speed-harnessing-concurrency-with-confidence-tickets-884842296127)
* 2024-05-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—forensic parsing via Artemis**](https://www.meetup.com/rustdc/events/299346490/)
* 2024-05-23 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477699/)
* 2024-05-23 | Virtual (Israel) | [Rust in Israel](https://rust.org.il/)
    * [**Web development in Rust using Rocket (Hebrew)**](https://www.meetup.com/code-mavens/events/300974367/)
* 2024-05-28 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/300533392/)
* 2024-05-28 & 2024-05-28 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Telemetry for Rust APIs – you can't fix what you can't see (fee)**](https://ti.to/mainmatter/rust-telemetry-may-2024)
* 2024-05-30 | Virtual + In Person (Barcelona, ES) | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298542326/)
* 2024-06-04 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191681/)
* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)

### Africa

* 2024-06-01 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Asia

* 2024-05-22 | Singapore, SG | [SG Rust Meetup](https://www.meetup.com/rust-singapore/)
    * [**SG Rustaceans! Updated - SG Rust Meetup at CraftsforGreen Whole Studio**](https://www.meetup.com/rust-singapore/events/300988123/)

### Europe

* 2024-05-16 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #7**](https://www.meetup.com/rust-meetup-augsburg/events/300174327/)
* 2024-05-16 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #68**](https://mobilizon.fr/events/14b51ccc-211f-400f-9615-707d9d871e78)
* 2024-05-21 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack Night**](https://www.meetup.com/rust-aarhus/events/300307155/)
* 2024-05-21 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**Save the date - Mai Meetup**](https://www.meetup.com/rust-zurich/events/300513957/)
* 2024-05-22 | Leiden, NL | [Future-proof Software Development by FreshMinds](https://www.meetup.com/freshminds-future-proof-software-development/)
    * [**Coding Dojo Session**](https://www.meetup.com/freshminds-future-proof-software-development/events/300566391/)
* 2024-05-23 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**2024 Rust Talks Bern #2**](https://www.meetup.com/rust-bern/events/300286917/)
* 2024-05-23 | Łodz, PL | [Mobica](https://www.linkedin.com/posts/mobica_rust-programming-embeddedsoftware-activity-7193232853717946369-CK68/)
    * [**Zapisz się na warsztat Rust / Embedded w Łodzi! / What's all the fuss about Rust?**](https://www.interankiety.pl/f/b4D7G7xO)
* 2024-05-23 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester May Code Night**](https://www.meetup.com/rust-manchester/events/300923207/)
* 2024-05-24 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #3: Discussions**](https://www.meetup.com/bordeaux-rust/events/300723854/)
* 2024-05-25 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/)
    * [**Ferris' Fika Forum #3 [Embedded lab edition]**](https://www.meetup.com/stockholm-rust/events/301014982/)
* 2024-05-28 - 2024-05-30 | Berlin, DE | [Oxidize](https://oxidizeconf.com/)
    * [**Oxidize Conf 2024**](https://oxidizeconf.com/)
* 2024-05-30 | Barcelona, ES | [Mainmatter](https://mainmatter.com/) & [BcnRust](https://www.meetup.com/es-ES/bcnrust/)
    * [**Rust for the web, Barcelona 2024**](https://www.meetup.com/es-ES/bcnrust/events/300765894/) 
* 2024-05-30 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288963/)
* 2024-05-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #47 sponsored by Microsoft!**](https://www.meetup.com/copenhagen-rust-community/events/300458222/)
* 2024-05-30 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/300453310/)
* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2025-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)

### North America

* 2024-05-16 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775539/)
* 2024-05-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509369/)
* 2024-05-20 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch, May 20**](https://www.meetup.com/bostonrust/events/300116765/)
* 2024-05-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186931/)
* 2024-05-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygchbdc/)
* 2024-05-25 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Talk Double Feature**](https://www.meetup.com/deep-dish-rust/events/300665520/)
* 2024-05-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/300775547/)
* 2024-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Boston Common Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/300116786/)
* 2024-06-08 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Porter Square Rust Lunch, Jun 8**](https://www.meetup.com/bostonrust/events/300116799/)

### Oceania

* 2024-05-28 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**a demo 🤯 & a lightning ⚡show ✨**](https://www.meetup.com/rust-sydney/events/300854266/)


### South America

* 2024-06-06 | Buenos Aires, AR | [Rust en Español | Rust Argentina](https://www.meetup.com/rust-argentina/)
    * [**Juntada de Junio**](https://www.meetup.com/rust-argentina/events/299740249)

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

> Unfortunately, most people seem to have taken the wrong lesson from Rust. They see all of this business with lifetimes and ownership as a dirty mess that Rust has had to adopt because it wanted to avoid garbage collection. But this is completely backwards! Rust adopted rules around shared mutable state and this enabled it to avoid garbage collection. These rules are a good idea regardless.

– [without boats](https://without.boats/blog/references-are-like-jumps/)

Thanks to [Jules Bertholet](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1567) for the last-minute suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
