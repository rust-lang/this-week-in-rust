Title: This Week in Rust 441
Number: 441
Date: 2022-05-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Foundation

* [A good start to the year!](https://foundation.rust-lang.org/news/2022-05-03-a-good-start-to-the-year/)

### Project/Tooling Updates

* [rust-analyzer changelog #127](https://rust-analyzer.github.io/thisweek/2022/05/02/changelog-127.html)
* [Redox OS 0.7.0](https://www.redox-os.org/news/release-0.7.0/)
* [Plotters is back!](https://aaronerhardt.github.io/blog/posts/plotters_is_back/)
* [Slint (UI crate) weekly update](https://slint-ui.com/thisweek/2022-05-02.html)
* [Fornjot (Code-CAD in Rust) - Weekly Dev Log - 2022-W16/W17 (Post-Vacation Edition)](https://www.fornjot.app/blog/weekly-dev-log/2022-w16-17/)
* [BonsaiDb (NoSQL Database) April Update: Dogfooding and Fixing Bugs](https://bonsaidb.io/blog/april-2022-update/)
* [This week in Databend #40: A Modern Cloud Data Warehouse for Everyone](https://weekly.databend.rs/2022-05-04-databend-weekly/)
* [This week in Fluvio #31: the programmable streaming platform](https://www.fluvio.io/news/this-week-in-fluvio-0031/)

### Observations/Thoughts

* [A shiny future with GATs](https://jackh726.github.io/rust/2022/05/04/a-shiny-future-with-gats.html)
* [C++ & Rust: Generics and Specialization](https://www.tangramvision.com/blog/c-rust-generics-and-specialization)
* [The Better Alternative to Lifetime GATs](https://sabrinajewson.org/blog/the-better-alternative-to-lifetime-gats)
* [Bugs that the Rust compiler catches for you](https://kerkour.com/bugs-rust-compiler-helps-prevent)
* [Creating an Easy Mode for Rust](https://tim.mcnamara.nz/post/683022094467039232/easy-mode-for-rust)
* [video] [Async Rust: Portability and Interoperability - Nick Cameron - Rust Linz, April 2022](https://www.youtube.com/watch?v=jxxLQsxJve8)
* [video] [Rust for the Kubernetes Ecosystem - Deepu K Sasidharan - Rust Linz, April 2022](https://www.youtube.com/watch?v=9icztoT4JVs)
* [audio] [Game Development with Rust and WebAssembly with Eric Smith](https://rustacean-station.org/episode/066-eric-smith/)

### Rust Walkthroughs

* [Patching Cargo Dependencies](https://gatowololo.github.io/blog/cargo-patch/)
* [Building a crawler in Rust: Scraping and Parsing HTML](https://kerkour.com/rust-crawler-scraping-and-parsing-html)
* [Save 5 minutes web scraping with rust](https://domenicoluciani.com/2021/04/30/save-5-minutes-web-scraping-with-rust.html)
* [Building a WebAssembly-powered serverless platform](https://blog.scottlogic.com/2022/04/16/wasm-faas.html)
* [Self-referential structs and alternatives](https://swatinem.de/blog/self-reference-alternatives/)
* [video] [Getting good at SNES games through DLL injection](https://fasterthanli.me/videos/getting-good-at-snes)

### Research

* [Stay Safe under Panic: Affine Rust Programming with Multiparty Session Types](https://arxiv.org/abs/2204.13464)
* [Rusty Links in Local Chains](https://arxiv.org/abs/2205.00795)

## Crate of the Week

This week's crate is [shuttle](https://shuttle.rs), a rustic declarative deployment solution for and at your service.

Thanks to [Nodar Daneliya](https://users.rust-lang.org/t/crate-of-the-week/2704/1058) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

* [RustConf Call for Proposals is open](https://cfp.rustconf.com/events/rustconf-2022)
* [maplibre-rs has several good first issues](https://github.com/maplibre/maplibre-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)
* [diesel.rs-website has several help wanted issues](https://github.com/sgrif/diesel.rs-website/labels/help%20wanted)

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

343 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-04-25..2022-05-02

* [add `do yeet` expressions to allow experimentation in nightly](https://github.com/rust-lang/rust/pull/96376)
* [enforce Copy bounds for repeat elements while considering lifetimes](https://github.com/rust-lang/rust/pull/95819)
* [enforce static lifetimes in consts during late resolution](https://github.com/rust-lang/rust/pull/95776)
* [ensure that `'_` and GAT yields errors](https://github.com/rust-lang/rust/pull/95312)
* [better error messages when collecting into `[T; n]`](https://github.com/rust-lang/rust/pull/96466)
* [erase type params when suggesting fully qualified path](https://github.com/rust-lang/rust/pull/96347)
* [fix incorrect suggestion for trait bounds involving binary operators](https://github.com/rust-lang/rust/pull/94034)
* [improve Error Messaging for Unconstructed Structs and Enum Variants in Generic Contexts](https://github.com/rust-lang/rust/pull/92569)
* [recover suggestions to introduce named lifetime under NLL](https://github.com/rust-lang/rust/pull/96409)
* [recover most `impl Trait` and `dyn Trait` lifetime bound suggestions under NLL](https://github.com/rust-lang/rust/pull/96385)
* [suggest calling method on nested field when struct is missing method](https://github.com/rust-lang/rust/pull/96372)
* [use the correct lifetime binder for elided lifetimes in path](https://github.com/rust-lang/rust/pull/96559)
* [when encountering a binding that could be a const or unit variant, suggest the right path](https://github.com/rust-lang/rust/pull/90988)
* [fix incremental perf regression unsafety checking](https://github.com/rust-lang/rust/pull/96425)
* [support arrays of zeros in `Vec`'s `__rust_alloc_zeroed` optimization](https://github.com/rust-lang/rust/pull/95362)
* [make (`e`)`println!` macros eagerly drop temporaries (for backport)](https://github.com/rust-lang/rust/pull/96490)
* [add `VecDeque::extend` from `vec::IntoIter` and `slice::Iter` specializations](https://github.com/rust-lang/rust/pull/95904)
* [implement `str` to `[u8]` conversion for refcounted containers](https://github.com/rust-lang/rust/pull/96078)
* [std: directly use pthread in UNIX parker implementation](https://github.com/rust-lang/rust/pull/96393)
* [hashbrown: correct the implementation of `Debug` for `ValuesMut` and `IntoValues` structures](https://github.com/rust-lang/hashbrown/pull/321)
* [hashbrown: fix underflow in `RawIterRange::size_hint`](https://github.com/rust-lang/hashbrown/pull/325)
* [stdarch: add stdsimd feature to `allow_internal_unstable` attribute in feature detect macros](https://github.com/rust-lang/stdarch/pull/1303)
* [stdarch: WASM simd128 pairwise/lane-wise](https://github.com/rust-lang/stdarch/pull/1302)
* [codegen\_gcc: add inline assembly `sym` operands as GCC input operands](https://github.com/rust-lang/rustc_codegen_gcc/pull/163)
* [cargo: add support for workspace inheritance](https://github.com/rust-lang/cargo/pull/10606)
* [clippy: extend `extra_unused_lifetimes` to handle impl lifetimes](https://github.com/rust-lang/rust-clippy/pull/8737)
* [clippy: `needless_late_init`: ignore `if let`, `let mut` and significant drops](https://github.com/rust-lang/rust-clippy/pull/8617)
* [clippy: `redundant_closure` fix false positive on coerced closure](https://github.com/rust-lang/rust-clippy/pull/8431)
* [clippy: fix ICE in `cast_slice_different_sizes`](https://github.com/rust-lang/rust-clippy/pull/8720)
* [clippy: ignore `redundant_pub_crate` in `useless_attribute`](https://github.com/rust-lang/rust-clippy/pull/8743)
* [rust-analyzer: reload project on `.cargo/config`(`.toml`) changes](https://github.com/rust-lang/rust-analyzer/pull/12093)
* [rust-analyzer: make "inline type alias" work for `Self` in impls](https://github.com/rust-lang/rust-analyzer/pull/12110)
* [rust-analyzer: diagnose unresolved derive macros](https://github.com/rust-lang/rust-analyzer/pull/12103)
* [rust-analyzer: provide Self in record literal completion](https://github.com/rust-lang/rust-analyzer/pull/12123)
* [rust-analyzer: fix outline mod completion with partial module name](https://github.com/rust-lang/rust-analyzer/pull/12111)
* [rust-analyzer: fix show `macro_rules` snippet in blocks](https://github.com/rust-lang/rust-analyzer/pull/12098)

### Rust Compiler Performance Triage

Performance overall improved in the last week, but some of this is due to fixing
regressions from prior weeks. This week also brings an average of 4% improvement
in memory usage across all profiles due to
[#95171](https://github.com/rust-lang/rust/pull/95171) bumping the LLVM/clang
used on x86_64-unknown-linux-gnu to compile C and C++ code linked into rustc.

Triage done by **@simulacrum**.
Revision range: [1c988cfa..468492](https://perf.rust-lang.org/?start=1c988cfa0b7f4d3bc5b1cb40dc5002f5adbfb9ad&end=468492c2af3993f18b1fe98052200575c4a2e678&absolute=false&stat=instructions%3Au)

**Summary**:

|            | Regressions 😿 <br />(primary) | Regressions 😿 <br />(secondary) | Improvements 🎉 <br />(primary) | Improvements 🎉 <br />(secondary) | All 😿 🎉 <br />(primary) |
|:----------:|:------------------------------:|:--------------------------------:|:-------------------------------:|:---------------------------------:|:------------------------:|
| count      | 13                             | 1                                | 78                              | 29                                | 91                       |
| mean       | 0.8%                           | 0.3%                             | -0.9%                           | -0.8%                             | -0.7%                    |
| max        | 1.5%                           | 0.3%                             | -2.7%                           | -2.1%                             | -2.7%                    |


4 Regressions, 3 Improvements, 1 Mixed; 1 of them in rollups

52 artifact comparisons made in total

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-05-03.md) for more.

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

* [disposition: merge] [impl Read and Write for `VecDeque<u8>`](https://github.com/rust-lang/rust/pull/95632)
* [disposition: merge] [Make write/print macros eagerly drop temporaries](https://github.com/rust-lang/rust/pull/96455)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Advance disclosure of security vulnerabilities](https://github.com/rust-lang/rfcs/pull/3259)

## Upcoming Events

Rusty Events between 2022-05-04 - 2022-06-01 🦀

### Virtual

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
* 2022-05-11 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydchbpb/)
* 2022-05-11 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/Xe61Zebj6tY53HR7A)
* 2022-05-12 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/tzjtssydchbqb/)
* 2022-05-12 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/swgrssydchbqb/)
* 2022-05-17 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydchbwb/)
* 2022-05-18 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydchbxb/)
* 2022-05-18 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out night**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydchbxb/)
* 2022-05-24 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/284399980/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)
* 2022-05-25 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydchbhc/)
* 2022-06-01 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcjbcb/)
* 2022-06-01 | Philadelphia, PA, US | [Rust Philly (Rust Philadelphia)](https://www.meetup.com/RustPhilly/)
    * [**Remote Book Club: Rust for Rustaceans Chapter Discussion**](https://www.meetup.com/RustPhilly/events/qkbktsydcjbcb/)

### North America

* 2022-05-11 | Atlanta, GA, US | [Rust ATL](https://www.meetup.com/Rust-ATL/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/Rust-ATL/events/pczdssydchbpb/)
* 2022-05-12 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydchbqb/)
* 2022-05-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydchbwb/)
* 2022-05-24 | San Francisco, CA, US | [Rust Bay Area](https://www.meetup.com/Rust-Bay-Area/)
    * [**(@ Google) What is soundness anyways?**](https://www.meetup.com/Rust-Bay-Area/events/285563981/)

### Europe

* 2022-05-06 | Zurich, CH | [Rust Zurich](https://www.meetup.com/Rust-Zurich/)
    * [**Rust programming language in the high-performance computing environment**](https://www.meetup.com/Rust-Zurich/events/285457518/)
* 2022-05-09 | Helsinki, FI | [Finland Rust Meetup](https://www.meetup.com/Finland-Rust-Meetup/)
    * [**May meetup**](https://www.meetup.com/Finland-Rust-Meetup/events/285433622/)
* 2022-05-19 & 05-20 | Berlin, DE | [Entwickler.de](https://entwickler.de/)
    * [**Rust Summit (paid)**](https://entwickler.de/rust-summit)
* 2022-05-24 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)]
    * [**Rust Developer Meetup: Lightning Talks @ Fiberplane**](https://www.meetup.com/rust-amsterdam-group/events/285291653/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Element**

* [Software Engineer - VoIP (Go/Rust) (London, UK)](https://apply.workable.com/elementio/j/5BD58AFB6F/)

**Meilisearch**

* [Senior Rust Engineer - Core Engine (Paris, FR and Remote)](https://jobs.lever.co/meili/5b9f780a-0c80-41a7-aba0-bd86834e6823)

**Aembit**

* [Senior Software Engineer (Rust) (Remote US)](https://www.linkedin.com/jobs/view/2876048244/)

**PropelAuth**

* [Founding Backend Engineer (Remote, PST preferred)](https://www.ycombinator.com/companies/propelauth/jobs/b0dl3wz-founding-backend-engineer)

**ANIXE**

* [Rust Software Engineer (Remote or hybrid work PL, GR)](https://anixe.bamboohr.com/jobs/view.php?id=184&source=bamboohr)

**Stockly**

* [Back-end developer (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-rust-grpc-postgresql_paris)
* [Back-end developer - Engine (Rust, gRPC, PostgreSQL) (Paris, FR)](https://www.welcometothejungle.com/fr/companies/stockly-1/jobs/back-end-developer-engine-team-rust-grpc-postgresql_paris)

**Parity**

* [Rust / Core Engineer - Parachains Engineering (Remote/Berlin/Lisbon/UK)](https://grnh.se/24949fb13us)
* [Rust / Core Engineer - Parachains Protocol (Remote/Berlin/Lisbon/UK)](https://grnh.se/06ef2e673us)
* [Rust Engineer - General opening (Remote/Berlin/Lisbon/UK)](https://grnh.se/1cf2de503us)

**Kraken**

* [Engineering Manager - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/53def500-b146-40da-89a8-98adfd7e84e4)
* [Site Reliability Engineer - Rust - Core Backend (Remote)](https://jobs.lever.co/kraken/1c6b290f-e430-430d-9b40-a258d07686b0)
* [Rust Engineer, Desktop GUI - Cryptowatch (Remote)](https://jobs.lever.co/kraken/2442ee5c-56b6-4a73-a477-8cdda2b218d5)

**Rust Foundation**

* [Rust Infrastructure Engineer (Remote)](https://foundation.rust-lang.org/careers/)

**Tempus Ex**

* [Several full-time Rust positions available (San Francisco, CA, US, Atlanta, GA, US, Austin, TX, US, and Remote)](https://tempus-ex.com/careers)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> "Ah but logic errors can happen with all languages" yes and I'm sure trains occasionally run into trees as well, but cars are way more likely to. 🙃

– [amos on twitter](https://twitter.com/fasterthanlime/status/1519457942474477569)

Thanks to [Jacques “erelde” Rimbault](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1225) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/uil19h/this_week_in_rust_441/)</small>
