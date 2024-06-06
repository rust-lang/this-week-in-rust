Title: This Week in Rust 550
Number: 550
Date: 2024-06-05
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://x.com/ThisWeekInRust) on X(formerly Twitter) or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Foundation
* [Welcoming Rust-C++ Interoperability Engineer Jon Bauman to the Rust Foundation Team](https://foundation.rust-lang.org/news/welcoming-rust-c-interoperability-engineer-jon-bauman-to-the-rust-foundation-team/)

### RustNL 2024
* [Visual Application Design for Rust - Rik Arends](https://www.youtube.com/watch?v=NPP2_6KMA60)
* [ThRust in Space: Initial Momentum - Michaël Melchiore](https://www.youtube.com/watch?v=O09rje6yC90)
* [Arc in the Linux Kernel - Alice Ryhl](https://www.youtube.com/watch?v=gr9v0FFXaZ8)
* [Making Connections - Mara Bos](https://www.youtube.com/watch?v=aENHzYAFkeE)
* [Replacing OpenSSL One Step at a Time - Joe Birr-Pixton](https://www.youtube.com/watch?v=10ymtv1J7Os)
* [Fortifying Rust's FFI with Enscapsulated Functions - Leon Schuermann](https://www.youtube.com/watch?v=O4sVw4YQB)
* [Oxidizing Education - Henk Oordt](https://www.youtube.com/watch?v=KwZM0lSTvyk)
* [Postcard: An Unreasonably Effective Tool for Machine to Machine Communication - James Munns](https://www.youtube.com/watch?v=HtBFvTH5ZKE)
* [Introducing June - Sophia Turner](https://www.youtube.com/watch?v=c1isq1Bdmic)
* [Robius: Immersive and Seamless Multiplatform App Development in Rust - Kevin Boos](https://www.youtube.com/watch?v=Dg4hlfettn8)
* [Compression Carcinized: Implementing zlib in Rust - Folkert de Vries](https://www.youtube.com/watch?v=mvzHQdCLkOY)
* [K23: A Secure Research OS Running WASM - Jonas Kruckenberg](https://www.youtube.com/watch?v=GjDwj7RWOgs)
* [Async Rust in Embedded Systems with Embassy - Dario Nieuwenhuis](https://www.youtube.com/watch?v=H7NtzyP9q8E)
* [Xilem: Let's Build High Performance Rust UI - Raph Levien](https://www.youtube.com/watch?v=OvfNipIcRiQ)
* [Rust Poisoning My Wrist for Fun - Ulf Lilleengen](https://www.youtube.com/watch?v=u6q9l89EOXI)
* [Type Theory for Busy Engineers - Niko Matsakis](https://www.youtube.com/watch?v=9qLACD9Bfbk)

### Newsletters
* [This Month in Rust GameDev #51 - May 2024](https://gamedev.rs/news/051/)

### Project/Tooling Updates
* [Enter paradis — A new chapter in Rust's parallelism story](https://andreaslongva.com/blog/enter-paradis/)
* [Tiny Glade, VJ performances, and 2d lighting](https://thisweekinbevy.com/issue/2024-06-03-tiny-glade-vj-performances-and-2d-lighting)
* [Diesel 2.2.0](https://diesel.rs/news/2_2_0_release.html)
* [Pigg 0.1.0](https://github.com/andrewdavidmackenzie/pigg/releases/tag/0.1.0)
* [git-cliff 2.3.0 is released! (highly customizable changelog generator)](https://git-cliff.org/blog/2.3.0)

### Observations/Thoughts
* [The borrow checker within](https://smallcultfollowing.com/babysteps/blog/2024/06/02/the-borrow-checker-within/)
* [Don't Worry About Lifetimes](https://corrode.dev/blog/lifetimes/)
* [rust is not about memory safety](https://o-santi.github.io/blog/rust-is-not-about-memory-safety/)
* [On Dependency Usage in Rust](https://landaire.net/on-dependency-usage-in-rust/)
* [Context Managers: Undroppable Types for Free](https://blog.yoshuawuyts.com/achieving-undroppable-types-by-leveraging-context-managers/)
* [Rust and dynamically-sized thin pointers](https://john-millikin.com/rust-and-dynamically-sized-thin-pointers)
* [Rust is for the Engine, Not the Game](https://barretts.club/posts/rust-for-the-engine/)
* [audio] [Thunderbird - Brendan Abolivier, Software Engineer](https://corrode.dev/podcast/s02e03-thunderbird/)

### Rust Walkthroughs
* [Build with Naz : Rust typestate pattern](https://developerlife.com/2024/05/28/typestate-pattern-rust/)
* [How to build a plugin system in Rust](https://www.arroyo.dev/blog/rust-plugin-systems)
* [Forming Clouds](https://isaacdaou.st/blog/forming-clouds/)
* [Rust error handling: Option & Result](https://bitfieldconsulting.com/posts/rust-errors-option-result)
* [Let's build a Load Balancer in Rust - Part 3](https://marcobacis.com/blog/load-balancer-rust-3/)
* [The Ultimate Guide to Rust Newtypes](https://www.howtocodeit.com/articles/ultimate-guide-rust-newtypes)

### Miscellaneous
* [Highlights from "I spent 6 years developing a puzzle game in Rust and it just shipped, AMA"](https://gamesbymason.com/2024/06/01/wor-ama/)

## Crate of the Week

This week's crate is [layoutparser-ort](https://docs.rs/layoutparser-ort), a simplified port of LayoutParser for ML-based document layout element detection.

Despite there being no suggestions, llogiq is reasonably happy with his choice. Are you?

[No matter what your answer is, please submit your suggestions and votes for next week][submit_crate]!

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

*No Calls for participation in projects were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [Scientific Computing in Rust 2024](https://scientificcomputing.rs/) | Closes 2024-06-14 | online | Event date: 2024-07-17 - 2024-07-19
* [Rust Ukraine 2024](https://docs.google.com/forms/d/e/1FAIpQLSc9S_95oaCsFyrULF4iBQOIiTcMlOpG07izgquYLBCKFAYTKQ/viewform) | Closes 2024-07-06 | Online + Ukraine, Kyiv | Event date: 2024-07-27
* [Conf42 Rustlang 2024](https://www.papercall.io/conf42-rustlang-2024) | Closes 2024-07-22 | online | Event date: 2024-08-22

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (Formerly twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

308 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-05-28..2024-06-04

* [`-Znext-solver`: eagerly normalize when adding goals](https://github.com/rust-lang/rust/pull/125343)
* [`fn_arg_sanity_check`: fix panic message](https://github.com/rust-lang/rust/pull/125695)
* [add `--print=check-cfg` to get the expected configs](https://github.com/rust-lang/rust/pull/124320)
* [add `-Zfixed-x18`](https://github.com/rust-lang/rust/pull/124655)
* [also InstSimplify `&raw*`](https://github.com/rust-lang/rust/pull/125796)
* [also resolve the type of constants, even if we already turned it into an error constant](https://github.com/rust-lang/rust/pull/125807)
* [avoid unwrap diag.code directly in `note_and_explain_type_err`](https://github.com/rust-lang/rust/pull/125774)
* [check index `value <= 0xFFFF_FF00`](https://github.com/rust-lang/rust/pull/125821)
* [coverage: avoid overflow when the MC/DC condition limit is exceeded](https://github.com/rust-lang/rust/pull/125700)
* [coverage: optionally instrument the RHS of lazy logical operators](https://github.com/rust-lang/rust/pull/125756)
* [coverage: rename MC/DC `conditions_num` to `num_conditions`](https://github.com/rust-lang/rust/pull/125754)
* [create const block DefIds in typeck instead of ast lowering](https://github.com/rust-lang/rust/pull/124650)
* [do not equate `Const`'s ty in `super_combine_const`](https://github.com/rust-lang/rust/pull/125671)
* [do not suggest unresolvable builder methods](https://github.com/rust-lang/rust/pull/125397)
* [a small diagnostic improvement for `dropping_copy_types`](https://github.com/rust-lang/rust/pull/125433)
* [don't recompute `tail` in `lower_stmts`](https://github.com/rust-lang/rust/pull/125790)
* [don't suggest turning non-char-literal exprs of ty `char` into string literals](https://github.com/rust-lang/rust/pull/125640)
* [enable DestinationPropagation by default](https://github.com/rust-lang/rust/pull/115105)
* [fold item bounds before proving them in `check_type_bounds` in new solver](https://github.com/rust-lang/rust/pull/125786)
* [implement `needs_async_drop` in rustc and optimize async drop glue](https://github.com/rust-lang/rust/pull/124662)
* [improve diagnostic output of `non_local_definitions` lint](https://github.com/rust-lang/rust/pull/125089)
* [make `ProofTreeBuilder` actually generic over `Interner`](https://github.com/rust-lang/rust/pull/125598)
* [make `body_owned_by` return the `Body` instead of just the `BodyId`](https://github.com/rust-lang/rust/pull/125711)
* [make `repr(packed)` vectors work with SIMD intrinsics](https://github.com/rust-lang/rust/pull/125311)
* [make lint: `lint_dropping_references lint_forgetting_copy_types lint_forgetting_references` give suggestion if possible](https://github.com/rust-lang/rust/pull/125531)
* [omit `non-needs_drop drop_in_place` in vtables](https://github.com/rust-lang/rust/pull/122662)
* [opt-in to `FulfillmentError` generation to avoid doing extra work in the new solver](https://github.com/rust-lang/rust/pull/125864)
* [reintroduce name resolution check for trying to access locals from an inline const](https://github.com/rust-lang/rust/pull/125705)
* [reject `CVarArgs` in `parse_ty_for_where_clause`](https://github.com/rust-lang/rust/pull/125863)
* [show files produced by `--emit foo` in json artifact notifications](https://github.com/rust-lang/rust/pull/122597)
* [silence some resolve errors when there have been glob import errors](https://github.com/rust-lang/rust/pull/125381)
* [stop using `translate_args` in the new solver](https://github.com/rust-lang/rust/pull/125776)
* [support mdBook preprocessors for TRPL in rustbook](https://github.com/rust-lang/rust/pull/125408)
* [test codegen for `repr(packed,simd)` → `repr(simd)`](https://github.com/rust-lang/rust/pull/125904)
* [tweak relations to no longer rely on `TypeTrace`](https://github.com/rust-lang/rust/pull/125664)
* [unroll first iteration of `checked_ilog` loop](https://github.com/rust-lang/rust/pull/124294)
* [uplift `{Closure,Coroutine,CoroutineClosure}Args` and friends to `rustc_type_ir`](https://github.com/rust-lang/rust/pull/125775)
* [use parenthetical notation for `Fn` traits](https://github.com/rust-lang/rust/pull/125778)
* [add some more specific checks to the MIR validator](https://github.com/rust-lang/rust/pull/125851)
* [miri: avoid making a full copy of all new allocations](https://github.com/rust-lang/rust/pull/125633)
* [miri: fix "local crate" detection](https://github.com/rust-lang/miri/pull/3644)
* [don't inhibit random field reordering on `repr(packed(1))`](https://github.com/rust-lang/rust/pull/125360)
* [avoid checking the edition as much as possible](https://github.com/rust-lang/rust/pull/125828)
* [increase vtable layout size](https://github.com/rust-lang/rust/pull/123572)
* [stabilise `IpvNAddr::`{`BITS`, `to_bits`, `from_bits`} (`ip_bits`)](https://github.com/rust-lang/rust/pull/125551)
* [stabilize `custom_code_classes_in_docs` feature](https://github.com/rust-lang/rust/pull/124577)
* [stablize `const_binary_heap_constructor`](https://github.com/rust-lang/rust/pull/125211)
* [make `std::env::`{`set_var`, `remove_var`} unsafe in edition 2024](https://github.com/rust-lang/rust/pull/124636)
* [implement feature `integer_sign_cast`](https://github.com/rust-lang/rust/pull/125884)
* [NVPTX: avoid `PassMode::Direct` for args in C abi](https://github.com/rust-lang/rust/pull/117671)
* [genericize `ptr::from_raw_parts`](https://github.com/rust-lang/rust/pull/125701)
* [`std::pal::unix::thread` fetching min stack size on netbsd](https://github.com/rust-lang/rust/pull/125577)
* [add an intrinsic for `ptr::metadata`](https://github.com/rust-lang/rust/pull/124251)
* [change `f32::midpoint` to upcast to f64](https://github.com/rust-lang/rust/pull/121062)
* [rustc-hash: replace hash with faster and better finalized hash](https://github.com/rust-lang/rustc-hash/pull/37)
* [cargo test: Auto-redact elapsed time](https://github.com/rust-lang/cargo/pull/13973)
* [cargo add: Avoid escaping double-quotes by using string literals](https://github.com/rust-lang/cargo/pull/14006)
* [cargo config: Ensure `--config net.git-fetch-with-cli=true` is respected](https://github.com/rust-lang/cargo/pull/13992)
* [cargo new: Dont say were adding to a workspace when a regular package is in root](https://github.com/rust-lang/cargo/pull/13987)
* [cargo toml: Ensure targets are in a deterministic order](https://github.com/rust-lang/cargo/pull/13989)
* [cargo vendor: Ensure sort happens for vendor](https://github.com/rust-lang/cargo/pull/14004)
* [cargo: allows the default git/gitoxide configuration to be obtained from the ENV and config](https://github.com/rust-lang/cargo/pull/13687)
* [cargo: adjust custom err from cert-check due to libgit2 1.8 change](https://github.com/rust-lang/cargo/pull/13970)
* [cargo: skip deserialization of unrelated fields with overlapping name](https://github.com/rust-lang/cargo/pull/14000)
* [clippy: `many_single_char_names`: deduplicate diagnostics](https://github.com/rust-lang/rust-clippy/pull/12859)
* [clippy: add `needless_character_iteration` lint](https://github.com/rust-lang/rust-clippy/pull/12815)
* [clippy: deprecate `maybe_misused_cfg` and `mismatched_target_os`](https://github.com/rust-lang/rust-clippy/pull/12875)
* [clippy: disable `indexing_slicing` for custom `Index` impls](https://github.com/rust-lang/rust-clippy/pull/12488)
* [clippy: fix `redundant_closure` suggesting incorrect code with `F: Fn()`](https://github.com/rust-lang/rust-clippy/pull/12865)
* [clippy: let `non_canonical_impls` skip proc marco](https://github.com/rust-lang/rust-clippy/pull/12857)
* [clippy: ignore array from `deref_addrof` lint](https://github.com/rust-lang/rust-clippy/pull/12864)
* [clippy: make `str_to_string` machine-applicable](https://github.com/rust-lang/rust-clippy/pull/12871)
* [rust-analyzer: add `Function::fn_ptr_type(…)` for obtaining name-erased function type](https://github.com/rust-lang/rust-analyzer/pull/17312)
* [rust-analyzer: don't mark `#[rustc_deprecated_safe_2024]` functions as unsafe](https://github.com/rust-lang/rust-analyzer/pull/17329)
* [rust-analyzer: enable completions within derive helper attributes](https://github.com/rust-lang/rust-analyzer/pull/17328)
* [rust-analyzer: fix container search failing for tokens originating within derive attributes](https://github.com/rust-lang/rust-analyzer/pull/17326)
* [rust-analyzer: fix diagnostics clearing when flychecks run per-workspace](https://github.com/rust-lang/rust-analyzer/pull/17302)
* [rust-analyzer: only generate snippets for `extract_expressions_from_format_string` if snippets are supported](https://github.com/rust-lang/rust-analyzer/pull/17333)
* [rustfmt: collapse nested if detected by clippy](https://github.com/rust-lang/rustfmt/pull/6169)
* [rustfmt: rustfmt should not remove inner attributes from inline const blocks](https://github.com/rust-lang/rustfmt/pull/6173)
* [rustfmt: rust rewrite `check_diff` (Skeleton)](https://github.com/rust-lang/rustfmt/pull/6166)
* [rustfmt: use `with_capacity` in `rewrite_path`](https://github.com/rust-lang/rustfmt/pull/6174)

### Rust Compiler Performance Triage


A quiet week; we did have one quite serious regression (#115105, "enable
DestinationPropagation by default"), but it was shortly reverted (#125794).
The only other PR identified as potentially problematic was rollup
[PR #125824](https://github.com/rust-lang/rust/pull/125824), but even
that is relatively limited in its effect.

Triage done by **@pnkfelix**.
Revision range: [a59072ec..1d52972d](https://perf.rust-lang.org/?start=a59072ec4fb6824213df5e9de8cae4812fd4fe97&end=1d52972dd8592edf4026aa577c8ce69acc0ac2d1&absolute=false&stat=instructions%3Au)

3 Regressions, 5 Improvements, 6 Mixed; 4 of them in rollups
57 artifact comparisons made in total


[Full report here](https://github.com/rust-lang/rustc-perf/blob/fba75cc08937425ab274959581401b862a0b3068/triage/2024-06-03.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [Change crates.io policy to not offer crate transfer mediation](https://github.com/rust-lang/rfcs/pull/3646)

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Allow constraining opaque types during subtyping in the trait system](https://github.com/rust-lang/rust/pull/125447)
* [disposition: merge] [TAIT decision on "may define implies must define"](https://github.com/rust-lang/rust/issues/117861)
* [disposition: merge] [Stabilize Wasm relaxed SIMD](https://github.com/rust-lang/rust/pull/117468)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team RFCs entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2024-06-05 - 2024-07-03 🦀

### Virtual

* 2024-06-05 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047896/)
* 2024-06-06 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: Your first contribution to an Open Source Rust project**](https://www.meetup.com/code-mavens/events/301156302/)
* 2024-06-06 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477702/)
* 2024-06-09 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Rust Maven Workshop: GitHub pages for Rust developers (English)**](https://www.meetup.com/code-mavens/events/301215326/)
* 2024-06-11 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298341709/)
* 2024-06-12 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust for Rustaceans Book Club: Chapter 8 - Asynchronous Programming**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/301314544/)
* 2024-06-13 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897800/)
* 2024-06-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/297945258/)
* 2024-06-16 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Workshop: Web development in Rust using Rocket (English)**](https://www.meetup.com/code-mavens/events/301294669/)
* 2024-06-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/299346963/)
* 2024-06-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/298631733/)
* 2024-06-20 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://meet.jit.si/RustHackAndLearnBerlin) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298477705/)
* 2024-06-25 | Virtual (Dallas, TX, US)| [Dallas Rust User Group](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygcjbhc/)
* 2024-06-27 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298897826/)
* 2024-07-02 | Virtual (Buffalo, NY) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/300191673/)
* 2024-07-03 | Virtual | [Training 4 Programmers LLC](https://www.eventbrite.com/o/training-4-programmers-llc-80387368983)
    * [**Build Web Apps with Rust and Leptos**](https://www.eventbrite.com/e/build-web-apps-with-rust-and-leptos-tickets-904804503627?aff=odcleoeventsincollection)
* 2024-07-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/300328025/)

### Europe

* 2024-06-05 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/)
    * [**Rust Hack & Learn June 2024**](https://www.meetup.com/rust-meetup-hamburg/events/299235215/)
* 2024-06-06 | Madrid, ES | [MadRust](https://www.meetup.com/madrust/)
    * [**Introducción a Rust y el futuro de los sistemas DLT**](https://www.meetup.com/madrust/events/301318288/)
* 2024-06-06 | Vilnius, LT | [Rust Vilnius](https://www.meetup.com/rust-in-vilnius/)
    * [**Enjoy our second Rust and ZIG event**](https://www.meetup.com/rust-in-vilnius/events/301012097/)
* 2024-06-06 | Wrocław, PL | [Rust Wroclaw](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #37**](https://www.meetup.com/rust-wroclaw/events/301322042/)
* 2024-06-11 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust Hack Night #6: Discord bots**](https://www.meetup.com/copenhagen-rust-community/events/301439744/)
* 2024-06-11 | Paris, FR | [Rust Paris](https://mobilizon.fr/@rust_paris)
    * [**Paris Rust Meetup #69**](https://mobilizon.fr/events/681b42dd-a456-4bfc-99e2-d1c60e867cbb)
* 2024-06-12 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/301012491/)
* 2024-06-18 | Frankfurt/Main, DE | [Rust Frankfurt Meetup](https://www.meetup.com/rust-frankfurt)
    * [**Rust Frankfurt is Back!**](https://www.meetup.com/rust-frankfurt/events/301441434/)
* 2024-06-19 - 2024-06-24 | Zürich, CH | [RustFest Zürich](https://rustfest.ch/)
    * [**RustFest Zürich 2024**](https://rustfest.ch/)
* 2024-06-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Talk Night at Trifork**](https://www.meetup.com/rust-aarhus/events/300865116/)
* 2024-06-25 | Gdańsk, PL | [Rust Gdansk](https://www.meetup.com/rust-gdansk/)
    * [**Rust Gdansk Meetup #3**](https://www.meetup.com/rust-gdansk/events/301014697/)
* 2024-06-27 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - Title**](https://www.meetup.com/rust-berlin/events/299288965/)
* 2024-06-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #48 sponsored by Google!**](https://www.meetup.com/copenhagen-rust-community/events/300458252/)

### North America

* 2024-06-08 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Porter Square Rust Lunch, Jun 8**](https://www.meetup.com/bostonrust/events/300116799/)
* 2024-06-11 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup**](https://www.meetup.com/rust-nyc/events/301386836/)
* 2024-06-12 | Detroit, MI, US | [Detroit Rust](https://www.meetup.com/detroitrust/)
    * [**Detroit Rust Meet - Ann Arbor**](https://www.meetup.com/detroitrust/events/301387848/)
* 2024-06-13 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust/)
    * [**Monthly Meetup: Topic TBD!**](https://www.meetup.com/spokane-rust/events/300020010/)
* 2024-06-17 | Minneapolis, MN US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/301411625/)
* 2024-06-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/299186953/)
* 2024-06-20 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/299509396/)
* 2024-06-26 | Austin, TX, US | [Rust ATC](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/301066942/)
* 2024-06-27 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/)
    * [**Music City Rust Developers: Holding Pattern**](https://www.meetup.com/music-city-rust-developers/events/301411746/)

### Oceania

* 2024-06-14 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**June 2024 Rust Melbourne Meetup**](https://www.meetup.com/rust-melbourne/events/301311680/)
* 2024-06-20 | Auckland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Full Stack Rust + Writing a compiler for fun and (no) profit**](https://www.meetup.com/rust-akl/events/301193761/)
* 2024-06-25 | Canberra, ACt, AU | [Canberra Rust User Group (CRUG)](https://www.meetup.com/rust-canberra/)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/300749371/)

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

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1cixuzr/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Every PR is Special™

– [Hieyou Xu describing being on t-compiler review rotation](https://jieyouxu.github.io/blog/review-rotation/)

Sadly, there was no suggestion, so llogiq came up with something hopefully suitable.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1d97pjo/this_week_in_rust_550/)</small>
