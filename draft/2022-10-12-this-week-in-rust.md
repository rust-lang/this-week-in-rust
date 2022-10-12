Title: This Week in Rust 464
Number: 464
Date: 2022-10-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

* [Announcing error-stack v0.2](https://hash.dev/blog/error-stack-update-0-2)

### Observations/Thoughts
* [RAII: Compile-Time Memory Management in C++ and Rust](https://www.thecodedmessage.com/posts/raii/)

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [pci-driver](https://crates.io/crates/pci-driver), a crate to develop user-space PCI(e) linux drivers.

Thanks to [Alberto Faria](https://users.rust-lang.org/t/crate-of-the-week/2704/1111) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-10-03..2022-10-10

* [use BOLT in CI to optimize LLVM](https://github.com/rust-lang/rust/pull/94381)
* [stabilize `half_open_range_patterns`](https://github.com/rust-lang/rust/pull/102275)
* [check `WhereClauseReferencesSelf` after all other object safety checks](https://github.com/rust-lang/rust/pull/102764)
* [check generic argument compatibility when projecting assoc ty](https://github.com/rust-lang/rust/pull/102488)
* [delay evaluating lint primary message until after it would be suppressed](https://github.com/rust-lang/rust/pull/102567)
* [disable compressed debug sections on i586-gnu](https://github.com/rust-lang/rust/pull/102748)
* [don't ICE when trying to copy unsized value in const prop](https://github.com/rust-lang/rust/pull/102559)
* [enable inline stack probes on X86 with LLVM 16](https://github.com/rust-lang/rust/pull/102503)
* [suggest `.into()` when all other coercion suggestions fail](https://github.com/rust-lang/rust/pull/102496)
* [suggest `==` to wrong assign expr](https://github.com/rust-lang/rust/pull/102708)
* [suggest calling method if fn does not exist](https://github.com/rust-lang/rust/pull/102694)
* [suggest `unwrap_or_else` when a closure is given](https://github.com/rust-lang/rust/pull/102441)
* [fix MIR inlining of `asm_unwind`](https://github.com/rust-lang/rust/pull/102778)
* [fix `#[derive(Default)]` on a generic `#[default]` enum adding unnecessary `Default` bounds](https://github.com/rust-lang/rust/pull/101040)
* [fix unwind drop glue for if-then scopes](https://github.com/rust-lang/rust/pull/102394)
* [lint against nested opaque types that don't satisfy associated type bounds](https://github.com/rust-lang/rust/pull/102568)
* [make tests capture the error printed by a `Result` return](https://github.com/rust-lang/rust/pull/102794)
* [only allow `~const` bounds for traits with `#[const_trait]`](https://github.com/rust-lang/rust/pull/102647)
* [recover from impl Trait in type param bound](https://github.com/rust-lang/rust/pull/102345)
* [remove `TypeckResults` from `InferCtxt`](https://github.com/rust-lang/rust/pull/101632)
* [show let-else suggestion on stable](https://github.com/rust-lang/rust/pull/102820)
* [skip chained OpaqueCast when building captures](https://github.com/rust-lang/rust/pull/102853)
* [trying to suggest additional lifetime parameter](https://github.com/rust-lang/rust/pull/102323)
* [uplift `clippy::for_loops_over_fallibles` lint into rustc](https://github.com/rust-lang/rust/pull/99696)
* [don't ICE when normalizing closure input tys](https://github.com/rust-lang/rust/pull/99818)
* [make `const_err` a hard error](https://github.com/rust-lang/rust/pull/102091)
* [panic-on-uninit: adjust checks to 0x01-filling](https://github.com/rust-lang/rust/pull/101061)
* [introduce `{char, u8}::is_ascii_octdigit`](https://github.com/rust-lang/rust/pull/101308)
* [std: use futex in `Once`](https://github.com/rust-lang/rust/pull/99505)
* [`EscapeAscii` is not an `ExactSizeIterator`](https://github.com/rust-lang/rust/pull/99880)
* [slice: `#[inline]` a couple iterator methods](https://github.com/rust-lang/rust/pull/96711)
* [add `Vec::push_within_capacity` - fallible, does not allocate](https://github.com/rust-lang/rust/pull/89123)
* [add `T` to `impl Debug for PhantomData`](https://github.com/rust-lang/rust/pull/99099)
* [only export `__tls_*` on wasm32-unknown-unknown](https://github.com/rust-lang/rust/pull/102440)
* [add `AsFd` implementations for stdio lock types on WASI](https://github.com/rust-lang/rust/pull/101768)
* [add `ptr::Alignment` type](https://github.com/rust-lang/rust/pull/102072)
* [reduce `CString` allocations in std as much as possible](https://github.com/rust-lang/rust/pull/93668)
* [avoid repeated re-initialization of the `BufReader` buffer](https://github.com/rust-lang/rust/pull/102760)
* [do the `calloc` optimization for `Option<bool>`](https://github.com/rust-lang/rust/pull/102596)
* [`From<Alignment>` for {`usize`, `NonZeroUsize`}](https://github.com/rust-lang/rust/pull/102862)
* [Make `Hash`{`Set`, `Map`}`::with_hasher` unstably const](https://github.com/rust-lang/rust/pull/102574)
* [implement `Ready::into_inner()`](https://github.com/rust-lang/rust/pull/101189)
* [futures: implement `Clone` for `Drain`](https://github.com/rust-lang/futures-rs/pull/2650)
* [codegen\_gcc: fix `fmaddsub`](https://github.com/rust-lang/rustc_codegen_gcc/pull/229)
* [codegen\_gcc: fix simd bitmask](https://github.com/rust-lang/rustc_codegen_gcc/pull/230)
* [codegen\_gcc: fix simd select bitmask](https://github.com/rust-lang/rustc_codegen_gcc/pull/231)
* [codegen\_gcc: simd: enable `simd_as` intrinsic](https://github.com/rust-lang/rustc_codegen_gcc/pull/228)
* [cargo: import `cargo remove` into cargo](https://github.com/rust-lang/cargo/pull/11099)
* [cargo: add completions for `cargo remove`](https://github.com/rust-lang/cargo/pull/11204)
* [cargo: add retry support to sparse registries](https://github.com/rust-lang/cargo/pull/11069)
* [cargo: config file loaded via CLI takes priority over env vars](https://github.com/rust-lang/cargo/pull/11077)
* [cargo: fix sparse registry lockfile urls containing 'registry+sparse+'](https://github.com/rust-lang/cargo/pull/11177)
* [cargo: source replacement ambiguity](https://github.com/rust-lang/cargo/pull/10907) (RFC [#3289](https://rust-lang.github.io/rfcs/3289-source_replacement_ambiguity.html))
* [rustdoc: render more cross-crate HRTBs properly](https://github.com/rust-lang/rust/pull/102707)
* [bindgen: generated name override](https://github.com/rust-lang/rust-bindgen/pull/2228)
* [bindgen: context: fix tokenization of C++20 inline namespace](https://github.com/rust-lang/rust-bindgen/pull/2294)
* [clippy: add `disallowed_macros` lint](https://github.com/rust-lang/rust-clippy/pull/9495)
* [clippy: add `manual_filter` lint for `Option`](https://github.com/rust-lang/rust-clippy/pull/9451)
* [clippy: new `implicit_saturating_add` lint](https://github.com/rust-lang/rust-clippy/pull/9549)
* [clippy: add a temporary workaround for  multiline formart arg inlining](https://github.com/rust-lang/rust-clippy/pull/9599)
* [clippy: don't suggest moving tuple structs with a significant drop to late evaluation](https://github.com/rust-lang/rust-clippy/pull/9610)
* [clippy: fix `arithmetic_side_effects` false negative](https://github.com/rust-lang/rust-clippy/pull/9559)
* [clippy: don't ignore literal references on `arithmetic-side-effects`](https://github.com/rust-lang/rust-clippy/pull/9587)
* [clippy: `FormatArgsExpn`: find comma spans and ignore weird proc macro spans](https://github.com/rust-lang/rust-clippy/pull/9586)
* [clippy: further enhance `needless_borrow`, mildly refactor `redundant_clone`](https://github.com/rust-lang/rust-clippy/pull/9386)
* [clippy: `match_single_binding` add curlies for more cases to fix suggestion](https://github.com/rust-lang/rust-clippy/pull/9601)
* [clippy: `suboptimal_flops` lint for multiply and subtract](https://github.com/rust-lang/rust-clippy/pull/9581)
* [clippy: extend `box-default` lint, add suggestion](https://github.com/rust-lang/rust-clippy/pull/9585)
* [clippy: let `upper_case_acronyms` check the enum name](https://github.com/rust-lang/rust-clippy/pull/9580)
* [clippy: `unsafe_removed_from_name`: fix false positive when `#[allow]`ed](https://github.com/rust-lang/rust-clippy/pull/9593)
* [rust-analyzer: add `convert_named_struct_to_tuple_struct` assist](https://github.com/rust-lang/rust-analyzer/pull/13303)
* [rust-analyzer: prefer similar tokens when expanding macros speculatively](https://github.com/rust-lang/rust-analyzer/pull/13392)
* [rust-analyzer: fix `generate_method`: correct method indentation inside generated impl and change gen loc](https://github.com/rust-lang/rust-analyzer/pull/13333)
* [rust-analyzer: expand unmatched mbe fragments to reasonable default token trees](https://github.com/rust-lang/rust-analyzer/pull/13384)
* [rust-analyzer: honor cfg attributes on params when lowering their patterns](https://github.com/rust-lang/rust-analyzer/pull/13380)
* [rust-analyzer: make go-to-def work for `#[doc = include_str!("path")]`](https://github.com/rust-lang/rust-analyzer/pull/13362)
* [rust-analyzer: in VSCode, correctly resolve relative paths to errors](https://github.com/rust-lang/rust-analyzer/pull/13367)
* [rust-analyzer: treat enum variants as generic item on their own](https://github.com/rust-lang/rust-analyzer/pull/13339)
* [rust-analyzer: use `BoundVar`s from current generic scope](https://github.com/rust-lang/rust-analyzer/pull/13344)
* [perf: add basic runtime benchmark infrastructure](https://github.com/rust-lang/rustc-perf/pull/1423)
* [add n-body simulation runtime benchmark](https://github.com/rust-lang/rustc-perf/pull/1459)

### Rust Compiler Performance Triage

Overall, a fairly quiet week where the change to primary benchmarks ended up breaking exactly even.
Secondary benchmarks saw improvements but not in large enough numbers for it to be particularly noteworthy.

Triage done by **@rylev**.
Revision range: [02cd79a..1e926f0](https://perf.rust-lang.org/?start=02cd79afb8080fce8c8ce35533c54d8ecf8f390e&end=1e926f06528ecb2503f026e2fd53cb735d487b10&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u) | mean | range | count |
|:----------------:|:----:|:-----:|:-----:|
| Regressions ❌ <br /> (primary) | 0.8% | [0.2%, 1.4%] | 19    |
| Regressions ❌ <br /> (secondary) | 1.0% | [0.3%, 1.8%] | 9     |
| Improvements ✅ <br /> (primary) | -0.6% | [-1.8%, -0.3%] | 29    |
| Improvements ✅ <br /> (secondary) | -1.0% | [-6.4%, -0.2%] | 39    |
| All ❌✅ (primary) | -0.0% | [-1.8%, 1.4%] | 48    |


3 Regressions, 1 Improvements, 6 Mixed; 4 of them in rollups
41 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-10-11.md)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

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

* [disposition: merge] [make unaligned_reference a hard error](https://github.com/rust-lang/rust/pull/102513)
* [disposition: merge] [Stabilize raw-dylib for non-x86](https://github.com/rust-lang/rust/issues/102793)
* [disposition: merge] [Only apply `ProceduralMasquerade` hack to older versions of `rental`](https://github.com/rust-lang/rust/pull/94063)
* [disposition: merge] [Remove save-analysis.](https://github.com/rust-lang/rust/pull/101841)
* [disposition: merge] [make order_dependent_trait_objects show up in future-breakage reports](https://github.com/rust-lang/rust/pull/102635)
* [disposition: merge] [merge functionality of io::Sink into io::Empty](https://github.com/rust-lang/rust/pull/98154)
* [disposition: merge] [Stabilize `duration_checked_float`](https://github.com/rust-lang/rust/pull/102271)
* [disposition: merge] [Change process spawning to inherit the parent's signal mask by default](https://github.com/rust-lang/rust/pull/101077)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [unsafe attributes](https://github.com/rust-lang/rfcs/pull/3325)
* [new] [Support upcasting of `dyn Trait` values](https://github.com/rust-lang/rfcs/pull/3324)
* [new] [Restrictions](https://github.com/rust-lang/rfcs/pull/3323)

## Upcoming Events

Rusty Events between 2022-10-12 - 2022-11-09 🦀

### Virtual

* 2022-10-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcnbhb/)
* 2022-10-05 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydcnbhb/)
* 2022-10-06 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online #18**](https://www.meetup.com/rust-noris/events/hlvbvsydcnbrb/)
* 2022-10-08 | Virtual | [Rust GameDev](https://gamedev.rs/)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-10-11 | Virtual (Berlin, DE) | [Open TechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/288628471/)
* 2022-10-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcnbpb/)
* 2022-10-11 | Virtual (Saarbrücken, DE) | [Rust-Saar](https://www.meetup.com/rust-saar/)
    * [**Meetup: 23u16**](https://www.meetup.com/rust-saar/events/288768344/)
* 2022-10-11 | Virtual (Weiden, DE) | [Digital Craftsmanship Nordoberpfalz](https://www.meetup.com/digital-craftsmanship-nordoberpfalz/)
    [**Woher kommt der Hype? Rust in 45 Minuten**](https://www.meetup.com/digital-craftsmanship-nordoberpfalz/events/286681839/)
* 2022-10-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-12 | Virtual (Erlangen, DE) | [Rust Franken](https://www.meetup.com/rust-nerf/)
    * [**Rust Franken Meetup #4**](https://www.meetup.com/rust-nerf/events/288723552/)
* 2022-10-12 | Virtual (San Francisco, CA, US / Redmond, WA, US / London UK) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/) | [**Redmond Reactor Mirror Event**](https://www.meetup.com/microsoft-reactor-redmond/events/288475797/) | [**London Reactor Mirror Event**](https://www.meetup.com/microsoft-reactor-london/events/288475801/)
* 2022-10-13 | Virtual (Berlin, DE) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-15 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 2 (CuteCopter): Reverse Engineering a tiny drone**](https://www.meetup.com/rust-noris/events/287347851/)
* 2022-10-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Impractical Rust: The HATETRIS World Record**](https://www.meetup.com/rustdc/events/vdhxgsydcnbxb/)
* 2022-10-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rapid Prototyping in Rust: Write fast like Python; Run fast like C**](https://www.meetup.com/vancouver-rust/events/288641106/)
* 2022-10-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcnbbc/)
* 2022-10-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcnbhc/)
* 2022-10-26 | Virtual (Redmond, WA, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Your First Rust Project: Rust Basics**](https://www.meetup.com/microsoft-reactor-redmond/events/288475815/)
* 2022-10-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Using Applicative Functors to parse command line options**](https://www.meetup.com/charlottesville-rust-meetup/events/288867237/)
* 2022-11-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcpbcb/)
* 2022-11-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/287027659/)
* 2022-11-02 | Virtual (Redmond, WA, US / San Francisco, SF, US) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: From Java Dev to Rust Developer**](https://www.meetup.com/microsoft-reactor-redmond/events/288475833/) | [**San Francisco Reactor Mirror Event**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475838/) | [**London Reactor Mirror Event**](https://www.meetup.com/microsoft-reactor-london/events/288475832/)

### Asia

* 2022-10-11 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Cost-Efficient Rust in Practice**](https://www.meetup.com/tokyo-rust-meetup/events/288597490/)

### Europe

* 2022-10-06 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #29**](https://www.meetup.com/rust-wroclaw/events/288667400/)
* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)
* 2022-10-13 | Berlin, DE + Virtual | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #53**](https://www.meetup.com/rust-paris/events/288735204/)

### North America

* 2022-10-13 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcnbrb/)
* 2022-10-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcnbxb/)
* 2022-10-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Anyhow ? Turbofish ::<> / HTTP calls and errors in Rust.**](https://www.meetup.com/rust-nyc/events/288756215/)
* 2022-10-20 | New York, NY, US | [Cloud Native New York](https://www.meetup.com/cloud-native-new-york/)
    * [**Cloud-native Search Engine for Log Management and Analytics.**](https://www.meetup.com/cloud-native-new-york/events/288818963/)
* 2022-10-25 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**Rust DHCP**](https://www.meetup.com/rust-toronto/events/288589539/)
* 2022-10-27 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Bevy Crash Course with Nathan and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcnbkc/)

### Oceania

* 2022-10-10 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**Rust Lightning Talks**](https://www.meetup.com/rust-sydney/events/288746516/)
* 2022-10-20 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Tune Up Edition: software engineering management**](https://www.meetup.com/rust-wellington/events/288738684/)

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

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> There's a lot of weird debate about whether Rust in the kernel is useful or not... in my experience, it's way more useful than I could've ever imagined!
>
> I went from 1st render to a stable desktop that can run run games, browsers, etc. in about two days of work on my driver (!!!)
>
> All the concurrency bugs just vanish with Rust! Memory gets freed when it needs to be freed! Once you learn to make Rust work with you, I feel like it guides you into writing correct code, even beyond the language's safety promises. It's seriously magic! ✨
>
> There is absolutely no way I wouldn't have run into race conditions, UAFs, memory leaks, and all kinds of badness if I'd been writing this in C.
>
> In Rust? Just some logic bugs and some core memory management issues. Once those were fixed, the rest of the driver just worked!!

– [Asahi Lina on twitter](https://twitter.com/LinaAsahi/status/1577667445719912450?s=20&t=0kECRUOhaXrf31ECygGJ8w)

[llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1308) is mightily pleased with his suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
