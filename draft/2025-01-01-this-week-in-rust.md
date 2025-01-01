Title: This Week in Rust 580
Number: 580
Date: 2025-01-01
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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [fake](https://crates.io/crates/fake), a library (and recently console utility) to generate fake data of various types.

Thanks to [llogiq](https://users.rust-lang.org/t/crate-of-the-week/2704/1384) for the half-self-suggestion!

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

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [X (formerly Twitter)](https://x.com/ThisWeekInRust) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

331 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2024-12-24..2024-12-31

* [enable LSX feature for LoongArch OpenHarmony target](https://github.com/rust-lang/rust/pull/134751)
* [explicitly set float ABI for all ARM targets](https://github.com/rust-lang/rust/pull/134932)
* [A couple datalog/borrowck cleanups](https://github.com/rust-lang/rust/pull/134914)
* [`rustc_codegen_ssa`: Buffer file writes in `link_rlib`](https://github.com/rust-lang/rust/pull/134866)
* [account for C string literals and `format_args` in `HiddenUnicodeCodepoints` lint](https://github.com/rust-lang/rust/pull/134956)
* [account for removal of multiline span in suggestion](https://github.com/rust-lang/rust/pull/134664)
* [actually print all the relevant parts of a coroutine in verbose mode](https://github.com/rust-lang/rust/pull/134741)
* [add `--no-capture`/`--nocapture` as bootstrap arguments](https://github.com/rust-lang/rust/pull/134809)
* [add a compiler intrinsic to back `bigint_helper_methods`](https://github.com/rust-lang/rust/pull/133663)
* [avoid ICE in borrowck](https://github.com/rust-lang/rust/pull/134627)
* [compiler: add a statement-of-intent to `rustc_abi`](https://github.com/rust-lang/rust/pull/134941)
* [compute liveness constraints in location-sensitive polonius](https://github.com/rust-lang/rust/pull/134670)
* [consider arm to diverge if guard diverges](https://github.com/rust-lang/rust/pull/134735)
* [consider empty-unreachable otherwise branches in MatchBranchSimplification](https://github.com/rust-lang/rust/pull/131244)
* [default to short backtraces for dev builds of rustc itself](https://github.com/rust-lang/rust/pull/134743)
* [implement `default_overrides_default_fields` lint](https://github.com/rust-lang/rust/pull/134737)
* [improve default target options for `x86_64-unknown-linux-none`](https://github.com/rust-lang/rust/pull/134765)
* [improve type mutation for certain structures](https://github.com/rust-lang/rust/pull/134724)
* [make -Csoft-float have an effect on all ARM targets](https://github.com/rust-lang/rust/pull/134885)
* [make `ty::Error` implement all auto traits](https://github.com/rust-lang/rust/pull/134798)
* [make sure we check the future type is `Sized` in `AsyncFn*`](https://github.com/rust-lang/rust/pull/134933)
* [skip parenthesis around tuple `struct` field calls](https://github.com/rust-lang/rust/pull/134834)
* [skip parenthesis if `.` makes statement boundary unambiguous](https://github.com/rust-lang/rust/pull/134833)
* [some random region tweaks](https://github.com/rust-lang/rust/pull/134827)
* [strip debuginfo from rustc-main and rustdoc](https://github.com/rust-lang/rust/pull/134803)
* [miri: FD handling: avoid unnecessary dynamic downcasts](https://github.com/rust-lang/miri/pull/4114)
* [miri: epoll: avoid some clones](https://github.com/rust-lang/miri/pull/4115)
* [miri: error on some invalid flag combinations](https://github.com/rust-lang/miri/pull/4109)
* [miri: fix toolchain flag parsing](https://github.com/rust-lang/miri/pull/4113)
* [miri: socket read/write cleanup](https://github.com/rust-lang/miri/pull/4112)
* [stabilize `const_alloc_layout`](https://github.com/rust-lang/rust/pull/134768)
* [stabilize `const_swap`](https://github.com/rust-lang/rust/pull/134757)
* [stabilize `style_edition = "2024"` in-tree](https://github.com/rust-lang/rust/pull/134929)
* [make `slice::as_flattened_mut` unstably const](https://github.com/rust-lang/rust/pull/134927)
* [tidy up bigint multiplication methods](https://github.com/rust-lang/rust/pull/132195)
* [from iterator for more tuples](https://github.com/rust-lang/rust/pull/132431)
* [add `into_array` conversion destructors for `Box`, Rc`, and Arc`](https://github.com/rust-lang/rust/pull/134379)
* [avoid short writes in LineWriter](https://github.com/rust-lang/rust/pull/134620)
* [unify `fs::copy` and `io::copy` on Linux](https://github.com/rust-lang/rust/pull/134547)
* [fix forgetting to save statx availability on success](https://github.com/rust-lang/rust/pull/134649)
* [fix mistake in windows file open](https://github.com/rust-lang/rust/pull/134819)
* [fix renaming symlinks on Windows](https://github.com/rust-lang/rust/pull/134786)
* [windows: use WriteFile to write to a UTF-8 console](https://github.com/rust-lang/rust/pull/134622)
* [windows: use `FILE_ALLOCATION_INFO` for truncation](https://github.com/rust-lang/rust/pull/134722)
* [cargo: `fix(package)`: check dirtiness of path fields in manifest](https://github.com/rust-lang/cargo/pull/14966)
* [cargo: `fix(package)`: check dirtiness of symlinks source files](https://github.com/rust-lang/cargo/pull/14981)
* [cargo: `fix(package)`: warn if symlinks checked out as plain text files](https://github.com/rust-lang/cargo/pull/14994)
* [cargo: `refactor(package)`: split `cargo_package` to modules](https://github.com/rust-lang/cargo/pull/14982)
* [cargo: fix: assure possibly blocking non-files (like FIFOs) won't be picked up for publishing](https://github.com/rust-lang/cargo/pull/14977)
* [cargo: moved manifest metadata tracking from fingerprint to dep info](https://github.com/rust-lang/cargo/pull/14973)
* [cargo: test: make path arguments more generic and flexible](https://github.com/rust-lang/cargo/pull/14979)
* [cargo: test: relax `bad_crate_type` to only match error message prefix](https://github.com/rust-lang/cargo/pull/14990)
* [cargo: test: relax panic output assertion](https://github.com/rust-lang/cargo/pull/14989)
* [rustdoc: use shorter paths as preferred canonical paths](https://github.com/rust-lang/rust/pull/134806)
* [unsafe binder support in rustdoc](https://github.com/rust-lang/rust/pull/134857)
* [clippy: `borrow_interior_mutable_const` ICE into FN](https://github.com/rust-lang/rust-clippy/pull/13877)
* [clippy: auto-fix `if_not_else`](https://github.com/rust-lang/rust-clippy/pull/13809)
* [clippy: correct suggestion for `manual_div_ceil` lint](https://github.com/rust-lang/rust-clippy/pull/13864)
* [clippy: do not remove required parentheses in `borrow_as_ptr` suggestion](https://github.com/rust-lang/rust-clippy/pull/13884)
* [clippy: do not trigger `trailing_empty_array` in tests](https://github.com/rust-lang/rust-clippy/pull/13844)
* [clippy: fix arguments of `ExprKind::MethodCall`](https://github.com/rust-lang/rust-clippy/pull/13890)
* [clippy: fix parentheses when replacing `matches!(…, None)` with `.is_none()`](https://github.com/rust-lang/rust-clippy/pull/13906)
* [clippy: make `inconsistent_struct_constructor` "all fields are shorthand" requirement configurable](https://github.com/rust-lang/rust-clippy/pull/13737)
* [clippy: remove description of known problems in `match_same_arms` that have already been resolved](https://github.com/rust-lang/rust-clippy/pull/13873)
* [clippy: remove obsolete comment](https://github.com/rust-lang/rust-clippy/pull/13850)
* [clippy: use the correct `ParamEnv` when checking future's output type](https://github.com/rust-lang/rust-clippy/pull/13863)
* [clippy: use the full lifetime name in suggestions](https://github.com/rust-lang/rust-clippy/pull/13907)
* [rust-analyzer: cleanup target fetching for cargo metadata](https://github.com/rust-lang/rust-analyzer/pull/18754)
* [rust-analyzer: cleanup toolchain info fetching](https://github.com/rust-lang/rust-analyzer/pull/18785)
* [rust-analyzer: decouple proc-macro server protocol from the server implementation](https://github.com/rust-lang/rust-analyzer/pull/18792)
* [rust-analyzer: show go-to-type-def actions for subst when hovering](https://github.com/rust-lang/rust-analyzer/pull/18801)
* [rust-analyzer: show substitution where hovering over generic things](https://github.com/rust-lang/rust-analyzer/pull/18707)
* [rust-analyzer: unify handling of path diagnostics in hir-ty](https://github.com/rust-lang/rust-analyzer/pull/18743)
* [rust-analyzer: fix bug of "fill match arm" action in `tokio::main` macro](https://github.com/rust-lang/rust-analyzer/pull/18794)
* [rust-analyzer: fix missing name `enum` when hovering on fields in variants](https://github.com/rust-lang/rust-analyzer/pull/18756)
* [rust-analyzer: fix render of literal to be rendered in codeblock](https://github.com/rust-lang/rust-analyzer/pull/18795)
* [rust-analyzer: fix replace-if-let-with-match generates non-exhausive match](https://github.com/rust-lang/rust-analyzer/pull/18797)
* [rust-analyzer: avoid generating colliding names in `extract_variable`](https://github.com/rust-lang/rust-analyzer/pull/18791)
* [rust-analyzer: consider `Enum::Variant` even when it comes from a different crate](https://github.com/rust-lang/rust-analyzer/pull/18779)
* [rust-analyzer: do not merge spans if they have different anchors](https://github.com/rust-lang/rust-analyzer/pull/18784)
* [rust-analyzer: fix flycheck diagnostics flickering for binary targets](https://github.com/rust-lang/rust-analyzer/pull/18778)
* [rust-analyzer: fix invalid `-O` flag used by cfg discovery](https://github.com/rust-lang/rust-analyzer/pull/18789)
* [rust-analyzer: fix metrics workflow using the wrong download-artifact version](https://github.com/rust-lang/rust-analyzer/pull/18755)
* [rust-analyzer: incorrect `file_id` used for ranges in outgoing calls](https://github.com/rust-lang/rust-analyzer/pull/18802)
* [rust-analyzer: populate cargo config env vars for crates](https://github.com/rust-lang/rust-analyzer/pull/18807)
* [rust-analyzer: implement parameter variance inference](https://github.com/rust-lang/rust-analyzer/pull/18774)
* [rust-analyzer: treat ; as a terminator rather part of a glued expression](https://github.com/rust-lang/rust-analyzer/pull/18744)

### Rust Compiler Performance Triage

A pretty quiet week, with the exception of a significant improvement due to
landing LTO for C / C++ programs compiled as part of the build.

Triage done by **@simulacrum**.
Revision range: [0eca4dd3..93722f7e](https://perf.rust-lang.org/?start=0eca4dd3205a01dba4bd7b7c140ec370aff03440&end=93722f7ed56bcf27839a6355074095c4320b7d37&absolute=false&stat=instructions%3Au)

0 Regressions, 1 Improvements, 1 Mixed; 0 of them in rollups
53 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2024-12-30.md)

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
* *No RFCs entered Final Comment Period this week.*

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Cargo Tracking Issues or PRs entered Final Comment Period this week.*

##### [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+)
* *No Language Team Proposals entered Final Comment Period this week.*

##### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

##### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline Tracking Issues or PRs entered Final Comment Period this week.*

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: `Foo { .. }` pattern matches non-struct types](https://github.com/rust-lang/rfcs/pull/3753)
* [new] [RFC: add LLM text version to rustdoc](https://github.com/rust-lang/rfcs/pull/3751)
* [new] [RFC: cfg_os_version_min](https://github.com/rust-lang/rfcs/pull/3750)

## Upcoming Events

Rusty Events between 2025-01-01 - 2025-01-29 🦀

### Virtual
* 2024-12-26 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898145)
* 2025-01-02| Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633277/)
* 2025-01-04 | Virtual (Kampala, UG) | [Rust Circle Kampala](https://www.eventbrite.com/o/rust-circle-kampala-65249289033/)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-01-08 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**BlockMesh Network implemented in Rust with Ohad Dahan (Virtual, English)**](https://www.meetup.com/code-mavens/events/304951805)
* 2025-01-09 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298898167)
* 2025-01-09 | Miami, FL, US | [Rust Miami](https://www.meetup.com/rust-miami/)
    * [**Rust / Wasm on Serverless and Frontend**](https://www.meetup.com/rust-miami/events/305122950)
* 2025-01-09 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820279/)
* 2025-01-14 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/302815031)
* 2025-01-15 | Virtual (Tel Aviv-Yafo, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/events/)
    * [**An introduction to WASM in Rust with Márk Tolmács (Virtual, English)**](https://www.meetup.com/code-mavens/events/305064546)
* 2025-01-15 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Leptos**](https://www.meetup.com/vancouver-rust/events/304051782)
* 2025-01-16 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://berline.rs/) + [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust Hack and Learn**](https://berline.rs/2024/12/19/rust-hack-and-learn.html) | [**Mirror: Rust Hack n Learn Meetup**](https://www.meetup.com/rust-berlin/events/298633278/)
* 2025-01-21 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**Exploring Rust Enums with Yoni Peleg (Virtual, Hebrew)**](https://www.meetup.com/rust-tlv/events/305110744)
* 2025-01-21 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyhccbcc)
* 2025-01-22 | Virtual (Rotterdam, NL) | [Bevy Game Development](https://www.meetup.com/bevy-game-development/events/)
    * [**Bevy Meetup #8**](https://www.meetup.com/bevy-game-development/events/305111151)

### Asia
* 2025-01-12 | Virtual (Tel Aviv-Yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2025 at Abra in Raanana**](https://www.meetup.com/rust-tlv/events/304898730/)

### Europe
* 2025-01-08 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305038426)
* 2025-01-09 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/303154281)
* 2025-01-16 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/305144321)
* 2025-01-21 | Ghent, BE | [Systems Programming Ghent](https://sysghent.be)
    * [**Tech Talks & Dinner: Insights on Systems Programming Side Projects (in Rust) - Leptos (full-stack Rust with webassembly), Karyon (distributed p2p software in Rust), FunDSP (audio synthesis in Rust)**](https://www.meetup.com/systems-programming-ghent/events/305201540/?slug=systems-programming-ghent&eventId=305201540)
* 2025-01-21 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Self-Organized Peer-to-Peer Networks using Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/303604074)


### North America
* 2024-12-26 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/wqkgntygcqbjc/)
* 2025-01-10 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Lechmere Rust Lunch, Jan 10**](https://www.meetup.com/bostonrust/events/304951467)
* 2025-01-16 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust Game Development Series 1: Community Introductions**](https://www.meetup.com/music-city-rust-developers/events/304333017)
* 2025-01-18 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/boston-rust-meetup-25317522anphwzdw/events/)
    * [**Back Bay Rust Lunch, Jan 18**](https://www.meetup.com/bostonrust/events/304951470)
* 2025-01-21 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/302638258)
* 2025-01-22 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhccbdc)

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

> Hear, hear! Rust is the real deep state. They knew all along that memory-related bugs would dominate the Vulnerability Rating Taxonomy. Coincidence? I think not. 🐛🔧

– [@amoghavarsha@infosec.exchange on mastodon](https://infosec.exchange/@amoghavarsha/113741018641283042)

Thanks to [llogiq](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1646) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)


*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*
*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
