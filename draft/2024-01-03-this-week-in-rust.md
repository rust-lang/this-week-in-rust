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

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

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

Rusty Events between 2024-01-03 - 2024-01-31 🦀

### Virtual

* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)
* 2024-01-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/fvdtgtygccbmb/)
* 2024-01-11 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687491/)
* 2024-01-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128172/)

### Europe

* 2023-12-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs and plushies**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)
* 2023-12-28 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna/)
    * [**Rust Dojo 3: Holiday Edition**](https://www.meetup.com/rust-vienna/events/297826979/)
* 2024-01-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/)
    * [**Reading Rust Meetup at Browns**](https://www.meetup.com/reading-rust-workshop/events/296020357/)
* 2024-01-11 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Meetup #36**](https://www.meetup.com/rust-wroclaw/events/298029291/)
* 2024-01-13 | Helsinki, FI | [Finland Rust-lang Group](https://www.meetup.com/finland-rust-meetup/)
    * [**January Meetup**](https://www.meetup.com/finland-rust-meetup/events/297811750/)

### North America

* 2023-12-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)
* 2024-01-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Beacon Hill Rust Lunch**](https://www.meetup.com/bostonrust/events/297633937/)
* 2024-01-08 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/298003192/)
* 2024-01-09 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564978/)
* 2024-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)
* 2024-01-14 | Cambridge, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Alewife Rust Lunch**](https://www.meetup.com/bostonrust/events/297634920/)
* 2024-01-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/297452643/)
* 2024-01-17 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Happy Hour**](https://www.meetup.com/deep-dish-rust/events/298003233/)

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
