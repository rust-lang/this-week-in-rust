Title: This Week in Rust 524
Number: 524
Date: 2023-12-06
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tag us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) on Twitter or [@ThisWeekinRust](https://mastodon.social/@thisweekinrust) on mastodon.social, or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust) and archives can be viewed at [this-week-in-rust.org](https://this-week-in-rust.org/).
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
* [Rust temporary lifetimes and "super let"](https://blog.m-ou.se/super-let/)
* [Behind the Scenes of Rust String Formatting: format_args!()](https://blog.m-ou.se/format-args/)
* [Profiling Rust programs the easy way](https://ntietz.com/blog/profiling-rust-programs-the-easy-way/)
* [Three problems of pinning](https://without.boats/blog/three-problems-of-pinning/)
* [Rust std fs slower than Python!? No, it's hardware!](https://xuanwo.io/2023/04-rust-std-fs-slower-than-python/)
* [Leafpipe - Turning lots and lots of audio/visual into pretty stimuli](https://half-shot.uk/blog/leafpipe/)
* [Why Type Safety is Important](https://www.shuttle.rs/blog/2023/11/29/type-safety)

### Rust Walkthroughs

### Research

### Miscellaneous
* [Building a Hypercube Interpolator (in Rust)](https://jlogan03.github.io/interpn/)
* [Embassy on ESP: GPIO](https://apollolabsblog.hashnode.dev/embassy-on-esp-gpio)
* [video] [Speeding Up Rust Code](https://www.youtube.com/watch?v=2IHPvPmzS8g)
* [video] [Axum 0.6 to 0.7 in 5 quick steps](https://www.youtube.com/watch?v=MvWCX5ckuDE)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Ockam - Library - Validate CBOR structs according to the cddl schema for `nodes/models` misc](https://github.com/build-trust/ockam/issues/6690)
* [Ockam - Command - refactor to use typed interfaces to implement commands for `nodes`](https://github.com/build-trust/ockam/issues/6701)
* [Ockam - Library - Slim down the `NodeManagerWorker` for `node / node status`](https://github.com/build-trust/ockam/issues/6707)
* [zerocopy - Test the output of zerocopy-derive](https://github.com/google/zerocopy/issues/367)
* [zerocopy - Use cargo-semver-checks to make sure `derive` feature doesn't change API surface](https://github.com/google/zerocopy/issues/422)
* [zerocopy - Verify that `all-jobs-succeeded` CI job depends on all other jobs](https://github.com/google/zerocopy/issues/444)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

<!-- Perf results go here -->

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
* [disposition: merge] [Add lint against ambiguous wide pointer comparisons ](https://github.com/rust-lang/rust/pull/117758)
* [disposition: merge] [Stabilize `ptr::{from_ref, from_mut`}](https://github.com/rust-lang/rust/pull/117824)
* [disposition: merge] [Tracking issue for `any::type_name_of_val`](https://github.com/rust-lang/rust/issues/66359)
* [disposition: merge] [rustdoc: allow resizing the sidebar / hiding the top bar](https://github.com/rust-lang/rust/pull/115660)
* [disposition: merge] [Don't merge cfg and doc(cfg) attributes for re-exports](https://github.com/rust-lang/rust/pull/113091)
* [disposition: merge] [Make `IMPLIED_BOUNDS_ENTAILMENT` into a hard error from a lint](https://github.com/rust-lang/rust/pull/117984)
* [disposition: merge] [Exhaustiveness: reveal opaque types properly](https://github.com/rust-lang/rust/pull/116821)
* [disposition: merge] [fix fn/const items implied bounds and wf check](https://github.com/rust-lang/rust/pull/104098)
* [disposition: merge] [guarantee that char and u32 are ABI-compatible](https://github.com/rust-lang/rust/pull/118032)
* [disposition: merge] [Use version-sorting for all sorting](https://github.com/rust-lang/rust/pull/115046)
* [disposition: merge] [Tracking Issue for `arc_unwrap_or_clone`](https://github.com/rust-lang/rust/issues/93610)
* [disposition: merge] [adds a column number to `dbg!()`](https://github.com/rust-lang/rust/pull/114962)
* [disposition: merge] [Allow any `const` expression blocks in `thread_local!`](https://github.com/rust-lang/rust/pull/116392)
* [disposition: merge] [check `FnDef` return type for WF](https://github.com/rust-lang/rust/pull/115538)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: Make Cargo respect minimum supported Rust version (MSRV) when selecting dependencies](https://github.com/rust-lang/rfcs/pull/3537)
* [RFC: Trait for !Sized thin pointers](https://github.com/rust-lang/rfcs/pull/3536)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-06 - 2024-01-03 🦀

### Virtual

* 2023-12-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/297172140)
* 2023-12-10 | Virtual (Cardiff, UK) | [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**Rust & C++ Christmas Game Jam Finale**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/296583652/)
* 2023-12-12 | Virtual | [Mainmatter](https://mainmatter.com)
    * [**Workshop: Telemetry for Rust applications**](https://rust-telemetry-workshop.mainmatter.com)
* 2023-12-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Second Tuesday**](https://www.meetup.com/dallasrust/events/297532862/)
* 2023-12-14 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/296833674/)
* 2023-12-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/295679660/)
* 2023-12-17 | Virtual (Tel Aviv, IL) | [Code Mavens](https://www.meetup.com/code-mavens/)
    * [**Don't panic! - Our journey to error handling in Rust**](https://www.meetup.com/code-mavens/events/297334993/)
* 2023-12-18 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Virtual (Berlin, DE) | [OpenTechSchool Berlin](https://www.meetup.com/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/opentechschool-berlin/events/295679827/) | [**Mirror**](https://berline.rs/)
* 2023-12-19 | Virtual (Washington, DC, US) | [Rust DC](https://www.meetup.com/rustdc/)
    * [**Mid-month Rustful**](https://www.meetup.com/rustdc/events/297128156/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Adventures in egui app dev**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 2023-12-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)

### Asia

* 2023-12-10 | Tokyo, JP | [Tokyo Rust Meetup](https://www.meetup.com/tokyo-rust-meetup/)
    * [**Fun with Labels, Macros, and Unsafe Rust**](https://www.meetup.com/tokyo-rust-meetup/events/297674153)
* 2023-12-16 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europe

* 2023-12-06 | Cologne, DE | [Rust Cologne](https://www.meetup.com/rustcologne/events)
    * [**Advent Of Rust**](https://www.meetup.com/rustcologne/events/297100007/)
* 2023-12-07 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust and Talk at Danske Commodities**](https://www.meetup.com/rust-aarhus/events/296223513/)
* 2023-12-07 | Lille, FR | [Rust Lille](https://www.meetup.com/meetup-group-zgphbyet/)
    * [**Rust Lille #5**](https://www.meetup.com/meetup-group-zgphbyet/events/297477578/)
* 2023-12-07 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**Rust London Christmas Party with Realm (RSVP on Eventbrite)**](https://www.meetup.com/rust-london-user-group/events/297703135/)
* 2023-12-14 | Augsburg, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Augsburg Rust Meetup #4**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/297025700/)
* 2023-12-14 | Basel, CH | [Rust Basel](https://www.meetup.com/rust-basel/)
    * [**Testing: Learn from the pros**](https://www.meetup.com/rust-basel/events/297599586/)
* 2023-12-18 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 5 - hybrid**](https://www.meetup.com/rust-munich/events/296429053/)
* 2023-12-19 | Heidelberg, DE | [Nix Your Bugs & Rust Your Engines](https://rheinneckar.events/@nix_rust)
    * [**Nix Your Bugs & Rust Your Engines #1**](https://rheinneckar.events/events/298e520c-89ca-4754-96f8-e252b96b7a46)
* 2023-12-19 | Leipzig, DE | [Rust - Modern Systems Programming in Leipzig](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/)
    * [**Tauri, an Electron-alternative**](https://www.meetup.com/rust-modern-systems-programming-in-leipzig/events/295504276/)

### North America

* 2023-12-07 | Seattle, WA, US | [Seattle Rust User Group](https://www.meetup.com/seattle-rust-user-group/)
    * [**Seattle Rust User Group Meetup**](https://www.meetup.com/seattle-rust-user-group/events/297533440/)
* 2023-12-11 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760172/)
* 2023-12-12 | Seattle, WA, US | [Cap Hill Rust Coding/Hacking/Learning](https://www.meetup.com/cap-hill-rust/)
    * [**Rusty Coding/Hacking/Learning Night**](https://www.meetup.com/cap-hill-rust/events/296564619/)
* 2023-12-12 | New York, NY, US | [Rust NYC](https://www.meetup.com/rust-nyc/)  
    * [**Rust NYC Monthly Mixer: Share, Show, & Tell! 🦀**](https://www.meetup.com/rust-nyc/events/297659937/)
* 2023-12-13 | Chicago, IL, US | [Deep Dish Rust](https://www.meetup.com/deep-dish-rust/)
    * [**Rust Hack Night**](https://www.meetup.com/deep-dish-rust/events/297671182/)
* 2023-12-14 | Lehi, UT, US | [Utah Rust](https://www.meetup.com/utah-rust/)
    * [**Sending Data with Channels w/Herbert Wolverson**](https://www.meetup.com/utah-rust/events/297720170/)
* 2023-12-14 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/297628069/)
* 2023-12-15 | Somerville, MA, US | [Boston Rust Meetup](https://www.meetup.com/bostonrust/)
    * [**Ball Square Rust Lunch**](https://www.meetup.com/bostonrust/events/297633899/)
* 2023-12-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/vwljctyfcqbzb/)
* 2023-12-27 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/xvkdgtyfcqbkc/)

### Oceania

* 2023-12-11 | Perth, WA, AU | [Rust Perth Meetup Group](https://www.meetup.com/perth-rust-meetup-group/)
    * [**Rust End of Year Event**](https://www.meetup.com/perth-rust-meetup-group/events/297191089/)

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

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin), [bennyvasquez](https://github.com/bennyvasquez).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
