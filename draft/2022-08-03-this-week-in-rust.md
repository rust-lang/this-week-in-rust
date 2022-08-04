Title: This Week in Rust 454
Number: 454
Date: 2022-08-03
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
* [Fornjot (code-first CAD in Rust) - Weekly Release - 2022-W31](https://www.fornjot.app/blog/weekly-release/2022-w31/)
* [Zellij 0.31.0: Sixel support, search panes and custom status-bar keybindings](https://zellij.dev/news/sixel-search-statusbar/)
* [Ogma v0.5 Release](https://www.kurtlawrence.info/blog/ogma-v05-release)
* [Slint UI crate weekly updates](https://slint-ui.com/thisweek/2022-08-01.html)

### Observations/Thoughts

### Rust Walkthroughs

* [Advanced shellcode in Rust](https://kerkour.com/advanced-shellcode-in-rust)

### Research

[RRust: A Reversible Embedded Language](https://blog.erk.dev/posts/rrust)

### Miscellaneous

## Crate of the Week

This week's crate is [lending-iterator](https://lib.rs/crates/lending-iterator), a type similar to `std::iter::Iterator`, but with some type trickery that allows it to `.windows_mut(_)` safely.

Thanks to [Daniel H-M](https://users.rust-lang.org/t/crate-of-the-week/2704/1094) for the self-nomination!

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

391 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-07-25..2022-08-01

* [Add diagnostic when using `public` instead of `pub`](https://github.com/rust-lang/rust/pull/99903)
* [Expose `size_hint()` for `TokenStream`'s iterator](https://github.com/rust-lang/rust/pull/99703)
* [suggest dereferencing index when trying to use a reference of usize as index](https://github.com/rust-lang/rust/pull/99671)
* [suggest removing a semicolon and boxing the expressions for if-else](https://github.com/rust-lang/rust/pull/99974)
* [suggest removing the tuple struct field for the unwrapped value](https://github.com/rust-lang/rust/pull/99593)
* [improve `cannot move out of` error message](https://github.com/rust-lang/rust/pull/99629)
* [don't ICE on invalid dyn calls](https://github.com/rust-lang/rust/pull/99673)
* [chalk: solve auto traits for closures](https://github.com/rust-lang/chalk/pull/755)
* [add `Self: ~const Trait` to traits with `#\[const_trait\]`](https://github.com/rust-lang/rust/pull/99704)
* [miri: add default impls for `FileDescriptor` methods](https://github.com/rust-lang/miri/pull/2444)
* [miri: use `cargo_metadata` in cargo-miri](https://github.com/rust-lang/miri/pull/2450)
* [miri: use real exec on `cfg(unix)` targets](https://github.com/rust-lang/miri/pull/2426)
* [codegen: use new {re,de,}allocator annotations in llvm](https://github.com/rust-lang/rust/pull/99574)
* [use `FxIndexSet` for `region_bound_pairs`](https://github.com/rust-lang/rust/pull/99725)
* [lexer improvements](https://github.com/rust-lang/rust/pull/99884)
* [optimize `UnDerefer`](https://github.com/rust-lang/rust/pull/99667)
* [implement network primitives with ideal Rust layout, not C system layout](https://github.com/rust-lang/rust/pull/78802)
* [fix `slice::ChunksMut` aliasing](https://github.com/rust-lang/rust/pull/94247)
* [optimize `vec::IntoIter::next_chunk` impl](https://github.com/rust-lang/rust/pull/98553)
* [cargo: support for negative --jobs parameter, counting backwards from max CPUs](https://github.com/rust-lang/cargo/pull/10844)
* [rustdoc: add support for `#[rustc_must_implement_one_of]`](https://github.com/rust-lang/rust/pull/99235)
* [rustdoc: align invalid-html-tags lint with commonmark spec](https://github.com/rust-lang/rust/pull/99873)
* [rustfmt: nicer skip context for macro/attribute](https://github.com/rust-lang/rustfmt/pull/5459)
* [clippy: move `assertions_on_result_states` to restriction](https://github.com/rust-lang/rust-clippy/pull/9273)
* [clippy: read and use deprecated configuration (as well as emitting a warning)](https://github.com/rust-lang/rust-clippy/pull/9252)
* [clippy: remove "blacklist" terminology](https://github.com/rust-lang/rust-clippy/pull/8974)
* [clippy: `unwrap_used`: don't recommend using `expect` when the `expect_used` lint is not allowed](https://github.com/rust-lang/rust-clippy/pull/9223)
* [rust-analyzer: find original ast node before compute ref match](https://github.com/rust-lang/rust-analyzer/pull/12830)
* [rust-analyzer: find standalone `proc-macro-srv` on windows too](https://github.com/rust-lang/rust-analyzer/pull/12878)
* [rust-analyzer: publish extension for 32-bit ARM systems](https://github.com/rust-lang/rust-analyzer/pull/12920)
* [rust-analyzer: calculate completions after type anchors](https://github.com/rust-lang/rust-analyzer/pull/12895)
* [rust-analyzer: do completions in path qualifier position](https://github.com/rust-lang/rust-analyzer/pull/12899)
* [rust-analyzer: don't complete marker traits in expression position](https://github.com/rust-lang/rust-analyzer/pull/12901)
* [rust-analyzer: fix pattern completions adding unnecessary braces](https://github.com/rust-lang/rust-analyzer/pull/12898)
* [rust-analyzer: complete path of existing record expr](https://github.com/rust-lang/rust-analyzer/pull/12906)

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

* [disposition: merge] [relate `closure_substs.parent_substs()` to parent fn in NLL](https://github.com/rust-lang/rust/pull/98835)
* [disposition: merge] [Don't derive `PartialEq::ne`.](https://github.com/rust-lang/rust/pull/98655)
* [disposition: merge] [Guarantees of content preservation on `try_reserve` failure?](https://github.com/rust-lang/rust/issues/99606)
* [disposition: merge] [Partially stabilize std::backtrace from backtrace](https://github.com/rust-lang/rust/pull/99573)
* [disposition: merge] [Tracking Issue for ptr_const_cast](https://github.com/rust-lang/rust/issues/92675)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* *No New or Updated RFCs were created this week.*

## Upcoming Events

Rusty Events between 2022-08-03 - 2022-08-31 🦀

### Virtual

* 2022-08-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydclbfb/)
* 2022-08-03 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/dvvtvsydclbfb/)
* 2022-08-05 | Virtual + Portland, OR, US | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-09 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/vndgwsydclbmb/)
* 2022-08-09 | Virtual (Myrtle Point, OR, US) | [#EveryoneCanContribute Cafe](https://www.meetup.com/everyonecancontribute-cafe/)
    * [**Summer Chill & Learn: including OpenTelemetry getting started with Rust**](https://www.meetup.com/everyonecancontribute-cafe/events/286609523/)
* 2022-08-10 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydclbnb/)
* 2022-08-11 | Virtual (Nürnberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydclbpb/)
* 2022-08-12 | Virtual + Tokyo, JP | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://www.reddit.com/r/rust/comments/w7bktx/2022_tokyo_and_elsewhere_rust_game_hack_event_aug/)
* 2022-08-13 | Virtual | [Rust Gamedev](https://gamedev.rs/)
    * [**Rust Gamedev Monthly Meetup**](https://www.google.com/url?q=https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2Eop9Blil9YUWeTq472NIi)
* 2022-08-16 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydclbvb/)
* 2022-08-17 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydclbwb/)
* 2022-08-18 | Virtual (Charlottesville, VA, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Hierarchical Task Network compiler written in Rust**](https://www.meetup.com/charlottesville-rust-meetup/events/287203159/)
* 2022-08-18 | Virtual (Stuttgart, DE) | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydclbxb/)
* 2022-08-24 | Virtual + Wellington, NZ | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-30 | Virtual + Dallas, TX, US | [Dallas Rust](https://www.meetup.com/Dallas-Rust/)
    * [**Last Tuesday**](https://www.meetup.com/dallas-rust/events/qndgwsydclbnc/)

### Asia

* 2022-08-12 | Tokyo, JP + Virtual | [tonari](https://gallery.tonari.no/en/tonari-lab)
    * [**Tokyo Rust Game Hack 2022 edition: The Bombercrab Challenge**](https://bombercrab-rust-game-hack.peatix.com/view)

### Europe

* 2022-08-30 | Utrecht, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Run Rust Anywhere**](https://www.meetup.com/rust-nederland/events/287302224/)

### North America

* 2022-08-05 | Portland, OR, US + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2022**](https://rustconf.com/)
* 2022-08-06 | Portland, OR, US | [Rust Project Teams](https://www.rust-lang.org/governance)
    * [**RustConf 2022 PostConf Unconf**](https://www.eventbrite.com/e/rustconf-postconf-unconf-registration-373057423797) | [**Blog post**](https://blog.rust-lang.org/2022/06/28/rust-unconference.html)
* 2022-08-10 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydclbnb/)
* 2022-08-11 | Columbus, OH, US| [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydclbpb/)
* 2022-08-16 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydclbvb/)
* 2022-08-23 | Toronto, ON, CA | [Rust Toronto](https://www.meetup.com/rust-toronto/)
    * [**WebAssembly plugins in Rust**](https://www.meetup.com/rust-toronto/events/287284601/)
* 2022-08-25 | Ciudad de México, MX | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Concurrencia & paralelismo con Rust**](https://www.meetup.com/rust-mx/events/287561814/)
* 2022-08-25 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Hello World Cargo Crates Using Github Actions with jojobyte and Food!**](https://www.meetup.com/utah-rust/events/kvrxqsydclbpb/)

### Oceania

* 2022-08-24 | Wellington, NZ + Virtual | [Rust Wellington](https://www.meetup.com/rust-wellington/)
    * [**Flywheel Edition: 3 talks on Rust!**](https://www.meetup.com/rust-wellington/events/287280642/)
* 2022-08-26 | Melbourne, VIC, AU | [Rust Melbourne](https://www.meetup.com/rust-melbourne/)
    * [**August 2022 Meetup**](https://www.meetup.com/rust-melbourne/events/287468753/)

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

> ❤️🦀❤️
>
> 100,000 issues filled with love, compassion and a wholesome community. Thank you, Rust community, for being one of the most, if not straight out the most, welcoming programming communities out there. Thank you, Rust teams, for the tireless hours you spend every day on every aspect of this project. Thank you to the Rust team alumni for the many hours spent growing a plant and the humility of passing it to people you trust to continue taking care of it. Thank you everyone for RFCs, giving voice to the community, being those voices AND listening to each other.
>
> This community has been and continue to be one of the best I have ever had the pleasure of being a part of. The language itself has many things to love and appreciate about it, from the humane error messages to giving the people the power to express high performance code without sacrificing readability for the ones to come after us. But nothing, truly nothing, takes the cake as much as the community that's building it, answering questions, helping and loving each other. Every single day.
>
> Congratulations everyone for 100,000 issues and PRs! And thank you for being you. Because Rust is Beautiful, for having you as part of it.
>
> To the times we spent together and the many more to come!

– [mathspy on the rust-lang/rust github](https://github.com/rust-lang/rust/issues/100000)

Thanks to [Sean Chen](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1275) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
