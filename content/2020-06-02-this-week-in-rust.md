Title: This Week in Rust 341
Number: 341
Date: 2020-06-02
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

There is no *This Week in Rust* podcast this week, next week's episode will cover both this week and next week.

# We Stand With You

Since our previous issue, there has been a lot of news about the civil
rights discourse in the United States, spawned by the murder of George Floyd
by a member of the Minneapolis Police Department. We stand with Black Lives
Matter and our Black siblings now and always.

We believe this is not a matter of taking a political stance, but a matter
of supporting basic human rights and equality.

We believe that Rustaceans have a duty to our community and to the rest of
the world to ensure that people feel comfortable and welcome wherever they
may be. In our own community, the Rust Code of Conduct explicitly states that
we intend to make everybody feel safe, but this does not just apply to us.

Just as we support Rustaceans, we also support humanity as a whole. It is time
for social progress to be made. We support those risking their own well-being
to show support for George Floyd, Breonna Taylor, Ahmaud Aubery, and everyone
else who has faced injustice at the hands of members of the police. We stand
with the protesters hoping to make the world better. 

If you want to show your support,
[here is a website of curated resources](https://blacklivesmatters.carrd.co/).
We encourage you to speak out, as one more voice is one step closer to a
better world.

# Updates from Rust Community

## News & Blog Posts

* [RustConf 2020 Registration is Open](https://rustconf.com/)
* [2020 Contributor Survey](https://blog.rust-lang.org/inside-rust/2020/05/27/contributor-survey.html)
* [A retrospective on the 2018 rust-lang.org redesign](https://blog.rust-lang.org/inside-rust/2020/05/26/website-retrospective.html)
* [Rust Disassembly: Part 1](https://giordi91.github.io/post/disassemlbyrust1/)
* [Fuzzing Sequoia-PGP](https://blog.hackeriet.no/fuzzing-sequoia/)
* [Mutex in async world](https://kitsu.me/posts/2020_06_01_mutex_in_async_world)
* [Custom types in Diesel](https://kitsu.me/posts/2020_05_24_custom_types_in_diesel)
* [Auto-Vectorization for Newer Instruction Sets in Rust](https://www.nickwilcox.com/blog/autovec2/)
* [Current State of Embedded Rust for Flight Controllers](https://gist.github.com/tstellanova/81c963f556522447dd007a0c3a84ebc3)
* [3D boids swimming along in perfect harmony; Implementing the boids flocking algorithm in Rust](https://www.reddit.com/r/rust/comments/gsldbi/3d_boids_swimming_along_in_perfect_harmony/)
* [Coverage Marks](https://ferrous-systems.com/blog/coverage-marks/)
* [Ringbahn: a safe, ergonomic API for io-uring in Rust](https://boats.gitlab.io/blog/post/ringbahn/)
* [rust-analyzer changelog #26](https://rust-analyzer.github.io/thisweek/2020/05/25/changelog-26.html)
* [Why I'm enjoying learning Rust as a Java programmer](https://opensource.com/article/20/5/rust-java)
* [Contributing to Rust](https://blog.elinvynia.com/posts/2020-05-26-contributing-to-rust.html)
* [Get a Look on Key Rust Crates for WebAssembly](https://blog.knoldus.com/get-a-look-on-key-rust-crates-for-webassembly/)
* [10 Most Loved Programming Languages: Rust, TypeScript, and More](https://insights.dice.com/2020/05/29/10-most-loved-programming-languages-rust-typescript-more/)
* [Creating a Ruby Gem with Rust](https://richardpatching.com/2020/05/22/creating-a-ruby-gem-with-rust.html)
* [Designing the Rust Unleash API client](https://medium.com/cognite/designing-the-rust-unleash-api-client-6809c95aa568)
* [How to build a WebSocket server with Rust](https://blog.logrocket.com/how-to-build-a-websocket-server-with-rust/)
* [Invoking Functions on Distributed Game Objects](https://dev.to/autodidaddict/invoking-functions-on-distributed-game-objects-37b1)
* [IPv6 and Rust](https://blog.apnic.net/2020/06/02/ipv6-and-rust/)
* [Rust as a High Level Language](https://llogiq.github.io/2020/05/30/hi.html)
* [Sorting algorithms in Rust](https://dev.to/jlkiri/sorting-algorithms-in-rust-1386)
* [Programming Servo: integrating streams](https://medium.com/@polyglot_factotum/programming-servo-integrating-readablestream-1a7faebeeed7?source=friends_link&sk=e297efa0a9e9d59d59233f3ec7038b1c)
* [To Rust or not to Rust](https://oldmill.cz/2020-05-31-to-rust-or-not.html)
* [Spanish] [Aprende Rust en español](https://dev.to/robertohuertasm/aprende-rust-en-espanol-1pea)
* [Chinese] [Simple sorting algorithms](https://www.bilibili.com/read/cv4991161)
* [video] [A Rust & Wasm tutorial on building Bitcoin infrastructure. Rust beginner-friendly!](https://www.youtube.com/watch?v=qaykNPHJcyw)
* [video] [Crust of Rust: Iterators](https://www.youtube.com/watch?v=yozQ9C69pNs&feature=emb_logo)
* [video] [Rust and Tell Berlin - May 2020](https://www.youtube.com/watch?v=rpilJV-eIVw&feature=emb_logo)

# Crate of the Week

This week's crate is [jql](https://github.com/yamafaktory/jql), a JSON Query Language CLI tool.

Thanks to [Davy Duperron](https://users.rust-lang.org/t/crate-of-the-week/2704/775) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation


Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ruma-events: Add predefined push notification rules](https://github.com/ruma/ruma-events/issues/105)
* [ruma-events: Add helpers to construct the fallback for rich replies](https://github.com/ruma/ruma-events/issues/81)
* [ruma-events: Create a distinct type for PushCondition::RoomMemberCount::is](https://github.com/ruma/ruma-events/issues/104)
* [Writing database management system in Rust. When the flame is born from the ashes](https://alex-dukhno.github.io/2020-05-30-Writing-database-management-system-in-Rust.-When-the-flame-is-born-from-the-ashes/)


If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

442 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-25..2020-06-01

* [implement unsafe blocks in unsafe fn](https://github.com/rust-lang/rust/pull/71862) (RFC [#2585](https://rust-lang.github.io/rfcs/2585-unsafe-block-in-unsafe-fn.html))
* [exhaustiveness checking: work around type normalization issues](https://github.com/rust-lang/rust/pull/72506)
* [suggest using `std::mem::drop` function instead of explicit destructor call](https://github.com/rust-lang/rust/pull/72383)
* [add a lint against references to packed fields](https://github.com/rust-lang/rust/pull/72270)
* [avoid setting wrong obligation cause span of associated type mismatch](https://github.com/rust-lang/rust/pull/72807)
* [account for trailing comma when suggesting `where` clauses](https://github.com/rust-lang/rust/pull/72715)
* [fix diagnostics for `@ ..` binding pattern in tuples and tuple structs](https://github.com/rust-lang/rust/pull/72677)
* [chalk: request hidden opaque types lazily](https://github.com/rust-lang/chalk/pull/478)
* [miri: synchronization primitive cleanup](https://github.com/rust-lang/miri/pull/1441)
* [`from_u32_unchecked`: check validity, and fix UB in Wtf8](https://github.com/rust-lang/rust/pull/72683)
* [implement `total_cmp` for `f32`, `f64`](https://github.com/rust-lang/rust/pull/72568)
* [override `Box::<[T]>::clone_from`](https://github.com/rust-lang/rust/pull/72499)
* [add `Extend::`{`extend_one`, `extend_reserve`}](https://github.com/rust-lang/rust/pull/72162)
* [make pointer offset methods/intrinsics const](https://github.com/rust-lang/rust/pull/71500)
* [`impl From<[T; N]> for Box<[T]>`](https://github.com/rust-lang/rust/pull/71095)
* [stabilization of `weak-into-raw`](https://github.com/rust-lang/rust/pull/72288)
* [resolve UB in Arc/Weak interaction, part 2](https://github.com/rust-lang/rust/pull/72533)
* [stabilize `str_strip` feature](https://github.com/rust-lang/rust/pull/72466)
* [`impl Step for char` (make `Range*<char>` iterable)](https://github.com/rust-lang/rust/pull/72413)
* [add `Peekable::next_if`](https://github.com/rust-lang/rust/pull/72310)
* [various minor improvements to `Ipv6Addr::Display`](https://github.com/rust-lang/rust/pull/72407)
* [`SocketAddr` and friends now correctly pad its content](https://github.com/rust-lang/rust/pull/72398)
* [implement PartialOrd and Ord for SocketAddr*](https://github.com/rust-lang/rust/pull/72239)
* [tweak and stabilize `Atomic`N`::fetch_update`](https://github.com/rust-lang/rust/pull/71843)
* [stabilize `Atomic`N`::fetch_`{`min`, `max`}](https://github.com/rust-lang/rust/pull/72324)
* [stdarch: add 64 bit integer AVX512f comparisons and the intrinsics needed to test them](https://github.com/rust-lang/stdarch/pull/856)
* [stdarch: add 64 bit AVX512f le and ge comparisons](https://github.com/rust-lang/stdarch/pull/861)
* [libm: use macros for more division/array checks](https://github.com/rust-lang/libm/pull/244)

## Rust Compiler Performance Triage

This is a new section containing the results of a weekly check on how rustc's
perf has changed.

* [2020-06-02](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020.md#2020-06-02)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [`impl AsRef<[T]>` for `vec::IntoIter<T>`](https://github.com/rust-lang/rust/pull/72583)
* [disposition: merge] [Add raw_ref macros](https://github.com/rust-lang/rust/pull/72279)
* [disposition: merge] [Tracking issue for `std::io::{BufReader, BufWriter}::capacity`](https://github.com/rust-lang/rust/issues/68833)

## New RFCs

* [add lang-team Major Change Proposals as a "pre-RFC" step](https://github.com/rust-lang/rfcs/pull/2936)
* [Unsafe statics](https://github.com/rust-lang/rfcs/pull/2937)
* [Request for creating pipes with fd other than 0,1,2](https://github.com/rust-lang/rfcs/pull/2939)

# Upcoming Events

### Online
* [June 3. Johannesburg, ZA - Remote - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/270827463/)
* [June 8. Auckland, NZ - Remote - Rust AKL](https://www.meetup.com/rust-akl/events/266876685/)
* [June 9. Seattle, WA - Remote - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcjbmb/)
* [June 11. San Diego, CA - Remote - San Diego Rust Meetup](https://www.meetup.com/San-Diego-Rust/events/270938860/)

### North America
* [June 3. Indianapolis, IN, US - Indy.rs Meetup](https://www.meetup.com/indyrs/events/dtqwprybcjbfb/)
* [June 11. Columbus, OH, US - Columbus Rust Society Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybcjbpb/)
* [June 11. Lehi, UT, US - Utah Rust - Lightning Talks](https://www.meetup.com/utah-rust/events/269263282/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Software Engineer - Rust - IOTA Foundation - Remote](https://iota.bamboohr.com/jobs/view.php?id=105)
* [Senior Rust Engineer - Polymath - Remote](https://polymath.bamboohr.com/jobs/view.php?id=96)
* [Rust Back End Engineer, Core Banking - TrueLayer - Milan, Italy](https://apply.workable.com/truelayer/j/37748BA121/)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust enables belligerent refactoring – making dramatic changes and then working with the compiler to bring the project back to a working state.

– [Pankaj Chaudhary on Knoldus Blog](https://blog.knoldus.com/some-extensive-projects-working-with-rust)

Thanks to [Maxim Vorobjov](https://users.rust-lang.org/t/twir-quote-of-the-week/328/880) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gvwvep/this_week_in_rust_341/)</small>
