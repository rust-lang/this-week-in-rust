Title: This Week in Rust 600
Number: 600
Date: 2025-05-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at
[@thisweekinrust.bsky.social](https://bsky.app/profile/thisweekinrust.bsky.social) on Bluesky or
[@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or
[send us a pull request](https://github.com/rust-lang/this-week-in-rust).
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

### Project/Tooling Updates

### Observations/Thoughts
* [Evolution of Rust compiler errors](https://kobzol.github.io/rust/rustc/2025/05/16/evolution-of-rustc-errors.html)
* [For your eyes only](https://bitfieldconsulting.com/posts/for-your-eyes-only)
* [Disable debuginfo to improve Rust compile times](https://kobzol.github.io/rust/rustc/2025/05/20/disable-debuginfo-to-improve-rust-compile-times.html)

### Rust Walkthroughs

* [video] [Build with Naz : Rust, Memory performance & latency - locality, access, allocate, cache lines](https://www.youtube.com/watch?v=ywkEmwkX0Lc)

### Research

### Miscellaneous
* [Scanner- The Team Accelerating Log Analysis With Rust](https://filtra.io/rust/interviews/scanner-may-25)

## Crate of the Week

This week's crate is [makepad](https://makepad.nl), an in development shader based live designable OSS UI-Framework.

Thanks to [crazust](https://users.rust-lang.org/t/crate-of-the-week/2704/1435) for the suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Calls for Testing
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.

If you are a feature implementer and would like your RFC to appear in this list, add a
`call-for-testing` label to your RFC along with a comment providing testing instructions and/or
guidance on which aspect(s) of the feature need testing.

* *No calls for testing were issued this week by [Rust](https://github.com/rust-lang/rust/labels/call-for-testing),
  [Rust language RFCs](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing) or
  [Rustup](https://github.com/rust-lang/rustup/labels/call-for-testing).*

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

353 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2025-05-13..2025-05-20

#### Compiler

* [improve `dangerous_implicit_aurorefs` diagnostic output](https://github.com/rust-lang/rust/pull/140768)
* [improve ternary operator recovery](https://github.com/rust-lang/rust/pull/141003)
* [perf: fast path for `register_region_obligation`](https://github.com/rust-lang/rust/pull/141129)

#### Library

* [add `std::io::Seek` instance for `std::io::Take`](https://github.com/rust-lang/rust/pull/138023)
* [optimize `ToString` implementation for integers](https://github.com/rust-lang/rust/pull/136264)
* [stop using TLS in signal handler](https://github.com/rust-lang/rust/pull/140628)
* [stabilize `#![feature(non_null_from_ref)]`](https://github.com/rust-lang/rust/pull/140511)
* [stabilize the avx512 target features](https://github.com/rust-lang/rust/pull/138940)

#### Cargo

* [cargo: allow configuring arbitrary codegen backends](https://github.com/rust-lang/cargo/pull/15562)
* [cargo: feat: skip `publish=false` pkg when publishing entire workspace](https://github.com/rust-lang/cargo/pull/15525)
* [cargo: stabilize doctest-xcompile](https://github.com/rust-lang/cargo/pull/15462)

#### Rustdoc

#### Clippy

* [`comparison_chain`: do not lint on 2 blocks expression](https://github.com/rust-lang/rust-clippy/pull/14811)
* [`empty_struct_with_brackets`: do not lint code coming from macro expansion](https://github.com/rust-lang/rust-clippy/pull/14623)
* [`excessive_precision`: Fix false positive when exponent has leading zero](https://github.com/rust-lang/rust-clippy/pull/14824)
* [`match_same_arms`, `ifs_same_cond`: lint once per same arm/condition](https://github.com/rust-lang/rust-clippy/pull/14637)
* [`needless_match`: do not pretend that `return` is not significant in an expression](https://github.com/rust-lang/rust-clippy/pull/14757)
* [`unnecessary_wraps`: do not include the whole body in the lint span](https://github.com/rust-lang/rust-clippy/pull/14777)
* [add new `useless_concat` lint](https://github.com/rust-lang/rust-clippy/pull/13829)
* [add the `allow_exact_repetitions` option to the `module_name_repetititions` lint](https://github.com/rust-lang/rust-clippy/pull/14261)
* [check if dropping an expression may have indirect side-effects](https://github.com/rust-lang/rust-clippy/pull/14594)
* [`useless_as_ref`: do not call `TyCtxt::type_of()` on a trait](https://github.com/rust-lang/rust-clippy/pull/14830)
* [fix ICE while computing type layout](https://github.com/rust-lang/rust-clippy/pull/14837)
* [fix false positive of `useless_conversion` when using `.into_iter().any()`](https://github.com/rust-lang/rust-clippy/pull/14800)
* [fix: `unnecessary_to_owned` false positive when map key is a reference](https://github.com/rust-lang/rust-clippy/pull/14834)
* [make lint span smaller for needless return](https://github.com/rust-lang/rust-clippy/pull/14790)
* [post `non_std_lazy_statics` type warnings onto the right node](https://github.com/rust-lang/rust-clippy/pull/14740)
* [reenable linting on UFCS `deref` calls](https://github.com/rust-lang/rust-clippy/pull/14808)
* [rewrite `non_copy_const`](https://github.com/rust-lang/rust-clippy/pull/13207)
* [use interned symbols instead of strings in more places](https://github.com/rust-lang/rust-clippy/pull/14840)

#### Rust-Analyzer

* [debounce workspace fetching for workspace structure changes](https://github.com/rust-lang/rust-analyzer/pull/19814)
* [highlight unsafe operations](https://github.com/rust-lang/rust-analyzer/pull/19687)
* [don't allow duplicate crates in the `all_crates` list](https://github.com/rust-lang/rust-analyzer/pull/19794)
* [improve asm support](https://github.com/rust-lang/rust-analyzer/pull/19801)
* [removing all unused imports removes used imports for imports used for Derive macros](https://github.com/rust-lang/rust-analyzer/pull/19793)
* [request cancellation while processing changed files](https://github.com/rust-lang/rust-analyzer/pull/19757)

### Rust Compiler Performance Triage

A relatively quiet week, likely caused by not that many PRs being merged as many contributors
were at RustWeek and the All Hands event. There were several nice wins in trait solving;
the benchmark suite now contains benchmarks that use the new trait solver, so we can now
focus on optimizing its performance in the near future.

Triage done by **@kobzol**.

Revision range: [718ddf66..59372f2c](https://perf.rust-lang.org/?start=718ddf660e6a1802c39b4962cf7eaa4db57025ef&end=59372f2c81ba74554d9a71b12a4ed7f29adb33a2&absolute=false&stat=instructions%3Au)

**Summary**:
| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 0.6%  | [0.1%, 1.8%]   | 25    |
| Regressions ❌ <br /> (secondary)  | 0.9%  | [0.1%, 3.1%]   | 23    |
| Improvements ✅ <br /> (primary)   | -0.3% | [-0.6%, -0.1%] | 33    |
| Improvements ✅ <br /> (secondary) | -2.2% | [-9.2%, -0.1%] | 26    |
| All ❌✅ (primary)                 | 0.1%  | [-0.6%, 1.8%]  | 58    |

2 Regressions, 5 Improvements, 3 Mixed; 3 of them in rollups
41 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/07f51987e0e9ca879a542ff365a4bac82d9c66f6/triage/2025-05-20.md).

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### Tracking Issues & PRs
##### [Rust](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [Tracking Issue for AVX512 intrinsics](https://github.com/rust-lang/rust/issues/111137)
* [rustdoc: on mobile, make the sidebar full width and linewrap](https://github.com/rust-lang/rust/pull/139831)

*No Items entered Final Comment Period this week for
[Cargo](https://github.com/rust-lang/cargo/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Rust RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period),
[Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc),
[Language Team](https://github.com/rust-lang/lang-team/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc+) or
[Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc).*

Let us know if you would like your PRs, Tracking Issues or RFCs to be tracked as a part of this list.

#### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2025-05-21 - 2025-06-18 🦀

### Virtual
* 2025-05-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Linking**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-22 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820302)
* 2025-05-22 | Virtual (Girona, ES) | [Rust Girona](https://lu.ma/rust-girona)
    * [**Sessió setmanal de codificació / Weekly coding session**](https://lu.ma/8zabmc3w)
* 2025-05-25 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307668643)
* 2025-05-27 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Fourth Tuesday**](https://www.meetup.com/dallasrust/events/305361435)
* 2025-05-27 | Virtual (Tel Aviv, IL) | [Code Mavens 🦀 - 🐍 - 🐪](https://www.meetup.com/code-mavens/)
    * [**Rust at Work - conversation with Eli Shalom & Igal Tabachnik of Eureka Labs**](https://www.meetup.com/code-mavens/events/307673680/)
* 2025-05-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/300820285)
* 2025-05-29 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**שיחה חופשית ווירטואלית על ראסט**](https://www.meetup.com/rust-tlv/events/307730629)
* 2025-06-01 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/307795210)
* 2025-06-03 | Virtual (Tel Aviv-yafo, IL) | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**Why Rust? למה ראסט? -**](https://www.meetup.com/rust-tlv/events/307801358)
* 2025-06-04 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/302031665)
* 2025-06-05 | Virtual (Berlin, DE) | [Rust Berlin](https://www.meetup.com/rust-berlin)
    * [**Rust Hack and Learn**](https://www.meetup.com/rust-berlin/events/300820301)
* 2025-06-07 | Virtual (Kampala, UG) | [Rust Circle Meetup](https://www.eventbrite.com/o/rust-circle-kampala-65249289033)
    * [**Rust Circle Meetup**](https://www.eventbrite.com/e/rust-circle-meetup-tickets-628763176587)
* 2025-06-08 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjblb)
* 2025-06-10 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/305020417)
* 2025-06-10 | Virtual (London, UK) | [Women in Rust](https://www.meetup.com/women-in-rust)
    * [**👋 Community Catch Up**](https://www.meetup.com/women-in-rust/events/307560326)
* 2025-06-12 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup)
    * [**Meet, swap, and learn!**](https://www.meetup.com/charlottesville-rust-meetup/events/307767236)
* 2025-06-15 | Virtual (Dallas, TX, US) | [Dallas Rust User Meetup](https://www.meetup.com/dallasrust)
    * [**Rust Readers Discord Discussion: Async Rust**](https://www.meetup.com/dallasrust/events/kvqfrtyhcjbtb)
* 2025-06-17 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/305170853)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)

### Asia
* 2025-05-24 | Bangalore/Bengaluru, IN | [Rust Bangalore](https://hasgeek.com/rustbangalore)
    * [**May 2025 Rustacean meetup**](https://hasgeek.com/rustbangalore/may-2025-rustacean-meetup/)
* 2025-06-08 | Tel Aviv-yafo, IL | [Rust 🦀 TLV](https://www.meetup.com/rust-tlv)
    * [**In person Rust June 2025 at AWS in Tel Aviv**](https://www.meetup.com/rust-tlv/events/306414888)

### Europe
* 2025-05-22 | Augsburg, DE | [Rust Augsburg](https://rust-augsburg.github.io/meetup/introduction.html)
    * [**Rust meetup #13:A Practical Guide to Telemetry in Rust**](https://rust-augsburg.github.io/meetup/Meetup_13.html)
* 2025-05-22 | Bern, CH | [Rust Bern](https://www.meetup.com/rust-bern)
    * [**2025 Rust Talks Bern #3 @zentroom**](https://www.meetup.com/rust-bern/events/307559606)
* 2025-05-22 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris)
    * [**Rust meetup #77**](https://www.meetup.com/rust-paris/events/307565141)
* 2025-05-22 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Rust Meetup @UXStream**](https://www.meetup.com/stockholm-rust/events/307653223)
* 2025-05-27 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel)
    * [**Rust Meetup #11 @ Letsboot Basel**](https://www.meetup.com/rust-basel/events/307567083)
* 2025-05-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - May at Bitcredit 🦀**](https://www.meetup.com/rust-vienna/events/307825439)
* 2025-05-28 | Frankfurt, DE | [Rust Rhein-Main](https://www.meetup.com/rust-rhein-main)
    * [**Marcel Koch and the (un)holy Trinity of Flutter, Rust and C**](https://www.meetup.com/rust-rhein-main/events/307836400)
* 2025-05-29 | Oslo, NO | [Rust Oslo](https://www.meetup.com/rust-oslo)
    * [**Rust Hack'n'Learn at Kampen Bistro**](https://www.meetup.com/rust-oslo/events/305809683)
* 2025-05-31 | Stockholm, SE | [Stockholm Rust](https://www.meetup.com/stockholm-rust)
    * [**Ferris' Fika Forum #12**](https://www.meetup.com/stockholm-rust/events/307766469)
* 2025-06-04 | Ghent, BE | [Systems Programming Ghent](https://www.sysghent.be/)
    * [**Grow smarter with embedded Rust**](https://www.meetup.com/systems-programming-ghent/events/307269551)
* 2025-06-04 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/events/)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-04 | Oxford, UK | [Oxford Rust Meetup Group](https://www.meetup.com/oxford-rust-meetup-group)
    * [**Oxford Rust and C++ social**](https://www.meetup.com/oxford-rust-meetup-group/events/307673867)
* 2025-06-05 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich)
    * [**Rust Munich 2025 / 2 - Hacking Evening**](https://www.meetup.com/rust-munich/events/307105443)
* 2025-06-11 | Reading, UK | [Reading Rust Workshop](https://www.meetup.com/reading-rust-workshop)
    * [**Reading Rust Meetup**](https://www.meetup.com/reading-rust-workshop/events/305045448)
* 2025-06-17 | Leipzig, SN, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig)
    * [**Topic TBD**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/305741641)

### North America
* 2025-05-21 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Linking**](https://www.meetup.com/vancouver-rust/events/307184332)
* 2025-05-23 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Seaport Rust Lunch, May 23**](https://www.meetup.com/bostonrust/events/307936039)
* 2025-05-28 | Albuquerque, NM, US | [At Ideas and Coffee](https://www.meetup.com/ideas-and-coffee)
    * [**Intro Level Rust Get-together**](https://www.meetup.com/ideas-and-coffee/events/307645653)
* 2025-05-28 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/307720951)
* 2025-05-29 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl)
    * [**Rust-Atl**](https://www.meetup.com/rust-atl/events/307152367)
* 2025-05-29 | Mountain View, CA, US | [Hacker Dojo](https://www.meetup.com/hackerdojo/events/)
    * [**RUST MEETUP at HACKER DOJO**](https://www.meetup.com/hackerdojo/events/307498676)
* 2025-05-31 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Harvard Square Rust Lunch, May 31**](https://www.meetup.com/bostonrust/events/307936097)
* 2025-06-05 | Saint Louis, MO, US | [STL Rust](https://www.meetup.com/stl-rust)
    * [**Leptos web framework**](https://www.meetup.com/stl-rust/events/305534867)
* 2025-06-08 | Boston, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust)
    * [**Boston University Rust Lunch, June 8**](https://www.meetup.com/bostonrust/events/307936165)
* 2025-06-17 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/307595021)
* 2025-06-18 | Hybrid (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/307730493)

### Oceania
* 2025-06-16 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group)
    * [**Christchurch Rust Meetup**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/307808896)

### South America
* 2025-05-31 | São Paulo, BR | [Rust São Paulo Meetup](https://www.meetup.com/rust-sao-paulo-meetup)
    * [**Encontro do Rust-SP na WillBank**](https://www.meetup.com/rust-sao-paulo-meetup/events/307308851)
* 2025-06-04 | Montevideo, DE, UY | [Rust Meetup Uruguay](https://www.meetup.com/rust-uruguay)
    * [**Primera meetup de Rust de 2025!**](https://www.meetup.com/rust-uruguay/events/307341567)

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

> `/proc/self/mem` is outside the scope of rust's safety guarantees. otherwise this would open a can of worms that could not be closed except by forbidding debuggers from poking memory or marking every impure function as unsafe.
>
> like, what if you invoke gdb to poke memory? what if you modify .bash_profile to poke memory? what if you send an http request to a hypervisor to poke memory? what if you run a spin loop, and the noise of the fans whirring up activates a beam of ionizing radiation pointed directly at the CPU? what if opening the disk drive makes the computer fall off a cliff?

– [binarycat on rust-internals](https://internals.rust-lang.org/t/how-does-inline-assembly-and-the-physical-machine-fit-into-the-abstract-machine-model-or-exist-outside-of-it/22545/41)

Thanks to [Chayim Refael Friedman](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1687) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [U007D](https://github.com/U007D), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez), [bdillo](https://github.com/bdillo)*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
