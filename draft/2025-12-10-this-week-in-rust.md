Title: This Week in Rust 629
Number: 629
Date: 2025-12-10
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
* [Lessons learned from the Rust Vision Doc process](https://blog.rust-lang.org/2025/12/03/lessons-learned-from-the-rust-vision-doc-process/)
* [Updating Rust's Linux musl targets to 1.2.5](https://blog.rust-lang.org/2025/12/05/Updating-musl-1.2.5/)
* [Making it easier to sponsor Rust contributors](https://blog.rust-lang.org/2025/12/08/making-it-easier-to-sponsor-rust-contributors/)

### Newsletters
* [The Embedded Rustacean Issue #60](https://www.theembeddedrustacean.com/p/the-embedded-rustacean-issue-60)

### Project/Tooling Updates
* [Yew 0.22 - For Real This Time](https://yew.rs/blog/2025/11/29/release-0-22)
* [Ferrocene 25.11.0 now available!](https://ferrous-systems.com/blog/ferrocene-25-11-0/)
* [YM2149-rs 0.8.0 — Cycle-accurate YM2149/AY-3-8910 chiptune ecosystem](https://ym2149-rs.org/)
* [Duper's new superpowers!](https://duper.dev.br/blog/duper-s-new-superpowers.html)
* [Announcing redis-rs 1.0.0 release](https://github.com/redis-rs/redis-rs/blob/main/version1.md)
* [Announcing diesel-guard 0.2.0: Catch unsafe PostgreSQL migrations before they hit production](https://www.reddit.com/r/rust/comments/1phx68w/dieselguard_catch_unsafe_postgresql_migrations/)
* [video] [Recording from the Rust Seoul meetup: Zia, a programming language that defines itself (written in Rust)](https://www.youtube.com/watch?v=LbFTP3pITWU)

### Observations/Thoughts
* [Exploring deboa-macros: Ergonomic HTTP Client Macros for Rust](https://medium.com/@ararog/exploring-deboa-macros-ergonomic-http-client-macros-for-rust-d0c3df22e0a7)
* [Fighting the Client Spaghetti Monster with Rust Traits](http://gnunicorn.org/writings/spaghetti-monster-clients-rust-traits-final-boss/)
* [Rust unit testing: buffered file reading](https://jorgeortiz.dev/posts/rust_unit_testing_file_buf_reading/)
* [Firecracker deep dive: How Rust and microVMs are revolutionizing cloud infrastructure](https://kerkour.com/firecracker-deep-dive-rust)
* [iksemel rusted](https://thinkerf.blogspot.com/2025/12/iksemel-rusted.html)
* [Postfix Macros and `let place`](https://nadrieril.github.io/blog/2025/12/09/postfix-macros-and-let-place.html)
* [Should we get rid of clippy::manual_try_fold?](https://blog.veeso.dev/blog/en/should-we-get-rid-of-clippy-manual-try-fold/)

### Rust Walkthroughs
* [Get in Line - superfast SPSC Queue](https://abhikja.in/blog/2025-12-07-get-in-line/)
* [Emulating avx-512 intrinsics in Miri](https://trifectatech.org/blog/emulating-avx-512-intrinsics-in-miri/)
* [How to speed up the Rust compiler in December 2025](https://nnethercote.github.io/2025/12/05/how-to-speed-up-the-rust-compiler-in-december-2025.html)
* [From trees to graphs: speeding up vector search 10x with Hannoy](https://blog.kerollmops.com/from-trees-to-graphs-speeding-up-vector-search-10x-with-hannoy)
* [series] [Part 1: Tokenization, Building an LLM From Scratch in Rust](https://www.tag1.com/white-paper/part1-tokenization-building-an-llm-from-scratch-in-rust/)

### Miscellaneous
* [Pydantic: The Python Darling That Loves Rust](https://filtra.io/rust/interviews/pydantic-dec-25)
* [video] [AWS re:Invent 2025 - Unleash Rust's potential on AWS (DEV307)](https://www.youtube.com/watch?v=buBBQ5mXAi8)

## Crate of the Week

This week's crate is [mdbook-lint](https://github.com/joshrotenberg/mdbook-lint), a markdown linter geared towards mdbook, but useful with any markdown.

Thanks to [josh rotenberg](https://users.rust-lang.org/t/crate-of-the-week/2704/1502) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by
  [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Cargo](https://github.com/rust-lang/cargo/labels/call-for-testing),
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
*No Calls for participation were submitted this week.*

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines] or through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

[guidelines]:https://github.com/rust-lang/this-week-in-rust?tab=readme-ov-file#call-for-participation-guidelines

### CFP - Events

Are you a new or experienced speaker looking for a place to share something cool? This section highlights events that are being planned and are accepting submissions to join their event as a speaker.

<!-- CFPs go here, use this format: * [**event name**](URL to CFP)| Date CFP closes in YYYY-MM-DD | city,state,country | Date of event in YYYY-MM-DD -->
<!-- or if none - *No Calls for papers or presentations were submitted this week.* -->

* [**RustWeek 2026**](https://sessionize.com/rustweek-2026/) | CFP closes 2025-12-31 | Utrecht, The Netherlands | 2026-05-19 - 2026-05-20

If you are an event organizer hoping to expand the reach of your event, please submit a link to the website through a [PR to TWiR](https://github.com/rust-lang/this-week-in-rust) or by reaching out on [Bluesky](https://bsky.app/profile/thisweekinrust.bsky.social) or [Mastodon](https://mastodon.social/@thisweekinrust)!

## Updates from the Rust Project

494 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-12-02..2025-12-09

#### Compiler
* [early return on duplicate span lowerings](https://github.com/rust-lang/rust/pull/149060)
* [miri: support `fstat` in linux](https://github.com/rust-lang/miri/pull/4714)

#### Library
* [`c_variadic`: make `VaList` abi-compatible with C](https://github.com/rust-lang/rust/pull/141980)
* [add `#[inline]` to `Layout::is_size_align_valid`](https://github.com/rust-lang/rust/pull/149690)
* [add `Option::into_flat_iter`](https://github.com/rust-lang/rust/pull/148487)
* [also introduce `Peekable::next_if_map_mut` next to `next_if_map`](https://github.com/rust-lang/rust/pull/149520)
* [assume the returned value in `.filter(…).count()`](https://github.com/rust-lang/rust/pull/149495)
* [implement `Allocator` for `&mut A` where `A: Allocator + ?Sized`](https://github.com/rust-lang/rust/pull/146826)
* [implement `Vec::from_fn`](https://github.com/rust-lang/rust/pull/149699)
* [remove initialized-bytes tracking from `BorrowedBuf` and `BorrowedCursor`](https://github.com/rust-lang/rust/pull/148937)
* [stabilize `array_windows`](https://github.com/rust-lang/rust/pull/148814)

#### Cargo
* [`lint`: new `implicit_minimum_version_req` lint](https://github.com/rust-lang/cargo/pull/16321)
* [`timings`: derive concurrency data from unit data](https://github.com/rust-lang/cargo/pull/16350)
* [`lints`: handle lints separately at ws pkg level](https://github.com/rust-lang/cargo/pull/16367)
* [`clean`: Optimize (legacy) clean with multiple -p specifiers](https://github.com/rust-lang/cargo/pull/16264)
* [don't read the config file twice when `$CARGO_HOME` is a symlink](https://github.com/rust-lang/cargo/pull/16325)
* [support for rustdoc mergeable cross-crate info](https://github.com/rust-lang/cargo/pull/16309)

#### Clippy
* [`len_without_is_empty`: allow `is_empty(&self)` with `len(&mut self)`](https://github.com/rust-lang/rust-clippy/pull/16194)
* [fix `map_entry` false positive when it would cause `MutexGuard` to be held across an](https://github.com/rust-lang/rust-clippy/pull/16199)
* [fix `nonstandard_macro_braces` false negative on macros with empty args](https://github.com/rust-lang/rust-clippy/pull/15601)
* [fix `panicking_unwrap` false positive on field access with implicit deref](https://github.com/rust-lang/rust-clippy/pull/16196)
* [fix `tuple_array_conversions` false positive when binded vars are used before conversion](https://github.com/rust-lang/rust-clippy/pull/16197)
* [fix `useless_conversion` wrongly unmangled macros](https://github.com/rust-lang/rust-clippy/pull/16171)
* [fix broken `while_let_on_iterator` suggestion for non-sized types](https://github.com/rust-lang/rust-clippy/pull/16100)

#### Rust-Analyzer
* [add config hide placeholders type hints](https://github.com/rust-lang/rust-analyzer/pull/21203)
* [fix `make::unnamed_param` result a `untyped_param`](https://github.com/rust-lang/rust-analyzer/pull/21044)
* [fix nested expr missing semicolon in incomplete-let](https://github.com/rust-lang/rust-analyzer/pull/21198)
* [fix pub in `enum` variant field for `no_such_field`](https://github.com/rust-lang/rust-analyzer/pull/21221)
* [allow multiple discover operations](https://github.com/rust-lang/rust-analyzer/pull/21164)
* [don't implement sizedness check via `all_field_tys()`](https://github.com/rust-lang/rust-analyzer/pull/21215)
* [fix completion in format strings](https://github.com/rust-lang/rust-analyzer/pull/21210)
* [fixed impl display to show trait generic args](https://github.com/rust-lang/rust-analyzer/pull/21226)
* [more proc-macro-srv proto fixes](https://github.com/rust-lang/rust-analyzer/pull/21195)
* [register `define_opaque` builtin attribute macro](https://github.com/rust-lang/rust-analyzer/pull/21183)
* [resolve const generic param-env panic in type projection](https://github.com/rust-lang/rust-analyzer/pull/21235)
* [skip cfg attributes in macro input attribute stripping](https://github.com/rust-lang/rust-analyzer/pull/21205)
* [no complete unit RetType in resugar async assoc item](https://github.com/rust-lang/rust-analyzer/pull/21222)

### Rust Compiler Performance Triage

Overall result is negative this week, but both main regressions are on track to be addressed. No outstanding changes otherwise.

Triage done by **@panstromek**.
Revision range: [eca9d93f..55495234](https://perf.rust-lang.org/?start=eca9d93f9057f9a48ff691bd65e7daf2f94c1b67&end=554952348a7dd13851f25789f6bb1061f45c4b60&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.4%  | [0.1%, 4.3%]   | 111   |
| Regressions ❌ <br /> (secondary)  | 0.4%  | [0.1%, 2.2%]   | 97    |
| Improvements ✅ <br /> (primary)   | -1.0% | [-1.3%, -0.7%] | 2     |
| Improvements ✅ <br /> (secondary) | -0.2% | [-0.3%, -0.0%] | 9     |
| All ❌✅ (primary)                 | 0.4%  | [-1.3%, 4.3%]  | 113   |


3 Regressions, 2 Improvements, 3 Mixed; 3 of them in rollups
30 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/1e31d44e9db6e283552733052331af16e14e58e2/triage/2025/2025-12-08.md)


### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs entered Final Comment Period this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Warn on codegen attributes on required trait methods](https://github.com/rust-lang/rust/pull/148756)
* [NFC normalize lifetime identifiers](https://github.com/rust-lang/rust/pull/149192)
* [don't normalize where-clauses when checking well-formedness](https://github.com/rust-lang/rust/pull/148477)

[Compiler Team](https://github.com/rust-lang/compiler-team/issues?q=label%3Amajor-change%20%20label%3Afinal-comment-period) [(MCPs only)](https://forge.rust-lang.org/compiler/mcp.html)
* [Destabilise target-spec-json](https://github.com/rust-lang/compiler-team/issues/944)

[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: `#[export_visibility = ...]` attribute](https://github.com/rust-lang/rfcs/pull/3834)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Leadership Council](https://github.com/rust-lang/leadership-council/issues?q=state%3Aopen%20label%3Afinal-comment-period) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-12-10 - 2026-01-07 🦀

### Virtual
* 2025-12-10 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/li5de4ts)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/310728572/)
* 2025-12-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/310002338/)
* 2025-12-17 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://luma.com/6v2rorp3)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-18 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046644/)
* 2025-12-23 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361448/)
* 2025-12-25 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/306046673/)
* 2026-01-01 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin/events/)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/306046646/)
* 2026-01-03 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763888717)
* 2026-01-07 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/events/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/312102790/)

### Asia
* 2025-12-13 | Kuala Lumpur, MY | [Rust Malaysia](https://t.me/rustlangmalaysia)
    * [**FULL: Advent of Code Year End Hack**](https://forms.gle/97jqPeGvpUjMXA8x9)
* 2025-12-14 | Beijing, CN | [Voice AI and Rust Meetup (Rust for AI, lowcoderust.com)](https://www.meetup.com/wasm-rust-meetup/events/)
    * [**GDPS语音AI agent WorkShop+Meetup**](https://www.meetup.com/wasm-rust-meetup/events/312264659/)
* 2025-12-20 | Bangalore, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**December 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/december-2025-rustacean-meetup/)
* 2026-01-06 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv/events/)
    * [**In person Rust January 2026 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/311759516/)

### Europe
* 2025-12-10 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group)
    * [**Rust LDN Talks: Christmas Party with London Gophers & Red Badger**](https://www.meetup.com/rust-london-user-group/events/312264843/)
* 2025-12-10 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 4 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105932/)
* 2025-12-10 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/308944053/)
* 2025-12-11 | Geneva, CH | [PostTenebrasLab](https://www.posttenebraslab.ch/wiki/start)
    * [**Rust Meetup Geneva**](https://www.posttenebraslab.ch/wiki/events/monthly_meeting/rust_meetup)
* 2025-12-15 | Trondheim, NO | [Rust Trondheim](https://www.meetup.com/rust-trondheim)
    * [**Rust Advent of Code Hackathon**](https://www.meetup.com/rust-trondheim/events/312278650/)
* 2025-12-16 | Bergen, NO | [Rust Bergen](https://www.meetup.com/bergen-rust-new-technology/events/)
    * [**Rust Bergen #3 @ Zrch**](https://www.meetup.com/de-de/bergen-rust-new-technology/events/312037597)
* 2025-12-16 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust Hack Night #12: Advent of Code**](https://www.meetup.com/copenhagen-rust-community/events/312295930/)
* 2025-12-16 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/308592258/)
* 2025-12-19 | Lyon, FR | [Rust Lyon](https://www.meetup.com/rust-lyon)
    * [**Rust Lyon Meetup #11**](https://www.meetup.com/rust-lyon/events/312180836/)
* 2026-01-07 | Girona, ES | [Rust Girona](https://lu.ma/rust-girona)
    * [**Rust Girona Hack & Learn 01 2026**](https://luma.com/mdymp686)

### North America
* 2025-12-10 | Chicago, IL, US | [Chicago Rust Meetup](https://www.meetup.com/chicago-rust-meetup)
    * [**Rust Happy Hour**](https://www.meetup.com/chicago-rust-meetup/events/312289655/)
* 2025-12-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Competetive Robotics with Rust**](https://www.meetup.com/utah-rust/events/311613704/)
* 2025-12-11 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/312103517/)
* 2025-12-11 | San Diego, CA, US | [San Diego Rust](https://www.meetup.com/san-diego-rust)
    * [**San Diego Rust December Meetup - Back in person!**](https://www.meetup.com/san-diego-rust/events/312009598/)
* 2025-12-11 | Hybrid (Seattle, WA, US) | [Seattle Rust User Group](https://www.meetup.com/join-srug)
    * [**December, 2025 SRUG (Seattle Rust User Group) Meetup**](https://www.meetup.com/seattle-rust-user-group/events/311351054/)
* 2025-12-13 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Alewife Rust Lunch, Dec 13**](https://www.meetup.com/bostonrust/events/311917267/)
* 2025-12-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/308865807/)
* 2025-12-17 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/312076080/)
* 2025-12-17 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/309926569/)
* 2025-12-17 | Spokane, WA, US | [Spokane Rust](https://www.meetup.com/spokane-rust)
    * [**Year-End Social Meetup w/ Python, Rust, and Others Local User Groups**](https://www.meetup.com/spokane-rust/events/312292668/)
* 2025-12-20 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Back Bay Rust Lunch, Dec 20**](https://www.meetup.com/bostonrust/events/311917280/)
* 2026-01-01 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust/events/)
    * [**Cancelled**](https://www.meetup.com/stl-rust/events/311396047/)

### Oceania
* 2025-12-11 | Brisbane City, QL, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane)
    * [**Rust Meetup Dec 2025**](https://www.meetup.com/rust-brisbane/events/312027415/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

## Jobs

Please see the latest [Who's Hiring thread on r/rust](https://www.reddit.com/r/rust/comments/1ow6s90/official_rrust_whos_hiring_thread_for_jobseekers/)

# Quote of the Week

> \[..\] if a breaking change is going to happen, it’s much better to make lock automatically panic than to make panics silently unlock.

– [Rain on their blog](https://sunshowers.io/posts/on-poisoning)

Thanks to [hkBst](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1735) for the suggestion!

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

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
