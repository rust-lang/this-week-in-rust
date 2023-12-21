Title: This Week in Rust 526
Number: 526
Date: 2023-12-20
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
* [Fornjot 0.48.0 - open source b-rep CAD kernel written in Rust](https://www.fornjot.app/blog/release/0.48.0/)
* [Committing to Rust for kernel code](https://lwn.net/Articles/952029/)
* [A Rust implementation of Android's Binder](https://lwn.net/Articles/953116/)
* [Preventing atomic-context violations in Rust code with klint](https://lwn.net/Articles/951550/)
* [Rust for Linux — in space](https://lwn.net/Articles/954974/)

* [series] [Multithreading and Memory-Mapping: Refining ANN Performance with Arroy](https://blog.kerollmops.com/multithreading-and-memory-mapping-refining-ann-performance-with-arroy)

* [Introducing FireDBG - a Time Travel Visual Debugger for Rust](https://firedbg.sea-ql.org/blog/2023-12-12-introducing-firedbg/)

- [Ratatui 0.25.0 is released! - a Rust library for cooking up terminal user interfaces](https://ratatui.rs/highlights/v025/)

### Observations/Thoughts
* [The Most Common Rust Compiler Errors as Encountered in RustRover: Part 1](https://blog.jetbrains.com/rust/2023/12/14/the-most-common-rust-compiler-errors-as-encountered-in-rustrover-part-1/)

* [Nine Rules for SIMD Acceleration of your Rust Code (Part 2): General Lessons from Boosting Data Ingestion in the range-set-blaze Crate by 7x](https://medium.com/towards-data-science/nine-rules-for-simd-acceleration-of-your-rust-code-part-2-6a104b3be6f3)

### Rust Walkthroughs
* [Generic types for function parameters in Rust 🦀](https://rust.code-maven.com/generic-types-for-simple-function)

* [Benchmarking Rust Compiler Settings with Criterion: Controlling Criterion with Scripts and Environment Variables](https://medium.com/towards-data-science/benchmarking-rust-compiler-settings-with-criterion-62db50cd62fb)

### Research

### Miscellaneous
* [Getting started with Tiny HTTP building a web application in Rust](https://rust.code-maven.com/tiny-http)

## Crate of the Week

<!-- COTW goes here -->

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

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A lot of noise in the results this week; there was an lull in the noise
recently, so our auto-inferred noise threshold went down, and thus five PR's
were artificially flagged this week (and three supposed improvements were just
reverting to the mean). Beyond that, we had three nice improvements: the first
to debug builds in #117962 (by ceasing emission of expensive+unused
`.debug_pubnames` and `.debug_pubtypes`), a second to diesel and serde in
#119048 (by avoiding some unnecessary work), and a third to several benchmarks
in #117749 (by adding some caching of an internal compiler structure).

Triage done by **@pnkfelix**.
Revision range: [57010939..bf9229a2](https://perf.rust-lang.org/?start=57010939ed1d00076b4af0ed06a81ec69ea5e4a8&end=bf9229a2e366b4c311f059014a4aa08af16de5d8&absolute=false&stat=instructions%3Au)

6 Regressions, 9 Improvements, 3 Mixed; 5 of them in rollups
67 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/b9ecf1aba002cd6b33d06f784e088839636d7e92/triage/2023-12-18.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week, [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: postpone] [RFC: Precise Pre-release Deps](https://github.com/rust-lang/rfcs/pull/3263)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* [disposition: merge] [Support async recursive calls (as long as they have indirection)](https://github.com/rust-lang/rust/pull/117703)
* [disposition: merge] [make soft_unstable show up in future breakage reports](https://github.com/rust-lang/rust/pull/116274)
* [disposition: merge] [Tracking Issue for ip_in_core](https://github.com/rust-lang/rust/issues/108443)

### [Language Reference](https://github.com/rust-lang/reference/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Language Reference RFCs entered Final Comment Period this week.*

### [Unsafe Code Guidelines](https://github.com/rust-lang/unsafe-code-guidelines/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)
* *No Unsafe Code Guideline RFCs entered Final Comment Period this week.*

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [RFC: patchable-function-entry](https://github.com/rust-lang/rfcs/pull/3543)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-12-20 - 2024-01-17 🦀

### Virtual

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
* 2023-12-19 | Virtual (Linz, AT) [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 35th Edition**](https://www.meetup.com/rust-linz/events/297909995/)
* 2023-12-20 | Virtual (Vancouver, BC, CA) | [Vancouver Rust](https://www.meetup.com/vancouver-rust/)
    * [**Adventures in egui app dev**](https://www.meetup.com/vancouver-rust/events/292763506/)
* 2023-12-26 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallasrust/)
    * [**Last Tuesday**](https://www.meetup.com/dallasrust/events/mvdtgtyfcqbjc/)
* 2023-12-28 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/297687485/)
* 2024-01-03 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/wqzhftygccbfb)

### Asia

* 2023-12-16 | Delhi, IN | [Rust Delhi](https://www.meetup.com/rustdelhi/)
    * [**Meetup #4**](https://www.meetup.com/rustdelhi/events/297652586/)

### Europe

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
* 2023-12-27 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust hacknight #1: CLIs, TUIs and plushies**](https://www.meetup.com/copenhagen-rust-community/events/297894275/)

### North America

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
* 2024-01-09 | Minneapolis, MN, US | [Minneapolis Rust Meetup](https://www.meetup.com/minneapolis-rust-meetup/)
    * [**Minneapolis Rust Meetup Happy Hour**](https://www.meetup.com/minneapolis-rust-meetup/events/297760207/)

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
