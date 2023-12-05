Title: This Week in Rust 524
Number: 524
Date: 2023-12-06
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [symbols](https://crates.io/crates/symbols), a utility to quickly create proc-macros to solidify database tables into enums enabling compile time foreign key checks.

Thanks to [Marco Napetti](https://users.rust-lang.org/t/crate-of-the-week/2704/1267) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

369 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-11-27..2023-12-04

* [add `-Zfunction-return={keep,thunk-extern}` option](https://github.com/rust-lang/rust/pull/116892)
* [account for `!` arm in tail `match` expr](https://github.com/rust-lang/rust/pull/117526)
* [add `never_patterns` feature gate](https://github.com/rust-lang/rust/pull/118157)
* [add `pretty_terminator` to pretty stable-mir](https://github.com/rust-lang/rust/pull/118172)
* [add an assume that the index is inbounds to `slice::get_unchecked`](https://github.com/rust-lang/rust/pull/116915)
* [`rustc_span`: Use correct edit distance start length for suggestions](https://github.com/rust-lang/rust/pull/118381)
* [added `linker_arg(s)` Linker trait methods for link-arg to be prefixed "-Wl," for cc-like linker args and not verbatim](https://github.com/rust-lang/rust/pull/118202)
* [allow setting `rla` labels via `rustbot`](https://github.com/rust-lang/rust/pull/114708)
* [avoid per-register closure expansions](https://github.com/rust-lang/rust/pull/118347)
* [`generic_const_exprs`: suggest to add the feature, not use it](https://github.com/rust-lang/rust/pull/118486)
* [change `SwitchTarget` representation in StableMIR](https://github.com/rust-lang/rust/pull/118461)
* [`rustc_hir_typeck`: Fix ICE when probing for non-ASCII function alternative](https://github.com/rust-lang/rust/pull/118514)
* [constProp: correctly remove const if unknown value assigned to it](https://github.com/rust-lang/rust/pull/118426)
* [coverage: skip spans that can't be un-expanded back to the function body](https://github.com/rust-lang/rust/pull/118525)
* [cut code size for feature hashing](https://github.com/rust-lang/rust/pull/118348)
* [detect Python-like slicing and suggest how to fix](https://github.com/rust-lang/rust/pull/111133)
* [detect and reject malformed `repr(Rust)` hints](https://github.com/rust-lang/rust/pull/118366)
* [dispose `llvm::TargetMachines` prior to `llvm::Context` being disposed](https://github.com/rust-lang/rust/pull/118464)
* [dont suggest `!` for path in function call if it has generic args](https://github.com/rust-lang/rust/pull/118342)
* [eagerly return `ExprKind::Err` on `yield`/`await` in wrong coroutine context](https://github.com/rust-lang/rust/pull/118419)
* [effects: run `enforce_context_effects` for all method calls](https://github.com/rust-lang/rust/pull/118282)
* [explain a good reason for why LocalValue does not store the type of the local](https://github.com/rust-lang/rust/pull/118482)
* [fix ICE: `fn_arg_names: unexpected item DefId(..)`](https://github.com/rust-lang/rust/pull/118526)
* [fix `PartialEq` args when `#[const_trait]` is enabled](https://github.com/rust-lang/rust/pull/118379)
* [fix an ICE when a valtree failed to evaluate](https://github.com/rust-lang/rust/pull/118498)
* [fix parser ICE from attrs](https://github.com/rust-lang/rust/pull/118542)
* [fix the issue of suggesting unwrap/expect for shorthand field](https://github.com/rust-lang/rust/pull/118413)
* [give dev-friendly error message for incorrect config profiles](https://github.com/rust-lang/rust/pull/118323)
* [handle recursion limit for subtype and well-formed predicates](https://github.com/rust-lang/rust/pull/117754)
* [implement thread parking for xous](https://github.com/rust-lang/rust/pull/116839)
* [more targeted errors when extern types end up in places they should not](https://github.com/rust-lang/rust/pull/118551)
* [new solver: improve instrument annotations](https://github.com/rust-lang/rust/pull/118454)
* [on Fn arg mismatch for a fn path, suggest a closure](https://github.com/rust-lang/rust/pull/117805)
* [pass +forced-atomics feature for riscv32{i,im,imc}-unknown-none-elf](https://github.com/rust-lang/rust/pull/114499)
* [perform LTO optimisations with wasm-ld + -Clinker-plugin-lto](https://github.com/rust-lang/rust/pull/118378)
* [print list of missing target features when calling a function with target features outside an unsafe block](https://github.com/rust-lang/rust/pull/118333)
* [provide structured suggestion for type mismatch in loop](https://github.com/rust-lang/rust/pull/118072)
* [remove the memcpy-on-equal-ptrs assumption](https://github.com/rust-lang/rust/pull/118265)
* [replace `once_cell::sync::OnceCell` with std `OnceLock`](https://github.com/rust-lang/rust/pull/118528)
* [report errors in jobserver inherited through environment variables](https://github.com/rust-lang/rust/pull/113730)
* [restore `#![no_builtins]` crates participation in LTO](https://github.com/rust-lang/rust/pull/113923)
* [restrict what symbols can be used in `#[diagnostic::on_unimplemented]` format strings](https://github.com/rust-lang/rust/pull/118495)
* [rustc: harmonize `DefKind` and `DefPathData`](https://github.com/rust-lang/rust/pull/118573)
* [simplify indenting in THIR printing](https://github.com/rust-lang/rust/pull/118341)
* [tweak message on ADT with private fields building](https://github.com/rust-lang/rust/pull/118453)
* [tweak parsing recovery of enums, for exprs and match arm patterns](https://github.com/rust-lang/rust/pull/117565)
* [warn against using intrinsics that leave the scope of our memory model](https://github.com/rust-lang/rust/pull/118128)
* [add more information to StableMIR Instance](https://github.com/rust-lang/rust/pull/118524)
* [codegen, miri: fix computing the offset of an unsized field in a packed `struct`](https://github.com/rust-lang/rust/pull/118540)
* [miri: support 'promising' alignment for symbolic alignment check](https://github.com/rust-lang/rust/pull/117840)
* [miri: SIMD bitmasks: use 'round up to multiple of 8' rather than 'clamp to at least 8'](https://github.com/rust-lang/miri/pull/3206)
* [miri: add new SIMD intrinsics](https://github.com/rust-lang/miri/pull/3204)
* [miri: remove Stacked Borrows GC heuristics](https://github.com/rust-lang/miri/pull/3194)
* [also add `is_empty` to const raw slices](https://github.com/rust-lang/rust/pull/118231)
* [move exposed-provenance APIs into separate feature gate](https://github.com/rust-lang/rust/pull/118487)
* [stabilize C string literals](https://github.com/rust-lang/rust/pull/117472)
* [add substring API for `OsStr`](https://github.com/rust-lang/rust/pull/118484)
* [optimize `str::iter::Chars::advance_by`](https://github.com/rust-lang/rust/pull/115331)
* [add `track_caller` for arith ops](https://github.com/rust-lang/rust/pull/114841)
* [expand in-place iteration specialization to Flatten, FlatMap and ArrayChunks](https://github.com/rust-lang/rust/pull/110353)
* [cargo resolver: De-prioritize no-rust-version in MSRV resolver](https://github.com/rust-lang/cargo/pull/13066)
* [cargo resolver: Remove unused public-deps error handling](https://github.com/rust-lang/cargo/pull/13036)
* [cargo toml: Decouple logic from schema](https://github.com/rust-lang/cargo/pull/13080)
* [cargo: add `--public` for `cargo add`](https://github.com/rust-lang/cargo/pull/13046)
* [cargo: add more doc comments for gc changes](https://github.com/rust-lang/cargo/pull/13055)
* [cargo: reorder `--remap-path-prefix` flags for `-Zbuild-std`](https://github.com/rust-lang/cargo/pull/13065)
* [cargo: fixed uninstall a running binary failed on Windows](https://github.com/rust-lang/cargo/pull/13053)
* [cargo: fixes error count display is different when there's only one error left](https://github.com/rust-lang/cargo/pull/12484)
* [cargo: have cargo add --optional `<dep>` create a `<dep>` = `"dep:<dep>` feature](https://github.com/rust-lang/cargo/pull/13071)
* [cargo: include declared list of features in fingerprint for `-Zcheck-cfg`](https://github.com/rust-lang/cargo/pull/13012)
* [cargo: remove the outdated comment](https://github.com/rust-lang/cargo/pull/13076)
* [rustdoc: Add highlighting for comments in items declaration](https://github.com/rust-lang/rust/pull/117869)
* [rustdoc-search: allow spaces around `:` in path query](https://github.com/rust-lang/rust/pull/118452)
* [clippy: `missing_asserts_for_indexing`: accept length equality checks](https://github.com/rust-lang/rust-clippy/pull/11837)
* [clippy: `option_if_let_else`: do not trigger on expressions returning `()`](https://github.com/rust-lang/rust-clippy/pull/11896)
* [clippy: `redundant_closure_call`: avoid duplicated `async` keyword when triggering on closure that returns `async` block](https://github.com/rust-lang/rust-clippy/pull/11363)
* [clippy: `redundant_guards`: catch `is_empty`, `starts_with` and `ends_with` on slices and `str`s](https://github.com/rust-lang/rust-clippy/pull/11818)
* [clippy: add lint against unit tests in doctests](https://github.com/rust-lang/rust-clippy/pull/11872)
* [clippy: allow `allow`ing `upper_case_acronyms` on `enum` variants](https://github.com/rust-lang/rust-clippy/pull/11898)
* [clippy: expanding lint `blocks_in_if_conditions` to check match expr as well](https://github.com/rust-lang/rust-clippy/pull/11853)
* [clippy: new lint: `repeat_vec_with_capacity`](https://github.com/rust-lang/rust-clippy/pull/11597)
* [rust-analyzer: debug use cargo workspace root as `cwd`](https://github.com/rust-lang/rust-analyzer/pull/15993)
* [rust-analyzer: implement completion for the callable fields](https://github.com/rust-lang/rust-analyzer/pull/15879)
* [rust-analyzer: initial support for implicit drop inlay hint](https://github.com/rust-lang/rust-analyzer/pull/16000)
* [rust-analyzer: don't make `MissingMatchArms` diagnostic for empty match body](https://github.com/rust-lang/rust-analyzer/pull/15971)
* [rust-analyzer: improve error handling for top-level `let` statements](https://github.com/rust-lang/rust-analyzer/pull/15961)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

<!--
### [Approved Major Change Proposals (MCP)](https://forge.rust-lang.org/compiler/mcp.html)
<!~~ MCPs occur infrequently, so this section is commented out by default. ~~>
<!~~ MCPs which have been approved or rejected this week go here, use this format: * [major change accepted|rejected] [Topic](URL) ~~>
-->

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

<!-- RFCs which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No RFCs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

<!-- Tracking Issues which have entered FCP go here, use this format: * [disposition: merge|close] [Topic](URL) -->
<!-- or if none entered FCP this week, use: * *No Tracking Issues or PRs entered Final Comment Period this week.* -->
<!-- * [disposition: ] []() -->

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
<!-- Remove this section if empty>

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

<!-- New or updated RFCs go here, use this format: * [new|updated] [Topic](URL) -->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- * [new|updated] []() -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

<!-- Calls for Testing go here, use this format:
    * [<RFC Topic>](<RFC URL>)
        * [Tracking Issue](<Tracking Issue URL>)
        * [Testing steps](<Testing Steps URL>)
-->
<!-- or if there are no new or updated RFCs this week, use: * *No New or Updated RFCs were created this week.* -->
<!-- Remember to remove the `call-for-testing` label from the RFC so that the maintainer can signal for testers again, if desired. -->

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-06 - 2024-01-03 🦀

### Virtual

* 2023-11-29 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Atomics & Locks Book Club Final Chapter! (Chapter 10)**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583091/)
* 2023-11-30 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833665/)
* 2023-11-30 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**Automating expertise with cargo-semver-checks**](https://www.meetup.com/rust-dublin/events/296346693/)
* 2023-12-01 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Kick-Off!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583626/)
* 2023-12-02 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587?aff=ebdssbdestsearch)
* 2023-12-05 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679797/) | [**Mirror**](https://berline.rs/)
* 2023-12-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/297021574/)
* 2023-12-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/297172140)
* 2023-12-10 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583652/)
* 2023-12-12 | Virtual | [Mainmatter](https://mainmatter.com)
    * [**Workshop: Telemetry for Rust applications**](https://rust-telemetry-workshop.mainmatter.com)
* 2023-12-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/297532862/)
* 2023-12-14| Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 2023-12-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 2023-12-17 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Don't panic! - Our journey to error handling in Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 2023-12-18 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Mirror**](https://berline.rs/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Adventures in egui app dev**](https://www.meetup.com/vancouver-rust/events/292763506/)

### Asia

* 2023-12-16 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europe

* 2023-11-30 | Brussels, BE | [Lambda Brussels](https://lambda-brussels.glitch.me/)
    * [**Lambda Brussels**](https://lambda-brussels.glitch.me/)
* 2023-11-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #42 sponsored by Nine A/S**](https://www.meetup.com/copenhagen-rust-community/events/297405705/)
* 2023-11-30 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Vienna Meetup - November**](https://www.meetup.com/rust-vienna/events/297382145/)
* 2023-11-30 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**November Meetup**](https://www.meetup.com/rust-zurich/events/297312190/)
* 2023-12-06 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**December Meetup**](https://www.meetup.com/rustcologne/events/297100007/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-12-07 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 2023-12-14 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 2023-12-18 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Heidelberg, DE | [Nix Your Bugs & Rust Your Engines](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 2023-12-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, an Electron-alternative**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### North America

* 2023-11-29 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/296657831/)
* 2023-11-30 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628043/)
* 2023-12-07 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/297533440/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 2023-12-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)  
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/297659937/)
* 2023-12-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)

### Oceania

* 2023-11-28 | Canberra, ACT, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**November Meetup**](https://www.meetup.com/rust-canberra/events/296391733/)
* 2023-12-05 | Aukland, NZ | [Rust AKL](https://www.meetup.com/rust-akl/)
    * [**Rust AKL: Advanced Async Tricks + Interruptible Software**](https://www.meetup.com/rust-akl/events/297271684/)
* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

> NVIDIA's firmware, Airlie said, comes with a set of include files that, in turn, define structures that change over time. To deal with these changes, the driver is going to need some sort of automated ABI generation; he noted that the developers working on the Apple M1 GPU driver have run into the same problem. This problem could be made easier to tackle, he suggested, if the driver were, like the M1 driver, to be rewritten in Rust.

– [Jonathan Corbet paraphrasing David Airlie on Linux Weekly News](https://lwn.net/SubscriberLink/953144/b85b695d0c760692)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1494) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
