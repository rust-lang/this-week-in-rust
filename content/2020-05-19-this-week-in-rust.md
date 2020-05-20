Title: This Week in Rust 339
Number: 339
Date: 2020-05-19
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Starting with this issue, there is now also a weekly *This Week in Rust Podcast* highlighting a few of the stories from each issue. You can check out the very first episode on [The Rustacean Station](https://rustacean-station.org/episode/015-twir-339/).

# Updates from Rust Community

## News & Blog Posts

* [Five Years of Rust](https://blog.rust-lang.org/2020/05/15/five-years-of-rust.html)
* [Structuring and handling errors in 2020](https://nick.groenen.me/posts/rust-error-handling/)
* [Taking Advantage of Auto-Vectorization in Rust](https://nickwilcox.github.io/blog/autovec/)
* [State of Web Routing in Rust](https://pksunkara.com/posts/state-of-routing-in-rust/)
* [Rust releases for single and multiple targets with GitHub Actions](https://mateuscosta.me/rust-releases-with-github-actions)
* [Getting Started With Rust By Building A Tiny Markdown Compiler](https://jesselawson.org/rust/getting-started-with-rust-by-building-a-tiny-markdown-compiler/)
* [Parcel and Rust: A WASM Romcom](https://dev.to/potatostudios_/parcel-and-rust-a-wasm-romcom-545e)
* [Reading temperature sensor in Rust using Raspberry Pi GPIO](https://dev.to/citizen_stig/reading-temperature-sensor-in-rust-using-raspberry-pi-gpio-bhf)
* [Rust Analyzer - Next Few Years](https://rust-analyzer.github.io/blog/2020/05/18/next-few-years.html)
* [Rust Closures in FFI](http://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/)
* [The case for using Rust for Automotive Software](https://medium.com/@sojan.james/the-case-for-using-rust-for-automotive-software-19400779f126)
* [Gamedev #4: Benefits of full-stack Rust](https://www.jakobmeier.ch/blogging/Paddlers_4.html)
* [RISC-V OS using Rust Chapter 9: Block IO](http://osblog.stephenmarz.com/ch9.html)
* [A Guide to Global Data in Rust](https://github.com/paulkernfeld/global-data-in-rust)
* [audio] [What's New in Rust 1.42 and 1.43](https://rustacean-station.org/episode/014-rust-1.42-1.43/)
* [video] [Jonathan Teaches Jason Rust!](https://www.youtube.com/watch?v=EzQ7YIIo1rY&feature=youtu.be)
* [video] [Educational Rust coding - Building a web app](https://www.twitch.tv/videos/623988324)
* [video] [Rust and C++ Cardiff Virtual Meetup](https://www.youtube.com/watch?v=s8WMaVU3EBs&feature=youtu.be)

# Crate of the Week

This week's crate is [apply](https://crates.io/crates/apply), a tiny library for chaining free functions into method call chains.

Thanks to [Trevor Spiteri](https://users.rust-lang.org/t/crate-of-the-week/2704/769) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [clap-rs: Subcommand bin_name on Windows contains ".exe" in the middle instead of at the end (or not at all)](https://github.com/clap-rs/clap/issues/992)
* [keikan: Update Rendering Code to be PBR Compliant](https://github.com/Tloru/keikan/issues/1)
* [keikan: Implementing Different Objects](https://github.com/Tloru/keikan/issues/2)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

359 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-05-11..2020-05-18

* [add built in PSP target](https://github.com/rust-lang/rust/pull/72062)
* [emit a warning when optimization fuel runs out](https://github.com/rust-lang/rust/pull/72067)
* [literal error reporting cleanup](https://github.com/rust-lang/rust/pull/72047)
* [incomplete features can also be unsound](https://github.com/rust-lang/rust/pull/72045)
* [be less aggressive with `DroplessArena`/`TypedArena` growth](https://github.com/rust-lang/rust/pull/71872)
* [provide separate option for std debug asserts](https://github.com/rust-lang/rust/pull/72146)
* [rework the `std::iter::Step` trait](https://github.com/rust-lang/rust/pull/69659)
* [simpler slice `Iterator` methods](https://github.com/rust-lang/rust/pull/72166)
* [make `RawVec::grow` mostly non-generic](https://github.com/rust-lang/rust/pull/72013)
* [implement `FromStr` for `OsString`](https://github.com/rust-lang/rust/pull/71662)
* [make `offset` `must_use`](https://github.com/rust-lang/rust/pull/72143)
* [cargo: ignore broken console output in some situations](https://github.com/rust-lang/cargo/pull/8236)
* [cargo: handle LTO with an rlib/cdylib crate type](https://github.com/rust-lang/cargo/pull/8254)
* [cargo: gracefully handle errors during a build](https://github.com/rust-lang/cargo/pull/8247)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved last week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Transition to rust-analyzer as our official LSP (Language Server Protocol) implementation](https://github.com/rust-lang/rfcs/pull/2912)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Stabilize AtomicN::fetch_min and AtomicN::fetch_max](https://github.com/rust-lang/rust/pull/72324)
* [disposition: merge] [Stabilize process_set_argv0 feature for Unix](https://github.com/rust-lang/rust/pull/72123)
* [disposition: merge] [impl From <Cow\> for Box, Rc, and Arc](https://github.com/rust-lang/rust/pull/71447)
* [disposition: close] [Tracking issue for non_static_type_id](https://github.com/rust-lang/rust/issues/41875)

## New RFCs
* [RFC: Reading into uninitialized buffers](https://github.com/rust-lang/rfcs/pull/2930)

# Upcoming Events

### Online
* [May 20. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/qnrgnrybchbbc/).
* [May 21. Turin, IT - Rust Turin Meetup](https://community.mozilla.org/events/gruppo-di-studio-di-rust/)
* [May 26. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybchbjc/)
* [May 26. Berlin, DE - Rust and Tell](https://www.meetup.com/Rust-Berlin/events/270319545/)
* [May 27. Montréal, QC, CA - Remote - RustMTL May 2020](https://www.meetup.com/Rust-Montreal/events/270635425)


### North America
* [May 25. Durham, NC, US - Triangle Rustaceans - Project Night and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybchbhc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> The whole motivation behind exceptions is to allow one to write ones business logic, concentrate on what one likes to think ones program will do, without having lots of fiddly error checking and handling code obscuring that logic. Error situations are therefore swept under the carpet with "try" and kept out of sight with "catch".
>
> However in my world view failure is not exceptional, it is a common happening, it's too important to be hidden away. Therefor failure handling should be in ones face in the code you write. Certainly in the face of those that read it.

– [ZiCog on rust-users](https://users.rust-lang.org/t/did-rust-make-the-right-choice-about-error-handling/41736/29)

Thanks to [Lzutao](https://users.rust-lang.org/t/twir-quote-of-the-week/328/872) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]().</small>
