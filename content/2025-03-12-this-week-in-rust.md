Title: This Week in Rust 590
Number: 590
Date: 2025-03-12
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
* [Announcing rustup 1.28.1](https://blog.rust-lang.org/2025/03/04/Rustup-1.28.1.html)
* [Inferred const generic arguments: Call for Testing!](https://blog.rust-lang.org/inside-rust/2025/03/05/inferred-const-generic-arguments.html)
* [This Month in Our Test Infra: January and February 2025](https://blog.rust-lang.org/inside-rust/2025/03/11/test-infra-jan-feb-2025.html)

### Project/Tooling Updates
* [Native Git support in Zed - Zed Blog](https://zed.dev/blog/git)
* [tfmcp 🦀: A Rust-Implemented Tool to Operate Terraform from LLMs](https://syu-m-5151.hatenablog.com/entry/2025/03/10/091144)
* [What's new in SeaORM 1.1](https://www.sea-ql.org/blog/2025-03-08-whats-new-in-sea-orm-1.1/)

### Observations/Thoughts
* [Rust in 2025: Targeting foundational software](https://smallcultfollowing.com/babysteps/blog/2025/03/10/rust-2025-intro/)
* [A Happy Day for Rust](https://steveklabnik.com/writing/a-happy-day-for-rust/)
* [Rust Learning Resources 2025](https://corrode.dev/blog/rust-learning-resources-2025/)
* [Taming A Voracious Rust Proxy](https://fly.io/blog/taming-rust-proxy/)
* [Succinct data structures](https://blog.startifact.com/posts/succinct/)
* [When is "this trait can be implemented" part of the trait's public API?](https://predr.ag/blog/when-is-trait-can-be-implemented-public-api/)
* [When are Rust's const fns executed?](https://felixwrt.dev/posts/const-fn/)
* [Rust trait object layout](https://neugierig.org/software/blog/2025/03/trait-object-layout.html)
* [The Art of Formatting Code](https://mcyoung.xyz/2025/03/11/formatters/)
* [video] [Rust is the New C](https://www.youtube.com/watch?v=3e-nauaCkgo)
* [audio] [Rust with Guillaume Gomez](https://rustacean-station.org/episode/guillaume-gomez/)

### Rust Walkthroughs
* [Writing into uninitialized buffers in Rust](https://blog.sunfishcode.online/writingintouninitializedbuffersinrust/)
* [Translating bzip2 with c2rust](https://trifectatech.org/blog/translating-bzip2-with-c2rust/)
* [Nine Pico PIO Wats with Rust: Raspberry Pi programmable IO pitfalls illustrated with a musical example (Part 1)](https://towardsdatascience.com/nine-pico-pio-wats-with-rust-part-1-9d062067dc25/)
* [Async Rust for Dummies](https://blog.veeso.dev/blog/en/async-rust-for-dummies/)
* [How we built our 2025 Embedded World Demos](https://ferrous-systems.com/blog/embedded-world-2025-demos/)
* [video] [Ratatui - terminal user interfaces in Rust with Orhun Parmaksız - build ratatop in pair programming](https://www.youtube.com/watch?v=OkmYsa25pIw)
* [video] [Derive Macros: Or, How I Learned to Stop Worrying and Love the proc_macro2::TokenStream](https://www.youtube.com/watch?v=ALZr9BwWHQU&t=1769s)
* [video] [Porting the guff plot device to Rust](https://www.youtube.com/watch?v=bbWcGAOsbIE)

### Miscellaneous
* [Rust Communities/User Groups World Map](https://mamaicode.github.io/rust-communities-map/)

## Crate of the Week

This week's crate is [eval-macro](https://crates.io/crates/eval-macro), a crate that allows to evaluate macros at compile time, giving similar feel to Zig's comptime.

Thanks to [Aleksander Krauze](https://users.rust-lang.org/t/crate-of-the-week/2704/1419) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer
and would like your RFC to appear in this list, add a `call-for-testing` label to your RFC along
with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

Let us know if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](URL to issue) -->
<!-- * [ - ]() -->
<!-- or if none - *No Calls for participation were submitted this week.* -->
*No Calls for participation were submitted this week.* 

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->
* [**GOSIM Rust Spotlight - Nominate and support your favorite projects!**](https://spotlight.gosim.org/rust2025#deadline-extended) | Closes 2025-03-15 at 7:59am UTC | Utrecht, NL | 2025-05-13 - 2025-05-17

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

555 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-04..2025-03-11

#### Compiler

* [ergonomic ref counting](https://github.com/rust-lang/rust/pull/134797)
* [on long spans, trim the middle of them to make them fit in the terminal width](https://github.com/rust-lang/rust/pull/137757)
* [split the `Edges` iterator](https://github.com/rust-lang/rust/pull/137655)
* [perf: change TaskDeps to start preallocated with 128 capacity](https://github.com/rust-lang/rust/pull/137563)
* [perf: speed up target feature computation](https://github.com/rust-lang/rust/pull/137586)

#### Library

* [stabilize `[T]::split_off...` methods](https://github.com/rust-lang/rust/pull/137829)
* [stabilize `box_uninit_write`](https://github.com/rust-lang/rust/pull/137850)
* [stabilize `const_char_classify, const_sockaddr_setters`](https://github.com/rust-lang/rust/pull/138129)
* [stabilize `const_vec_string_slice`](https://github.com/rust-lang/rust/pull/137319)
* [stabilize `string_extend_from_within`](https://github.com/rust-lang/rust/pull/137569)
* [stabilize feature `const_copy_from_slice`](https://github.com/rust-lang/rust/pull/138098)
* [override default `Write` methods for cursor-like types](https://github.com/rust-lang/rust/pull/137107)
* [specialize `OsString::push` and `OsString as From` for UTF-8](https://github.com/rust-lang/rust/pull/137777)
* [perf: improve the generic MIR in the default `PartialOrd::le` and friends](https://github.com/rust-lang/rust/pull/137904)
* [count char width at most once in `Formatter::pad`](https://github.com/rust-lang/rust/pull/136662)
* [fix char count in `Display` for `ByteStr`](https://github.com/rust-lang/rust/pull/137772)
* [fix crash in `BufReader::peek()`](https://github.com/rust-lang/rust/pull/137832)

#### Cargo

* [cargo tree: Add `--depth public` behind `-Zunstable-options`](https://github.com/rust-lang/cargo/pull/15243)
* [cargo: add terminal integration via ANSI OSC 9;4 sequences](https://github.com/rust-lang/cargo/pull/14615)
* [cargo: don't use `$CARGO_BUILD_TARGET` in `cargo metadata`](https://github.com/rust-lang/cargo/pull/15271)
* [cargo: add completions for add --path](https://github.com/rust-lang/cargo/pull/15288)
* [cargo: add completions for install --path](https://github.com/rust-lang/cargo/pull/15266)
* [cargo: respect --frozen everywhere --offline or --locked is accepted](https://github.com/rust-lang/cargo/pull/15263)

#### Rustdoc

* [fix `O(tests)` stack usage in edition 2024 mergeable doctests](https://github.com/rust-lang/rust/pull/138281)
* [search: increase strictness of typechecking](https://github.com/rust-lang/rust/pull/137981)* [rustdoc: add attribute-related tests for rustdoc JSON](https://github.com/rust-lang/rust/pull/138033)
* [hide item that is not marked as `doc(inline)` and whose src is `doc(hidden)`](https://github.com/rust-lang/rust/pull/137534)

#### Clippy

* [clippy: `arbitrary_source_item_ordering`: Make alphabetic ordering in module item groups optional](https://github.com/rust-lang/rust-clippy/pull/13718)
* [clippy: `unnecessary_to_owned`: don't call `iter()` on a temporary object](https://github.com/rust-lang/rust-clippy/pull/14243)
* [clippy: add missing tests annotations for `ui-internal`](https://github.com/rust-lang/rust-clippy/pull/14388)
* [clippy: don't trigger `blocks_in_conditions` when the condition contains a `return`](https://github.com/rust-lang/rust-clippy/pull/14338)
* [clippy: don't trigger `unnecessary_debug_formatting` in tests](https://github.com/rust-lang/rust-clippy/pull/14347)
* [clippy: fix `manual_let_else` missing binding mode](https://github.com/rust-lang/rust-clippy/pull/14204)
* [clippy: better help for `mixed_case_hex_literals`](https://github.com/rust-lang/rust-clippy/pull/14235)
* [clippy: improve `needless_pass_by_value` suggestion](https://github.com/rust-lang/rust-clippy/pull/13880)
* [clippy: make `struct_field_names` check private fields of public structs](https://github.com/rust-lang/rust-clippy/pull/14076)
* [clippy: refactor function after adding a new diagnostic item](https://github.com/rust-lang/rust-clippy/pull/14306)
* [clippy: remove Known problems section for `vec_box`](https://github.com/rust-lang/rust-clippy/pull/14252)
* [clippy: rename the MSRV alias `MANUAL_DIV_CEIL` to `DIV_CEIL`](https://github.com/rust-lang/rust-clippy/pull/14329)
* [clippy: use `size_of` from the prelude instead of imported](https://github.com/rust-lang/rust-clippy/pull/14355)
* [clippy: `io_error_other`: walk back to the root context to compute the span](https://github.com/rust-lang/rust-clippy/pull/14349)

#### Rust-Analyzer

* [rust-analyzer: `fix(hir): VariantDef` is `impl HasSource`](https://github.com/rust-lang/rust-analyzer/pull/19314)
* [rust-analyzer: add missing name-ref parents to syntactic highlighting](https://github.com/rust-lang/rust-analyzer/pull/19326)
* [rust-analyzer: add warning and debug information when `cargo metadata` fails](https://github.com/rust-lang/rust-analyzer/pull/19290)
* [rust-analyzer: adjust relevance scoring threshold to consistent with existing implem…](https://github.com/rust-lang/rust-analyzer/pull/19297)
* [rust-analyzer: add diagnostic for dangling dyn and impl](https://github.com/rust-lang/rust-analyzer/pull/19265)
* [rust-analyzer: warn the user when a rename will change the meaning of the program](https://github.com/rust-lang/rust-analyzer/pull/19079)
* [rust-analyzer: `path` macro hygiene](https://github.com/rust-lang/rust-analyzer/pull/19327)
* [rust-analyzer: syntax highlightingg punct filtering ignoring mods](https://github.com/rust-lang/rust-analyzer/pull/19292)
* [rust-analyzer: fix diagnostics being cleared right after being received](https://github.com/rust-lang/rust-analyzer/pull/19333)
* [rust-analyzer: normalize projections in evaluated const display and layout calculation](https://github.com/rust-lang/rust-analyzer/pull/19330)
* [rust-analyzer: prevent wrong invocations of `needs_parens_in` with non-ancestral "parent"s](https://github.com/rust-lang/rust-analyzer/pull/19324)
* [rust-analyzer: highlight unsafe operations as unsafe, not definitions](https://github.com/rust-lang/rust-analyzer/pull/19274)
* [rust-analyzer: improve keyword completion for 'let' and 'let mut'](https://github.com/rust-lang/rust-analyzer/pull/19279)
* [rust-analyzer: log build script error output in `load_cargo::load_workspace_at`](https://github.com/rust-lang/rust-analyzer/pull/19311)
* [rust-analyzer: make `GenericParamsCollector::type_or_consts` not unnecessarily `pub(crate)`](https://github.com/rust-lang/rust-analyzer/pull/19343)
* [rust-analyzer: make change annotations per text-edit](https://github.com/rust-lang/rust-analyzer/pull/19332)
* [rust-analyzer: move loaded project MSRV back to 1.78, show notification for the warning](https://github.com/rust-lang/rust-analyzer/pull/19308)
* [rust-analyzer: rank ADT constructors as constructors for completion scoring](https://github.com/rust-lang/rust-analyzer/pull/19325)

### Rust Compiler Performance Triage

This week we had to merge a lot of large rollups due to many problems with our CI infrastructure,
which made analysis harder. Even though the aggregated stats look like there were a lot of regressions,
it is skewed by two large regressions happening on an uncommon optimized incremental build and a
documentation build of a single crate. The documentation regression is being tracked, and fixes to
some other regressions are already in progress.

Triage done by **@kobzol**.
Revision range: [daf59857..9fb94b32](https://perf.rust-lang.org/?start=daf59857d6d2b87af4b846316bf1561a6083ed51&end=9fb94b32df38073bf63d009df77ed10cb1c989d0&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.2%  | [0.2%, 58.8%]   | 149   |
| Regressions ❌ <br /> (secondary)  | 4.2%  | [0.2%, 165.8%]  | 127   |
| Improvements ✅ <br /> (primary)   | -1.1% | [-14.0%, -0.3%] | 31    |
| Improvements ✅ <br /> (secondary) | -2.9% | [-38.4%, -0.1%] | 43    |
| All ❌✅ (primary)                 | 0.8%  | [-14.0%, 58.8%] | 180   |

2 Regressions, 2 Improvements, 5 Mixed; 4 of them in rollups
37 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/38617ae5d7a849d2f7fc7a712c737768b6ee4a90/triage/2025-03-11.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Deprecate the per-build-target `edition` field in `Cargo.toml`](https://github.com/rust-lang/rfcs/pull/3772)

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Uplift `clippy::invalid_null_ptr_usage` lint](https://github.com/rust-lang/rust/pull/119220)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC for doc_cfg, doc_cfg_auto, doc_cfg_hide and doc_cfg_show features](https://github.com/rust-lang/rfcs/pull/3631)

#### Other Areas
* No Items entered Final Comment Period this week for
  [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: const ergonomics for NonZero\<T\>](https://github.com/rust-lang/rfcs/pull/3786)

## Upcoming Events

Rusty Events between 2025-03-12 - 2025-04-09 🦀

### Virtual
* 2025-03-13 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820296)
* 2025-03-18 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**crum: Complex Numbers and Complex Matrices in Rust with Frans Slabber**](https://www.meetup.com/code-mavens/events/305823397/)
* 2025-03-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful—Rust GameDev Basics with Raylib by Tony Bradley**](https://www.meetup.com/rustdc/events/305170694)
* 2025-03-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Bacon & Performance Benchmarking**](https://www.meetup.com/vancouver-rust/events/305470139)
* 2025-03-20 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**Rust and embedded programming with Leon Vak (online in English)**](https://www.meetup.com/code-mavens/events/306357728)
* 2025-03-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361431)
* 2025-03-25 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**Lunch & Learn: SKIing into Rust - crafting a simple interpreter**](https://www.meetup.com/women-in-rust/events/305988711)
* 2025-03-27 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820297)
* 2025-04-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/events/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/305304228)
* 2025-04-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031661)
* 2025-04-03 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820282/)
* 2025-04-05 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Communicate with Channels in Rust**](https://www.eventbrite.com/e/communicate-with-channels-in-rust-tickets-1278267335009)
* 2025-04-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/303522530)

### Asia
* 2025-03-15 | Beijing, CN | [WebAssembly and Rust Meetup (Wasm Empowering AI)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**KCD Beijing 2025**](https://www.meetup.com/wasm-rust-meetup/events/303112196)
* 2025-03-19 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust March 2025 at Jit in Tel Aviv**](https://www.meetup.com/rust-tlv/events/305697580)
* 2025-03-28 | Kowloon Tong, HK | [Rust Asia](https://www.rustasiaconf.com/)
    * [**Rust Asia 2025**](https://www.rustasiaconf.com/)
* 2025-04-05 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**April 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)

### Europe
* 2025-03-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045445)
* 2025-03-13 | Biel, CH | [Rust Bern](https://www.meetup.com/rust-bern/events/)
    * [**2025 Rust Talks Bern #2 @ BFH Biel**](https://www.meetup.com/rust-bern/events/306169573)
* 2025-03-14 | Paris, FR | [Rust in Paris](https://www.rustinparis.com/)
    * [**Rust in Paris 2025**](https://www.rustinparis.com/schedule)
* 2025-03-18 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/events/)
    * [**Rust Meetup #10 @ MDPI Basel**](https://www.meetup.com/rust-basel/events/306121044)
* 2025-03-18 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**(1) Mago; (2) Iggy; (2) Rust binary sizes**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303729673)
* 2025-03-20 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**March Talks! (Two)**](https://www.meetup.com/rust-and-friends/events/306524042)
* 2025-03-20 | Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/events/)
    * [**Rust/C++ Meetup Prague (March 2025)**](https://www.meetup.com/rust-prague/events/306512157)
* 2025-03-25 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Hack Night - Robot Edition**](https://www.meetup.com/rust-aarhus/events/306478352)
* 2025-03-25 | Eindhoven, NL | [RustNL](https://www.meetup.com/rust-amsterdam/events/)
    * [**Rust x Julia Meetup Eindhoven**](https://www.meetup.com/rust-nederland/events/306434865)
* 2025-03-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Deep Dive into Async Rust**](https://www.meetup.com/london-rust-project-group/events/306643055)
* 2025-03-26 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main/events/)
    * [**“Beyond blazingly fast!” Performance Optimierungen in Rust**](https://www.meetup.com/rust-rhein-main/events/306659893)
* 2025-03-26 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester Talks March**](https://www.meetup.com/rust-manchester/events/305897029)
* 2025-03-26 | Warsaw, PL | [Rustikon](https://www.rustikon.dev/)
    * [**Rustikon**](https://www.rustikon.dev/)
* 2025-03-27 | Augsburg, DE | [Rust Meetup Augsburg](https://rust-augsburg.github.io/meetup)
    * [**Rust Meetup #12: Testing in Rust**](https://rust-augsburg.github.io/meetup/Meetup_12.html)
* 2025-04-02 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/306097261)
* 2025-04-02 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541535)
* 2025-04-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/stockholm-rust/events/306627608)
* 2025-04-03 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809680)
* 2025-04-08 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**3. Rust Moravia Meetup (Real Embedded Rust)**](https://www.meetup.com/rust-moravia/events/306377283)
* 2025-04-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045446)

### North America
* 2025-03-13 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/306484710)
* 2025-03-13 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**PDXRust Meetup: Finding One Way Out**](https://www.meetup.com/pdxrust/events/306658850)
* 2025-03-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638264)
* 2025-03-18 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/events/)
    * [**Monthly Meetup: Intro to Rust and Python; Using Rustup, Cargo, and Rust!**](https://www.meetup.com/spokane-rust/events/306368210)
* 2025-03-20 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/306473234)
* 2025-03-20 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 3: Community Presentations**](https://www.meetup.com/music-city-rust-developers/events/304333074/)
* 2025-03-20 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**March, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/join-srug/events/305658448)
* 2025-03-21 | México City, MX | [Rust MX](https://www.meetup.com/rust-mx/events/)
    * [**Rust y AWS Lambda. Preparando el camino para desplegar ML/AI**](https://www.meetup.com/rust-mx/events/306406018)
* 2025-03-26 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcfbjc)
* 2025-03-27 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**We're going again!**](https://www.meetup.com/rust-atl/events/306470345)
* 2025-03-31 | Boulder, CO, US | [Solid State Depot](https://www.meetup.com/solidstatedepot/events/)
    * [**Boulder Rust: Bryan presents Rusted Hardware**](https://www.meetup.com/solidstatedepot/events/306573447)
* 2025-04-03 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**April Monthly Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 2025-04-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - resource-constrained internationalization (i18n)**](https://www.meetup.com/stl-rust/events/304890140)

### South America
* 2025-03-15 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup/events/)
    * [**Encontro do Rust-SP na CloudWalk**](https://www.meetup.com/rust-sao-paulo-meetup/events/306034427)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ivrkhs/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Treat *anything* starting with `cargo` as if it is `cargo run`. This applies even to commands that do not build anything, such as `cargo clean`, and third-party plugins, such as `cargo audit`.

– [Sergey "Shnatsel" Davidoff on /r/rust](https://old.reddit.com/r/rust/comments/1j2i3s0/psa_do_not_run_any_cargo_commands_on_untrusted)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1661) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1ja2n2d/this_week_in_rust_590/)</small>
