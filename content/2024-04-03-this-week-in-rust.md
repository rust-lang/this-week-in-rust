Title: This Week in Rust 541
Number: 541
Date: 2024-04-03
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
* [Announcing Rust 1.77.1](https://blog.rust-lang.org/2024/03/28/Rust-1.77.1.html)
* [Changes to `u128`/`i128` layout in 1.77 and 1.78](https://blog.rust-lang.org/2024/03/30/i128-layout-update.html)

### Newsletters
* [This Week In Bevy: 2d Lighting, Particle Systems, Meshlets, and more](https://thisweekinbevy.com/issue/2024-04-01-2d-lighting-particle-systems-meshlets-and-more)

### Project/Tooling Updates
* [Dioxus 0.5: Signal Rewrite, Remove lifetimes, CSS Hotreloading, and more!](https://dioxuslabs.com/blog/release-050)
* [EtherCrab 0.4.0: Pure Rust EtherCAT, now with Distributed Clocks](https://wapl.es/ethercrab-0-4-io-uring-derives-ethercat-distributed-clocks/)
* [nethsm 0.1.0 - first release for this high level library for the Nitrokey NetHSM](https://gitlab.archlinux.org/archlinux/signstar/-/tree/main/nethsm)
* [BugStalker v0.1.3 released - first release of rust debugger](https://github.com/godzie44/BugStalker/)
* [git-cliff 2.2.0 is released! (highly customizable changelog generator)](https://git-cliff.org/blog/2.2.0)

### Observations/Thoughts
* [On Reusing Arc and Rc in Rust](https://radekmie.dev/blog/on-reusing-arc-and-rc-in-rust/)
* [Who killed the network switch?](https://cliffle.com/blog/who-killed-the-network-switch/)
* [Xr0 Makes C Safer than Rust](https://xr0.dev/safer)
* [Easy Mode Rust](https://llogiq.github.io/2024/03/28/easy.html)
* [Bashing Bevy To Bait Internet Strangers Into Improving My Code](https://oneirical.github.io/bevyrage/)
* [Conway's Game of Life Through Time](https://silasmarvin.dev/conways-game-of-life-through-time)
* [Functions Everywhere, Only Once: Writing Functions for the Everywhere Computer](https://fission.codes/blog/functions-everywhere-only-once/)
* [Rust Bytes: Is Rust the Future of JavaScript Tooling?](https://weeklyrust.substack.com/p/rust-bytes-is-rust-the-future-of)
* [Explaining the internals of async-task from the ground up](https://notgull.net/async-task-explained-part1/)
* [Programming ESP32 with Rust: OTA firmware update](https://quan.hoabinh.vn/post/2024/3/programming-esp32-with-rust-ota-firmware-update)
* [Fast Development In Rust, Part 2](https://blog.sdf.com/p/fast-development-in-rust-part-2)

### Rust Walkthroughs
* [Modelling Universal Domain Types in Rust](https://mmapped.blog/posts/25-domain-types.html)
* [video] [developerlife.com - Get started with unit testing in Rust](https://www.youtube.com/watch?v=Xt495QLrFFk&list=PLofhE49PEwmwLR_4Noa0dFOSPmSpIg_l8)

### Research
* [Rust Digger: More than 14% of crates configure rustfmt. 35 Have both rustfmt.toml and .rustfmt.toml](https://rust-digger.code-maven.com/news/rustfmt-stats)

### Miscellaneous
* [Building a Managed Postgres Service in Rust: Part 1](https://tembo.io/blog/managed-postgres-rust)
* [Beware of the DashMap deadlock](https://dev.to/acter/beware-of-the-dashmap-deadlock-lij)
* [Embedded Rust Bluetooth on ESP: BLE Client](https://apollolabsblog.hashnode.dev/embedded-rust-bluetooth-on-esp-ble-client)
* [Building Stock Market Engine from scratch in Rust (I)](https://medium.com/@harshiljani2002/building-stock-market-engine-from-scratch-in-rust-i-9be7c110e137)
* [Rust Unit and Integration Testing in RustRover](https://blog.jetbrains.com/rust/2024/04/02/rust-unit-and-integration-testing-in-rustrover/)
* [podcast] [cargo-semver-checks with Predrag Gruevski — Rustacean Station](https://rustacean-station.org/episode/predrag-gruevski/)
* [video] [Data Types - Part 3 of Idiomatic Rust in Simple Steps](https://www.youtube.com/watch?v=NSiZdNjkzBk)
* [video] [Deconstructing WebAssembly Components by Ryan Levick @ Wasm I/O 2024](https://www.youtube.com/watch?v=zqfF7Ssa2QI)
* [video] [Extreme Clippy for new Rust crates](https://www.youtube.com/watch?v=dEkr5c5Kul8)
* [video] [playlist] [Bevy GameDev Meetup #2 - March 2024](https://www.youtube.com/playlist?list=PLbvvWoCXmXkJRb8fPcVV1hAhaZHaGC56v)

## Crate of the Week

This week's crate is [cargo-unfmt](https://crates.io/crates/cargo-unfmt), a formatter that formats your code into block-justified text, which sacrifices some readability for esthetics.

Thanks to [Felix Prasanna](https://users.rust-lang.org/t/crate-of-the-week/2704/1301) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No calls for testing were issued this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [greptimedb - Support specifying time ranges in the `COPY FROM` statement to avoid importing unwanted data](https://github.com/GreptimeTeam/greptimedb/issues/3511)
* [greptimedb - Support converting UNIX epoch numbers to specified timezone in `to_timezone` function](https://github.com/GreptimeTeam/greptimedb/issues/3477)
* [mirrord - Capability to modify the local listen address](https://github.com/metalbear-co/mirrord/issues/2319)
* [mirrord - Fix all check-rust-docs warnings](https://github.com/metalbear-co/mirrord/issues/1399)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Braintree](https://github.com/juspay/hyperswitch/issues/4058)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Fiserv](https://github.com/juspay/hyperswitch/issues/4059)
* [Hyperswitch - [REFACTOR]: Remove Default Case Handling - Globepay](https://github.com/juspay/hyperswitch/issues/4060)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, ordered by when the CFP closes, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
* [RustConf 2024](https://foundation.rust-lang.org/news/the-rustconf-2024-call-for-talk-proposals-is-open/) | Closes 2024-04-25 | Montreal, Canada | Event date: 2024-09-10
* [RustLab 2024](https://sessionize.com/rustlab-2024) | Closes 2024-05-01 | Florence, Italy | Event date: 2024-11-09 - 2024-11-11
* [EuroRust 2024](https://www.papercall.io/eurorust-2024)| Closes 2024-06-03 | Vienna, Austria & online | Event date: 2024-10-10
* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/)| Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust).

## Updates from the Rust Project

431 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-03-26..2024-04-02

* [CFI: (actually) check that methods are object-safe before projecting their receivers to `dyn Trait` in CFI](https://github.com/rust-lang/rust/pull/123066)
* [CFI: abstract Closures and Coroutines](https://github.com/rust-lang/rust/pull/123106)
* [CFI: fix drop and `drop_in_place`](https://github.com/rust-lang/rust/pull/123075)
* [CFI: fix methods as function pointer cast](https://github.com/rust-lang/rust/pull/123071)
* [CFI: support calling methods on supertraits](https://github.com/rust-lang/rust/pull/123012)
* [add a `CurrentGcx` type to let the deadlock handler access `TyCtxt`](https://github.com/rust-lang/rust/pull/115220)
* [add basic trait impls for `f16` and `f128`](https://github.com/rust-lang/rust/pull/123085)
* [add detection of (`Partial`)`Ord` methods in the `ambiguous_wide_pointer_comparisons` lint](https://github.com/rust-lang/rust/pull/121268)
* [add rust-lldb pretty printing for Path and PathBuf](https://github.com/rust-lang/rust/pull/120557)
* [assert that ADTs have the right number of args](https://github.com/rust-lang/rust/pull/123214)
* [codegen const panic messages as function calls](https://github.com/rust-lang/rust/pull/122671)
* [coverage: re-enable `UnreachablePropagation` for coverage builds](https://github.com/rust-lang/rust/pull/122860)
* [delegation: fix ICE on wrong `Self` instantiation](https://github.com/rust-lang/rust/pull/123101)
* [delegation: fix ICE on wrong `self` resolution](https://github.com/rust-lang/rust/pull/123091)
* [do not attempt to write `ty::Err` on binding that isn't from current HIR Owner](https://github.com/rust-lang/rust/pull/123202)
* [don't check match scrutinee of postfix match for unused parens](https://github.com/rust-lang/rust/pull/123096)
* [don't inherit codegen attrs from parent static](https://github.com/rust-lang/rust/pull/123310)
* [eagerly instantiate closure/coroutine-like bounds with placeholders to deal with binders correctly](https://github.com/rust-lang/rust/pull/122267)
* [eliminate `UbChecks` for non-standard libraries](https://github.com/rust-lang/rust/pull/122975)
* [ensure std is prepared for cross-targets](https://github.com/rust-lang/rust/pull/122205)
* [fix diagnostics for async block cloning](https://github.com/rust-lang/rust/pull/122589)
* [fixup parsing of `rustc_never_type_options` attribute](https://github.com/rust-lang/rust/pull/123320)
* [function ABI is irrelevant for reachability](https://github.com/rust-lang/rust/pull/123063)
* [improve example on inserting to a sorted vector to avoid shifting equal elements](https://github.com/rust-lang/rust/pull/122945)
* [in `ConstructCoroutineInClosureShim`, pass receiver by mut ref, not mut pointer](https://github.com/rust-lang/rust/pull/123049)
* [load missing type of impl associated constant from trait definition](https://github.com/rust-lang/rust/pull/123130)
* [make `TyCtxt::coroutine_layout` take coroutine's kind parameter](https://github.com/rust-lang/rust/pull/123021)
* [match ergonomics 2024: implement mutable by-reference bindings](https://github.com/rust-lang/rust/pull/123080)
* [match lowering: build the `Place` instead of keeping a `PlaceBuilder` around](https://github.com/rust-lang/rust/pull/122439)
* [match lowering: consistently merge simple or-patterns](https://github.com/rust-lang/rust/pull/123067)
* [match lowering: handle or-patterns one layer at a time](https://github.com/rust-lang/rust/pull/122046)
* [match lowering: sort `Eq` candidates in the failure case too](https://github.com/rust-lang/rust/pull/122459)
* [pattern analysis: Require `enum` indices to be contiguous](https://github.com/rust-lang/rust/pull/123242)
* [replace regions in const canonical vars' types with `'static` in next-solver canonicalizer](https://github.com/rust-lang/rust/pull/123170)
* [require Debug for `Pointee::Metadata`](https://github.com/rust-lang/rust/pull/123181)
* [require `DerefMut` and `DerefPure` on `deref!()` patterns when appropriate](https://github.com/rust-lang/rust/pull/122835)
* [rework opaque type region inference](https://github.com/rust-lang/rust/pull/116891)
* [simplify proc macro bridge state](https://github.com/rust-lang/rust/pull/122939)
* [simplify trim-paths feature by merging all debuginfo options together](https://github.com/rust-lang/rust/pull/122450)
* [store segment and module in `UnresolvedImportError`](https://github.com/rust-lang/rust/pull/122766)
* [suggest associated type bounds on problematic associated equality bounds](https://github.com/rust-lang/rust/pull/122120)
* [suggest correct path in `include_bytes!`](https://github.com/rust-lang/rust/pull/121833)
* [use the `Align` type when parsing alignment attributes](https://github.com/rust-lang/rust/pull/122972)
* [warn against implementing Freeze](https://github.com/rust-lang/rust/pull/123268)
* [enable cargo miri test doctests](https://github.com/rust-lang/rust/pull/123055)
* [miri: avoid mutating the global environment](https://github.com/rust-lang/miri/pull/3421)
* [miri: cotrol stacked borrows consistency check with its own feature flag](https://github.com/rust-lang/miri/pull/3434)
* [miri: experiment with macOS M1 runners](https://github.com/rust-lang/miri/pull/3433)
* [miri: extern-so: give the version script a better name; show errors from failing to build the C lib](https://github.com/rust-lang/miri/pull/3437)
* [miri: speed up Windows CI](https://github.com/rust-lang/miri/pull/3436)
* [miri: tree Borrows: Make tree root always be initialized](https://github.com/rust-lang/miri/pull/3415)
* [don't emit load metadata in debug mode](https://github.com/rust-lang/rust/pull/122849)
* [avoid some unnecessary query invocations](https://github.com/rust-lang/rust/pull/121387)
* [stop doing expensive work in `opt_suggest_box_span` eagerly](https://github.com/rust-lang/rust/pull/123006)
* [stabilize `ptr.is_aligned,` move `ptr.is_aligned_to` to a new feature gate](https://github.com/rust-lang/rust/pull/121948)
* [stabilize `unchecked_{add,sub,mul}`](https://github.com/rust-lang/rust/pull/122520)
* [make `{integer}::from_str_radix` constant](https://github.com/rust-lang/rust/pull/99322)
* [optimize `core::char::CaseMappingIter`](https://github.com/rust-lang/rust/pull/122616)
* [implement `Vec::pop_if`](https://github.com/rust-lang/rust/pull/123107)
* [remove len argument from `RawVec::reserve_for_push`](https://github.com/rust-lang/rust/pull/122976)
* [less generic code for Vec allocations](https://github.com/rust-lang/rust/pull/122396)
* [`UnixStream`: override `read_buf`](https://github.com/rust-lang/rust/pull/123084)
* [`num::NonZero::get` can be 1 transmute instead of 2](https://github.com/rust-lang/rust/pull/123139)
* [fix error message for `env!` when env var is not valid Unicode](https://github.com/rust-lang/rust/pull/122663)
* [futures: make access inner of `futures::io::{BufReader,BufWriter}` not require inner trait bound](https://github.com/rust-lang/futures-rs/pull/2848)
* [regex-syntax: accept `{,n}` as an equivalent to `{0,n}`](https://github.com/rust-lang/regex/pull/1086)
* [cargo add: Preserve comments when updating simple deps](https://github.com/rust-lang/cargo/pull/13655)
* [cargo generate-lockfile: hold lock before querying index](https://github.com/rust-lang/cargo/pull/13657)
* [cargo toml: Warn on unused workspace.dependencies keys on virtual workspaces](https://github.com/rust-lang/cargo/pull/13664)
* [cargo fix: bash completion fallback in `nounset` mode](https://github.com/rust-lang/cargo/pull/13686)
* [clippy: `large_stack_frames`: print total size and largest component](https://github.com/rust-lang/rust-clippy/pull/12582)
* [clippy: `type_id_on_box`: lint on any `Box<dyn _>`](https://github.com/rust-lang/rust-clippy/pull/11350)
* [clippy: accept `String` in `span_lint*` functions directly to avoid unnecessary clones](https://github.com/rust-lang/rust-clippy/pull/12453)
* [clippy: allow `filter_map_identity` when the closure is typed](https://github.com/rust-lang/rust-clippy/pull/12562)
* [clippy: allow `manual_unwrap_or_default` in const function](https://github.com/rust-lang/rust-clippy/pull/12570)
* [clippy: don't emit `duplicated_attribute` lint on "complex" `cfg`s](https://github.com/rust-lang/rust-clippy/pull/12555)
* [clippy: elide unit variables linted by `let_unit` and use `()` directly instead](https://github.com/rust-lang/rust-clippy/pull/12603)
* [clippy: fix `manual_unwrap_or_default` suggestion ignoring side-effects](https://github.com/rust-lang/rust-clippy/pull/12579)
* [clippy: fix suggestion for `len_zero` with macros](https://github.com/rust-lang/rust-clippy/pull/11996)
* [clippy: make sure checked type implements `Try` trait when linting `question_mark`](https://github.com/rust-lang/rust-clippy/pull/12563)
* [clippy: move `box_default` to style, do not suggest turbofishes](https://github.com/rust-lang/rust-clippy/pull/12601)
* [clippy: move `mixed_attributes_style` to style](https://github.com/rust-lang/rust-clippy/pull/12572)
* [clippy: new lint `legacy_numeric_constants`](https://github.com/rust-lang/rust-clippy/pull/12312)
* [clippy: restrict `manual_clamp` to const case, bring it out of nursery](https://github.com/rust-lang/rust-clippy/pull/12543)
* [rust-analyzer: add `rust-analyzer.cargo.allTargets` to configure passing `--all-targets` to cargo invocations](https://github.com/rust-lang/rust-analyzer/pull/16924)
* [rust-analyzer: implement resolving and lowering of Lifetimes (no inference yet)](https://github.com/rust-lang/rust-analyzer/pull/16805)
* [rust-analyzer: fix crate IDs when multiple workspaces are loaded](https://github.com/rust-lang/rust-analyzer/pull/16961)
* [rust-analyzer: ADT hover considering only type or const len not lifetimes](https://github.com/rust-lang/rust-analyzer/pull/16967)
* [rust-analyzer: check for client support of relative glob patterns before using them](https://github.com/rust-lang/rust-analyzer/pull/16957)
* [rust-analyzer: lifetime length are not added in count of params in highlight](https://github.com/rust-lang/rust-analyzer/pull/16960)
* [rust-analyzer: revert debug extension priorities](https://github.com/rust-lang/rust-analyzer/pull/16964)
* [rust-analyzer: silence mismatches involving unresolved projections](https://github.com/rust-lang/rust-analyzer/pull/16968)
* [rust-analyzer: use lldb when debugging with C++ extension on MacOS](https://github.com/rust-lang/rust-analyzer/pull/16965)
* [rust-analyzer: pattern analysis: Use contiguous indices for `enum` variants](https://github.com/rust-lang/rust-analyzer/pull/16979)
* [rust-analyzer: prompt the user to reload the window when enabling test explorer](https://github.com/rust-lang/rust-analyzer/pull/16975)
* [rust-analyzer: resolve tests per file instead of per crate in test explorer](https://github.com/rust-lang/rust-analyzer/pull/16971)

### Rust Compiler Performance Triage

A pretty quiet week, with most changes (dropped from the report below) being
due to continuing bimodality in the performance data. No particularly notable
changes landed.

Triage done by **@simulacrum**.
Revision range: [73476d49..3d5528c](https://perf.rust-lang.org/?start=73476d49904751f8d90ce904e16dfbc278083d2c&end=3d5528c287860b918e178a34f04ff903325571b3&absolute=false&stat=instructions%3Au)

1 Regressions, 2 Improvements, 5 Mixed; 0 of them in rollups
61 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-04-01.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Merge RFC 3543: patchable-function-entry](https://github.com/rust-lang/rfcs/commit/c39fdca1e3c6d4e8be116420b2270423b473848c)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* *No RFCs entered Final Comment Period this week.*

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Pass list of defineable opaque types into canonical queries](https://github.com/rust-lang/rust/pull/122077)
* [disposition: merge] [Document overrides of `clone_from()` in core/std](https://github.com/rust-lang/rust/pull/122201)
* [disposition: merge] [Tracking Issue for `Seek::seek`_relative](https://github.com/rust-lang/rust/issues/117374)
* [disposition: merge] [Tracking Issue for generic `NonZero`](https://github.com/rust-lang/rust/issues/120257)
* [disposition: merge] [Tracking Issue for `cstr_count_bytes`](https://github.com/rust-lang/rust/issues/114441)
* [disposition: merge] [privacy: Stabilize lint `unnameable_types`](https://github.com/rust-lang/rust/pull/120144)
* [disposition: merge] [Stabilize Wasm target features that are in phase 4 and 5](https://github.com/rust-lang/rust/pull/117457)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [feat(add): Stabilize MSRV-aware version req selection](https://github.com/rust-lang/cargo/pull/13608)

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: Add `freeze` intrinsic and related library functions](https://github.com/rust-lang/rfcs/pull/3605)
* [new] [RFC: Add a special TryFrom and Into derive macro, specifically for C-Style enums](https://github.com/rust-lang/rfcs/pull/3604)
* [new] [re-organise the compiler team](https://github.com/rust-lang/rfcs/pull/3599)

## Upcoming Events

Rusty Events between 2024-04-03 - 2024-05-01 🦀

### Virtual

* 2024-04-03 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 4 - Error Handling**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/299507234/)
* 2024-04-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047892/)
* 2024-04-04 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368794/)
* 2024-04-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**BlueR: a Rust Based Tool for Robust and Safe Bluetooth Control**](https://www.meetup.com/dallasrust/events/298341660/)
* 2024-04-11 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477689/)
* 2024-04-11 | Virtual (Nürnberg, DE) | [Rust Nüremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945256/)
* 2024-04-15 & 2024-04-16 | Virtual | [Mainmatter](https://mainmatter.com/)
    * [**Remote Workshop: Testing for Rust projects – going beyond the basics**](https://ti.to/mainmatter/rust-testing-april-2024)
* 2024-04-16 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**A reverse proxy with Tower and Hyperv1**](https://www.meetup.com/rust-dublin/events/300144192/)
* 2024-04-16 | Virtual (Washinigton, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346486/)
* 2024-04-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298542323/)
* 2024-04-18 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368799/)
* 2024-04-25 | Virtual + In Person (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477692/)
* 2024-04-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcgbnc/)
* 2024-05-01 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047895/)

### Africa

* 2024-04-05 | Kampala, UG | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)

### Europe

* 2024-04-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/)
    * [**Rust Meetup Reboot 3**](https://www.meetup.com/cambridge-rust-meetup/events/299730322/)
* 2024-04-10 | Cologne/Köln, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**This Month in Rust, April**](https://www.meetup.com/rustcologne/events/300191375/)
* 2024-04-10 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester April 2024**](https://www.meetup.com/rust-manchester/events/299887934/)
* 2024-04-10 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/299488225/)
* 2024-04-11 | Bordeaux, FR | [Rust Bordeaux](https://www.meetup.com/bordeaux-rust/)
    * [**Rust Bordeaux #2 : Présentations**](https://www.meetup.com/bordeaux-rust/events/299628716/)
* 2024-04-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/299694473/)
* 2024-04-15 | Zagreb, HR | [impl Zagreb for Rust](https://www.meetup.com/zagreb-rust-meetup/)
    * [**Rust Meetup 2024/04: Building cargo projects with NIX**](https://www.meetup.com/zagreb-rust-meetup/events/299905015/)
* 2024-04-16 | Bratislava, SK | [Bratislava Rust Meetup Group](https://www.meetup.com/bratislava-rust-meetup-group/)
    * [**Rust Meetup by Sonalake #5**](https://www.meetup.com/bratislava-rust-meetup-group/events/299302952/)
* 2024-04-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**winnow/nom**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/300024630/)
* 2024-04-16 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2024 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/298507657/)
* 2024-04-17 | Bergen, NO | [Hubbel kodeklubb](https://www.meetup.com/bergen-html-css-meetup-group/)
    * [**Lær Rust med Conways Game of Life**](https://www.meetup.com/bergen-html-css-meetup-group/events/300031586/)
* 2024-04-20 | Augsburg, DE | [Augsburger Linux-Infotag 2024](https://www.luga.de/static/LIT-2024/)
   * [**Augsburger Linux-Infotag 2024: Workshop Einstieg in Embedded Rust mit dem Raspberry Pico WH**](https://www.luga.de/static/LIT-2024/talks/einstieg_in_embedded_rust_mit_dem_raspberry_pico_wh/)
* 2024-04-23 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust'n'Tell - Rust for the Web**](https://www.meetup.com/rust-berlin/events/300047151/)
* 2024-04-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/299564517/)
* 2024-04-25 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell**](https://www.meetup.com/rust-berlin/events/299288960/)
* 2024-04-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Fullstack Rust - Workshop #2**](https://www.meetup.com/rust-basel/events/299933581/)

### North America

* 2024-04-04 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803577/)
* 2024-04-04 | Portland, OR, US | [PDXRust Meetup](https://www.meetup.com/pdxrust/)
    * [**Hack Night and First Post-Pandemic Meetup Restart**](https://www.meetup.com/pdxrust/events/300043905/)
* 2024-04-09 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/300121681/)
* 2024-04-10 | Boulder, CO, US | [Boulder Rust Meetup](https://www.meetup.com/boulder-rust-meetup/)
    * [**Rust Meetup: Better Builds w/ Flox + Hangs**](https://www.meetup.com/boulder-rust-meetup/events/300019409/)
* 2024-04-11 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509326/)
* 2024-04-11 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300019993/)
* 2024-04-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Davis Square Rust Lunch, Apr 15**](https://www.meetup.com/bostonrust/events/300116673/)
* 2024-04-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186907/)
* 2024-04-16 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group: Meet Servo and Robius Open Source Projects**](https://www.meetup.com/seattle-rust-user-group/events/299908469/)
* 2024-04-18 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/299803586/)
* 2024-04-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/299960315/)
* 2024-04-25 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers - Async Rust on Embedded**](https://www.meetup.com/music-city-rust-developers/events/299976876/)
* 2024-04-26 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch, Apr 26**](https://www.meetup.com/bostonrust/events/300116689/)

### Oceania

* 2024-04-30 | Canberra, ACT, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/)
    * [**April Meetup**](https://www.meetup.com/rust-canberra/events/300023000/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1bpg8b8/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Panstromek: I remember reading somewhere (probably here) that borrow checking has `O(n^3)` asymptotic complexity, relative to the size of the function.
>
> Nadrieril: Compared to match exhaustiveness which is NP-hard and trait solving which is undecidable, a polynomial complexity feels refreshingly sane.

– [Panstromek and Nadrieril on zulip](https://rust-lang.zulipchat.com/#narrow/stream/131828-t-compiler/topic/Is.20Borrow.20Checking.20Cubic.3F/near/429533622)

Thanks to [Kevin Reid](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1553) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1bveowo/this_week_in_rust_541/)</small>
