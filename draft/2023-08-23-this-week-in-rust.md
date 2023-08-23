Title: This Week in Rust 509
Number: 509
Date: 2023-08-23
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
* [This Week in Ars Militaris VII](https://arsmilitaris.com/#this-week-in-ars-militaris-vii)

### Project/Tooling Updates

* [actix-contrib-logger v0.1.0: drop-in replacement for the Actix Web HTTP Logger middleware](https://github.com/mrsarm/rust-actix-contrib-logger)

### Observations/Thoughts
* [Rust 1.71.1](https://youtu.be/djpujv8M7w8)
* [Exploring the Rust compiler benchmark suite](https://kobzol.github.io/rust/rustc/2023/08/18/rustc-benchmark-suite.html)
* [Pre-RFC: Sandboxed, deterministic, reproducible, efficient Wasm compilation of proc macros](https://internals.rust-lang.org/t/pre-rfc-sandboxed-deterministic-reproducible-efficient-wasm-compilation-of-proc-macros/19359)
* [RustShip: a new Rust podcast](https://www.marcoieni.com/2023/08/rustship-a-new-rust-podcast/)

### Rust Walkthroughs
* [Delightful command-line utilities with Rust](https://www.youtube.com/watch?v=Y-LTWNciEks)
* [ESP32 Standard Library Embedded Rust: Analog Temperature Sensing using the ADC](https://apollolabsblog.hashnode.dev/esp32-standard-library-embedded-rust-analog-temperature-sensing-using-the-adc)
* [Bare Metal Space Invaders](https://blog.fponzi.me/2023-08-13-bare-metal-space-invaders.html)
* [series] [Distributed Tracing in Rust, Episode 2: tracing basics](https://heikoseeberger.de/2023-08-18-dist-tracing-2/)
* [Secure database access using Ockam](https://www.ockam.io/blog/basic_web_app)

### Research
* [Fixing Rust Compilation Errors using LLMs](https://arxiv.org/abs/2308.05177)

### Miscellaneous
* [Shuttle Launchpad #6: A little CRUD](https://www.shuttle.rs/launchpad/issues/2023-11-08-issue-06-CRUD-Postgres)

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but did not know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [ZeroCopy - CI step "Set toolchain version" can fail without stopping CI job 1](https://github.com/google/zerocopy/issues/225)
* [ZeroCopy - Prevent panics statically 1](https://github.com/google/zerocopy/issues/202)
* [RON - Rusty byte strings in RON, deprecate base64 (byte) strings](https://github.com/ron-rs/ron/pull/438)
* [heed - create guides on ways to use heed](https://github.com/meilisearch/heed/issues/196)
* [Ockam - Use a user-friendly name for the shared services to show it in the tray menu](https://github.com/build-trust/ockam/issues/5686)
* [Ockam - In the `Share a service` window, the `Port` should be renamed to `Address` and support such format](https://github.com/build-trust/ockam/issues/5685)
* [Ockam - In the `Share a service` window, the `Name` attribute should not have the `/service/` prefix](https://github.com/build-trust/ockam/issues/5684)
* [Hyperswitch - remove unused function for merchant connector account](https://github.com/juspay/hyperswitch/issues/1998)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

A week with very few real regressions and some good improvements through work done by @cjgillot who found a few spots where the compiler was doing unnecessary work.

Triage done by **@rylev**.
Revision range: [e845910..d4a881](https://perf.rust-lang.org/?start=e8459109bbb440764c1c877032189a27b9e76c4e&end=d4a881e1433cd10e424843353e1f939f5a798f4e&absolute=false&stat=instructions%3Au)

**Summary**:

| (instructions:u)                   | mean  | range          | count |
|:----------------------------------:|:-----:|:--------------:|:-----:|
| Regressions ❌ <br /> (primary)    | 1.4%  | [0.5%, 2.6%]   | 13    |
| Regressions ❌ <br /> (secondary)  | 0.6%  | [0.3%, 0.8%]   | 8     |
| Improvements ✅ <br /> (primary)   | -0.7% | [-1.4%, -0.3%] | 59    |
| Improvements ✅ <br /> (secondary) | -0.8% | [-1.3%, -0.3%] | 38    |
| All ❌✅ (primary)                 | -0.3% | [-1.4%, 2.6%]  | 72    |


3 Regressions, 2 Improvements, 2 Mixed; 2 of them in rollups
28 artifact comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2023-08-22.md) 

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
* [disposition: merge] [Tracking Issue for const `[u8]::is_ascii` (`const_slice_is_ascii`)](https://github.com/rust-lang/rust/issues/111090)
* [disposition: merge] [Implement `From<[T; N]>` for `Rc<[T]>` and `Arc<[T]>`](https://github.com/rust-lang/rust/pull/114041)
* [disposition: merge] [Tracking Issue for `Saturating` type](https://github.com/rust-lang/rust/issues/87920)
* [disposition: merge] [Implement `From<{&,&mut} [T; N]`> for `Vec<T>` where `T: Clone`](https://github.com/rust-lang/rust/pull/111278)
* [disposition: merge] [Tracking Issue for os_str_bytes](https://github.com/rust-lang/rust/issues/111544)
* [disposition: merge] [Tracking Issue for `io::Error::other`](https://github.com/rust-lang/rust/issues/91946)
* [disposition: merge] [`impl TryFrom<char> for u16`](https://github.com/rust-lang/rust/pull/114065)
* [disposition: merge] [rustdoc: show inner enum and struct in type definition for concrete type](https://github.com/rust-lang/rust/pull/114855)
* [disposition: merge] [Replace old private-in-public diagnostic with type privacy lints](https://github.com/rust-lang/rust/pull/113126)
* [disposition: merge] [Implement `PartialOrd` and `Ord` for `Discriminant`](https://github.com/rust-lang/rust/pull/106418)
* [disposition: merge] [stop adding dropck outlives requirements for `[T; 0]`](https://github.com/rust-lang/rust/issues/110288)
* [disposition: merge] [make Cell::swap panic if the Cells partially overlap](https://github.com/rust-lang/rust/pull/114795)
* [disposition: merge] [Add note that Vec::as_mut_ptr() does not materialize a reference to the internal buffer](https://github.com/rust-lang/rust/pull/113859)
* [disposition: merge] [Document lack of panic safety guarantees of `Clone::clone_from`](https://github.com/rust-lang/rust/pull/98461)
* [disposition: merge] [Command: also print removed env vars](https://github.com/rust-lang/rust/pull/114379)
* [disposition: merge] [impl Step for IP addresses](https://github.com/rust-lang/rust/pull/113748)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)
* [new] [RFC: `expose-fn-type`](https://github.com/rust-lang/rfcs/pull/3476)

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* *No RFCs issued a call for testing this week.*

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

## Upcoming Events

Rusty Events between 2023-08-23 - 2023-09-20 🦀

### Virtual

* 2023-08-23 | Virtual (Linz, AT) | [Rust Linz](https://www.meetup.com/rust-linz/)
    * [**Rust Meetup Linz - 32nd Edition**](https://www.meetup.com/rust-linz/events/294718621/)
* 2023-08-24 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/295250677/)
* 2023-08-24 | Virtual (Ciudad de México, MX) | [Rust MX](https://www.meetup.com/rust-mx/)
    * [**Macros Procedurales y Metalenguajes en Rust**](https://www.meetup.com/rust-mx/events/295545343)
* 2023-09-05 | Virtual (Buffalo, NY, US) | [Buffalo Rust Meetup](https://www.meetup.com/buffalo-rust-meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/295207389/)
* 2023-09-05 | Virtual (Munich, DE) | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/294049877)
* 2023-09-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/ngnwftyfcmbkb/)
* 2023-09-12 - 2023-09-15 | Virtual (Albuquerque, NM, US) | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-12 | Virtual (Dallas, TX, US) | [Dallas Rust](https://www.meetup.com/dallas-rust/)
    * [**Second Tuesday**](https://www.meetup.com/dallas-rust/events/gqdlgtyfcmbqb/)
* 2023-09-13 | Virtual (Boulder, CO, US) | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/295011539)
* 2023-09-13 | Virtual (Cardiff, UK)| [Rust and C++ Cardiff](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff)
    * [**The unreasonable power of combinator APIs**](https://www.meetup.com/rust-and-c-plus-plus-in-cardiff/events/294748626)
* 2023-09-14 | Virtual (Nuremberg, DE) | [Rust Nuremberg](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/289732655)

### Asia

* 2023-09-06 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**RustTLV @ Final - September Edition**](https://www.meetup.com/rust-tlv/events/295441355/)

### Europe

* 2023-08-23 | London, UK | [Rust London User Group](https://www.meetup.com/rust-london-user-group/)
    * [**LDN Talks Aug 2023: Rust London x RNL (The next Frontier in App Development)**](https://www.meetup.com/rust-london-user-group/events/295338396/)
* 2023-08-24 | Aarhus, DK | [Rust Aarhus](https://www.meetup.com/rust-aarhus/)
    * [**Rust Aarhus Hack and Learn at Trifork**](https://www.meetup.com/rust-aarhus/events/293950871/)
* 2023-08-31 | Augsburg, DE | [Rust Meetup Augsburg](https://www.meetup.com/rust-meetup-augsburg/)
    * [**Augsburg Rust Meetup #2**](https://www.meetup.com/rust-meetup-augsburg/events/294538503/)
* 2023-09-05 | Munich, DE + Virtual | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich 2023 / 4 - hybrid**](https://www.meetup.com/rust-munich/events/294186101/)
* 2023-09-21 | Bern, CH | [Rust Bern](https://www.meetup.com/de-DE/rust-bern/)
    * [**Third Rust Bern Meetup**](https://www.meetup.com/rust-bern/events/295503351/)

### North America

* 2023-08-23 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch - Fareground**](https://www.meetup.com/rust-atx/events/295008514)
* 2023-08-24 | Mountain View, CA, US | [Mountain View Rust Meetup](https://www.meetup.com/mv-rust-meetup/)
    * [**Rust Meetup at Hacker Dojo**](https://www.meetup.com/mv-rust-meetup/events/295107743/)
* 2023-08-30 | Copenhagen, DK | [Copenhagen Rust Community](https://www.meetup.com/copenhagen-rust-community/)
    * [**Rust metup #39 sponsored by Fermyon**](https://www.meetup.com/copenhagen-rust-community/events/294806394)
* 2023-09-06 | Bellevue, WA, US | [The Linux Foundation](https://www.linuxfoundation.org/)
    * [**Rust Global**](https://events.linuxfoundation.org/rust-global/)
* 2023-09-12 - 2023-09-15 | Albuquerque, NM, US  + Virtual | [RustConf](https://rustconf.com/)
    * [**RustConf 2023**](https://rustconf.com/)
* 2023-09-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/295545278)

### Oceania

* 2023-08-24 | Brisbane, QLD, AU | [Rust Brisbane](https://www.meetup.com/rust-brisbane/)
    * [**August Meetup**](https://www.meetup.com/rust-brisbane/events/295415680/)
* 2023-09-13 | Perth, WA, AU | [Rust Perth](https://www.linkedin.com/groups/7439562/)
    * [**Rust Meetup 2: Lunch & Learn**](https://www.linkedin.com/events/7097356771584880640/)

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
