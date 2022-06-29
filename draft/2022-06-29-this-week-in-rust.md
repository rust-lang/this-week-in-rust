Title: This Week in Rust 449
Number: 449
Date: 2022-06-29
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
- [Call for applications: Knurling-rs Summer of Code 2022 🦀](https://ferrous-systems.com/blog/knurling-summer-of-code/)

* [Cross v0.2.2 Released](https://www.reddit.com/r/rust/comments/vk2xfc/cross_v022_released/)

* [Release-plz: release Rust packages from CI](https://www.marcoieni.com/2022/06/release-plz-release-rust-packages-from-ci/)

* [Fornjot (code-first CAD in Rust) - Weekly Dev Log - 2022-W25](https://www.fornjot.app/blog/weekly-dev-log/2022-w25/)

* [Play Ping-Pong with Lunatic 🦀 UDP](https://missmissm.medium.com/play-ping-pong-with-lunatic-udp-ef557a22a604)

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

<!-- COTW goes here -->

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

<!-- Rust updates go here -->

### Rust Compiler Performance Triage

Overall, a positive week for compiler performance with regressions mainly being relegated to smaller issues. This can be see by the fact that 95 test cases in real world crates were improvements while regressions only happened in 22 test cases. The largest improvement was by @nnethercote where the compilation of `#[derive(Debug)]` was improved. This led to an average of 1% improvement in compile times across 124 real world crate test cases.

Triage done by **@rylev**.
Revision range: [abace0..baf382](https://perf.rust-lang.org/?start=abace0a1f17986d89aedf610819deab2b4afee56&end=baf382e63c023259fa1f9042f8f479f183ca6ed3&absolute=false&stat=instructions%3Au)

**Summary**:

|            | mean | max | count |
|:----------:|:----:|:---:|:-----:|
| Regressions 😿 <br /> (primary) | 0.6% | 2.4% | 22    |
| Regressions 😿 <br /> (secondary) | 0.5% | 1.0% | 35    |
| Improvements 🎉 <br /> (primary) | -1.1% | -5.2% | 95    |
| Improvements 🎉 <br /> (secondary) | -2.3% | -10.3% | 35    |
| All 😿🎉 (primary) | -0.8% | -5.2% | 117   |

3 Regressions, 3 Improvements, 4 Mixed; 5 of them in rollups
34 artifact comparisons made in total

### [Call for Testing](https://github.com/rust-lang/rfcs/issues?q=label%3Acall-for-testing)
An important step for RFC implementation is for people to experiment with the
implementation and give feedback, especially before stabilization.  The following
RFCs would benefit from user testing before moving forward:

* [RFC: Serve crates-io registry over HTTP as static files](https://github.com/rust-lang/rfcs/pull/2789)
    * [Testing steps](https://github.com/rust-lang/rfcs/pull/2789#issuecomment-1166155551)

If you are a feature implementer and would like your RFC to appear on the above list, add the new `call-for-testing`
label to your RFC along with a comment providing testing instructions and/or guidance on which aspect(s) of the feature
need testing.

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No RFCs were approved this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the 'final comment period' for RFCs and key PRs
which are reaching a decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* *No RFCs entered Final Comment Period this week.*

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Remove restrictions on compare-exchange memory ordering.](https://github.com/rust-lang/rust/pull/98383)
* [disposition: merge] [ptr::copy and ptr::swap are doing untyped copies](https://github.com/rust-lang/rust/pull/97712)
* [disposition: merge] [Tracking issue for `IntoFuture`](https://github.com/rust-lang/rust/issues/67644)
* [disposition: merge] [Implement `FusedIterator` for `std::net::[Into]Incoming`](https://github.com/rust-lang/rust/pull/97300)
* [disposition: merge] [Tracking Issue for core_ffi_c](https://github.com/rust-lang/rust/issues/94501)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [RFC 3283: Backward compatible default features](https://github.com/rust-lang/rfcs/pull/3283)

## Upcoming Events

Rusty Events between 2022-06-29 - 2022-07-27 🦀

### Virtual

* 2022-06-29 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/qwgrssydcjbmc/)
* 2022-06-30 | Linz, AT | [Rust Linz](https://www.meetup.com/Rust-Linz/)
    * [**Rust Meetup Linz - 23th Edition**](https://www.meetup.com/Rust-Linz/events/286029968/)
* 2022-07-05 | Austin, TX, US | [WebAssembly and WasmEdge](https://www.meetup.com/webassembly-and-wasmedge/)
    * [**Monthly WasmEdge Community Meeting #10**](https://www.meetup.com/webassembly-and-wasmedge/events/zzdnrsydckbhb/)
* 2022-07-05 | Beijing, CN | [WebAssembly and Rust Meetup (Rustlang)](https://www.meetup.com/wasm-rust-meetup/)
    * [**Monthly WasmEdge Community Meeting, a CNCF sandbox WebAssembly runtime**](https://www.meetup.com/wasm-rust-meetup/events/jbfnrsydckbhb/)
* 2022-07-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/buffalo-rust-meetup/events/xgmfssydckbhb/)
* 2022-07-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/qwtdjsydckbjb/)
* 2022-07-07 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Rust, nalgebra, and Fourier Optics**](https://www.meetup.com/charlottesville-rust-meetup/events/285818136/)
* 2022-07-09 | Virtual | [Rust Game Dev](https://github.com/rust-gamedev/wg)
    * [**Monthly Meetup**](https://discord.gg/yNtPTb2&sa=D&source=calendar&usd=2&usg=AOvVaw2pjyb-YBsl99IFDmrOKoOK)
* 2022-07-13 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/zvxcsrydckbrb/)
* 2022-07-13 | Malaysia, MY | [Rust Malaysia Meetup](https://rust-malaysia.github.io/meetup/)
    * [**Rust Malaysia Meetup**](https://forms.gle/rFzwUjh5YT1pVci6A)
* 2022-07-14 | Nürnberg, DE | [Rust Nurnberg DE](https://www.meetup.com/rust-noris/)
    * [**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/hlvbvsydckbsb/)
* 2022-07-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Rust Study/Hack/Hang-out**](https://www.meetup.com/Vancouver-Rust/events/nwcmpsydckbbc/)
* 2022-07-19 | Sydney, NSW, AU | [Rust Australia](https://github.com/RustAU)
    * [**Rust Lightning Talks**](https://github.com/RustAU/Virtual)
* 2022-07-19 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Mid-month Rustful**](https://www.meetup.com/RustDC/events/vdhxgsydckbzb/)
* 2022-07-21 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/rust-community-stuttgart/events/qtvtvsydckbcc/)
* 2022-07-31 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Intro to Monads for Rustaceans**](https://www.meetup.com/Seattle-Rust-Meetup/events/286692243/)

### Asia

* 2022-07-04 | Tel Aviv, IL | [Rust TLV](https://www.meetup.com/rust-tlv/)
    * [**Rust Interop, Rewrites and fun**](https://www.meetup.com/rust-tlv/events/286610368/)

### Europe

* 2022-06-29 | Nijmegen, NL | [Rust Nederland](https://www.meetup.com/rust-nederland/)
    * [**Get started with Rust and stories from the frontlines**](https://www.meetup.com/rust-nederland/events/286582960/)
* 2022-06-30 | Copenhagen, DK | [Copenhagen Rust Group](https://cph.rs/)
    * [**Hack Night #26**](https://cph.rs/)

### North America

* 2022-06-29 | Austin, TX, US | [Rust ATX](https://www.meetup.com/rust-atx/)
    * [**Rust Lunch**](https://www.meetup.com/rust-atx/events/286596997/)
* 2022-07-13 | Atlanta, GA, US | [Rust Atlanta](https://www.meetup.com/rust-atl/)
    * [**Grab a beer with fellow Rustaceans**](https://www.meetup.com/rust-atl/events/pczdssydckbrb/)
* 2022-07-14 | Columbus, IL | [Columbus Rust Society](https://www.meetup.com/columbus-rs/)
    * [**Monthly Meeting**](https://www.meetup.com/columbus-rs/events/dpkhgrydckbsb/)
* 2022-07-19 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/wjkjssydckbzb/)
* 2022-07-26 | Austin, TX, US | [ATX Rustaceans](https://www.meetup.com/atx-rustaceans/)
    * [**First Official Meetup - July 2022**](https://www.meetup.com/atx-rustaceans/events/285878081/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

<!-- QOTW goes here -->

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
