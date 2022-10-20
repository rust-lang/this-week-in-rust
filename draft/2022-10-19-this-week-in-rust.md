Title: This Week in Rust 465
Number: 465
Date: 2022-10-19
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

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [HyperQueue](https://github.com/It4innovations/hyperqueue), a runtime for ergonomic execution of programs on a distributed cluster.

Thanks to [Jakub Beránek](https://users.rust-lang.org/t/crate-of-the-week/2704/1113) for the self-suggestion!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

388 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-10-10..2022-10-17

* [support casting boxes to dyn*](https://github.com/rust-lang/rust/pull/102641)
* [support default-body trait functions with return-position `impl Trait` in traits](https://github.com/rust-lang/rust/pull/101679)
* [mark derived `StructuralEq` as automatically derived](https://github.com/rust-lang/rust/pull/103089)
* [allow compiling the `wasm32-wasi` std library with atomics](https://github.com/rust-lang/rust/pull/102372)
* [detect and reject out-of-range integers in format string literals](https://github.com/rust-lang/rust/pull/102529)
* [drop temporaries created in a condition, even if it's a let chain](https://github.com/rust-lang/rust/pull/102998)
* [fix `let` keyword removal suggestion in structs](https://github.com/rust-lang/rust/pull/102927)
* [make `dyn*` casts into a coercion, allow `dyn*` upcasting](https://github.com/rust-lang/rust/pull/101832)
* [make `overlapping_impls` not generic](https://github.com/rust-lang/rust/pull/102931)
* [point out incompatible closure bounds](https://github.com/rust-lang/rust/pull/101360)
* [populate effective visibilities in `rustc_resolve`](https://github.com/rust-lang/rust/pull/102026)
* [print return-position `impl Trait` in trait verbosely if `-Zverbose`](https://github.com/rust-lang/rust/pull/102904)
* [add suggestion to the "missing native library" error](https://github.com/rust-lang/rust/pull/103000)
* [suggest `==` to the first expr which has `ExprKind::Assign` kind](https://github.com/rust-lang/rust/pull/102765)
* [suggest candidates for unresolved import](https://github.com/rust-lang/rust/pull/102876)
* [suggest parentheses for possible range method calling](https://github.com/rust-lang/rust/pull/102454)
* [suppress irrefutable let patterns lint for prefixes in match guards](https://github.com/rust-lang/rust/pull/103031)
* [unify `tcx.constness` query and param env constness checks](https://github.com/rust-lang/rust/pull/102830)
* [remove type traversal for mir constants](https://github.com/rust-lang/rust/pull/102355)
* [scoped threads: pass closure through MaybeUninit to avoid invalid dangling references](https://github.com/rust-lang/rust/pull/102589)
* [never panic in `thread::park` and `thread::park_timeout`](https://github.com/rust-lang/rust/pull/102412)
* [use semaphores for thread parking on Apple platforms](https://github.com/rust-lang/rust/pull/102773)
* [nicer errors from `assert_unsafe_precondition`](https://github.com/rust-lang/rust/pull/102732)
* [optimize TLS on Windows](https://github.com/rust-lang/rust/pull/102655)
* [stabilize `map_first_last`](https://github.com/rust-lang/rust/pull/101727)
* [constify `Location` methods](https://github.com/rust-lang/rust/pull/101030)
* [add `MaybeUninit` array transpose `From` impls](https://github.com/rust-lang/rust/pull/102023)
* [add `Box<[T; N]>: TryFrom<Vec<T>>`](https://github.com/rust-lang/rust/pull/101837)
* [add `IsTerminal` trait to determine if a descriptor or handle is a terminal](https://github.com/rust-lang/rust/pull/98033)
* [add `is_empty()` method to `core::ffi::CStr`](https://github.com/rust-lang/rust/pull/102445)
* [panic for invalid arguments of `{integer primitive}::ilog{,2,10}` in all modes](https://github.com/rust-lang/rust/pull/102578)
* [impl `AsFd` and `AsRawFd` for `io::`{`Stdin`, `Stdout`, `Stderr`}, not the sys versions](https://github.com/rust-lang/rust/pull/102847)
* [prevent UB in child process after calling `libc::fork`](https://github.com/rust-lang/rust/pull/102460)
* [fix `Duration::{try_,}from_secs_f{32,64}(-0.0)`](https://github.com/rust-lang/rust/pull/103059)
* [SIMD: mark more mask functions inline](https://github.com/rust-lang/portable-simd/pull/309)
* [futures: fix soundness hole in join macros](https://github.com/rust-lang/futures-rs/pull/2649)
* [cargo: fix deadlock when build scripts are waiting for input on stdin](https://github.com/rust-lang/cargo/pull/11205)
* [cargo: support 'publish.timeout' config behind '-Zpublish-timeout'](https://github.com/rust-lang/cargo/pull/11230)
* [rustdoc: change default level of `invalid_html_tags` to warning and stabilize it](https://github.com/rust-lang/rust/pull/101720)
* [clippy: add `as_ptr_cast_mut` lint](https://github.com/rust-lang/rust-clippy/pull/9572)
* [clippy: add `unused_format_specs` lint](https://github.com/rust-lang/rust-clippy/pull/9637)
* [clippy: add a suggestion and a note about orphan rules for `from_over_into`](https://github.com/rust-lang/rust-clippy/pull/9649)
* [clippy: add new lint `partial_pub_fields`](https://github.com/rust-lang/rust-clippy/pull/9658)
* [clippy: change `uninlined_format_args` into a style lint](https://github.com/rust-lang/rust-clippy/pull/9600)
* [clippy: don't lint `ptr_arg` when used as an incompatible trait object](https://github.com/rust-lang/rust-clippy/pull/9645)
* [clippy: fix `to_string_in_format_args` in parens](https://github.com/rust-lang/rust-clippy/pull/9590)
* [clippy: don't lint `default_numeric_fallback` on constants](https://github.com/rust-lang/rust-clippy/pull/9636)
* [clippy: don't lint `unnecessary_cast` on negative hexadecimal literals when cast as floats](https://github.com/rust-lang/rust-clippy/pull/9609)
* [clippy: `zero_prefixed_literal`: Do not advise to use octal form if not possible](https://github.com/rust-lang/rust-clippy/pull/9652)
* [clippy: add `cast-nan-to-int` lint](https://github.com/rust-lang/rust-clippy/pull/9617)
* [clippy: fix `box-default` linting `no_std` non-boxes](https://github.com/rust-lang/rust-clippy/pull/9655)
* [clippy: fix: `uninlined_format_args` shouldn't inline panic! before 2021 edition](https://github.com/rust-lang/rust-clippy/pull/9605)
* [rust-analyzer: migrate assists to format args captures, part 2](https://github.com/rust-lang/rust-analyzer/pull/13399)
* [rust-analyzer: diagnose some incorrect usages of the question mark operator](https://github.com/rust-lang/rust-analyzer/pull/13354)
* [rust-analyzer: fix formatting requests hanging when r-a is still starting](https://github.com/rust-lang/rust-analyzer/pull/13428)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* * *No Tracking Issues or PRs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Add RFC for calling default trait methods](https://github.com/rust-lang/rfcs/pull/3329)
* [new] [Add lang-team advisors team](https://github.com/rust-lang/rfcs/pull/3327)

## Upcoming Events

Rusty Events between 2022-10-19 - 2022-11-16 🦀

### Virtual

* 2022-10-12 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust and C++ Cardiff Virtual Meet**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/288154536/)
* 2022-10-12 | Virtual (Erlangen, DE) | [Rust Franken](https://www.meetup.com/rust-nerf/)
    * [**Rust Franken Meetup #4**](https://www.meetup.com/rust-nerf/events/288723552/)
* 2022-10-12 | Virtual (San Francisco, CA, US / Redmond, WA, US / New York, NY, US / Toronto, CA / London, UK) | [Microsoft Reactor San Francisco](https://www.meetup.com/microsoft-reactor-san-francisco/)
    * [**Getting Started with Rust: Building Rust Projects**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475796/) | [**Redmond Mirror**](https://www.meetup.com/microsoft-reactor-redmond/events/288475797/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475794/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475799/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475801/)
* 2022-10-13 | Virtual (Berlin, DE) | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-15 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Deep Dive Session 2 (CuteCopter): Reverse Engineering a tiny drone**](https://www.meetup.com/rust-noris/events/287347851/)
* 2022-10-18 | Virtual (Myrtle Point, OR, US) | [#EveryoneCanContribute Cafe](https://www.meetup.com/everyonecancontribute-cafe/)
    * [**Cloud Native: KubeCon NA - expectations, learnings, etc. -- incl. WebAssembly and Containers --  OpenTelemetry + Rust**](https://www.meetup.com/everyonecancontribute-cafe/events/287161943/)
* 2022-10-18 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful—Impractical Rust: The HATETRIS World Record**](https://www.meetup.com/rustdc/events/vdhxgsydcnbxb/)
* 2022-10-19 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydcnbqb/)
* 2022-10-19 | Virtual (Chennai, IN) | [Techceleration at Toyota Connected](https://www.meetup.com/techceleration/)
    * [**Techceleration's! Let's Talk Tech! Rust | BreakTheCode Contest - 14th Edition**](https://www.meetup.com/techceleration/events/288942414/)
* 2022-10-19 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Rapid Prototyping in Rust: Write fast like Python; Run fast like C**](https://www.meetup.com/vancouver-rust/events/288641106/)
* 2022-10-19 | Virtual | [Boston NoSQL Database Group (ScyllaDB)](https://www.meetup.com/meetup-group-boston-nosql-database-group/)
    * [**p99 Conf: All Things Performance (including talks on Rust) - Free**](https://www.meetup.com/meetup-group-boston-nosql-database-group/events/288464419/) | [**Official conference page**](https://www.p99conf.io)
* 2022-10-20 | Virtual (México City, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Graphul, un web framework escrito en Rust**](https://www.meetup.com/rust-mx/events/289023645/)
* 2022-10-20 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydcnbbc/)
* 2022-10-25 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/288925790/)
* 2022-10-25 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydcnbhc/)
* 2022-10-26 | Virtual (Redmond, WA, US / New York, NY, US / Toronto, CA / Stockholm, SE) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Your First Rust Project: Rust Basics**](https://www.meetup.com/microsoft-reactor-redmond/events/288475815/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475818/) | [**Stockholm Mirror**](https://www.meetup.com/microsoft-reactor-stockholm/events/288475819/)
* 2022-10-27 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Using Applicative Functors to parse command line options**](https://www.meetup.com/charlottesville-rust-meetup/events/288867237/)
* 2022-10-27 | Karlsruhe, DE | [The Karlsruhe Functional Programmers Meetup Group](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/)
    * [**Stammtisch (gemeinsam mit der C++ UG KA) (various topics, from C++ to Rust...)**](https://www.meetup.com/the-karlsruhe-functional-programmers-meetup-group/events/288972651/)
* 2022-11-01 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/hlgvxsydcpbcb/)
* 2022-11-02 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydcpbdb/)
* 2022-11-02 | Virtual (Redmond, WA, US / San Francisco, SF, US / New York, NY, US / Toronto, CA / London, UK) | [Microsoft Reactor Redmond](https://www.meetup.com/microsoft-reactor-redmond/)
    * [**Getting Started with Rust: From Java Dev to Rust Developer**](https://www.meetup.com/microsoft-reactor-redmond/events/288475833/) | [**San Francisco Mirror**](https://www.meetup.com/microsoft-reactor-san-francisco/events/288475838/) | [**New York Mirror**](https://www.meetup.com/microsoft-reactor-new-york/events/288475839/) | [**Toronto Mirror**](https://www.meetup.com/microsoft-reactor-toronto/events/288475836/) | [**London Mirror**](https://www.meetup.com/microsoft-reactor-london/events/288475832/) 
* 2022-11-02 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/)
    * [**Rust and C++ Cardiff Virtual Meet**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/289052285/)
* 2022-11-08 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydcpblb/)
* 2022-11-08 | Virtual (Stockholm, SE) | [Func Prog Sweden](https://www.meetup.com/func-prog-sweden/)
    * [**Tenth Func Prog Sweden MeetUp 2022 – Online (with "Ready for Rust" by Erik Dörnenburg)**](https://www.meetup.com/func-prog-sweden/events/288323896/)

### Europe

* 2022-10-12 | Berlin, DE | [Rust Berlin](https://www.meetup.com/rust-berlin/)
    * [**Rust and Tell - EuroRust B-Sides**](https://www.meetup.com/rust-berlin/events/288175448/)
* 2022-10-12 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Iterators in Rust**](https://www.meetup.com/rustcologne/events/288977470/)
* 2022-10-13 | Berlin, DE + Virtual | [EuroRust](https://eurorust.eu/)
    * [**EuroRust (Oct 13-14)**](https://eurorust.eu/schedule)
* 2022-10-13 | Roma, IT | [Rust Roma](https://www.meetup.com/rust-roma/)
    * [**GraphQL server with Rust #Aperitech**](https://www.meetup.com/rust-roma/events/289022361/)
* 2022-10-20 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London x JFrog SwampUP After Party**](https://www.meetup.com/rust-london-user-group/events/289027990/)
* 2022-10-25 | Paris, FR | [Rust Paris](https://www.meetup.com/rust-paris/)
    * [**Rust Paris meetup #53**](https://www.meetup.com/rust-paris/events/288735204/)
* 2022-10-26 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks October 2022: Host by Amazon Prime Video**](https://www.meetup.com/rust-london-user-group/events/289023932/)
* 2022-10-27 | København, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #30**](https://www.meetup.com/copenhagen-rust-meetup-group/events/288179125/)
    
### North America

* 2022-10-13 | Columbus, OH, US | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydcnbrb/)
* 2022-10-18 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydcnbxb/)
* 2022-10-20 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)
    * [**Anyhow ? Turbofish ::<> / HTTP calls and errors in Rust.**](https://www.meetup.com/rust-nyc/events/288756215/)
* 2022-10-20 | New York, NY, US | [Cloud Native New York](https://www.meetup.com/cloud-native-new-york/)
    * [**Cloud-native Search Engine for Log Management and Analytics.**](https://www.meetup.com/cloud-native-new-york/events/288818963/)
* 2022-10-25 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**Rust DHCP**](https://www.meetup.com/rust-toronto/events/288589539/)
* 2022-10-27 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Bevy Crash Course with Nathan and Food!**](https://www.meetup.com/utah-rust/events/dsbpxsydcnbkc/)

### Oceania

* 2022-10-20 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**October Meetup**](https://www.meetup.com/rust-brisbane/events/289041763/)
* 2022-10-20 | Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Tune Up Edition: software engineering management**](https://www.meetup.com/rust-wellington/events/288738684/)
* 2022-11-09 | Sydney, NSW, AU | [Rust Sydney](https://www.meetup.com/rust-sydney/)
    * [**RustAU Sydney - Last physical for 2022 !**](https://www.meetup.com/rust-sydney/events/289061840/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org


<!--

Rust Jobs:

TWiR has stopped featuring individual job postings. You can read more about this change here:

https://github.com/rust-lang/this-week-in-rust/issues/3412

-->

## Jobs

Please see the latest [Who's Hiring thread on r/rust](INSERT_LINK_HERE)

# Quote of the Week

> I think it's worth noting that the fact that this program fails to compile whereas the analogous Python runs but gives the wrong answer is *exactly what Rust's ownership and borrowing system is about*.

– [Kevin Reid on rust-users](https://users.rust-lang.org/t/capturing-a-copy-of-a-local-variable-for-a-lambda/82522/5)

Thanks to [Kill The Mule](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1310) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
