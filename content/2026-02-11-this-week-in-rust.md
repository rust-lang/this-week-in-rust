Title: This Week in Rust 638
Number: 638
Date: 2026-02-11
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/main/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

Want TWIR in your inbox? [Subscribe here](https://this-week-in-rust.us11.list-manage.com/subscribe?u=fd84c1c757e02889a9b08d289&id=0ed8b72485).

## Updates from Rust Community

### Newsletters
* [Rust Trends Issue #75: Rust Is Becoming the AI Runtime](https://rust-trends.com/newsletter/rust-is-becoming-the-ai-runtime/)
* [GCC Front-End For Rust - January 2026 Monthly report](https://rust-gcc.github.io/2026/02/10/2026-01-monthly-report.html)

### Project/Tooling Updates
* [Fyrox 1.0.0-rc.2](https://fyrox.rs/blog/post/fyrox-game-engine-1-0-0-rc-2/)
* [Slint 1.15 Released](https://slint.dev/blog/slint-1.15-released)
* [The future for Tyr](https://lwn.net/SubscriberLink/1055590/12d48275b6f81988/)
* [Rustbridge v0.9: Building and bundling Rust shared libraries](https://github.com/jrobhoward/rustbridge/blob/v0.9.1/docs/RELEASE_NOTES_0.9.md)
* [Ariel OS v0.3.0: BLE, Sensors, UART, and More!](https://ariel-os.org/blog/ariel-os-0.3.0/)
* [CipherStash Proxy 2.1.20 - Postgres Searchable Encryption in pure Rust](https://github.com/cipherstash/proxy/discussions/361)

### Observations/Thoughts
* [Linux 7.0 Officially Concluding The Rust Experiment](https://www.phoronix.com/news/Linux-7.0-Rust)
* [Borrowed tuple indexing for HashMap](https://traxys.me/tuple_borrow.html)
* [What's so great about Rust?](https://bitfieldconsulting.com/posts/why-rust)
* [Deploying Rust to production checklist](https://kerkour.com/rust-production-checklist)
* [video] [Safe, Fast, and Scalable: Why gRPC-Rust Should Be Your Next RPC Framework](https://www.youtube.com/watch?v=l6YTt8ze4lI)
* [video] [Anodized: Specs Beyond Types in Rust](https://www.youtube.com/watch?v=JtYyhXs4t6w)
* [video] [impl Rust: Avro IDL tool in Rust via LLM](https://www.youtube.com/watch?v=vmKvw73V394)
* [audio] [Netstack.FM episode 26 — Email protocols with Mauro De Gennaro from Stalwart Labs](https://netstack.fm/#episode-26)

### Rust Walkthroughs
* [A future for bitflags](https://kodraus.github.io/rust/2026/02/06/bitflags-derive.html)
* [Hot Reloading in Rust? Subsecond and Dioxus to the rescue!](https://codethoughts.io/posts/2026-02-07-rust-hot-reloading/)
* [Benchmark 2 decimal crates of floating-point vs fixed-point](https://github.com/WuBingzheng/primitive_fixed_point_decimal/blob/master/benches/README.md)
* [Trying to support FreeBSD and Nix for my Rust CLI: Lessons Learned](https://ivaniscoding.github.io/posts/rustpackaging3/)
* [video] [Rama @ FOSDEM 2026 — Rethinking network services: Freedom and modularity with Rama](https://fosdem.org/2026/schedule/event/CKANPK-programmable_networking_with_rama/)
* [video] [Implementing TCP Echo Server in Rust](https://www.youtube.com/watch?v=qjOBZ_Xzuio)

## Crate of the Week

This week's crate is [zedbar](https://crates.io/crates/zedbar), a crate to read QR codes and a bunch of other barcode formats from images.

Thanks to [Brian Donovan](https://users.rust-lang.org/t/crate-of-the-week/2704/1536) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

*No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
  [Cargo](https://github.com/rust-lang/cargo/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen),
  [Rustup](https://github.com/rust-lang/rustup/issues?q=state%3Aopen%20label%3Acall-for-testing%20state%3Aopen) or
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing%20state%3Aopen).*

[Let us know](https://github.com/rust-lang/this-week-in-rust/issues) if you would like your feature to be tracked as a part of this list.

## Call for Participation; projects and speakers

### CFP - Projects

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

* [**Oxidize Conference**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-03-23 | Berlin, Germany | 2026-09-14 - 2026-09-16
* [**RustConf 2026**](https://sessionize.com/rustconf-2026/) | Last chance: CFP closes 2026-02-16 | Montreal, Quebec, Canada | 2026-09-08 - 2026-09-11

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

569 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-03..2026-02-10

#### Compiler
* [add `unreachable_cfg_select_predicates` lint](https://github.com/rust-lang/rust/pull/149960)
* [implement MVP for opaque generic const arguments](https://github.com/rust-lang/rust/pull/150823)
* [provide more context on trait bounds being unmet due to imperfect derive](https://github.com/rust-lang/rust/pull/151278)

#### Library
* [add `NonZero::<T>::from_str_radix`](https://github.com/rust-lang/rust/pull/151945)
* [implement `int_from_ascii` for `NonZero<T>`](https://github.com/rust-lang/rust/pull/152267)
* [add some conversion trait impls](https://github.com/rust-lang/rust/pull/145504)
* [align `ArrayWindows` trait impls with `Windows`](https://github.com/rust-lang/rust/pull/151613)
* [implement stdio FD constants](https://github.com/rust-lang/rust/pull/152071)
* [stabilize `core::hint::cold_path`](https://github.com/rust-lang/rust/pull/151576)
* [stabilize const ControlFlow predicates](https://github.com/rust-lang/rust/pull/152253)
* [stabilize new inclusive range type and iterator type](https://github.com/rust-lang/rust/pull/150522)
* [introduce path normalize methods at top of `std::path`](https://github.com/rust-lang/rust/pull/142957)

#### Cargo
* [`lints`: add `missing_lints_inheritance`](https://github.com/rust-lang/cargo/pull/16588)
* [`lints`: add `unused_workspace_package_fields` lint](https://github.com/rust-lang/cargo/pull/16585)
* [`timings`: enable text selection in the charts](https://github.com/rust-lang/cargo/pull/16607)
* [add host.runner for wrapping host build target executions](https://github.com/rust-lang/cargo/pull/16599)
* [fix known hosts parsing](https://github.com/rust-lang/cargo/pull/16596)

#### Clippy
* [fix `cmp_owned` false positive when `to_string` comes from macro input](https://github.com/rust-lang/rust-clippy/pull/16468)
* [fix: handle false negative for `str_to_string`](https://github.com/rust-lang/rust-clippy/pull/16512)

#### Rust-Analyzer
* [add `expression_types()`, `pattern_types()`, `binding_types()` to `DefWithBody`](https://github.com/rust-lang/rust-analyzer/pull/21584)
* [implement fine grained client side request cancellation support](https://github.com/rust-lang/rust-analyzer/pull/21380)
* [when autoimporting a segment followed by other segments, only consider items that will resolve with the after segments](https://github.com/rust-lang/rust-analyzer/pull/21574)
* [fix linking of postcard test](https://github.com/rust-lang/rust-analyzer/pull/21538)
* [cover more cases where we need parentheses in `&(impl Trait1 + Trait2)`](https://github.com/rust-lang/rust-analyzer/pull/21569)
* [fix `set_top_subtree_delimiter_span` using wrong index for close span](https://github.com/rust-lang/rust-analyzer/pull/21608)
* [fix loses associated bounds for `replace_derive_with_manual_impl`](https://github.com/rust-lang/rust-analyzer/pull/21583)
* [fix not complete `.not` in condition](https://github.com/rust-lang/rust-analyzer/pull/21526)
* [infer the expected len in `include_bytes!()`, to avoid mismatches](https://github.com/rust-lang/rust-analyzer/pull/21573)
* [lowering cycle fixes](https://github.com/rust-lang/rust-analyzer/pull/21579)
* [stale diagnostics with rust-project.json and rustc JSON](https://github.com/rust-lang/rust-analyzer/pull/21571)
* [sync `allow_normalization` to rustc](https://github.com/rust-lang/rust-analyzer/pull/21611)
* [truncate display version of commands consistently](https://github.com/rust-lang/rust-analyzer/pull/21580)
* [use `display_source_code()` in `ReferenceConversion`](https://github.com/rust-lang/rust-analyzer/pull/21578)
* [migrate getters and setters handler to SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21606)

### Rust Compiler Performance Triage

This week we saw quite a few improvements. Largest one comes from adding two targeted `with_capacity` calls in [#151929](https://github.com/rust-lang/rust/pull/151929).
Another source of multiple improvements is the ongoing migration away from using external files to store diagnostic messages.

Triage done by **@panstromek**.
Revision range: [a60d12cb..39219ceb](https://perf.rust-lang.org/?start=a60d12cbccfbeaf153f3cecb90454aa696ea4b3b&end=39219ceb97d1b37dda72517daa9ebe8364ffe186&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range           | count |
|:----------------------------------:|:-----:|:---------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 2.0%  | [2.0%, 2.0%]    | 1     |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.0%, 2.0%]    | 22    |
| Improvements ✅ <br /> (primary)   | -0.8% | [-2.8%, -0.2%]  | 179   |
| Improvements ✅ <br /> (secondary) | -3.1% | [-31.1%, -0.0%] | 211   |
| All ❌✅ (primary)                 | -0.7% | [-2.8%, 2.0%]   | 180   |

1 Regression, 6 Improvements, 7 Mixed; 9 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/c3789580abef82e4d72aaeb5e85cfd09f53e8ce8/triage/2026/2026-02-09.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Stabilize `str_as_str`](https://github.com/rust-lang/rust/pull/151603)
* [Tracking Issue for `#![feature(control_flow_ok)]`](https://github.com/rust-lang/rust/issues/140266)
* [Support importing path-segment keyword with renaming](https://github.com/rust-lang/rust/pull/146972)
* [`-Znext-solver` Remove the forced ambiguity hack from search graph](https://github.com/rust-lang/rust/pull/149904)
* [Make PinCoerceUnsized require Deref](https://github.com/rust-lang/rust/pull/149218)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Remove the translation `-Z` options](https://github.com/rust-lang/compiler-team/issues/967)

##### [Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Stabilize cargo script](https://github.com/rust-lang/cargo/pull/16569)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen), or 
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [RFC: Add compiler support for instrumenting functions](https://github.com/rust-lang/rfcs/pull/3917)
* [RFC: Add `MaybeDropped<T>`](https://github.com/rust-lang/rfcs/pull/3918) 


## Upcoming Events

Rusty Events between 2026-02-11 - 2026-03-11 🦀

### Virtual
* 2026-02-11 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Getting Started with Rust Part 2: Ownership and Structs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/312947249/)
* 2026-02-11 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/5bu9kas1)
* 2026-02-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455922/)
* 2026-02-12 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/312385179/)
* 2026-02-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/312951859/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-18 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/ir8s81ec)
* 2026-02-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**February, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-24 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/310254788/)
* 2026-02-24 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**Lunch & learn: Rust Pattern Matching Unpacked**](https://www.meetup.com/women-in-rust/events/312799411/)
* 2026-02-25 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/fvcjjuv8)
* 2026-02-26 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455923/)
* 2026-03-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftyjcfbgb/)
* 2026-03-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Presentation:  Tock OS Part #3 - Capsules and lower-level hardware drivers**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 2026-03-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (London, UK)| [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799450/)

### Asia
* 2026-02-11 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**Malaysia Rust Meetup February 2026**](https://docs.google.com/forms/d/e/1FAIpQLSfSCWkaD3LeQFleGcGsO4flR3mDKaEQknOTamGg7J7Pw9RoLw/viewform?usp=send_form)
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**February 2026 Rustacean meetup**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust February 2026 at Nuvoton in Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europe
* 2026-02-11 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #14 @ Optravis LLC**](https://www.meetup.com/rust-basel/events/312849882/)
* 2026-02-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/312954164/)
* 2026-02-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-02-18 - 2026-02-19 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-19 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313139277/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester February Talk**](https://www.meetup.com/rust-manchester/events/313172595/)
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**Rust at MWC Talent Arena — Workshops + Community Meetup**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn March 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Records, Shredded on Ice: A Primer on Parquet and Iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)

### North America
* 2026-02-11 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/events/)
    * [**Rust ATX at Cloudflare**](https://www.meetup.com/rust-atx/events/313147803/)
* 2026-02-12 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust)
    * [**Full Stack Web Development in Rust**](https://www.meetup.com/utah-rust/events/312565489/)
* 2026-02-12 | Portland, OR, US | [PDXRust](https://www.meetup.com/pdxrust/events/)
    * [**Implementing a Window Manager: developer workflows, C bindings, and Rust tooling**](https://www.meetup.com/pdxrust/events/313233869/)
* 2026-02-12 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust/events/)
    * [**San Diego Rust February Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/313211483/)
* 2026-02-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Allston Rust Lunch, Feb 14**](https://www.meetup.com/bostonrust/events/312483562/)
* 2026-02-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcdbwb/)
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**February, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers)
    * [**Community Meet and Greet**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
* 2026-02-21 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Somerville Union Square Rust Lunch, Feb 21**](https://www.meetup.com/bostonrust/events/313208518/)
* 2026-02-25 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312755776/)
* 2026-02-25 | Los Angeles, CA, US | [Rust Los Angeles](https://www.meetup.com/rust-los-angeles)
    * [**Rust LA: Rust as a Glue Layer- Infrastructure for AI-Native Applications**](https://www.meetup.com/rust-los-angeles/events/313097225/)
* 2026-02-26 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/events/)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/311228648/)
* 2026-02-26 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/events/)
    * [**Rust NYC: Compile-Time Solutions**](https://www.meetup.com/rust-nyc/events/313196004/)
* 2026-02-28 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**Boston University Rust Lunch, Feb 28**](https://www.meetup.com/bostonrust/events/313208529/)
* 2026-03-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**TBD**](https://www.meetup.com/stl-rust/events/312654992/)
* 2026-03-07 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**MIT Rust Lunch, Mar 7**](https://www.meetup.com/bostonrust/events/313208584/)

### Oceania
* 2026-02-11 | Brisbane City, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Brisbane Feb 2026**](https://www.meetup.com/rust-brisbane/events/313087789/)
* 2026-02-11 | Sydney, AU | [Rust Sydney](https://www.meetup.com/rust-sydney)
    * [**Welcome 🦀 to 2026**](https://www.meetup.com/rust-sydney/events/313074935/)
* 2026-02-24 | Canberra, AU | [Rust Canberra](https://www.meetup.com/rust-canberra/)
    * [**February Meetup**](https://www.meetup.com/rust-canberra/events/313199994/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1qkkqi9/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> Unpopular opinion: error handling in Rust is actually fantastic. Once you know the right patterns, which regrettably are NOT always obvious 😂 

– [Ian Wagner on fosstodon](https://fosstodon.org/@ianthetechie/116012332982905561)

Despite another week with a lamentable lack of suggestions, llogiq is pleased with what he found.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

This Week in Rust is edited by:

* [nellshamrell](https://github.com/nellshamrell)
* [llogiq](https://github.com/llogiq)
* [ericseppanen](https://github.com/ericseppanen)
* [extrawurst](https://github.com/extrawurst)
* [U007D](https://github.com/U007D)
* [mariannegoldin](https://github.com/mariannegoldin)
* [bdillo](https://github.com/bdillo)
* [opeolluwa](https://github.com/opeolluwa)
* [bnchi](https://github.com/bnchi)
* [KannanPalani57](https://github.com/KannanPalani57)
* [tzilist](https://github.com/tzilist)

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1r2p269/this_week_in_rust_638/)</small>
