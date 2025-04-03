Title: This Week in Rust 593
Number: 593
Date: 2025-04-02
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
* [The Embedded Rustacean Issue #42](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-42)
* [This Week in Bevy - 2025-03-31](https://thisweekinbevy.com/issue/2025-03-31-0160-rc2-breakout-on-game-boy-advance-and-bevyecs-for-static-sites)

### Project/Tooling Updates
* [Fjall 2.8](https://fjall-rs.github.io/post/fjall-2-8/)
* [EtherCrab, the pure Rust EtherCAT MainDevice, version 0.6 released](https://wapl.es/ethercrab-0-6/)
* [A process for handling Rust code in the core kernel](https://lwn.net/SubscriberLink/1015409/be9d004a43a7102d/)
* [api-version: axum middleware for header based version selection](https://heikoseeberger.de/2025-03-20-api-version/)

### Observations/Thoughts
* [Introducing Stringleton](https://simonask.github.io/introducing-stringleton/)
* [Rust Any Part 3: Finally we have Upcasts](https://lucumr.pocoo.org/2025/3/27/any-upcast/)
* [Towards fearless SIMD, 7 years later](https://linebender.org/blog/towards-fearless-simd/)
* [LLDB's TypeSystems: An Unfinished Interface](https://walnut356.github.io/posts/lldbs-typesystems-an-unfinished-interface/)
* [Mutation Testing in Rust](https://blog.frankel.ch/mutation-testing-rust/)

* [Embedding shared objects in Rust](https://blog.veeso.dev/blog/en/embedding-shared-objects-in-rust/)

### Rust Walkthroughs
* [Solving the ABA Problem in Rust with Hazard Pointers](https://minikin.me/blog/solving-the-aba-problem-in-rust-hazard-pointers)
* [Building a CoAP application on Ariel OS](https://christian.amsuess.com/blog/website/2025-03-27_ariel_coap/)
* [Inside ScyllaDB Rust Driver 1.0: A Fully Async Shard-Aware CQL Driver Using Tokio](https://www.scylladb.com/2025/03/31/inside-scylladb-rust-driver-1-0/)
* [Building a search engine from scratch, in Rust: part 2](https://jdrouet.github.io/posts/202503191700-search-engine-part-2/)
* [Introduction to Monoio: A High-Performance Rust Runtime](https://chesedo.me/blog/monoio-introduction/)
* [Getting started with Rust on Google Cloud](https://medium.com/google-cloud/getting-started-with-rust-on-google-cloud-ced48447ec91)

### Research

### Miscellaneous
* [An AlphaStation's SROM](https://www.thejpster.org.uk/blog/blog-2025-03-30/)
* [Real-World Verification of Software for Cryptographic Applications](https://cryptographycaffe.sandboxaq.com/posts/real-world-verification-of-software-for-cryptographic-applications/)
* [Public mdBooks](https://mdbooks.code-maven.com/)
* [video] [Networking in Bevy with ECS replication - Hennadii](https://www.youtube.com/watch?v=aDsVFmXD2cc)
* [video] [Intermediate Representations for Reactive Structures - Pete](https://www.youtube.com/watch?v=JeXOajFv8Dk)

## Crate of the Week

This week's crate is [candystore](https://docs.rs/candystore/latest/candystore/), a fast, persistent key-value store that does not require LSM or WALs.

Thanks to [Tomer Filiba](https://users.rust-lang.org/t/crate-of-the-week/2704/1424) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer
and would like your RFC to appear in this list, add a `call-for-testing` label to your RFC along
with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

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

438 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-03-25..2025-04-01

#### Compiler

* [allow defining opaques in statics and consts](https://github.com/rust-lang/rust/pull/138911)
* [avoid wrapping constant allocations in packed structs when not necessary](https://github.com/rust-lang/rust/pull/138503)
* [perform less decoding if it has the same syntax context](https://github.com/rust-lang/rust/pull/129827)
* [stabilize `precise_capturing_in_traits`](https://github.com/rust-lang/rust/pull/138128)
* [uplift `clippy::invalid_null_ptr_usage` lint as `invalid_null_arguments`](https://github.com/rust-lang/rust/pull/119220)

#### Library

* [allow spawning threads after TLS destruction](https://github.com/rust-lang/rust/pull/138702)
* [override PartialOrd methods for bool](https://github.com/rust-lang/rust/pull/138945)
* [simplify expansion for `format_args!()`](https://github.com/rust-lang/rust/pull/139131)
* [stabilize `const_cell`](https://github.com/rust-lang/rust/pull/137928)

#### Rustdoc

* [greatly simplify doctest parsing and information extraction](https://github.com/rust-lang/rust/pull/138104)
* [rearrange `Item`/`ItemInner`](https://github.com/rust-lang/rust/pull/138927)

#### Clippy

* [new lint: `char_indices_as_byte_indices`](https://github.com/rust-lang/rust-clippy/pull/13435)
* [add `manual_dangling_ptr` lint](https://github.com/rust-lang/rust-clippy/pull/14107)
* [respect `#[expect]` and `#[allow]` within function bodies for `missing_panics_doc`](https://github.com/rust-lang/rust-clippy/pull/14407)
* [do not make incomplete or invalid suggestions](https://github.com/rust-lang/rust-clippy/pull/14487)
* [do not warn about shadowing in a destructuring assigment](https://github.com/rust-lang/rust-clippy/pull/14381)
* [expand `obfuscated_if_else` to support `{then(), then_some()}.unwrap_or_default()`](https://github.com/rust-lang/rust-clippy/pull/14431)
* [fix the primary span of `redundant_pub_crate` when flagging nameless items](https://github.com/rust-lang/rust-clippy/pull/14516)
* [fix `option_if_let_else` suggestion when coercion requires explicit cast](https://github.com/rust-lang/rust-clippy/pull/14389)
* [fix `unnested_or_patterns` suggestion in `let`](https://github.com/rust-lang/rust-clippy/pull/14401)
* [make `collapsible_if` recognize the `let_chains` feature](https://github.com/rust-lang/rust-clippy/pull/14481)
* [make `missing_const_for_fn` operate on non-optimized MIR](https://github.com/rust-lang/rust-clippy/pull/14003)
* [more natural suggestions for `cmp_owned`](https://github.com/rust-lang/rust-clippy/pull/14247)
* [`collapsible_if`: prevent including preceeding whitespaces if line contains non blanks](https://github.com/rust-lang/rust-clippy/pull/14480)
* [properly handle expansion in `single_match`](https://github.com/rust-lang/rust-clippy/pull/14495)
* [validate paths in `disallowed_*` configurations](https://github.com/rust-lang/rust-clippy/pull/14397)

#### Rust-Analyzer

* [allow crate authors to control completion of their things](https://github.com/rust-lang/rust-analyzer/pull/19375)
* [avoid relying on `block_def_map()` needlessly](https://github.com/rust-lang/rust-analyzer/pull/19492)
* [fix debug sourceFileMap when using cppvsdbg](https://github.com/rust-lang/rust-analyzer/pull/19475)
* [fix `format_args` lowering using wrong integer suffix](https://github.com/rust-lang/rust-analyzer/pull/19460)
* [fix a bug in orphan rules calculation](https://github.com/rust-lang/rust-analyzer/pull/19466)
* [fix panic in progress due to splitting unicode incorrectly](https://github.com/rust-lang/rust-analyzer/pull/19490)
* [use medium durability for crate-graph changes, high for library source files](https://github.com/rust-lang/rust-analyzer/pull/19451)

### Rust Compiler Performance Triage

Positive week, with a lot of primary improvements and just a few secondary regressions. Single big regression got reverted.

Triage done by **@panstromek**.
Revision range: [4510e86a..2ea33b59](https://perf.rust-lang.org/?start=4510e86a41388733675465a8647d4235f3bf2023&end=2ea33b591050c4ca1a3752830b29112638faecf6&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | -     | -              | 0     |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.2%, 1.5%]   | 17    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-4.5%, -0.1%] | 136   |
| Improvements ✅ <br /> (secondary) | -0.6% | [-3.2%, -0.1%] | 59    |
| All ❌✅ (primary)                 | -0.4% | [-4.5%, -0.1%] | 136   |

[Full report here](https://github.com/rust-lang/rustc-perf/blob/9bd6fc2f4594023b82acd8d876dcf659aee9a931/triage/2025-03-31.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for slice::array_chunks](https://github.com/rust-lang/rust/issues/74985)
* [Stabilize `cfg_boolean_literals`](https://github.com/rust-lang/rust/pull/138632)
* [Promise `array::from_fn is generated in order of increasing indices`](https://github.com/rust-lang/rust/pull/139099)
* [Stabilize `repr128`](https://github.com/rust-lang/rust/pull/138285)
* [Stabilize `naked_functions`](https://github.com/rust-lang/rust/pull/134213)
* [Fix missing const for inherent pointer `replace` methods](https://github.com/rust-lang/rust/pull/136877)

##### [Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [core::marker::NoCell in bounds (previously known an [sic] `Freeze`)](https://github.com/rust-lang/rfcs/pull/3633)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
* [Stabilize automatic garbage collection.](https://github.com/rust-lang/cargo/pull/14287)

#### Other Areas
* No Items entered Final Comment Period this week for
  [Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
  [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc) or
  [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [Allow `&&`, `||`, and `!` in `cfg`](https://github.com/rust-lang/rfcs/pull/3796)

## Upcoming Events

Rusty Events between 2025-04-02 - 2025-04-30 🦀

### Virtual
* 2025-04-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031661)
* 2025-04-03 | Virtual (Nürnberg, DE) | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820282/)
* 2025-04-03 | Virtual | [Ardan Labs](https://www.eventbrite.com/o/ardan-labs-7092394651)
    * [**Communicate with Channels in Rust**](https://www.eventbrite.com/e/communicate-with-channels-in-rust-tickets-1278267335009)
* 2025-04-05 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-04-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/303522530)
* 2025-04-10 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820298)
* 2025-04-15 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170698)
* 2025-04-16 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/306231500)
* 2025-04-17 | Virtual and In-Person (Redmond, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**April, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-22 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361432)
* 2025-04-23 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/)
    * [**Beyond embedded - OS development in Rust **](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/307036053)
* 2025-04-24 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820299)
* 2025-04-24 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Part 2: Quantum Computers Can’t Rust-Proof This!"**](https://www.meetup.com/charlottesville-rust-meetup/events/306679733)

### Asia
* 2025-04-05 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**April 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/april-2025-rustacean-meetup/)
* 2025-04-22 | Tel Aviv-Yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust April 2025 at Braavos in Tel Aviv in collaboration with StarkWare**](https://www.meetup.com/rust-tlv/events/306530984)

### Europe
* 2025-04-02 | Cambridge, UK | [Cambridge Rust Meetup](https://www.meetup.com/cambridge-rust-meetup/events/)
    * [**Monthly Rust Meetup**](https://www.meetup.com/cambridge-rust-meetup/events/306553077)
* 2025-04-02 | Köln, DE | [Rust Cologne](https://www.meetup.com/rust-cologne-bonn/events/)
    * [**Rust in April: Rust Embedded, Show and Tell**](https://www.meetup.com/rustcologne/events/306940549)
* 2025-04-02 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 1 - hybrid**](https://www.meetup.com/rust-munich/events/306097261)
* 2025-04-02 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/306541535)
* 2025-04-02 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust/events/)
    * [**Rust Meetup @Funnel**](https://www.meetup.com/stockholm-rust/events/306627608)
* 2025-04-03 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo/events/)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809680)
* 2025-04-08 | Olomouc, CZ | [Rust Moravia](https://www.meetup.com/rust-moravia/events/)
    * [**3. Rust Moravia Meetup (Real Embedded Rust)**](https://www.meetup.com/rust-moravia/events/306377283)
* 2025-04-09 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 04 2025**](https://lu.ma/dlvfol30)
* 2025-04-09 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop/events/)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045446)
* 2025-04-10 | Karlsruhe, DE | [Rust Hack & Learn Karlsruhe](https://www.meetup.com/rust-hack-learn-karlsruhe/events/)
    * [**Karlsruhe Rust Hack and Learn Meetup bei BlueYonder**](https://www.meetup.com/rust-hack-learn-karlsruhe/events/306682264)
* 2025-04-15 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741632)
* 2025-04-15 | London, UK | [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**WIR x WCC: Finding your voice in Tech**](https://www.meetup.com/women-in-rust/events/306774769)
* 2025-04-19 | Istanbul, TR | [Türkiye Rust Community](https://kommunity.com/turkiye-rust-community/events)
    * [**Rust Konf Türkiye**](https://kommunity.com/turkiye-rust-community/events/rust-konf-turkiye-91f7b3a6)
* 2025-04-23 | London, UK | [London Rust Project Group](https://www.meetup.com/london-rust-project-group/events/)
    * [**Fusing Python with Rust using raw C bindings**](https://www.meetup.com/london-rust-project-group/events/306644439)
* 2025-04-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/events/)
    * [**Talk Night at MFT Energy**](https://www.meetup.com/rust-aarhus/events/305809344)
* 2025-04-24 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (evening pub)**](https://www.meetup.com/rust-and-friends/events/306911347)
* 2025-04-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester April Code Night**](https://www.meetup.com/rust-manchester/events/306899063)
* 2025-04-25 | Edinburgh, UK | [Rust and Friends](https://www.meetup.com/rust-edi/events/)
    * [**Rust and Friends (daytime coffee)**](https://www.meetup.com/rust-and-friends/events/306911357)
* 2025-04-29 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/events/)
    * [**Rust meetup #76**](https://www.meetup.com/rust-paris/events/306952202)

### North America
* 2025-04-03 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup/events/)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/306728493)
* 2025-04-03 | Montréal, QC, CA | [Rust Montréal](https://www.meetup.com/rust-montreal/events/)
    * [**April Monthly Social**](https://www.meetup.com/rust-montreal/events/306518514/)
* 2025-04-03 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**icu4x - resource-constrained internationalization (i18n)**](https://www.meetup.com/stl-rust/events/304890140)
* 2025-04-06 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Kendall Rust Lunch, Apr 6**](https://www.meetup.com/bostonrust/events/306844327)
* 2025-04-08 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Building a full-text search Postgres extension in Rust**](https://www.meetup.com/rust-nyc/events/306983122)
* 2025-04-10 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**TetaNES: A Vaccination for Rust—No Needle, Just the Borrow Checker**](https://www.meetup.com/pdxrust/events/306714209)
* 2025-04-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Coolidge Corner Brookline Rust Lunch, Apr 14**](https://www.meetup.com/bostonrust/events/306844334)
* 2025-04-17 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Using Rust For Web Series 1 : Why HTMX Is Bad**](https://www.meetup.com/music-city-rust-developers/events/304333092)
* 2025-04-17 | Redmond, WA, US | [Seattle Rust User Group](https://www.meetup.com/join-srug/events/)
    * [**April, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/305658454)
* 2025-04-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyhcgbfc)
* 2025-04-25 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Ball Square Rust Lunch, Apr 25**](https://www.meetup.com/bostonrust/events/306844343)

### Oceania
* 2025-04-09 | Sydney, NS, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/events/)
    * [**Crab 🦀 X 🕳️🐇**](https://www.meetup.com/rust-sydney/events/306978026)
* 2025-04-14 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/events/)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/306841248)
* 2025-04-22 | Barton, AC, AU | [Canberra Rust User Group](https://www.meetup.com/rust-canberra/events/)
    * [**April Meetup**](https://www.meetup.com/rust-canberra/events/306425557)

### South America
* 2025-04-03 | Buenos Aires, AR | [Rust en Español](https://www.meetup.com/rust-argentina/events/)
    * [**Abril - Lambdas y más!**](https://www.meetup.com/rust-argentina/events/306671000)

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

> If you write a bug in your Rust program, Rust doesn’t blame you. Rust asks “how could the compiler have spotted that bug”. 

– [Ian Jackson blogging about Rust](https://diziet.dreamwidth.org/19480.html)

Despite a lack of suggestions, llogiq is quite pleased with his choice.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
