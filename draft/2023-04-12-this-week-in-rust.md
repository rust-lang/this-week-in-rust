Title: This Week in Rust 490
Number: 490
Date: 2023-04-12
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

* [sequence_align: an open-source Python + Rust toolkit for efficient sequence alignment](https://blog.kensho.com/introducing-sequence-align-an-open-source-python-rust-toolkit-for-efficient-sequence-alignment-ccdce71d04c7)

### Observations/Thoughts

+ [How Kani helped find bugs in Hifitime](https://model-checking.github.io/kani-verifier-blog/2023/03/31/how-kani-helped-find-bugs-in-hifitime.html)

### Rust Walkthroughs
* [Writing a Fast C# Code-Search Tool in Rust](https://johnaustin.io/articles/2022/blazing-fast-structural-search-for-c-sharp-in-rust)
* [Understanding tracing's macros by rebuilding them from scratch](https://dietcode.io/p/tracing-macros)

- [A guide to aid you in your journey of becoming a Rustacean](https://rust-lang.guide/)

* [Flexible Tracing with Rust and OpenTelemetry](https://broch.tech/posts/rust-tracing-opentelemetry/)

### Research

### Miscellaneous

[CfP for EuroRust 2023 now open](https://www.papercall.io/eurorust-2023)

## Crate of the Week

This week's crate is [spacedisplay](https://github.com/funbiscuit/spacedisplay-rs), a small terminal app for analyzing used disk space.

Thanks to [Sviatoslav Kokurin](https://users.rust-lang.org/t/crate-of-the-week/2704/1183) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

* [rustc - help migrating to `derive(Diagnostic)` / struct-based diagnostics](https://rust-lang.zulipchat.com/#narrow/stream/328238-RustcContributor.3A.3Anew/topic/Diagnostics.20Translation) ([rust#100717](https://github.com/rust-lang/rust/issues/100717))
* [Hyperswitch - Return all the `missing_fields` in a request](https://github.com/juspay/hyperswitch/issues/451)
* [Hyperswitch - perf(logger): Remove unnecessary heap allocations](https://github.com/juspay/hyperswitch/issues/117)
* [Hyperswitch - Have `get_required_value` to use `ValidationError` in `OptionExt`](https://github.com/juspay/hyperswitch/issues/860)
* [Hyperswitch - RFC for optimizing compile time.](https://github.com/juspay/hyperswitch/issues/809)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

385 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-04-03..2023-04-10

* [migrate `rustc_macros` to syn 2.0](https://github.com/rust-lang/rust/pull/109663)
* [add ability to transmute (somewhat) with generic consts in arrays](https://github.com/rust-lang/rust/pull/106281)
* [allow `transmute`s to produce `OperandValue`s instead of needing `alloca`s](https://github.com/rust-lang/rust/pull/109843)
* [resolve: restore some effective visibility optimizations](https://github.com/rust-lang/rust/pull/109437)
* [better diagnostic when pattern matching tuple structs](https://github.com/rust-lang/rust/pull/109760)
* [check pattern refutability on THIR](https://github.com/rust-lang/rust/pull/108504)
* [diagnostics: account for glob shadowing when linting redundant imports](https://github.com/rust-lang/rust/pull/109599)
* [diagnostics: account for self type when looking for source of unsolved type variable](https://github.com/rust-lang/rust/pull/109957)
* [do not suppress `temporary_cstring_as_ptr` in macros](https://github.com/rust-lang/rust/pull/109944)
* [don't ICE when encountering `dyn*` in statics or consts](https://github.com/rust-lang/rust/pull/109921)
* [emit feature error for parenthesized generics in associated type bounds](https://github.com/rust-lang/rust/pull/109914)
* [extend `-Cdebuginfo` with new options and named aliases](https://github.com/rust-lang/rust/pull/109808)
* [fix `non_exhaustive_omitted_patterns` lint span](https://github.com/rust-lang/rust/pull/109838)
* [fix a couple ICEs in the new `CastKind::Transmute` code](https://github.com/rust-lang/rust/pull/110021)
* [fix buffer overrun in bootstrap and (test-only) `symlink_junction`](https://github.com/rust-lang/rust/pull/109960)
* [fix issue when there are multiple candidates for `edit_distance_with_substrings`](https://github.com/rust-lang/rust/pull/109395)
* [implement support for `GeneratorWitnessMIR` in new solver](https://github.com/rust-lang/rust/pull/109755)
* [more descriptive error when qself path doesnt have a trait on the RHS of `as`](https://github.com/rust-lang/rust/pull/109788)
* [never consider int and float vars for `FnPtr` candidates](https://github.com/rust-lang/rust/pull/109896)
* [only visit reachable blocks in ConstProp lint](https://github.com/rust-lang/rust/pull/109792)
* [prioritize param env candidates if they don't guide type inference](https://github.com/rust-lang/rust/pull/109724)
* [pull some tuple variant fields out into their own `struct`](https://github.com/rust-lang/rust/pull/109723)
* [remove the use of `-use-gnu-stack` when BOLTing LLVM](https://github.com/rust-lang/rust/pull/109945)
* [suggest defining const parameter when appropriate](https://github.com/rust-lang/rust/pull/110041)
* [tweak debug outputs to make debugging new solver easier](https://github.com/rust-lang/rust/pull/109956)
* [tweak tuple indexing suggestion](https://github.com/rust-lang/rust/pull/110096)
* [use SipHash-1-3 instead of SipHash-2-4 for StableHasher](https://github.com/rust-lang/rust/pull/107925)
* [yeet `owning_ref`](https://github.com/rust-lang/rust/pull/109971)
* [stabilize `is_some_and`](https://github.com/rust-lang/rust/pull/110019)
* [cargo: add `try_canonicalize` and use it over `std::fs::canonicalize`](https://github.com/rust-lang/cargo/pull/11866)
* [cargo: fix Cargo warning about unused sparse configuration key](https://github.com/rust-lang/cargo/pull/11930)
* [cargo: fix credential token format validation](https://github.com/rust-lang/cargo/pull/11951)
* [cargo: validate token on publish](https://github.com/rust-lang/cargo/pull/11952)
* [rustdoc: avoid including line numbers in Google SERP snippets](https://github.com/rust-lang/rust/pull/109977)
* [rustdoc: escape GAT args in more cases](https://github.com/rust-lang/rust/pull/109919)
* [rustdoc: make intra-doc link pass non-quadratic for repeated links](https://github.com/rust-lang/rust/pull/109876)
* [clippy: `needless_return`: do not trigger on ambiguous match arms return](https://github.com/rust-lang/rust-clippy/pull/10593)
* [clippy: add `manual_slice_size_calculation`](https://github.com/rust-lang/rust-clippy/pull/10601)
* [clippy: add `tests_outside_test_module` lint](https://github.com/rust-lang/rust-clippy/pull/10543)
* [clippy: new lint: `suspicious_doc_comments`](https://github.com/rust-lang/rust-clippy/pull/10497)
* [clippy: fix `mem_replace_option_with_none` not considering field variables](https://github.com/rust-lang/rust-clippy/pull/10594)
* [clippy: fix `single_component_path_imports` false positive on `self::<import>::..`](https://github.com/rust-lang/rust-clippy/pull/10566)
* [clippy: fix bug with getting parent directories in `lookup_conf_file`](https://github.com/rust-lang/rust-clippy/pull/10592)
* [clippy: make `redundant_async_block` a more complete late pass](https://github.com/rust-lang/rust-clippy/pull/10554)
* [clippy: mini-fix `double_must_use` for async functions](https://github.com/rust-lang/rust-clippy/pull/10589)
* [rust-analyzer: highlight escapes in char](https://github.com/rust-lang/rust-analyzer/pull/14512)
* [rust-analyzer: compute closure captures](https://github.com/rust-lang/rust-analyzer/pull/14470)
* [rust-analyzer: desugar async fn completely](https://github.com/rust-lang/rust-analyzer/pull/14486)
* [rust-analyzer: add doc-alias based completion](https://github.com/rust-lang/rust-analyzer/pull/14433)
* [rust-analyzer: convert nested function to closure assist](https://github.com/rust-lang/rust-analyzer/pull/14455)
* [rust-analyzer: drop support for non-syroot proc macro ABIs](https://github.com/rust-lang/rust-analyzer/pull/14432)
* [rust-analyzer: assist: autoderef in generate delegate methods](https://github.com/rust-lang/rust-analyzer/pull/14483)
* [rust-analyzer: fix block local impl trait solving regressions](https://github.com/rust-lang/rust-analyzer/pull/14505)
* [rust-analyzer: fix vscode project linking popup buttons being swapped](https://github.com/rust-lang/rust-analyzer/pull/14481)
* [rust-analyzer: insert whitespace between text and pound](https://github.com/rust-lang/rust-analyzer/pull/14493)
* [rust-analyzer: unify types in `infer_expr_coerce_never()`](https://github.com/rust-lang/rust-analyzer/pull/14520)
* [rust-analyzer: normalize associated types in paths in expressions](https://github.com/rust-lang/rust-analyzer/pull/14436)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

<!-- Approved RFCs go here, use this format: * [Topic](URL) -->
<!-- or if none were approved this week, use: * *No RFCs were approved this week.* -->
<!-- * []() -->

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

Rusty Events between 2023-04-12 - 2023-05-10 🦀

### Virtual

* 2023-04-12 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsryfcgbqb/)
* 2023-04-12 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Launch!**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292410256/)
* 2023-04-13 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Learning Rust By Building Small CLI Tools!**](https://www.meetup.com/charlottesville-rust-meetup/events/292674779/)
* 2023-04-13 | Virtual (Lehi, UT, US) | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Casual Spring Day**](https://www.meetup.com/utah-rust/events/292813786/)
* 2023-04-13 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsyfcgbrb/)
* 2023-04-15 | Virtual (Bangalore, IN) | [Rust India](https://hasgeek.com/rustlangin)
    * [**Rust India monthly meetup**](https://hasgeek.com/rustlangin/april-2023-rustacean-meetup/)
* 2023-04-17 | Virtual (Richmond, VA, US) | [Rustaceans RVA](https://www.meetup.com/rustaceans-rva/)
    * [**April Meetup**](https://www.meetup.com/rustaceans-rva/events/292712141/)
* 2023-04-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Introducing duplicate! and the peculiarities of proc macros**](https://www.meetup.com/rustdc/events/291830834/)
* 2023-04-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/-0)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfcgbzb/)
* 2023-04-20 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsyfcgbbc/)
* 2023-04-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsyfcgbhc/)
* 2023-04-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust-friendly websites and web apps**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292559177/)
* 2023-04-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Testing Tock, how unit tests in Rust improve and teach**](https://www.meetup.com/charlottesville-rust-meetup/events/292193436/)
* 2023-04-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 3: Protohackers Exercises Mob Coding (as far as we get)**](https://www.meetup.com/rust-noris/events/292149688/)
* 2023-05-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfchbdb/)
* 2023-05-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfchbfb/)

### Asia

* 2023-04-12 | Kuala Lumpur, MY | [Rust Malaysia](https://rust-malaysia.github.io/meetup/); [Telegram](https://t.me/golangmalaysia)
    * [**Rust Meetup Malaysia April 2023: How far is Dioxus from React? by Ivan Tham**](https://www.google.com/calendar/event?eid=MWI0bWJzY21qZTI2NWsyZDgzOG0xb2JkdTkgYXBkOXZtYmMyMmVnZW5tdHU1bDZjNWpiZmNAZw&ctz=America/Los_Angeles) | [Map](https://goo.gl/maps/w2ogftac6mqpBbvt5)
* 2023-04-18 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup)
    * [**Rewriting Relay's GraphQL Compiler in Rust**](https://www.meetup.com/tokyo-rust-meetup/events/292549607/)

### Europe

* 2023-04-13 | Roma, IT | [Rust Roma](https://www.meetup.com/rust-roma/)
    * [**Rules engine: from good to awesome (and beyond) with Rust**](https://www.meetup.com/rust-roma/events/292684621/)
* 2023-04-13 | Wrocław, PL | [Rust Wrocław](https://www.meetup.com/rust-wroclaw/)
    * [**Rust Wrocław Meetup #33**](https://www.meetup.com/rust-wroclaw/events/292581415/)
* 2023-04-18 | Montpellier, FR | [Montpellier Rust Meetup](https://www.meetup.com/montpellier-rust-meetup/)
    * [**Meetup Rust Avril 2023**](https://www.meetup.com/montpellier-rust-meetup/events/292805853/)
* 2023-04-19 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #58**](https://www.meetup.com/rust-paris/events/292575461/)
* 2023-04-19 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim/)
    * [**Rust Embedded with MicroBit:V2**](https://www.meetup.com/rust-trondheim/events/292680021/)
* 2023-04-19 | Zurich, CH | [Rust Zurich](https://www.meetup.com/de-DE/rust-zurich/)
    * [**sett: data encryption and transfer made easy(ier)**](https://www.meetup.com/de-DE/rust-zurich/events/292151879/)
* 2023-04-20 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus meetup #1 at Geanix**](https://www.meetup.com/rust-aarhus/events/292185072/)
* 2023-04-20 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 2 - hybrid**](https://www.meetup.com/rust-munich/events/291965920/)
* 2023-04-20 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**First Rust Bern Meetup!**](https://www.meetup.com/de-DE/rust-bern/events/292206056/)
* 2023-04-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfcgbcc/)
* 2023-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn April 2023**](https://www.meetup.com/rust-london-user-group/events/292729308/)
* 2023-04-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - April - Hosted by Sentry**](https://www.meetup.com/rust-vienna/events/292751465/)
* 2023-05-02 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/292297784/)
* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)

### North America

* 2023-04-13 | Lehi, UT, US + Virtual | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Casual Spring Day**](https://www.meetup.com/utah-rust/events/292813786/)
* 2023-04-13 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Autometrics: Easily add metrics in Rust and understand them in Prometheus**](https://www.meetup.com/rust-nyc/events/292430796/)
* 2023-04-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcgbxb/)
* 2023-04-19 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/292749528/)
* 2023-04-19 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Happy Hour and Beginner WASM Rust Hacking Session (#2!)**](https://www.meetup.com/minneapolis-rust-meetup/events/292814034/)
* 2023-04-20 | Mountain View, CA, US | [Mountain View Rust Study Group](https://www.meetup.com/rust-study-group/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/rust-study-group/events/292694348/)
* 2023-04-29 | Durham, NC, US | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Rust Social / Coffee Chat at Boxyard RTP**](https://www.meetup.com/triangle-rust/events/292833711/)

### Oceania

* 2023-04-13 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**🦀 Lightning Talks - 🐰 April Thingy 😊**](https://www.meetup.com/rust-sydney/events/292163549/)

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

> As an expert at being ignorant of what `Pin` does, I can assert with expertise that other ignorant readers have a hard time with `Pin`.

– [grom on rust-users](https://users.rust-lang.org/t/pin-tutorial-are-confusing-me/91003/3)

Thanks to [bugaevc](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1393) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
