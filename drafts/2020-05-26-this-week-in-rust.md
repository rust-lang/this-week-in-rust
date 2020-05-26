Title: This Week in Rust 340
Number: 340
Date: 2020-05-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*]()

# Updates from Rust Community

## News & Blog Posts

* [Actix-Web in Docker: How to build small and secure images](https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd)
* [Angular, Rust, WebAssembly, Node.js, Serverless, and... the NEW Azure Static Web Apps!](https://dev.to/azure/angular-rust-webassembly-node-js-serverless-and-the-new-azure-static-web-apps-cnb)
* [The Chromium project finds that around 70% of our serious security bugs are memory safety problems](https://www.chromium.org/Home/chromium-security/memory-safety)
* [Integration of AV-Metrics Into rav1e, the AV1 Encoder](https://dev.to/vibhoothi/integration-of-av-metrics-into-rav1e-the-av1-encoder-5h8h)
* [Oxidizing the technical interview](https://blog.mgattozzi.dev/oxidizing-the-technical-interview/)
* [Porting K-D Forests to Rust](https://tavianator.com/porting-k-d-forests-to-rust/)
* [Rust Macro Rules in Practice](https://dev.to/sassman/rust-macro-rules-in-practice-40ne)
* [Rust: Dropping heavy things in another thread can make your code 10000 times faster](https://abramov.io/rust-dropping-things-in-another-thread)
* [Rust's Runtime](https://blog.mgattozzi.dev/rusts-runtime/)
* [Zero To Production #0: Foreword](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/)
* [video] [Bringing WebAssembly outside the web with WASI by Lin Clark](https://www.youtube.com/watch?v=fh9WXPu0hw8)
* [video] [Microsoft's Safe Systems Programming Languages Effort](https://mybuild.microsoft.com/sessions/61de34c5-b111-4ece-928f-541854875862?source=sessions)
* [video] [Rust, WebAssembly, and the future of Serverless by Steve Klabnik](https://www.youtube.com/watch?v=CMB6AlE1QuI)
* [video] [Rust: Not as hard as you think / Meta/conf: Backend Meetup 2020](https://www.youtube.com/watch?v=n3kyvMVck_M) (In Russian)

# Crate of the Week

This week's crate is [apply](https://crates.io/crates/apply), a tiny library for chaining free functions into method call chains.

Thanks to [Trevor Spiteri](https://users.rust-lang.org/t/crate-of-the-week/2704/769) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

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
* [May 27. Wrocław, PL - Rust Wrocław Meetup #20](https://www.meetup.com/Rust-Wroclaw/events/270771184/)
* [May 27. London, UK - Remote - LDN Talks May 2020](https://www.meetup.com/Rust-London-User-Group/events/270526235/)


### North America
* [May 25. Durham, NC, US - Triangle Rustaceans - Project Night and Lightning Talks](https://www.meetup.com/triangle-rustaceans/events/mfglwpybchbhc/)


If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [Rust Back End Engineer, Core Banking - TrueLayer - Milan, Italy](https://apply.workable.com/truelayer/j/37748BA121/)

# Quote of the Week

> The whole motivation behind exceptions is to allow one to write ones business logic, concentrate on what one likes to think ones program will do, without having lots of fiddly error checking and handling code obscuring that logic. Error situations are therefore swept under the carpet with "try" and kept out of sight with "catch".
>
> However in my world view failure is not exceptional, it is a common happening, it's too important to be hidden away. Therefor failure handling should be in ones face in the code you write. Certainly in the face of those that read it.

– [ZiCog on rust-users](https://users.rust-lang.org/t/did-rust-make-the-right-choice-about-error-handling/41736/29)

Thanks to [Lzutao](https://users.rust-lang.org/t/twir-quote-of-the-week/328/872) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gmyv8h/this_week_in_rust_339/).</small>
