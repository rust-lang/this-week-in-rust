Title: This Week in Rust 493
Number: 493
Date: 2023-05-03
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [script-macro](https://github.com/untitaker/script-macro), an experimental way of writing simple proc-macros inline.

Thanks you to [Markus Unterwaditzer](https://users.rust-lang.org/t/crate-of-the-week/2704/1192) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

<!-- CFPs go here, use this format: * [project name - title of issue](link to issue) -->
<!-- * [ - ]() -->

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

390 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2023-04-24..2023-05-01

* [add loongarch64 asm! support](https://github.com/rust-lang/rust/pull/101069)
* [clear response values for overflow in new solver](https://github.com/rust-lang/rust/pull/110614)
* [consider polarity in new solver](https://github.com/rust-lang/rust/pull/110671)
* [do not resolve anonymous lifetimes in consts to be static](https://github.com/rust-lang/rust/pull/110984)
* [fix an ICE in conflict error diagnostics](https://github.com/rust-lang/rust/pull/110957)
* [improve error notes for packed `struct` reference diagnostic](https://github.com/rust-lang/rust/pull/110973)
* [improve niche placement by trying two strategies and picking the better result](https://github.com/rust-lang/rust/pull/108106)
* [include source error for LoadLibraryExW](https://github.com/rust-lang/rust/pull/110932)
* [lower `intrinsics::offset` to `mir::BinOp::Offset`](https://github.com/rust-lang/rust/pull/110822)
* [make sure that some stdlib method signatures aren't accidental refinements](https://github.com/rust-lang/rust/pull/110958)
* [only cache typeck results if it's the typeck root](https://github.com/rust-lang/rust/pull/111026)
* [provide better type hints when a type doesn't support a binary operator](https://github.com/rust-lang/rust/pull/110877)
* [remove `QueryEngine` trait](https://github.com/rust-lang/rust/pull/109611)
* [remove inline const deadcode in typeck](https://github.com/rust-lang/rust/pull/110893)
* [rewrite MemDecoder around pointers not a slice](https://github.com/rust-lang/rust/pull/110634)
* [share `BinOp::Offset` between CTFE and Miri](https://github.com/rust-lang/rust/pull/110944)
* [sprinkle some `#[inline]` in `rustc_data_structures::tagged_ptr`](https://github.com/rust-lang/rust/pull/110814)
* [suggest deref on comparison binop RHS even if type is not Copy](https://github.com/rust-lang/rust/pull/110550)
* [use MIR's `Offset` for pointer `add` too](https://github.com/rust-lang/rust/pull/110837)
* [use `?0` notation for ty/ct/int/float/region vars](https://github.com/rust-lang/rust/pull/110811)
* [miri: fix endianess handling in `eventfd::write`](https://github.com/rust-lang/miri/pull/2857)
* [miri: hide backtrace from stderr files](https://github.com/rust-lang/miri/pull/2854)
* [miri: tree Borrows: improved diagnostics](https://github.com/rust-lang/miri/pull/2828)
* [fix `std` compilation error for wasi+atomics](https://github.com/rust-lang/rust/pull/110587)
* [make `mem::replace` simpler in codegen](https://github.com/rust-lang/rust/pull/111010)
* [add `LazyCell::into_inner`](https://github.com/rust-lang/rust/pull/106152)
* [add shortcut for Grisu3 algorithm](https://github.com/rust-lang/rust/pull/110389)
* [loosen `From<&[T]> for Box<[T]>` bound to `T: Clone`](https://github.com/rust-lang/rust/pull/103406)
* [add support for allocators in `LinkedList`](https://github.com/rust-lang/rust/pull/103093)
* [hashbrown: remove lifetime on `RawIterHash`, for more flexibility & to match `RawIter`](https://github.com/rust-lang/hashbrown/pull/427)
* [cargo: add `-Zmsrv-policy` feature flag](https://github.com/rust-lang/cargo/pull/12043)
* [cargo: apply `[env]` to target info discovery rustc](https://github.com/rust-lang/cargo/pull/12029)
* [cargo: warn instead of error in `cargo package` on empty `readme` or `license-file` in manifest](https://github.com/rust-lang/cargo/pull/12036)
* [rustdoc: add a new lint for broken inline code](https://github.com/rust-lang/rust/pull/105848)
* [rustdoc: catch and don't blow up on impl Trait cycles](https://github.com/rust-lang/rust/pull/110631)
* [clippy: new lint: `manual_while_let_some`](https://github.com/rust-lang/rust-clippy/pull/10647)
* [clippy: add configuration for `semicolon_block` lints](https://github.com/rust-lang/rust-clippy/pull/10656)
* [clippy: don't apply `string_lit_as_bytes` if in macro expansion](https://github.com/rust-lang/rust-clippy/pull/10665)
* [clippy: fix `items_after_test_module`: Ignore imported modules](https://github.com/rust-lang/rust-clippy/pull/10719)
* [rust-analyzer: add hover for closures](https://github.com/rust-lang/rust-analyzer/pull/14690)
* [rust-analyzer: deduplicate crates when extending crate graphs](https://github.com/rust-lang/rust-analyzer/pull/14659)
* [rust-analyzer: don't wavy-underline iterator chains](https://github.com/rust-lang/rust-analyzer/pull/14686)
* [rust-analyzer: fix proc-macro-srv path config not working](https://github.com/rust-lang/rust-analyzer/pull/14671)
* [rust-analyzer: fix restart server button trying to start instead of restart the server](https://github.com/rust-lang/rust-analyzer/pull/14678)
* [rust-analyzer: fix status command panicking when additional LRU caches are set up](https://github.com/rust-lang/rust-analyzer/pull/14654)
* [rust-analyzer: fix vscode workspaces not working properly](https://github.com/rust-lang/rust-analyzer/pull/14651)
* [rust-analyzer: force InitializeParams windows path drives to uppercase](https://github.com/rust-lang/rust-analyzer/pull/14689)
* [rust-analyzer: handle nested types in `unwrap_result_return_type` assist](https://github.com/rust-lang/rust-analyzer/pull/14667)
* [rust-analyzer: handle dev-dependency cycles](https://github.com/rust-lang/rust-analyzer/pull/14475)
* [rust-analyzer: remove proc-macro server command from the rust-analyzer binary](https://github.com/rust-lang/rust-analyzer/pull/14658)

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

Rusty Events between 2023-05-03 - 2023-05-31 🦀

### Virtual

* 2023-04-26 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust-friendly websites and web apps**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292559177/)
* 2023-04-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Testing Tock, how unit tests in Rust improve and teach**](https://www.meetup.com/charlottesville-rust-meetup/events/292193436/)
* 2023-04-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community)
    * [**Rust meetup #35 at Google Cloud**](https://www.meetup.com/copenhagen-rust-community/events/292424926/)
* 2023-04-29 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 3: Protohackers Exercises Mob Coding (as far as we get)**](https://www.meetup.com/rust-noris/events/292149688/)
* 2023-05-02 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/lsjbbtyfchbdb/)
* 2023-05-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsyfchbfb/)
* 2023-05-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsyfchbmb/)
* 2023-05-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/gmkpctyfchbpb/)
* 2023-05-13 | Virtual | [Rust GameDev](https://discord.gg/yNtPTb2)
    * [**Rust GameDev Monthly Meetup**](https://discord.gg/yNtPTb2)
* 2023-05-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/jkxsctyfchbvb/)
* 2023-05-17 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust Atomics and Locks Book Club Chapter 2**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/292847157/)
* 2023-05-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/vancouver-rust/events/lqkkctyfchbwb/)

### Asia

* 2023-05-06 | Kyoto, JP | [Kansai Rust](https://www.meetup.com/kansai-rust/)
   * [**Rust Talk: Vec, arrays, and slices**](https://www.meetup.com/kansai-rust/events/293010553/)

### Europe

* 2023-04-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust Hack & Learn April 2023**](https://www.meetup.com/rust-london-user-group/events/292729308/)
* 2023-04-27 | Bordeaux, FR | [DedoTalk](https://www.meetup.com/dedotalk/)
    * [**#2 DedoTalk 🎙️ : Comment tester son code Rust?**](https://www.meetup.com/dedotalk/events/292842782/)
* 2023-04-27 | Vienna, AT | [Rust Vienna](https://www.meetup.com/rust-vienna)
    * [**Rust Vienna - April - Hosted by Sentry**](https://www.meetup.com/rust-vienna/events/292751465/)
* 2023-05-02 | Amsterdam, NL | [Rust Developers Amsterdam Group](https://www.meetup.com/rust-amsterdam-group/)
    * [**Fiberplane Rust Workshop**](https://www.meetup.com/rust-amsterdam-group/events/292297784/)
* 2023-05-10 | Amsterdam, NL | [RustNL](https://rustnl.org/)
    * [**RustNL 2023**](https://2023.rustnl.org/)
* 2023-05-19 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/rust-community-stuttgart)
    * [**OnSite Meeting**](https://www.meetup.com/rust-community-stuttgart/events/pdhvctyfchbzb/)

### North America

* 2023-04-29 | Durham, NC, US | [Triangle Rust](https://www.meetup.com/triangle-rust/)
    * [**Rust Social / Coffee Chat at Boxyard RTP**](https://www.meetup.com/triangle-rust/events/292833711/)
* 2023-05-03 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/293007744/)
* 2023-05-11 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Upcoming Event**](https://www.meetup.com/utah-rust/events/rrwbctyfchbpb/)
* 2023-05-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfchbvb/)

### Oceania

* 2023-04-27 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**April Meetup**](https://www.meetup.com/rust-brisbane/events/292965270/)
* 2023-05-03 | Christchurch, NZ | [Christchurch Rust Meetup Group](https://www.meetup.com/christchurch-rustlang-meetup-group/)
    * [**Christchurch Rust meetup meeting**](https://www.meetup.com/christchurch-rustlang-meetup-group/events/292993051/)

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

> Since it hasn't been said before, there is an important distinction that needs to be addressed. For anyone who has been doing embedded work for any length of time and hasn't yet been exposed to Rust, the only thing that can really be said is that the language is entirely unlike everything you've experienced before. There is just nothing comparable, and the only way to rationalize questions like *why use Rust at all* is to put some honest effort into learning and using it.
>
> Hearing things like "it's a bit like C++ except it's memory safe and thread safe, and it's actually practical to build kernels with it" will not sound convincing. You have to see it to believe it.
>
> It's as if you've spent an entire career writing assembly, and one day you hear something or other about a brand-new programming language claiming to be a "portable assembler" called C. It sounds too good to be true. And then the years pass, and all of the mystery and disbelief gives way to obviousness and precision engineering. That's sort of how it is when going from C to Rust.

– [Jay Oster](https://users.rust-lang.org/t/concerns-about-embedded-real-time-linux-using-rust/91416/12)

Thanks to [Michael Bryan](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1411) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
