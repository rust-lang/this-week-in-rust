Title: This Week in Rust 443
Number: 443
Date: 2022-05-18
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

* [What's new in SeaORM 0.8.0](https://www.sea-ql.org/SeaORM/blog/2022-05-15-whats-new-in-0.8.0/)
* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W19](https://www.fornjot.app/blog/weekly-dev-log/2022-w19/)
* [Introducing BlindAI, an open-source privacy-friendly AI deployment in Rust](https://blog.mithrilsecurity.io/introducing-blindai)
* [Slint (UI crate) weekly update -- Version 0.2.4 Release](https://slint-ui.com/thisweek/2022-05-16.html)

### Observations/Thoughts

* [Asteracea (platform-agnostic mixed-style frontend components)](https://blog.schichler.dev/posts/Asteracea/)
* [video] [Building a Postgres storage system in Rust](https://www.youtube.com/watch?v=kAQeout-mh8)

### Rust Walkthroughs

* [Bitsquatting attacks and exploit in Rust](https://kerkour.com/bitsquatting-attack-generator-in-rust)
* [Building a crawler in Rust: Scraping Javascript Single Page Applications (SPA) with a headless browser](https://kerkour.com/rust-crawler-javascript-single-page-application-headless-browser)
* [Compiler Adventures, part 3: Value Numbering](https://medium.com/@predrag.gruevski/compiler-adventures-part-3-value-numbering-12d087b891bd)
* [Rust Ergonomics: Default and From](https://elijahcaine.me/rust-default-from/)

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [cargo-supply-chain](https://crates.io/crates/cargo-supply-chain), a cargo subcommand to gather author, contributor and publisher data on crates in your dependency graph.

Despite a lack of nominations, llogiq is pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Polkadot Blockchain Academy - Course on all things Rust, Substrate, Polkadot and Parity](https://docs.google.com/forms/d/e/1FAIpQLScAFegoJeHQDMXI0RJMI4OsbV0N2aXS5WmoaEcy5ysbaT0FCw/viewform)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

341 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-05-09..2022-05-16

* [Add `EarlyBinder`](https://github.com/rust-lang/rust/pull/96883)
* [rustc: Stricter checking for `#[link]` attributes](https://github.com/rust-lang/rust/pull/96885)
* [Detect trait fulfillment in `subst_and_check_impossible_predicates`](https://github.com/rust-lang/rust/pull/96808)
* [Gracefully fail to resolve associated items instead of `delay_span_bug`](https://github.com/rust-lang/rust/pull/96806)
* [Stop suggesting non-existing fully qualified paths](https://github.com/rust-lang/rust/pull/96940)
* [Implement a lint to warn about unused macro rules](https://github.com/rust-lang/rust/pull/96150)
* [Check hidden types for well formedness at the definition site instead of only at the opaque type itself](https://github.com/rust-lang/rust/pull/96736)
* [Drop tracking: handle invalid assignments better](https://github.com/rust-lang/rust/pull/97031)
* [Forbid nested opaque types to reference HRTB from opaque types](https://github.com/rust-lang/rust/pull/97039)
* [Don't subst an `AdtDef` with its own substs](https://github.com/rust-lang/rust/pull/96882)
* [miri: Add a command line flag to avoid printing to stdout and stderr](https://github.com/rust-lang/miri/pull/2084)
* [miri: Print spans where tags are created and invalidated](https://github.com/rust-lang/miri/pull/2030)
* [miri: Use atomic RMW for `{mutex, rwlock, cond, srwlock}_get_or_create_id` functions](https://github.com/rust-lang/miri/pull/2114)
* [Initial work on Miri permissive-exposed-provenance](https://github.com/rust-lang/rust/pull/95826)
* [Use `FxIndexSet` to avoid sorting fake borrows](https://github.com/rust-lang/rust/pull/96888)
* [make sure `ScalarPair` enums have `ScalarPair` variants; add some layout sanity checks](https://github.com/rust-lang/rust/pull/96872)
* [optimize `insert_range` method of `IntervalSet`](https://github.com/rust-lang/rust/pull/96895)
* [Make `BorrowedFd::borrow_raw` a const fn](https://github.com/rust-lang/rust/pull/96232)
* [`ExitCode::exit_process()` method](https://github.com/rust-lang/rust/pull/95356)
* [Fix `array::IntoIter::fold` to use the optimized `Range::fold`](https://github.com/rust-lang/rust/pull/95602)
* [futures: Make `run_until_stalled` handle self-waking futures](https://github.com/rust-lang/futures-rs/pull/2593)
* [futures: Remove TryStreamExt::into_async_read Unpin bound](https://github.com/rust-lang/futures-rs/pull/2599)
* [cargo: Pass `--target` to `rustdoc` for `cargo test` if specified with host target](https://github.com/rust-lang/cargo/pull/10594)
* [cargo install: Support `foo@version` like cargo-add](https://github.com/rust-lang/cargo/pull/10650)
* [cargo yank: Support foo@version like cargo-add](https://github.com/rust-lang/cargo/pull/10597)
* [rustdoc: correct path to type alias methods](https://github.com/rust-lang/rust/pull/96887)
* [rustdoc: search result ranking fix](https://github.com/rust-lang/rust/pull/96879)
* [clippy: Add `duplicate_mod` lint](https://github.com/rust-lang/rust-clippy/pull/8832)
* [clippy: Add version filtering option to the lint list](https://github.com/rust-lang/rust-clippy/pull/8752)
* [clippy: Don't lint `vec_init_then_push` when further extended](https://github.com/rust-lang/rust-clippy/pull/8699)
* [clippy: Fix ICE in `let_unit_value` when calling a static or const callable type](https://github.com/rust-lang/rust-clippy/pull/8835)
* [clippy: Fix `match_single_binding` suggestion for assign expressions](https://github.com/rust-lang/rust-clippy/pull/8726)
* [clippy: Fix `redundant_allocation` warning for `Rc<Box<str>>`](https://github.com/rust-lang/rust-clippy/pull/8813)
* [clippy: New lint: `derive_partial_eq_without_eq`](https://github.com/rust-lang/rust-clippy/pull/8796)
* [clippy: Rename `eval_order_dependence` to `mixed_read_write_expression`, move to nursery](https://github.com/rust-lang/rust-clippy/pull/8621)
* [clippy: `undocumented_unsafe_blocks` does not trigger on unsafe trait impls](https://github.com/rust-lang/rust-clippy/pull/8761)
* [clippy: introduce `rc_clone_in_vec_init` lint](https://github.com/rust-lang/rust-clippy/pull/8769)
* [rust-analyzer: Config revamp](https://github.com/rust-lang/rust-analyzer/pull/12010)
* [rust-analyzer: Add binding mode inlay hints](https://github.com/rust-lang/rust-analyzer/pull/12253)
* [rust-analyzer: Allow reborrow inlay hints to be restricted to mutable reborrows only](https://github.com/rust-lang/rust-analyzer/pull/12226)
* [rust-analyzer: Change VSCode extension publisher to `rust-lang`](https://github.com/rust-lang/rust-analyzer/pull/12238)
* [rust-analyzer: Handle getters and setters in documentation template assist](https://github.com/rust-lang/rust-analyzer/pull/12274)
* [rust-analyzer: Improve "Generate `Deref` impl" assist](https://github.com/rust-lang/rust-analyzer/pull/12276)
* [rust-analyzer: Show inlay hints after a `}` to indicate the closed item](https://github.com/rust-lang/rust-analyzer/pull/12244)
* [rust-analyzer: Support variable substitution in VSCode settings](https://github.com/rust-lang/rust-analyzer/pull/12215)
* [rust-analyzer: include associated types in trait signature help](https://github.com/rust-lang/rust-analyzer/pull/12208)
* [rust-analyzer: Add cast expressions to param name inlay hint heuristics](https://github.com/rust-lang/rust-analyzer/pull/12201)
* [rust-analyzer: Fix config patching failing when appending suffixes](https://github.com/rust-lang/rust-analyzer/pull/12209)
* [rust-analyzer: Fix fill-arguments completions not working](https://github.com/rust-lang/rust-analyzer/pull/12245)
* [rust-analyzer: Fix incorrect hover actions config keys](https://github.com/rust-lang/rust-analyzer/pull/12246)
* [rust-analyzer: Fix old config patching overwriting callable snippet config unconditionally](https://github.com/rust-lang/rust-analyzer/pull/12228)
* [rust-analyzer: don't panic at fully qualified call syntax in signature help](https://github.com/rust-lang/rust-analyzer/pull/12202)
* [rust-analyzer: insert whitespace between 'mut' and 'self' in macro expansion](https://github.com/rust-lang/rust-analyzer/pull/12262)

### Rust Compiler Performance Triage

It was a somewhat quiet week with real-world benchmarks showing a slight improvement on average and only one real-world crate, `bitmaps`, experiencing regressions. Unfortunately, the cause of the regressions don't look straightforward though. The biggest performance win came from a change to not encode attributes in metadata that are only used within the local crate. This improved doc builds of 16 of the 18 real world crates we run in our performance suite!

Triage done by **@rylev**.
Revision range: [c51871..7355d](https://perf.rust-lang.org/?start=c51871c469f7ed3b35ae25d7e6e77bc73fbdd0e3&end=7355d971a954ed63293e4191f6677f60c1bc07d9&absolute=false&stat=instructions%3Au)

**Summary**:

|            | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:----------:|:------------------------------:|:--------------------------------:|:-------------------------------:|:---------------------------------:|:------------------------:|
| count      | 7                              | 9                                | 40                              | 43                                | 47                       |
| mean       | 1.6%                           | 2.6%                             | -0.6%                           | -1.3%                             | -0.3%                    |
| max        | 1.9%                           | 3.5%                             | -2.6%                           | -2.6%                             | -2.6%                    |


2 Regressions, 4 Improvements, 0 Mixed; 0 of them in rollups
51 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-05-17.md) for more.

### Call for Testing (*New!*)

An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Deduplicate Cargo workspace information](https://github.com/rust-lang/rfcs/pull/2906)
    - [Tracking Issue](https://github.com/rust-lang/cargo/issues/8415)
    - [Testing steps](https://github.com/rust-lang/cargo/blob/master/src/doc/src/reference/unstable.md#testing-notes)
* [Tracking Issue for scoped threads](https://github.com/rust-lang/rust/issues/93203)
    - [Feature documentation](https://doc.rust-lang.org/nightly/std/thread/fn.scope.html)
* [RFC: Packages as (optional) namespaces](https://github.com/rust-lang/rfcs/pull/3243)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: Add target configuration](https://github.com/rust-lang/rfcs/pull/3239)

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Create a types team](https://github.com/rust-lang/rfcs/pull/3254)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Neither require nor imply lifetime bounds on opaque type for well formedness](https://github.com/rust-lang/rust/pull/95474)
* [disposition: merge] [impl Read and Write for VecDeque\<u8\>](https://github.com/rust-lang/rust/pull/95632)
* [disposition: merge] [Tracking Issue for `int_roundings`](https://github.com/rust-lang/rust/issues/88581)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC: Rust SemVer 2](https://github.com/rust-lang/rfcs/pull/3266)

## Upcoming Events

Rusty Events between 2022-05-18 - 2022-06-15 🦀

### Virtual

* 2022-05-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydchbxb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/) & [Berline.rs](https://berline.rs/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)
* 2022-05-24 | Hyderbad, IN | [Rust Language Hyderbad](https://www.meetup.com/Rust-Hyderabad)
    * [**Rust Forum (half day event)**](https://www.meetup.com/Rust-Hyderabad/events/285837876/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways? (Livestream)**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-05-31 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/Dallas-Rust/events/jqxqwrydchbpc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)
* 2022-06-07 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydcjbkb/)
* 2022-06-07 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/xgmfssydcjbkb/)
* 2022-06-08 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcjblb/)
* 2022-06-09 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydcjbmb/)
* 2022-06-09 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/San-Diego-Rust/)
    * [**San Diego Rust June 2022 Tele-Meetup**](https://www.meetup.com/San-Diego-Rust/events/285952122/)
* 2022-06-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydcjbmb/)
* 2022-06-14 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydcjbsb/)
* 2022-06-15 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbtb/)
* 2022-06-15 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydcjbtb/)

### North America

* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**May Lightning Talks: State machines for sync, BonsaiDB, WASM Cloudflare Workers**](https://www.meetup.com/Rust-NYC/events/285925838/)
* 2022-06-08 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydcjblb/)
* 2022-06-09 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcjbmb/)

### Europe

* 2022-05-18 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/Stockholm-Rust/)
    * [**Rust Meetup @AWS**](https://www.meetup.com/Stockholm-Rust/events/285701456/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)
* 2022-05-30 | London, UK | [Rust London User Group](https://www.meetup.com/Rust-London-User-Group/)
    * [**LDN Talks May 2022 *Quickwit Takeover* RSVP @Skills Matter**](https://www.meetup.com/Rust-London-User-Group/events/285740296/)
* 2022-05-31 | Rome, IT | [Rust Roma](https://www.meetup.com/Rust-Roma/)
    * [**When Rocket is fueled by Diesel #Aperitech**](https://www.meetup.com/Rust-Roma/events/285587293/)

### Oceania

* 2022-05-26 | Brisbane City, QL | [Rust Brisbane](https://www.meetup.com/Rust-Brisbane/)
    * [**May Meetup**](https://www.meetup.com/Rust-Brisbane/events/285665676/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Disney**

* [Sr Rust Engineer (New York, NY, US, San Francisco, CA, US, Remote US/Europe)](https://jobs.disneycareers.com/job/new-york/sr-software-engineer-rust/391/25299106288)


<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

**Spire Global**

* [Software Engineer (Glasgow, UK, Remote UK, Luxembourg - Relocation Available)](https://spire.com/careers/job-openings/job/?gh_jid=4226230)

**Parity**

* [Several Rust Engineering Positions Available (Remote/Berlin/Lisbon/UK)](https://www.parity.io/jobs)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> It is worth remembering that there is an infinitely large set of programs but a very small set of useful programs. A programming language is a form of ad-hoc compression algorithm, it is intended to sort the set of useful programs to have shorter encodings than the undesirable ones. Programs with certain categories of error or security vulnerability should be harder or impossible to express.

– [david_chisnall on lobste.rs](https://lobste.rs/s/vtcocq/c_is_truly_all_we_need_everything_else_is#c_yrcjm1)

Thanks to [Anton Fetisov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1241) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
