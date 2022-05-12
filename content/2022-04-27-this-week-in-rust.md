Title: This Week in Rust 440
Number: 440
Date: 2022-04-27
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

* [Rust Library Team Aspirations](https://blog.rust-lang.org/inside-rust/2022/04/20/libs-aspirations.html)
* [video] [Cross Team Collaboration Fun Times (CTCFT)](https://www.youtube.com/watch?v=xOkI7xZ35fE)

### Project/Tooling Updates

* [rust-analyzer changelog #126](https://rust-analyzer.github.io/thisweek/2022/04/25/changelog-126.html)
* [IntelliJ Rust Changelog #169](https://intellij-rust.github.io/2022/04/25/changelog-169.html)
* [Slint (UI crate) weekly update](https://slint-ui.com/thisweek/2022-04-25.html)
* [Hello, Robyn!](https://www.sanskar.me/hello_robyn.html)
* [gitoxide - Rich repository information & blazingly fast clone-checkouts](https://github.com/Byron/gitoxide/discussions/398)
* [This week in Databend #39: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-04-27-databend-weekly/)
* [This week in Fluvio #30: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0030/)
* [Rust9x: Compile Rust code for Windows 95, NT and above](https://seri.tools/blog/announcing-rust9x/)

### Observations/Thoughts

* [What a better Rust would look like](https://kerkour.com/what-a-better-rust-would-look-like)
* [A little fixed point math for embedded audio](https://jamesmunns.com/blog/fixed-point-math/)
* [Indirect ownership, shallow borrow and self-referential data structures](https://yoyo-code.com/indirect-ownership-and-self-borrow/)
* [Rust traits and dependency injection](https://jmmv.dev/2022/04/rust-traits-and-dependency-injection.html)
* [What is raw identifier in rust?](https://inspektor.cloud/blog/raw-identifier-in-rust/)
* [How hard upgrading a Rust JWT library could be?](https://blog.orhun.dev/upgrading-rust-jwt/)
* [The magic of AsRef](https://swatinem.de/blog/magic-asref/)

### Rust Walkthroughs

* [Building a crawler in Rust: Implementing the crawler](https://kerkour.com/rust-crawler-implementation)
* [DevLog[0]: Building a serverless platform for Rust in 4 weeks](https://www.shuttle.rs/blog/2022/04/22/dev-log-0)
* [Unlocking greater performance in the MongoDB Rust Driver via raw BSON and zero copy deserialization](https://patrickfreed.github.io/rust/2022/04/27/unlocking-greater-performance-in-the-mongodb-rust-driver-via-raw-bson-and-zero-copy-deserialization.html)

### Research

* [video] [Learning and Programming Challenges of Rust: A Mixed-Methods Study](https://www.youtube.com/watch?v=STjQxTu3tS8)

### Miscellaneous

* [audio] [clap with Ed Page :: Rustacean Station](https://rustacean-station.org/episode/065-ed-page/)
* [DE] [Ferris Talk #8: Wasm loves Rust – WebAssembly und Rust jenseits des Browsers](https://www.heise.de/hintergrund/Ferris-Talk-8-Wasm-loves-Rust-WebAssembly-und-Rust-jenseits-des-Browsers-7064040.html)

## Crate of the Week

This week's crate is [czkawka](https://github.com/qarmin/czkawka), a GTK-based duplicate finder.

Despite a lack of nominations, llogiq is pleased with his pick.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [diesel - looking for feedback on 2.0 release candidate, help updating existing guides to new release](https://github.com/diesel-rs/diesel/releases/tag/v2.0.0-rc0)
* [plotters is looking for new maintainers](https://github.com/plotters-rs/plotters/issues/345)
* [config-rs - Re-thinking config-rs](https://github.com/mehcode/config-rs/issues/321)
* [ruma - several help wanted issues](https://github.com/ruma/ruma/issues?q=label%3A%22help+wanted%22)
* [matrix-rust-sdk - several help wanted issues](https://github.com/matrix-org/matrix-rust-sdk/labels/help%20wanted)
* [include_dir - Compression](https://github.com/Michael-F-Bryan/include_dir/issues/14)
* [include_dir - Exclude files matching a pattern](https://github.com/Michael-F-Bryan/include_dir/issues/13)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-04-18..2022-04-25

* [debuginfo: emit ZST struct debuginfo for unit type when CPP-like debuginfo is enabled](https://github.com/rust-lang/rust/pull/96316)
* [better error message for `_` in function signature in `impl Trait for Ty`](https://github.com/rust-lang/rust/pull/95395)
* [fix an invalid error for a suggestion to add a slice in pattern-matching](https://github.com/rust-lang/rust/pull/96122)
* [improve span for `consider adding an explicit lifetime bound` suggestions under NLL](https://github.com/rust-lang/rust/pull/96352)
* [improve diagnostic on failure to meet send bound on future in a foreign crate](https://github.com/rust-lang/rust/pull/94493)
* [make the lifetime accurate which is used in the region constraints part](https://github.com/rust-lang/rust/pull/96315)
* [miri: allow to track multiple alloc-ids, call-ids and pointer tags](https://github.com/rust-lang/miri/pull/2075)
* [miri: do not consider thread-local allocations read-only](https://github.com/rust-lang/miri/pull/2074)
* [interpret: fix writing uninit to an allocation](https://github.com/rust-lang/rust/pull/96162)
* [micro-optimize `ty::relate::relate_substs` by avoiding `match`](https://github.com/rust-lang/rust/pull/96020)
* [optimize `const_prop` mir-opt by accessing `local_decls` through `ecx`](https://github.com/rust-lang/rust/pull/96281)
* [remove visibility information from HIR](https://github.com/rust-lang/rust/pull/93970)
* [speed up `TokenCursor`](https://github.com/rust-lang/rust/pull/96210)
* [`alloc`: make `vec!` unavailable under `no_global_oom_handling`](https://github.com/rust-lang/rust/pull/96089)
* [unstably constify `impl<I: Iterator> IntoIterator for I`](https://github.com/rust-lang/rust/pull/90602)
* [add `as_slice` to slice iterator](https://github.com/rust-lang/rust/pull/92287)
* [improve Windows path prefix parsing](https://github.com/rust-lang/rust/pull/94887)
* [reduce allocations for path conversions on Windows](https://github.com/rust-lang/rust/pull/96314)
* [futures: create `copy_buf_abortable`, which enables to stop copying in the middle](https://github.com/rust-lang/futures-rs/pull/2507)
* [codegen\_gcc: don't emit `.intel_syntax` for non-x86 targets](https://github.com/rust-lang/rustc_codegen_gcc/pull/164)
* [cargo: prefer `key.workspace = true` to `key = { workspace = true }`](https://github.com/rust-lang/cargo/pull/10584)
* [rustdoc: optimize `IdMap`](https://github.com/rust-lang/rust/pull/96260)
* [rustdoc: optimize and refactor doc link resolution](https://github.com/rust-lang/rust/pull/96135)
* [rustdoc: resolve some more doc links early](https://github.com/rust-lang/rust/pull/96261)
* [rustdoc: unindent doc fragments on `Attributes` construction](https://github.com/rust-lang/rust/pull/96282)
* [rustdoc: make primitive synthetic impls for correct doc module](https://github.com/rust-lang/rust/pull/96301)
* [clippy: add `large_include_file` lint](https://github.com/rust-lang/rust-clippy/pull/8727)
* [clippy: add macro export exemption to `redundant_pub_crate`](https://github.com/rust-lang/rust-clippy/pull/8736)
* [clippy: fix missing whitespace in `collapsible_else_if` suggestion](https://github.com/rust-lang/rust-clippy/pull/8729)
* [clippy: fix `needless_match` false positive for if-let when the else block doesn't match to given expr](https://github.com/rust-lang/rust-clippy/pull/8700)
* [clippy: new lint bytes count to len](https://github.com/rust-lang/rust-clippy/pull/8711)
* [clippy: `manual_split_once`: lint manual iteration of `SplitN`](https://github.com/rust-lang/rust-clippy/pull/8717)
* [clippy: add `empty_drop`](https://github.com/rust-lang/rust-clippy/pull/8571)
* [clippy: `mistyped_literal_suffix`: improve integer suggestions, avoid wrong float suggestions](https://github.com/rust-lang/rust-clippy/pull/8742)
* [clippy: `wrong_self_convention` allows `is_*` to take `&mut self`](https://github.com/rust-lang/rust-clippy/pull/8738)
* [rust-analyzer: fix const generic panic in `dyn trait`](https://github.com/rust-lang/rust-analyzer/pull/12054)
* [rust-analyzer: reduce priority of `flyimport` completions](https://github.com/rust-lang/rust-analyzer/pull/12074)
* [rust-analyzer: restart proc-macro client when server reload](https://github.com/rust-lang/rust-analyzer/pull/12007)
* [rust-analyzer: display signature help when applying "Add `::<>`" assist](https://github.com/rust-lang/rust-analyzer/pull/12032)
* [rust-analyzer: prefer core/alloc over std in auto-imports if `#[no_std]` is conditional](https://github.com/rust-lang/rust-analyzer/pull/12041)
* [rust-analyzer: improve parameter completion](https://github.com/rust-lang/rust-analyzer/pull/12040)
* [rust-analyzer: index the correct `CargoWorkspace` with `rustc_private`](https://github.com/rust-lang/rust-analyzer/pull/12044)

### Rust Compiler Performance Triage

This was, in general, a positive week for compiler performance. There were many concentrated efforts on improving rustdoc performance with a lot of real world crates showing ~4-7% improvements in full build times. Additionally, there was further improvement to `macro_rules!` performance with many real world crates improving performance by as much as 18% in full builds! On the other hand, the regressions were mostly minor and largely relegated to secondary benchmarks.

Triage done by **@rylev**.
Revision range: [4ca19e0..1c988cf](https://perf.rust-lang.org/?start=4ca19e09d302a4cbde14f9cb1bc109179dc824cd&end=1c988cfa0b7f4d3bc5b1cb40dc5002f5adbfb9ad&absolute=false&stat=instructions%3Au)

4 Regressions, 6 Improvements, 3 Mixed; 1 of them in rollups
45 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-04-26.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Make EncodeWide implement FusedIterator](https://github.com/rust-lang/rust/pull/96397)
* [disposition: merge] [Enforce Copy bounds for repeat elements while considering lifetimes](https://github.com/rust-lang/rust/pull/95819)
* [disposition: merge] [Remove mutable_borrow_reservation_conflict lint and allow the code pattern](https://github.com/rust-lang/rust/pull/96268)
* [disposition: merge] [Implement str to [u8] conversion for refcounted containers](https://github.com/rust-lang/rust/pull/96078)
* [disposition: merge] [Stabilize `$$` and `${ignore}` in Rust 1.62.0](https://github.com/rust-lang/rust/pull/95860)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-04-27 - 2022-05-25 🦀

### Virtual

* 2022-04-27 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcgbkc/)
* 2022-04-28 | Linz, AU | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 21st Edition**](https://www.meetup.com/Rust-Linz/events/285248503/)
* 2022-05-02 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/kkbktsydchbdb/)
* 2022-05-03 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting #8**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydchbfb/)
* 2022-05-03 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/Wasm-Rust-Meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/Wasm-Rust-Meetup/events/jbfnrsydchbfb/)
* 2022-05-03 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/284996307/)
* 2022-05-04 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/285121667/)
* 2022-05-04 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/285282177/)
* 2022-05-05 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Dealing with failure: producing and consuming Errors in Rust**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/285078007/)
* 2022-05-09 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Mob Programming: Rome Tools**](https://www.meetup.com/RustPhilly/events/kkbktsydchbmb/)
* 2022-05-10 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399988/)
* 2022-05-10 | Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/Dallas-Rust/events/vqtjcsydchbnb/)
* 2022-05-10 | Rostock, DE | [Altow Academy](https://www.meetup.com/altow-academy/)
    * [**7. Rust Meetup Rostock**](https://www.meetup.com/altow-academy/events/283819127/)
* 2022-05-10 | Saarbrücken, DE | [Rust-Saar](https://www.meetup.com/Rust-Saar/)
    * [**Meetup: 21u16**](https://www.meetup.com/Rust-Saar/events/285483060/)
* 2022-05-10 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydchbnb/)
* 2022-05-11 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/Xe61Zebj6tY53HR7A)
* 2022-05-11 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydchbpb/)
* 2022-05-12 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/tzjtssydchbqb/)
* 2022-05-12 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydchbqb/)
* 2022-05-17 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydchbwb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)

### North America

* 2022-04-27 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/BostonRust/)
    * [**Boston Rust Meetup at Amazon**](https://www.meetup.com/BostonRust/events/284808948)
* 2022-04-27 | Mesa, AZ, US | [Desert Rust](https://www.meetup.com/Desert-Rustaceans/)
    * [**Rust: wasmcloud microservices**](https://www.meetup.com/Desert-Rustaceans/events/285397070/)
* 2022-04-27 | New York, NY, US | [Rust NYC](https://www.meetup.com/Rust-NYC/)
    * [**Intro to Tantivy: a full-text search engine library written in Rust**](https://www.meetup.com/Rust-NYC/events/285257552/)
* 2022-05-11 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydchbpb/)
* 2022-05-12 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydchbqb/)
* 2022-05-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydchbwb/)

### Europe

* 2022-04-28 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/)
    * [**Rust Manchester Pub Meet**](https://www.meetup.com/rust-manchester/events/285016109/)
* 2022-05-06 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Rust programming language in the high-performance computing environment**](https://www.meetup.com/Rust-Zurich/events/285457518/)
* 2022-05-09 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/Finland-Rust-Meetup/)
    * [**May meetup**](https://www.meetup.com/Finland-Rust-Meetup/events/285433622/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Element**

* [Software Engineer - VoIP (Go/Rust) (London, UK)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**Bytewax**

* [Senior Software Engineer (Remote)](https://bytewax.notion.site/Senior-Software-Engineer-a8ea13a594dc454c92a5d4baa15eb321)

**Cambrian Works**

* [Rust Software Engineer (In Spaaaace!) (Remote)](https://www.indeed.com/job/rust-software-engineer-1762efe7a9a9b442)
<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

**HashCloak**

* [Junior Research Engineer (Toronto, ON, CA, Remote)](https://hackmd.io/@hashcloak/HJz2Xn3Z9)

**NXLog**

* [Rust Developer (Remote, Europe or worldwide)](https://application.nxlog.org/jobs/detail/rust-developer-39)

**Zcash Foundation**

* [Core Engineer (Remote)](https://zfnd.org/careers/#Open_positions)

**RDX Works**

* [Senior Rust Software Engineer (Remote)](https://apply.workable.com/rdxworks/j/8DE666E0C4/)

**KidsLoop**

* [Senior Audio/Video Frontend Engineer (Seoul, KR)](https://kidsloop.bamboohr.com/jobs/view.php?id=413)
* [Senior Audio/ Video Backend Engineer (Seoul, KR)](https://kidsloop.bamboohr.com/jobs/view.php?id=397)


**Stockly**

* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)
* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-rust-grpc-postgresql_paris)

**Enso**

* [Senior Rust IDE Developer (Remote EU and US)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-ide-developer.md)
* [Senior Cloud Rust Engineer (Remote EU and US)](https://github.com/enso-org/hiring/blob/main/positions/senior-rust-cloud-developer.md)

**Kraken**

* [Backend Engineer, Kraken Futures - Rust (Remote)](https://jobs.lever.co/kraken/fe1e07f4-6d7c-4f65-9a8f-27cf3b3fd2b1)
* [Engineering Manager - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/53def500-b146-40da-89a8-98adfd7e84e4)
* [Senior Rust Engineer - Banking (Remote)](https://jobs.lever.co/kraken/d29373cd-3007-43e4-ab38-1cff5bc85e33)


*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is the most fundamental philosophy of both the Rust language and the Rust project: we don't think it's sufficient to build robust systems by only including people who don't make mistakes; we think it's better to provide tooling and process to catch and prevent mistakes.

– [Jane Lusby on the inside Rust blog](https://blog.rust-lang.org/inside-rust/2022/04/19/imposter-syndrome.html)

Thanks to [farnbams](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1220) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/udjgdw/this_week_in_rust_440/)</small>
