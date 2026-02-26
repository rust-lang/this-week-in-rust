Title: This Week in Rust 639
Number: 639
Date: 2026-02-18
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

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the linked Page](https://example.com/my_article)

If you add a link to a non-text content please prefix it with `[video]` or `[audio]`:

* [video] [Title of the linked video](https://example.com/my_video_article)
* [audio] [Title of the linked audio file](https://example.com/my_podcast)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official
* [Announcing Rust 1.93.1](https://blog.rust-lang.org/2026/02/12/Rust-1.93.1/)
* [crates.io: an update to the malicious crate notification policy](https://blog.rust-lang.org/2026/02/13/crates.io-malicious-crate-update/)
* [This Development-cycle in Cargo: 1.94](https://blog.rust-lang.org/inside-rust/2026/02/18/this-development-cycle-in-cargo-1.94/)

### Newsletters
* [Scientific Computing in Rust #15 (February 2026)](https://scientificcomputing.rs/monthly/2026-02)
* [The Embedded Rustacean Issue #65](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-65)

### Project/Tooling Updates
* [stochastic-rs: stochastic/quant simulations (and more)](https://rust-dd.com/post/stochastic-rs-v1-stable)
* [Banish v1.1.4: rule-based state-machine DSL](https://github.com/LoganFlaherty/banish/releases/tag/v1.1.4)
* [Building Volatility Surfaces in Rust](https://volsurf-rs.github.io/posts/building-vol-surfaces-in-rust/)
* [diesel-guard v0.6.0: custom checks for Postgres migrations](https://github.com/ayarotsky/diesel-guard/releases/tag/v0.6.0)
* [Selium WebAssembly Hypervisor is in Alpha](https://selium.com/news/alpha-release)
* [FerroTunnel: high-performance reverse tunnel](https://users.rust-lang.org/t/ferrotunnel-high-performance-embeddable-reverse-tunnel-for-rust-applications/138214)
* [Compendium: strace like tracer](https://pker.xyz/posts/compendium)
* [Containerized shell sessions with Shell-Cell](https://mr-leshiy-blog.web.app/blog/shell-cell/)
* [Introducing SurrealDB 3.0 - AI agent memory](https://surrealdb.com/blog/introducing-surrealdb-3-0--the-future-of-ai-agent-memory)
* [sighook 0.9.0: prepatched hook APIs](https://github.com/YinMo19/sighook/releases/tag/v0.9.0)
* [cue - A fast, lightweight file watcher that automatically runs your command on every save.](https://github.com/ozx1/cue)

### Observations/Thoughts
* [How Rust and Its Compiler Have Revolutionized Software Engineering and Reliability](https://kerkour.com/rust-software-engineering-reliability)
* [Async/await on the GPU](https://www.vectorware.com/blog/async-await-on-gpu/)
* [The Evolution of Async Rust: From Tokio to High-Level Applications](https://www.youtube.com/live/2aZaBZVJWm0?si=ienX-zcIBOtDxhj0)

### Rust Walkthroughs
* [Introduction to writing RISC-V contracts in Rust on Polkadot](https://dev.to/badery/introduction-to-writing-risc-v-contracts-in-rust-on-polkadot-29n7)
* [Shipping My Rust CLI to Windows: Lessons Learned (feat. Windows 98 and APE Bonus)](https://ivaniscoding.github.io/posts/rustpackaging4/)
* [Visualizing Persistent Vectors with Rust and WebAssembly](https://abishov.com/blog/pvec-rs-visualizing-structural-sharing/)
* [Recreating PlanetScale's pg_strict in Rust: A Build Log](https://saybackend.com/blog/recreating-planetscale-pg-strict-in-rust/)
* [series] [Part 5: A Witless Fool, Building an LLM from Scratch in Rust](https://www.tag1.com/how-to/part5-a-witless-fool-building-an-llm-from-scratch/)

### Miscellaneous
* [January 2026 Rust Jobs Report](https://filtra.io/rust/jobs-report/jan-26)
* [Rust Developer Ecosystem Survey 2025: Popularity, Trends, and Future](https://blog.jetbrains.com/rust/2026/02/11/state-of-rust-2025/)

## Crate of the Week

This week's crate is [banish](https://github.com/LoganFlaherty/banish), a proc macro to build rule-driven state machines using a declarative DSL.

Thanks to [Logan Flaherty](https://users.rust-lang.org/t/crate-of-the-week/2704/1547) for the self-suggestion!

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

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**Rust India Conference 2026**](https://hasgeek.com/rustbangalore/cfp-rust-india-conference-2026/) | CFP open until 2026-03-14 | Bangalore, IN | 2026-04-18
* [**Oxidize Conference**](https://pretalx.com/oxidize-conference-2026-2025/cfp) | CFP open until 2026-03-23 | Berlin, Germany | 2026-09-14 - 2026-09-16

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

564 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2026-02-10..2026-02-17

#### Compiler
* [handle race when coloring nodes concurrently as both green and red](https://github.com/rust-lang/rust/pull/151509)
* [implement RFC 3678: Final trait methods](https://github.com/rust-lang/rust/pull/151783)
* [replace `box_new` with lower-level intrinsics](https://github.com/rust-lang/rust/pull/148190)
* [shallow resolve ty and const vars to their root vars](https://github.com/rust-lang/rust/pull/151380)
* [show what lint was overruled](https://github.com/rust-lang/rust/pull/152452)

#### Library
* [implement feature `float_exact_integer_constants`](https://github.com/rust-lang/rust/pull/152512)
* [implement `BinaryHeap::from_raw_vec`](https://github.com/rust-lang/rust/pull/152502)
* [implement `carryless_mul`](https://github.com/rust-lang/rust/pull/152132)
* [support ADT types in type info reflection](https://github.com/rust-lang/rust/pull/151142)
* [optimize indexing slices and strs with inclusive ranges](https://github.com/rust-lang/rust/pull/145024)
* [stabilize `assert_matches`](https://github.com/rust-lang/rust/pull/137487)

#### Cargo
* [`lints`: Don't run on-by-default lints when MSRV is too old](https://github.com/rust-lang/cargo/pull/16618)
* [`lockfile-path`: Respect the config in fix, install](https://github.com/rust-lang/cargo/pull/16617)
* [`script`: Load config relative to the script](https://github.com/rust-lang/cargo/pull/16620)
* [`script`: Make the lockfile script-specific independent of build-dir](https://github.com/rust-lang/cargo/pull/16619)
* [changed build script run `output` dir to `stdout` in new build-dir layout](https://github.com/rust-lang/cargo/pull/16644)
* [suggest a `workspace.members` entry even from outside the workspace root](https://github.com/rust-lang/cargo/pull/16616)

#### Rustdoc
* [sort stable items first](https://github.com/rust-lang/rust/pull/149460)

#### Clippy
* [assume that any external function might return a type alias](https://github.com/rust-lang/rust-clippy/pull/16415)
* [do not lint main function in `must_use_candidates`](https://github.com/rust-lang/rust-clippy/pull/16552)
* [extend `iter_kv_map` to cover `flat_map` and `filter_map`](https://github.com/rust-lang/rust-clippy/pull/16519)
* [fix `RustcCallbacks::config()` in `clippy-driver`](https://github.com/rust-lang/rust-clippy/pull/16562)

#### Rust-Analyzer
* [improve hover too long parameter list](https://github.com/rust-lang/rust-analyzer/pull/21591)
* [fix `smol_str` compilation error](https://github.com/rust-lang/rust-analyzer/pull/21648)
* [fix complete semicolon in array expression](https://github.com/rust-lang/rust-analyzer/pull/21402)
* [fix incorrect Self path expand for `inline_call`](https://github.com/rust-lang/rust-analyzer/pull/21381)
* [do not resolve proc macros in value ns (as functions), only in macro ns, outside their defining crate](https://github.com/rust-lang/rust-analyzer/pull/21633)
* [don't assume `extern fn`s parameters are patterns](https://github.com/rust-lang/rust-analyzer/pull/21632)
* [handle `ref mut` bindings in `contains_explicit_ref_binding`](https://github.com/rust-lang/rust-analyzer/pull/21647)
* [use `ExprIsRead::Yes` for rhs of ordinary assignments](https://github.com/rust-lang/rust-analyzer/pull/21649)
* [migrate `covert_tuple_return_type` to `struct` assist to syntax editor](https://github.com/rust-lang/rust-analyzer/pull/21619)
* [migrate `generate_impl` assist to use AstNodeEdit](https://github.com/rust-lang/rust-analyzer/pull/21643)
* [migrate `introduce_named_lifetime` assist to SyntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21507)
* [migrate destructure tuple binding assist to syntaxEditor](https://github.com/rust-lang/rust-analyzer/pull/21618)
* [remove mutable edit in place with `edit::AstNodeEdit` in migrated assist handlers](https://github.com/rust-lang/rust-analyzer/pull/21636)

### Rust Compiler Performance Triage

Several pull requests introduced (usually very small) regressions across the board this week. On the
other hand, [#151380](https://github.com/rust-lang/rust/pull/151380) provided a nice performance win in the inference engine.
I would also like to bring attention to [#152375](https://github.com/rust-lang/rust/pull/152375),
which improved the parallel frontend. It is not shown in this report, because we don't yet have
many benchmarks for the parallel frontend, but this PR seemingly improved the `check` (wall-time)
performance with multiple frontend threads on several real-world crates by 5-10%!

Triage done by **@kobzol**.
Revision range: [39219ceb..3c9faa0d](https://perf.rust-lang.org/?start=39219ceb97d1b37dda72517daa9ebe8364ffe186&end=3c9faa0d037b9eecda4a440cc482ff7f960fb8a5&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.7%  | [0.2%, 3.1%]   | 96    |
| Regressions ❌ <br /> (secondary)  | 1.1%  | [0.0%, 5.7%]   | 62    |
| Improvements ✅ <br /> (primary)   | -0.4% | [-0.9%, -0.2%] | 8     |
| Improvements ✅ <br /> (secondary) | -2.6% | [-7.0%, -0.0%] | 45    |
| All ❌✅ (primary)                 | 0.6%  | [-0.9%, 3.1%]  | 104   |

2 Regressions, 0 Improvements, 9 Mixed; 4 of them in rollups
36 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/a1128be07cc42d2bed7a65068f82dce36964386a/triage/2026/2026-02-17.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs

##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen)

* [Inhibit all-absent-variant optimization for all enum reprs that inhibit layout optimization, not just repr(C).](https://github.com/rust-lang/rust/pull/146989)
* [stabilize `cfg_select!`](https://github.com/rust-lang/rust/pull/149783)
* [ptr::replace: make calls on ZST null ptr not UB](https://github.com/rust-lang/rust/pull/149169)
* [Never break between empty parens](https://github.com/rust-lang/rust/issues/152761)

##### [Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20label%3Afinal-comment-period%20state%3Aopen) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)

* [Add a `--min-recursion-limit` command line flag](https://github.com/rust-lang/compiler-team/issues/969)

##### [Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen)

* [Participation in Outreachy (dedication of funds)](https://github.com/rust-lang/leadership-council/issues/264)

*No Items entered Final Comment Period this week for
[Rust RFCs](https://github.com/rust-lang/rfcs/issues?q=state%3Aopen%20label%3Afinal-comment-period%20state%3Aopen),
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen), or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen%20label%3Afinal-comment-period%20sort%3Aupdated-desc%20state%3Aopen).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [Grants team and 2026 grants program](https://github.com/rust-lang/rfcs/pull/3919)
* [RFC: Extend manifest dependencies with `used`](https://github.com/rust-lang/rfcs/pull/3920)

## Upcoming Events

Rusty Events between 2026-02-18 - 2026-03-18 🦀

### Virtual
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
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/313303094/)
* 2026-03-05 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/events/)
    * [**Presentation:  Tock OS Part #3 - Capsules and lower-level hardware drivers**](https://www.meetup.com/charlottesville-rust-meetup/events/313264830/)
* 2026-03-05 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/events/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/313293173/)
* 2026-03-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763908777)
* 2026-03-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust/events/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/310254786/)
* 2026-03-10 | Virtual (London, UK)| [Women in Rust](https://www.meetup.com/women-in-rust/events/)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/312799450/)
* 2026-03-12 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/308455924/)
* 2026-03-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/events/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/rdhhptyjcfbwb/)
* 2026-03-18 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/events/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/dwnbwtyjcfbxb/)

### Asia
* 2026-02-21 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**February 2026 Rustacean meetup**](https://hasgeek.com/rustbangalore/february-2026-rustacean-meetup/)
* 2026-02-23 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust February 2026 at Nuvoton in Herzliya**](https://www.meetup.com/rust-tlv/events/312989544/)

### Europe
* 2026-02-18 - 2026-02-19 | London, UK | [Rust Nation UK](https://www.rustnationuk.com/)
    * [**Rust Nation UK 2026**](https://www.rustnationuk.com/)
* 2026-02-19 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/313139277/)
* 2026-02-24 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #5 @ Zrch: Doom on Embedded**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/313109606)
* 2026-02-24 | Manchester, UK | [Rust Manchester](https://www.meetup.com/rust-manchester/events/)
    * [**Rust Manchester February Talk**](https://www.meetup.com/rust-manchester/events/313172595/) | [**Event Page**](https://rustmanchester.co.uk/events/february-talks-2026/)
* 2026-02-25 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust meetup #65 Sponsored by Factbird**](https://www.meetup.com/copenhagen-rust-community/events/313341944/)
* 2026-02-26 | Prague, CZ | [Rust Czech Republic](https://www.meetup.com/rust-czech-republic/events/)
    * [**Informační teorie vs. filtry: Proč filtrování bitcoinového mempoolu NEFUNGUJE**](https://www.meetup.com/rust-czech-republic/events/313323947/)
* 2026-02-28 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #24 - crablings edition**](https://www.meetup.com/stockholm-rust/events/313367881/)
* 2026-03-04 | Barcelona, ES | [BcnRust](https://www.meetup.com/bcnrust/events/)
    * [**Rust at MWC Talent Arena — Workshops + Community Meetup**](https://www.meetup.com/bcnrust/events/313263086/)
* 2026-03-04 | Hamburg, DE | [Rust Meetup Hamburg](https://www.meetup.com/rust-meetup-hamburg/events/)
    * [**Rust Hack & Learn March 2026**](https://www.meetup.com/rust-meetup-hamburg/events/311942636/)
* 2026-03-04 | Oxford, UK | [Oxford ACCU/Rust Meetup.](https://www.meetup.com/oxford-rust-meetup-group/events/)
    * [**Records, Shredded on Ice: A Primer on Parquet and Iceberg**](https://www.meetup.com/oxford-rust-meetup-group/events/312664488/)
* 2026-03-12 | Geneva, CH | [Post Tenebras Lab](https://www.posttenebraslab.ch/)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2026-03-18 | Dortmund, DE | [Rust Dortmund](https://www.meetup.com/rust-dortmund/events/)
    * [**Rust Dortmund Meetup - Intro to Embedded Rust - March**](https://www.meetup.com/rust-dortmund/events/313338784/)

### North America
* 2026-02-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/310619456/)
* 2026-02-19 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**February, 2026 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/312274876/)
* 2026-02-19 | Nashville, TN, US | [Music City Rust Developers](https://www.meetup.com/music-city-rust-developers/events/)
    * [**Rust 101: What is Rust and how can I use it?**](https://www.meetup.com/music-city-rust-developers/events/312038658/)
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
* 2026-03-14 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/events/)
    * [**North End Rust Lunch, Mar 14**](https://www.meetup.com/bostonrust/events/313208587/)
* 2026-03-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/events/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/ghhwqtyjcfbwb/)

### Oceania
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

> Clearly there is such a thing as too much syntactic sugar (as one of my professors put it, "syntactic sugar causes semantic cancer"), but at the same time also clearly some syntactic sugar is worth having.

– [Ralf Jung on rust-internals](https://internals.rust-lang.org/t/pre-pre-rfc-splatting-for-named-arguments-and-function-overloading/24012/17)

Thanks to [robofinch](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1753) for the suggestion!

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/1r8n7fm/this_week_in_rust_639/)</small>
