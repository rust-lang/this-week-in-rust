Title: This Week in Rust 673
Number: 673
Date: 2026-10-14
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

<!-- IMPORTANT NOTE: We are no longer accepting pull request submissions for the Project/Tooling Updates section.
See here for details: https://github.com/rust-lang/this-week-in-rust/issues/8575 -->

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [hotcode](https://github.com/alordash/hotcode), a crate for hot-reloading Rust functions at runtime.

Thanks to [alordash](https://users.rust-lang.org/t/crate-of-the-week/2704/1697) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
[Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
[Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
[Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

603 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-10-06..2026-10-13

#### Compiler
* [allow unary operand types to be inferred later](https://github.com/rust-lang/rust/pull/159744)
* [avoid cloning impl trait AST nodes in late resolution](https://github.com/rust-lang/rust/pull/164074)
* [avoid double-checking `self.bound_vars'` len in ValidateBoundVars' TypeVisitor impl](https://github.com/rust-lang/rust/pull/164206)
* [check `is_indirect_first_projection` when replacing in RefProp](https://github.com/rust-lang/rust/pull/163778)
* [implement `clone_from` for `BorrowckDomain`](https://github.com/rust-lang/rust/pull/164089)
* [MIR move elimination (3/6): PreciseLiveness](https://github.com/rust-lang/rust/pull/163337)
* [perf: cache FFI safety results in `improper_ctypes` lint](https://github.com/rust-lang/rust/pull/163348)
* [reserve more capacity for Symbol interner](https://github.com/rust-lang/rust/pull/164150)
* [skip runtime fma detection when enabled at compile time on `x86_64`](https://github.com/rust-lang/compiler-builtins/pull/1330)
* [stabilize the WebAssembly `wide-arithmetic` feature](https://github.com/rust-lang/rust/pull/160877)
* [stabilize `optimize` attribute](https://github.com/rust-lang/rust/pull/157273)
* [treat union fields as aliasing each other in `value_analysis`](https://github.com/rust-lang/rust/pull/163425)

#### Library
* [add `fs::rename_noreplace`](https://github.com/rust-lang/rust/pull/162027)
* [directly deallocates the nodes instead of traversing every element when BTreemap dropping](https://github.com/rust-lang/rust/pull/162931)
* [fast path for `BTreeMap::Iter::next`](https://github.com/rust-lang/rust/pull/163521)
* [make `NonNull::{read,write}_volatile` unstably const](https://github.com/rust-lang/rust/pull/164051)
* [stabilize `ptr::try_cast_aligned`](https://github.com/rust-lang/rust/pull/154170)
* [vectorized `vec::dedup` pre-scan](https://github.com/rust-lang/rust/pull/162810)

#### Cargo
* [avoid absolute paths in fingerprints if they are relative to the workspace root](https://github.com/rust-lang/cargo/pull/17573)
* [fix: cargo installing from a directory source fails for binary packages with no dependencies](https://github.com/rust-lang/cargo/pull/17552)
* [`config`: Don't process includes for `credentials.toml` files](https://github.com/rust-lang/cargo/pull/17569)
* [`git`: Add future-incompat warning in prep for removing auto-detection](https://github.com/rust-lang/cargo/pull/17568)
* [`hints`: support minimum optimization levels](https://github.com/rust-lang/cargo/pull/17368)
* [`job`: Allow explicit child process breakaway on Windows](https://github.com/rust-lang/cargo/pull/17527)
* [`layout`: exclude a build-dir created as a parent from backups](https://github.com/rust-lang/cargo/pull/17560)
* [`mock-std`: Set stable crate versions to 0.0.0](https://github.com/rust-lang/cargo/pull/17565)
* [`pkgid-spec`: Builtin package spec parsing and formatting](https://github.com/rust-lang/cargo/pull/17539)
* [refactor: modernize some API usages for 1.99](https://github.com/rust-lang/cargo/pull/17584)
* [restrict workspace-relative dep-info to workspace packages](https://github.com/rust-lang/cargo/pull/17575)
* [`rustdoc`: suppress edition unspecified warnings](https://github.com/rust-lang/cargo/pull/17557)
* [`test`: mark requiring ssh for tests shelling out to it](https://github.com/rust-lang/cargo/pull/17563)

#### Rustdoc
* [fix more `incoherent_impl` linking](https://github.com/rust-lang/rust/pull/163841)
* [prefer local paths over remote ones when foreign item is locally reexported](https://github.com/rust-lang/rust/pull/164007)
* [check reexport chain if inline before hidden](https://github.com/rust-lang/rust/pull/164033)

#### Clippy
* [configuration: add `too_long_first_doc_paragraph` lint to the `check-private-items` list](https://github.com/rust-lang/rust-clippy/pull/17476)
* [do not trigger `manual_is_multiple_of` on literal divisors](https://github.com/rust-lang/rust-clippy/pull/16585)
* [ensure a deprecation warning is emitted for `lint-inconsistent-struct-field-initializers`](https://github.com/rust-lang/rust-clippy/pull/17841)
* [new lints `deprecated_attributes_without_`{`since`, `note`}](https://github.com/rust-lang/rust-clippy/pull/17609)
* [fix {`panic`, `todo`, `unimplemented`, `unreachable`}](https://github.com/rust-lang/rust-clippy/pull/17704)
* [fix `module_name_repetitions` false negative on grouped re-exports](https://github.com/rust-lang/rust-clippy/pull/17848)
* [fix `needless_lifetimes` linting where clauses lifetimes](https://github.com/rust-lang/rust-clippy/pull/17850)
* [`manual_filter`: don't lint when the condition needs `&mut` a…](https://github.com/rust-lang/rust-clippy/pull/17810)
* [`multiple_unsafe_ops_per_block`: fix false positive in with write to union field](https://github.com/rust-lang/rust-clippy/pull/17457)
* [new lint: `unnecessary_box_pin`](https://github.com/rust-lang/rust-clippy/pull/13598)
* [skip `try_from_instead_of_from_str` in macro expansions](https://github.com/rust-lang/rust-clippy/pull/17843)
* [support trailing `::self` for `unnecessary_self_imports`](https://github.com/rust-lang/rust-clippy/pull/17785)

#### Rust-Analyzer
* [support inline call in macro call](https://github.com/rust-lang/rust-analyzer/pull/23428)
* [add downcast handling in `enum` closure capture](https://github.com/rust-lang/rust-analyzer/pull/23476)
* [assist do not add lifetime only turbofish](https://github.com/rust-lang/rust-analyzer/pull/23493)
* [check call no-paren after path colons](https://github.com/rust-lang/rust-analyzer/pull/23487)
* [complete cfg key-value in string inside `cfg!()`](https://github.com/rust-lang/rust-analyzer/pull/23507)
* [complete in top-level after inner-doc](https://github.com/rust-lang/rust-analyzer/pull/23486)
* [don't source-map `#[doc]` strings that contain escapes](https://github.com/rust-lang/rust-analyzer/pull/23443)
* [enum niche decoding should match rustc](https://github.com/rust-lang/rust-analyzer/pull/23508)
* [fix handling of generic params in anon consts](https://github.com/rust-lang/rust-analyzer/pull/23179)
* [pass rustc path to unstable-book-gen](https://github.com/rust-lang/rust-analyzer/pull/23503)
* [when the initializer in `let` diverges, allow non-diverging `else`](https://github.com/rust-lang/rust-analyzer/pull/23500)
* [mini refactors in closure capture inference](https://github.com/rust-lang/rust-analyzer/pull/23482)
* [perf: querify some things that are called by the solver and are fairly expensive to compute](https://github.com/rust-lang/rust-analyzer/pull/23207)
* [stabilize optimize attribute](https://github.com/rust-lang/rust-analyzer/pull/22511)
* [sync target feature implications from `rustc_target`](https://github.com/rust-lang/rust-analyzer/pull/23483)
* [typeMismatch quickfix position error: `to_owned()`](https://github.com/rust-lang/rust-analyzer/pull/22185)

### Rust Compiler Performance Triage

No significant improvements or regressions this week. Most of the PRs marked as improvements had
a similar perf. result, which was caused by codegen noise in the measurements.

There were some nice wins for memory usage though, thanks to [#163915](https://github.com/rust-lang/rust/pull/163915) and especially [#163887](https://github.com/rust-lang/rust/pull/163887). 

Triage done by **@Kobzol**.
Revision range: [cc9a14f7..7211ea59](https://perf.rust-lang.org/?start=cc9a14f721fac5226338c61dcec7d5ab785bde82&end=7211ea594cbf137ef6118b1752b959dea7cb9088&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.2%  | [0.1%, 0.6%]   | 73    |
| Regressions ❌ <br /> (secondary)  | 0.3%  | [0.1%, 1.4%]   | 57    |
| Improvements ✅ <br /> (primary)   | -0.3% | [-1.0%, -0.1%] | 21    |
| Improvements ✅ <br /> (secondary) | -0.3% | [-1.7%, -0.1%] | 28    |
| All ❌✅ (primary)                 | 0.1%  | [-1.0%, 0.6%]  | 94    |

3 Regressions, 21 Improvements, 2 Mixed; 1 of them in rollups
27 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/c0ecdb487d59cfbae4516ca92bef60b4906dd98f/triage/2026/2026-10-11.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/main)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [tier 3 target policy: target CPU and features must be supported by upstream backend](https://github.com/rust-lang/rust/pull/162840)
* [Implement `AsMut<T>` and `AsRef<T>` for `!`.](https://github.com/rust-lang/rust/pull/161253)
* [borrowck dataflow: respect borrow shallowness when killing borrows that conflict with assignments](https://github.com/rust-lang/rust/pull/160902)
* [move implied bounds computation out of borrowck](https://github.com/rust-lang/rust/pull/160491)
* [Allow `:` after `pat_param`](https://github.com/rust-lang/rust/pull/162264)
* [1.100-beta regression: `box` patterns have been removed (feature `box_patterns`)](https://github.com/rust-lang/rust/issues/163830)
* [Stabilize `-Zembed-metadata`](https://github.com/rust-lang/rust/pull/163436)
* [Make `std::fs::{File, ReadDir, DirEntry}` always `needs_drop` even when unsupported.](https://github.com/rust-lang/rust/pull/162444)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [RFC: Allow (outer) cfg-attributes on elements of tuple types and tuple (struct) patterns](https://github.com/rust-lang/rfcs/pull/3532)

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)
* [nail down the final validity rules: references and unions](https://github.com/rust-lang/reference/pull/2337)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)
* [All Hands 2027 hotel pre-booking](https://github.com/rust-lang/leadership-council/issues/340)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*
Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Support registry withholding](https://github.com/rust-lang/rfcs/pull/4016)
* [RFC: unsafe `global_asm`](https://github.com/rust-lang/rfcs/pull/4014)
* [RFC for limited crates.io self-service version deletion](https://github.com/rust-lang/rfcs/pull/4015)

## Upcoming Events

Rusty Events between 2026-10-14 - 2026-11-11 🦀

### Virtual
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-15 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/316943302/)
* 2026-10-18 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: Third Sunday**](https://www.meetup.com/dallasrust/events/316563013/)
* 2026-10-20 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/316788257/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Operating Systems Book Club: Paging Mechanisms**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/316880405/)
* 2026-10-22 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Rust and the GPU from Native to Web: An Introduction to `wgpu`**](https://luma.com/k1978ath)
* 2026-10-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316272609/)
* 2026-10-26 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**No Python was harmed: teaching a tiny MCU to learn as it goes**](https://luma.com/1byfc495)
* 2026-10-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday Rust Bookclub**](https://www.meetup.com/dallasrust/events/310254771/)
* 2026-10-27 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & Learn: Reasoning with Async Rust**](https://www.meetup.com/women-in-rust/events/315297195/)
* 2026-10-29 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Creating a hexadecimal editor in Rust**](https://luma.com/q8i3385k)
* 2026-11-01 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Cursed Rust**](https://luma.com/omlooqv8)
* 2026-11-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Deep Learning: First Sunday**](https://www.meetup.com/dallasrust/events/316331277/)
* 2026-11-03 | Virtual | [Rust 🦀 Maven](https://luma.com/rust-maven)
    * [**Solving Real-World Planning Problems in Rust with SolverForge**](https://luma.com/rfbzk3ae)
* 2026-11-03 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/315773682/)
* 2026-11-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcpbgb/)
* 2026-11-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/316907780/)
* 2026-11-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254770/)

### Asia
* 2026-11-03 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust November 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/316440864/)

### Europe
* 2026-10-14 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust)
    * [**22nd bcnrust session**](https://www.meetup.com/bcnrust/events/316316234/)
* 2026-10-14 - 2026-10-17 | Hybrid (Barcelona, ES) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust 2026**](https://eurorust.eu/)
* 2026-10-14 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn)
    * [**Rust in October: Macro-magic**](https://www.meetup.com/rustcologne/events/316891100/)
* 2026-10-20 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Creating a realtime web Multiuser Dungeon game with Dioxus**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/313816496/)
* 2026-10-22 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/316799366/)
* 2026-10-22 | Toulouse, FR | [Rust Toulouse](https://www.meetup.com/rust-community-toulouse)
    * [**Rust Toulouse Meetup - Rust & Python interoperability**](https://www.meetup.com/rust-community-toulouse/events/316880185/)
* 2026-10-27 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Hack Night: Rust meets AI**](https://www.meetup.com/rust-aarhus/events/316796329/)
* 2026-10-27 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #89**](https://www.meetup.com/rust-paris/events/316865643/)
* 2026-10-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #31**](https://www.meetup.com/stockholm-rust/events/316721062/)
* 2026-11-01 - 2026-11-03 | Italy, IN | [RustLab](https://rustlab.it/)
    * [**RustLab - The International Conference on Rust in Italy**](https://rustlab.it/schedule)
* 2026-11-04 | Dublin, IE | [Rust Dublin](https://www.meetup.com/rust-dublin)
    * [**Rust 263: Getting Started with the New Rust Driver for Oracle AI Database**](https://www.meetup.com/rust-dublin/events/316898325/)
* 2026-11-05 | Milano, IT | [Rust Language Milan](https://www.meetup.com/rust-language-milano)
    * [**Digital Model Trains & Rust: Bare-Metal DCC Command Station on ESP32**](https://www.meetup.com/rust-language-milan/events/316213872/)

### North America
* 2026-10-14 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA October: AI & Rust w/ Oxen.AI & Origin Lab!**](https://www.meetup.com/rust-los-angeles/events/315795432/)
* 2026-10-15 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**October, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/316943302/)
* 2026-10-17 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Central Cambridge Rust Lunch, Oct 17**](https://www.meetup.com/bostonrust/events/316924092/)
* 2026-10-20 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/315783988/)
* 2026-10-21 | Hybrid (Vancouver, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Disposable Agent Sandboxes in Rust**](https://www.meetup.com/vancouver-rust/events/315210233/)
* 2026-10-21 | San Francisco, CA, US | [Bay Area Rust](https://luma.com/bayarearust)
    * [**Bay Area Rust - Embedded Meetup**](https://luma.com/ur4pm34i)
* 2026-10-22 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc)
    * [**Rust NYC w/Bryan Cantrill - Code Matters: Reflections on Rust in the LLM Age**](https://www.meetup.com/rust-nyc/events/316901518/)
* 2026-10-24 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Davis Square Rust Lunch, Oct 24**](https://www.meetup.com/bostonrust/events/316924143/)
* 2026-10-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/316655403/)
* 2026-10-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**North End Rust Lunch, Oct 31**](https://www.meetup.com/bostonrust/events/316924288/)
* 2026-11-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**HTMX and Rust**](https://www.meetup.com/stl-rust/events/316896797/)

### South America
* 2026-11-07 | Planalto Paulista, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP - Agora no Luma**](https://www.meetup.com/rust-sao-paulo-meetup/events/316812852/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> I think that there are a nonzero number of people who are experts in the field who would genuinely respond to "how does C work?" with "it doesn't, next question"

– [Clar Fon at the rust-lang zulip](https://rust-lang.zulipchat.com/#narrow/channel/219381-t-libs/topic/optimize_for_size/near/629933186)

Thanks to [kleines Filmröllchen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1824) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
