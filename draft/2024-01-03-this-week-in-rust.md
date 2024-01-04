Title: This Week in Rust 528
Number: 528
Date: 2024-01-03
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
* [Announcing Rust 1.75.0](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0.html)

### Foundation

### Newsletters

### Project/Tooling Updates
* [Rustdoc JSON in 2023](https://alona.page/posts/rustdoc-json-2023/)
* [2023 in Review: Establishing Rust as a Godot 4 language](https://godot-rust.github.io/dev/godot-rust-2023-review/)
* [Rust9x update: Rust 1.76.0-beta](https://seri.tools/blog/rust9x-1-76/)
* [Announcing smol-macros, smol-hyper and smol-axum](https://notgull.net/new-smol-rs-subcrates/)
* [Rust Language Bootstrap Team Progress Report 2023](https://onurozkan.dev/read/rust-language-bootstrap-team-progress-report-2023/)
* [gitoxide: The year in retrospective, and what's to come](https://github.com/Byron/gitoxide/discussions/1223)

### Observations/Thoughts
* [A few fast solutions for Advent of Code 2023](https://jordankaye.dev/posts/aoc-2023/)
* [An Update on Writing Memory Safety Bugs](https://orodu.net/2023/12/27/rust-abstractions.html)
* [Testing Your Embedded Rust](https://barretts.club/posts/embedded-tests/)
* [avatar.png](https://tuckersiemens.com/posts/avatar-png/)
* [Arc vs String, is Arc really faster?](https://blocklisted.github.io/blog/arc_str_vs_string_is_it_really_faster/)
* [Iggy.rs - building message streaming in Rust](https://blog.iggy.rs/posts/building-message-streaming-in-rust/)
* [Getting Started with Loco in Rust: Part 1](https://www.shuttle.rs/blog/2023/12/28/using-loco-rust-rails)

### Rust Walkthroughs

### Research

### Miscellaneous
* [video] [Rust Release Train 1.75](https://www.youtube.com/watch?v=wDzyLFT3AwY)
* [video] [Rust 1.75.0: 54 highlights in 20 minutes](https://www.youtube.com/watch?v=Z8xig7wEV68)

## Crate of the Week

This week's crate is [fast\_pool](https://crates.io/crates/fast_pool), a fast async pool based on the flume channel crate.

Thanks to [zhuxiujia](https://users.rust-lang.org/t/crate-of-the-week/2704/1276) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->
* [Hyperswitch - [FEATURE]: Implement Code cov for local system using makefile](https://github.com/juspay/hyperswitch/issues/1622)
* [Hyperswitch - [FEATURE]: Setup code coverage for local tests & CI](https://github.com/juspay/hyperswitch/issues/1587)
* [Hyperswitch - [FEATURE]: Add domain type for client secret](https://github.com/juspay/hyperswitch/issues/1357)
* [Hyperswitch - [FEATURE]: Have get_required_value to use ValidationError in OptionExt](https://github.com/juspay/hyperswitch/issues/860)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

### CFP - Speakers

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker. 

<!-- CFPs go here, use this format: * [**event name**](link to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->

If you are an event organizer hoping to expand the reach of your event, please submit a link to the submission website either through a PR to TWiR or on the [Rust-lang forums].[link TBD]

## Updates from the Rust Project

194 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-12-27..2024-01-02

* [`rustc_lint`: Enforce `rustc::potential_query_instability` lint](https://github.com/rust-lang/rust/pull/119251)
* [`rustc_lint`: Prevent triplication of various lints](https://github.com/rust-lang/rust/pull/119388)
* [`unused_bindings`: also walk bindings created by if-let guards](https://github.com/rust-lang/rust/pull/119402)
* [change `rustc_codegen_ssa`'s `atomic_cmpxchg` interface to return a pair of values](https://github.com/rust-lang/rust/pull/118705)
* [coverage: avoid a possible query stability hazard in `CoverageCounters`](https://github.com/rust-lang/rust/pull/119401)
* [coverage: prepare mappings separately from injecting statements](https://github.com/rust-lang/rust/pull/119438)
* [coverage: unexpand spans with `find_ancestor_inside_same_ctxt`](https://github.com/rust-lang/rust/pull/119336)
* [don't drop a hir node after lowering](https://github.com/rust-lang/rust/pull/119284)
* [don't suggest writing a bodyless arm if the pattern can never be a never pattern](https://github.com/rust-lang/rust/pull/119380)
* [don't validate / lint MIR before each pass](https://github.com/rust-lang/rust/pull/119377)
* [enable profiler in dist-powerpc-linux](https://github.com/rust-lang/rust/pull/119404)
* [fix infinite loop in `<BoundConstness as Display>`](https://github.com/rust-lang/rust/pull/119447)
* [fix invalid check-cfg Cargo feature diagnostic help](https://github.com/rust-lang/rust/pull/119425)
* [fix parenthesization of subexprs containing statement boundary](https://github.com/rust-lang/rust/pull/119105)
* [fix: correct the args for `disambiguate the associated function` diagnostic](https://github.com/rust-lang/rust/pull/118911)
* [fix: diagnostic for casting reference to slice](https://github.com/rust-lang/rust/pull/119175)
* [introduce `const Trait` (always-const trait bounds)](https://github.com/rust-lang/rust/pull/119099)
* [simplify `Parser::ident_or_error`](https://github.com/rust-lang/rust/pull/119359)
* [simplify bootstrap `--check-cfg` arguments](https://github.com/rust-lang/rust/pull/119441)
* [solaris support on bootstrap lock](https://github.com/rust-lang/rust/pull/119413)
* [subtree sync for `rustc_codegen_cranelift`](https://github.com/rust-lang/rust/pull/119470)
* [suggest `=>` → `>=` in comparisons](https://github.com/rust-lang/rust/pull/117303)
* [utilize the unused `llvm-tools` option](https://github.com/rust-lang/rust/pull/119378)
* [miri: fix integer overflow ICEs from `round_up_to_next_multiple_of`](https://github.com/rust-lang/miri/pull/3246)
* [miri: NaN non-determinism for intrinsics and libm functions](https://github.com/rust-lang/miri/pull/3244)
* [miri: support for tempfile crate on UNIX hosts](https://github.com/rust-lang/miri/pull/3240)
* [implement constant propagation on top of MIR SSA analysis](https://github.com/rust-lang/rust/pull/116012)
* [only store StableCrateId once in DefPathTable](https://github.com/rust-lang/rust/pull/119259)
* [shrink span encoding further](https://github.com/rust-lang/rust/pull/119367)
* [openbsd: `available_parallelism`: use the right API](https://github.com/rust-lang/rust/pull/119436)
* [cargo: `cargo add` - fix for adding features from repository with multiple packages](https://github.com/rust-lang/cargo/pull/13213)
* [cargo: `cargo fix`: always inherit the jobserver](https://github.com/rust-lang/cargo/pull/13225)
* [cargo: fix `fix::fix_in_dependency` to not rely on rustc](https://github.com/rust-lang/cargo/pull/13220)
* [cargo: rustfix: support inserting new lines](https://github.com/rust-lang/cargo/pull/13226)
* [rustdoc-search: count path edits with separate edit limit](https://github.com/rust-lang/rust/pull/119331)
* [rustdoc: treat query string `+` as space](https://github.com/rust-lang/rust/pull/119327)
* [clippy: check for redundant `matches!` with `Ready`, `Pending`, `V4`, `V6`](https://github.com/rust-lang/rust-clippy/pull/12029)
* [clippy: `[doc_markdown]`: Add "WebGL2", "WebGPU" to default `doc_valid_idents`](https://github.com/rust-lang/rust-clippy/pull/12018)
* [clippy: add external macro checks to `iter_without_into_iter` and `into_iter_without_iter`](https://github.com/rust-lang/rust-clippy/pull/12054)
* [clippy: don't lint `default_numeric_fallback` on return and local assigned macro calls with type stated](https://github.com/rust-lang/rust-clippy/pull/11957)
* [clippy: extend `unconditional_recursion` to check for ToString implementations](https://github.com/rust-lang/rust-clippy/pull/11980)
* [clippy: add `manual_is_variant_and` lint](https://github.com/rust-lang/rust-clippy/pull/11865)
* [clippy: add new lint `pub_underscore_fields`](https://github.com/rust-lang/rust-clippy/pull/10283)
* [clippy: suggest `str.lines` when splitting at hard-coded newlines](https://github.com/rust-lang/rust-clippy/pull/11987)
* [clippy: make `mutex_atomic` more type aware](https://github.com/rust-lang/rust-clippy/pull/12008)
* [clippy: new lint: `empty_enum_variants_with_brackets`](https://github.com/rust-lang/rust-clippy/pull/12047)
* [clippy: new lint: `thread_local_initializer_can_be_made_const`](https://github.com/rust-lang/rust-clippy/pull/12026)
* [clippy: new lint: `eager_transmute`](https://github.com/rust-lang/rust-clippy/pull/11981)
* [clippy: remove mitigations for incorrect node args](https://github.com/rust-lang/rust-clippy/pull/12041)
* [rust-analyzer: fix SyntaxContextID using incorrect self IDs](https://github.com/rust-lang/rust-analyzer/pull/16224)
* [rust-analyzer: fix out-of-bounds panic in some macros due to unhandled `self_ref`](https://github.com/rust-lang/rust-analyzer/pull/16221)

### Rust Compiler Performance Triage

Overall, this week had very few regressions and a moderate amount of improvements. The two biggest improvements came in how metadata was being encoded including a change to only store StableCrateId once in DefPathTable which yielded a 0.3% average improvement across 79 different benchmarks.

Triage done by **@rylev**.
Revision range: [1ab783112..67b6975](https://perf.rust-lang.org/?start=1ab783112ab4e4807304dbd249b39771246013ef&end=67b6975051b83ef2bd28f06e8467470d570aceb3&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.3%, 1.5%]    | 8     |
| Regressions ❌ <br /> (secondary)  | 0.8%  | [0.2%, 1.3%]    | 23    |
| Improvements ✅ <br /> (primary)   | -0.6% | [-2.6%, -0.2%]  | 121   |
| Improvements ✅ <br /> (secondary) | -5.2% | [-62.5%, -0.2%] | 53    |
| All ❌✅ (primary)                 | -0.5% | [-2.6%, 1.5%]   | 129   |


2 Regressions, 3 Improvements, 1 Mixed; 0 of them in rollups
46 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/fef95a1961b31e35d91f1ccde0a9783a1ac1d130/triage/2024-01-02.md)

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
* [disposition: merge] [rustdoc: search for tuples and unit by type with ()](https://github.com/rust-lang/rust/pull/118194)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: New range types for Edition 2024](https://github.com/rust-lang/rfcs/pull/3550)
* [Add RFC to discuss RustConf 2024 Steering Committee](https://github.com/rust-lang/rfcs/pull/3549)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2024-01-03 - 2024-01-31 🦀

### Virtual

* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 2024-01-06 | Virtual (Kampala, UG) | [Rust Circle](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763647997?aff=ebdssbdestsearch)
* 2024-01-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/298062049/)
* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679708/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)
* 2024-01-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/292763502/)
* 2024-01-21 | Virtual | [Rust Maven](https://meet-os.com/group/1)
    * [**Web development with Rocket - In English**](https://meet-os.com/event/1)
* 2024-01-23 | Virtual (Halifax, NS, CA) | [Rust Halifax](https://www.meetup.com/rust-tell-halifax/)
    * [**Rust&Tell - Halifax**](https://www.meetup.com/rust-tell-halifax/events/298011202/)
* 2024-01-24 | Virtual (Berlin, DE) | [WeAreDevelopers Community](https://www.meetup.com/wearedevelopers-community/)
    * [**WeAreDevelopers LIVE - Rust Day**](https://www.meetup.com/wearedevelopers-community/events/297065638/)
* 2024-01-25 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298058222/)
* 2024-01-28 | Virtual (Wrocław, PL) | [Stacja IT Wrocław](https://www.meetup.com/stacja-it-wroclaw/)
    * [**Wprowadzenie do języka Rust**](https://www.meetup.com/stacja-it-wroclaw/events/297899705/)
* 2024-01-30 | Virtual (Buffalo, NY, US) | [Buffalo Rust User Group](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group**](https://www.meetup.com/buffalo-rust-meetup/events/297965826/)
* 2024-01-30 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtygccbnc/)

### Europe

* 2024-01-10 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/)
    * [**Game development in Rust**](https://www.meetup.com/rustcologne/events/298303772/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Tampere, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)
* 2024-01-16 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/) 
    * [**Async in Rust**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297376712/)
* 2024-01-17 | Praha / Prague, CZ | [Rust Prague](https://www.meetup.com/rust-prague/)
    * [**Rust Meetup Renewed**](https://www.meetup.com/rust-prague/events/298005196/)
* 2024-01-17 | Zurich, CH | [Rust Zurich](https://www.meetup.com/rust-zurich/)
    * [**How to Community - January Meetup**](https://www.meetup.com/rust-zurich/events/298066842/)
* 2024-01-23 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Hack and Learn**](https://www.meetup.com/rust-aarhus/events/297463730/)

### North America

* 2024-01-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch**](https://www.meetup.com/bostonrust/events/297633937/)
* 2024-01-08 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 2024-01-09 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 2024-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 2024-01-09 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Rust NYC Monthly Meetup: A Deep Dive into Tower by Adrien Guillo**](https://www.meetup.com/rust-nyc/events/298169818/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)
* 2024-01-18 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/events/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/298304117/)
* 2024-01-22 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**North End Rust Lunch**](https://www.meetup.com/bostonrust/events/297634962/)
* 2024-01-24 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtygccbgc/)
* 2024-01-30 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Harvard Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297634994/)

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

> Some people don't believe in life after death... Rust doesn't believe in magic after compilation.

– [Stephan Sokolow on rust-users](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1504)

Thanks to [Todd Fleming](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1505) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
