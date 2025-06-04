Title: This Week in Rust 602
Number: 602
Date: 2025-06-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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
* [Announcing five new members of the compiler team](https://blog.rust-lang.org/inside-rust/2025/05/30/compiler-team-new-members/)
* [Redesigning the Initial Bootstrap Sequence](https://blog.rust-lang.org/inside-rust/2025/05/29/redesigning-the-initial-bootstrap-sequence/)

### Foundation

### Newsletters

### Project/Tooling Updates
* [ICU4X 2.0 released!](https://blog.unicode.org/2025/05/icu4x-20-released.html)
* [Freya 0.3](https://freyaui.dev/posts/0.3)
* [godot-rust May 2025 dev update](https://godot-rust.github.io/dev/may-2025-update/)
* [What's new in SeaORM 1.1.12](https://www.sea-ql.org/blog/2025-06-01-whats-new-in-sea-orm-1.1/)
* [git-cliff 2.9.0 is released!](https://git-cliff.org/blog/2.9.0/)
* [Ratatui's "Rat in the Wild" Challenge](https://github.com/ratatui/ratatui/discussions/1886)
* [rust-analyzer changelog #288](https://rust-analyzer.github.io/thisweek/2025/06/02/changelog-288.html)
* [Ratatui Block Border Merging](https://jslazak.com/ratatui-border-merging/)

### Observations/Thoughts
* [parking_lot: ffffffffffffffff...](https://fly.io/blog/parking-lot-ffffffffffffffff/)
* [How to deal with Rust dependencies](https://notgull.net/rust-dependencies/)
* [In Praise of Shuttle: Oxidizing the Capibara Web API](https://justinwoodring.com/blog/rewriting-the-capibara-web-api-in-rust/)
* [Reducing Cargo target directory size with -Zno-embed-metadata](https://kobzol.github.io/rust/rustc/2025/06/02/reduce-cargo-target-dir-size-with-z-no-embed-metadata.html)
* [Designing Error Types in Rust Libraries](https://d34dl0ck.me/rust-bites-designing-error-types-in-rust-libraries/index.html)
* [Why Use Structured Errors in Rust Applications?](https://home.expurple.me/posts/why-use-structured-errors-in-rust-applications/)
* [video] [The virtue of unsynn](https://www.youtube.com/watch?v=YtbUzIQw-so)
* [audio] [Proxying is just dumb routing](https://sdr-podcast.com/episodes/proxying-is-just-dumb-routing/)
* [audio] [David Lattimore - Faster Linker, Faster Builds](https://timclicks.dev/podcast/david-lattimore-faster-linker-faster-builds)
* [audio] [Rust with Niko Matsakis](https://corrode.dev/podcast/s04e04-rust/)
* [audio] [SWC with DongYoon Kang](https://rustacean-station.org/episode/dongyoon-kang/)
* [audio] [AccessKit with Matt Campbell and Arnold Loubriat](https://rustacean-station.org/episode/accesskit-with-matt-campbell-and-arnold-loubriat/)

### Rust Walkthroughs
* [C++ to Rust Phrasebook](https://cel.cs.brown.edu/crp/)
* [Async Traits Can Be Directly Backed By Manual Future Impls](https://blog.yoshuawuyts.com/async-traits-can-be-directly-backed-by-manual-future-impls/)
* [How we structure our build.rs in Rust](https://www.evolvebenchmark.com/blog-posts/how-we-wrap-external-c-and-cpp-libraries-in-rust)
* [video] [Start a new Bevy 2d Game with the Bevy CLI](https://www.youtube.com/watch?v=ez4oGeM3X2o)
* [video] [Build with Naz : Parse non-slice input with nom](https://www.youtube.com/watch?v=3IzAweJGdZU)

### Research
* [A first look at ROS 2 applications written in asynchronous Rust](https://arxiv.org/abs/2505.21323)

### Miscellaneous
* [Rethinking Data Streaming With Rust And InfinyOn](https://filtra.io/rust/interviews/infinyon-jun-25)

## Crate of the Week

This week's crate is [context-logger](https://github.com/alekseysidorov/context-logger), a lightweight, ergonomic library for adding structured context to your logs.

Thanks to [Aleksey Sidorov](https://users.rust-lang.org/t/crate-of-the-week/2704/1440) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

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

529 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-27..2025-06-03

#### Compiler

* [fix spans for unsafe binders](https://github.com/rust-lang/rust/pull/141781)
* [PGO new solver](https://github.com/rust-lang/rust/pull/141453)
* [add additional `TypeFlags` fast paths](https://github.com/rust-lang/rust/pull/141581)
* [add fast path for maybe-initializedness in liveness](https://github.com/rust-lang/rust/pull/141667)

#### Library

* [redesign stage 0 std](https://github.com/rust-lang/rust/pull/119899)
* [implement `((un)checked_)exact_div` methods for integers](https://github.com/rust-lang/rust/pull/141237)
* [add Range parameter to `BTreeMap::extract_if` and `BTreeSet::extract_if`](https://github.com/rust-lang/rust/pull/140825)
* [add `CStr::display`](https://github.com/rust-lang/rust/pull/139994)
* [add `Result::map_or_default` and `Option::map_or_default`](https://github.com/rust-lang/rust/pull/141659)
* [add `From<TryLockError>` for `io::Error`](https://github.com/rust-lang/rust/pull/141312)
* [add `const` support for float rounding methods](https://github.com/rust-lang/rust/pull/141521)
* [add `data_ptr` method to Mutex and RwLock](https://github.com/rust-lang/rust/pull/140369)

#### Cargo

* [trim-paths: remap all paths to `build.build-dir`](https://github.com/rust-lang/cargo/pull/15614)
* [fix cargo add overwriting symlinked Cargo.toml files](https://github.com/rust-lang/cargo/pull/15281)

#### Rustdoc

* [cleanups relating to allocations](https://github.com/rust-lang/rust/pull/141573)
* [display `doc(cfg(false))` properly](https://github.com/rust-lang/rust/pull/141747)
* [linking to a local proc macro no longer warns](https://github.com/rust-lang/rust/pull/141411)
* [use descriptive tooltip if doctest is conditionally ignored](https://github.com/rust-lang/rust/pull/141517)

#### Clippy

* [`explicit_deref_methods`: do not lint on method chains](https://github.com/rust-lang/rust-clippy/pull/14921)
* [`needless_return`: look inside `else if` parts as well](https://github.com/rust-lang/rust-clippy/pull/14798)
* [`while_let_loop`: Include `let` assignment in suggestion](https://github.com/rust-lang/rust-clippy/pull/14756)
* [add lint `infallible_try_from`](https://github.com/rust-lang/rust-clippy/pull/14813)
* [clean-up `modulo_arithmetic`](https://github.com/rust-lang/rust-clippy/pull/14898)
* [extend `manual_is_variant_and lint` to check for boolean map comparisons](https://github.com/rust-lang/rust-clippy/pull/14646)
* [fix `dbg_macro` fail to handle async coroutine desugar](https://github.com/rust-lang/rust-clippy/pull/14937)
* [fix `semicolon_outside_block` suggests wrongly when inside macros](https://github.com/rust-lang/rust-clippy/pull/14954)
* [improve wording of `manual_contains` docs](https://github.com/rust-lang/rust-clippy/pull/14917)
* [new restriction lint: `pointer_format`](https://github.com/rust-lang/rust-clippy/pull/14792)
* [optimize `unit_return_expecting_ord`](https://github.com/rust-lang/rust-clippy/pull/14905)
* [use interned symbols instead of strings in more places](https://github.com/rust-lang/rust-clippy/pull/14855)

#### Rust-Analyzer

* [account for `Generate` actions when filtering the allowed ones](https://github.com/rust-lang/rust-analyzer/pull/19899)
* [add a quickfix for accessing a private field of a `struct`](https://github.com/rust-lang/rust-analyzer/pull/19869)
* [add support for type-erased `Semantics<'db, dyn HirDatabase>`, by use of `DB: ?Sized`](https://github.com/rust-lang/rust-analyzer/pull/19850)
* [enable assist edit for `tuple<->named struct` for the `struct` and visiblity keywords](https://github.com/rust-lang/rust-analyzer/pull/19901)
* [desugar assist for `let pat = expr?;` → `let else`](https://github.com/rust-lang/rust-analyzer/pull/19881)
* [enhance renaming to include identifier variations that are generated by macros](https://github.com/rust-lang/rust-analyzer/pull/19893)
* [render padding information when hovering on structs](https://github.com/rust-lang/rust-analyzer/pull/19876)
* [cycle handlers for `HirDatabase::infer, const_param_ty_with_diagnostics`](https://github.com/rust-lang/rust-analyzer/pull/19894)
* [fix IDE layer not resolving some macro calls](https://github.com/rust-lang/rust-analyzer/pull/19879)
* [fix import insertion not being fully cfg aware](https://github.com/rust-lang/rust-analyzer/pull/19890)
* [fix inference of `AsyncFnX` return type](https://github.com/rust-lang/rust-analyzer/pull/19872)
* [handle included files better in IDE layer](https://github.com/rust-lang/rust-analyzer/pull/19880)
* [recognize salsa cycles in `thread_result_to_response`](https://github.com/rust-lang/rust-analyzer/pull/19888)
* [skip pattern analysis on type mismatches](https://github.com/rust-lang/rust-analyzer/pull/19875)

### Rust Compiler Performance Triage

A fairly busy week, with lots of changes to performance. Most of the changes
(at least in quantity of benchmarks) are attributable to an update of our PGO
collection to newer benchmarks as part of the 2025 refresh.

Triage done by **@simulacrum**.
Revision range: [2805e1dc..2fc3deed](https://perf.rust-lang.org/?start=2805e1dc4c18ed4c84d161502c48da870c56f68a&end=2fc3deed9fcb8762ad57191e0195f06f7543e4a5&absolute=false&stat=instructions%3Au)

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2025-06-02.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Add (back) unsupported_calling_conventions lint to reject more invalid calling conventions](https://github.com/rust-lang/rust/pull/141435)
* [Stabilize `if let` guards (`feature(if_let_guard)`)](https://github.com/rust-lang/rust/pull/141295)
* [Added `Clone` implementation for `ChunkBy`](https://github.com/rust-lang/rust/pull/138016)
* [Make the `dangerous_implicit_autorefs` lint deny-by-default](https://github.com/rust-lang/rust/pull/141661)
* [Make NonZero< char > possible](https://github.com/rust-lang/rust/pull/141001)
* [Tracking Issue for nonnull_provenance](https://github.com/rust-lang/rust/issues/135243)
* [disposition: close] [Implement operations for Wrapping< T > where Rhs = T](https://github.com/rust-lang/rust/pull/140567)
* [Split up the `unknown_or_malformed_diagnostic_attributes` lint](https://github.com/rust-lang/rust/pull/140717)
* [Lint on fn pointers comparisons in external macros](https://github.com/rust-lang/rust/pull/134536)
* [Specify the behavior of `file!`](https://github.com/rust-lang/rust/pull/134442)
* [Document representation of `Option<unsafe fn()>`](https://github.com/rust-lang/rust/pull/141447)
* [Stabilize `feature(generic_arg_infer)`](https://github.com/rust-lang/rust/pull/141610)
* [Allow `#![doc(test(attr(..)))]` everywhere](https://github.com/rust-lang/rust/pull/140560)
* [Tracking Issue for File lock API](https://github.com/rust-lang/rust/issues/130994)
* [disposition: unspecified] [Tracking Issue for `unsigned_signed_diff`](https://github.com/rust-lang/rust/issues/126041)
* [Stabilise `os_string_pathbuf_leak`](https://github.com/rust-lang/rust/pull/137992)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [[RFC] Add `#[export_ordinal(n)]` attribute](https://github.com/rust-lang/rfcs/pull/3641)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [de-RFC: Remove unsized_locals](https://github.com/rust-lang/rfcs/pull/3829)
* [new] [RFC: Procedural macros in same package as app](https://github.com/rust-lang/rfcs/pull/3826)
* [new] [RFC: Allow generic impls using local trait bounds](https://github.com/rust-lang/rfcs/pull/3821)

## Upcoming Events

Rusty Events between 2025-06-04 - 2025-07-02 🦀

### Virtual
* 2025-05-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820285)
* 2025-05-29 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307730629)
* 2025-06-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307795210)
* 2025-06-03 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Why Rust? למה ראסט? -**](https://www.meetup.com/rust-tlv/events/307801358)
* 2025-06-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031665)
* 2025-06-04 | Virtual | [Scientific Computing in Rust](https://scientificcomputing.rs)
     * [**Scientific Computing in Rust 2025**](https://scientificcomputing.rs/2025)
* 2025-06-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 2025-06-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-06-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307927093)
* 2025-06-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560326)
* 2025-06-11 | Virtual (Tel Aviv, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversation with Herbert Wolverson of Ardan Labs & LibreQoS**](https://www.meetup.com/code-mavens/events/308234298/)
* 2025-06-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 2025-06-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/308074808)
* 2025-06-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170853)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820303)
* 2025-06-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbdc)
* 2025-06-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361436)
* 2025-06-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Building Efficient Web Scrapers: Rust vs. Python for Data Ingestion**](https://www.meetup.com/women-in-rust/events/306683025)

### Asia
* 2025-06-08 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust June 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europe
* 2025-05-28 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Surprise Topic**](https://www.meetup.com/rust-rhein-main/events/307836400)
* 2025-05-29 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 2025-05-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 2025-06-04 | Ghent, BE | [Systems Programming Ghent](https://www.sysghent.be/)
    * [**Grow smarter with embedded Rust**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 2025-06-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Risc V - the new challenger for cpus in AI and embedded systems.**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 2025-06-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-10 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/308080874)
* 2025-06-10 | Warsaw, PL | [Rust Warsaw](https://www.meetup.com/rust-warsaw)
    * [**Rust Warsaw Meetup #5**](https://www.meetup.com/rust-warsaw/events/307955051)
* 2025-06-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 2025-06-17 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)
* 2025-06-19 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus)
    * [**Rust Aarhus meetup at Trifork**](https://www.meetup.com/rust-aarhus/events/308060489)
* 2025-06-19 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/308023524)
* 2025-06-20 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/308023512)
* 2025-06-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester)
    * [**Rust Manchester June Code Night**](https://www.meetup.com/rust-manchester/events/307919158)
* 2025-06-25 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group)
    * [**Lessons learnt from making a tiny game in nostd Rust**](https://www.meetup.com/london-rust-project-group/events/306809962)

### North America
* 2025-05-28 | Albuquerque, NM, US | [At Ideas and Coffee](https://www.meetup.com/ideas-and-coffee)
    * [**Intro Level Rust Get-together**](https://www.meetup.com/ideas-and-coffee/events/307645653)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 2025-05-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 2025-05-29 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307498676)
* 2025-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/307936097)
* 2025-06-05 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/308091592)
* 2025-06-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 2025-06-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, June 8**](https://www.meetup.com/bostonrust/events/307936165)
* 2025-06-11 | Phoenix, AZ, US | [Desert Rust](https://www.meetup.com/desert-rustaceans)
    * [**Rust <> Security**](https://www.meetup.com/desert-rustaceans/events/308010023)
* 2025-06-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)
* 2025-06-19 | Hybrid (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**June, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658476)
* 2025-06-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Using Rust For Web Series 3 : Final Presentations and Community Social**](https://www.meetup.com/music-city-rust-developers/events/304333108)
* 2025-06-19 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Monthly Meetup: Making CRUD with Rust!**](https://www.meetup.com/spokane-rust/events/307969600)
* 2025-06-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Lechmere Rust Lunch, June 20**](https://www.meetup.com/bostonrust/events/307936242)
* 2025-06-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcjbhc)

### Oceania
* 2025-06-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)
* 2025-06-24 | Barton, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra)
    * [**June Meetup**](https://www.meetup.com/rust-canberra/events/307520854)

### South America
* 2025-05-31 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 2025-06-04 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)
* 2025-06-12 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina)
    * [**Junio de WebAssembly!**](https://www.meetup.com/rust-argentina/events/307990465)

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

> Rust-based Python type checkers are like buses - you wait ages for one and then two come along at once.

– [u/fiddle_n on r/rustjerk](https://www.reddit.com/r/rustjerk/comments/1kx8uk8/comment/muns9to)

Thanks to [Brian Kung](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1695) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
