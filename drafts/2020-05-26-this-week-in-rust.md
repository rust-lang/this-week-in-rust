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


* [Compiling Rust binaries for Windows 98 SE and more: a journey](https://seri.tools/blog/compiling-rust-for-legacy-windows/)
* [Zero To Production #0: Foreword](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword/)
* [Some Extensive Projects Working with Rust](https://blog.knoldus.com/some-extensive-projects-working-with-rust/)
* [Writing Python inside your Rust code — Part 4](https://blog.m-ou.se/writing-python-inside-rust-4/)
* [Drawing SVG Graphics with Rust](https://cetra3.github.io/blog/drawing-svg-graphs-rust/)
* [Designing the Rust Unleash API Client](https://medium.com/cognite/designing-the-rust-unleash-api-client-6809c95aa568)
* [Conway's Game of Life on the NES in Rust](https://gridbugs.org/conways-game-of-life-on-the-nes-in-rust/)
* [How to organize your Rust tests](https://blog.logrocket.com/how-to-organize-your-rust-tests/)
* [Just: How I Organize Large Rust Programs](https://rodarmor.com/blog/tour-de-just)
* [Integrating Qt events into Actix and Rust](https://www.rubdos.be/corona/qt/rust/tokio/actix/2020/05/23/actix-qt.html)
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
* [audio] [Tech Except!ons: What Microsoft has to do with Rust? With Ryan Levick](https://anchor.fm/tech-exceptions/episodes/What-Microsoft-has-to-do-with-Rust--on-this-episode-with-Ryan-Levick-eec75h)
* [video] [Russian] [Rust: Not as hard as you think - Meta/conf: Backend Meetup 2020](https://www.youtube.com/watch?v=n3kyvMVck_M)
* [video] [3 Part Video for Beginners to Rust Porgramming on Iteration](https://tim.mcnamara.nz/post/618982870485172224/rust-iteration)
* [video] [Bringing WebAssembly outside the web with WASI by Lin Clark](https://www.youtube.com/watch?v=fh9WXPu0hw8)
* [video] [Microsoft's Safe Systems Programming Languages Effort](https://mybuild.microsoft.com/sessions/61de34c5-b111-4ece-928f-541854875862?source=sessions)
* [video] [Rust, WebAssembly, and the future of Serverless by Steve Klabnik](https://www.youtube.com/watch?v=CMB6AlE1QuI)

# Crate of the Week

This week's crate is [apply](https://crates.io/crates/apply), a tiny library for chaining free functions into method call chains.

Thanks to [Trevor Spiteri](https://users.rust-lang.org/t/crate-of-the-week/2704/769) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [pijama: easy issues](https://github.com/christianpoveda/pijama/issues?q=is%3Aissue+is%3Aopen+label%3AE-easy)
* [mdbx-rs: Add support for more compile time options](https://github.com/Kerollmops/mdbx-rs/issues/1)
* [ruma: Replace impl_enum! with strum derives](https://github.com/ruma/ruma-events/issues/90)
* [time-rs: Revamped parsing/formatting](https://github.com/time-rs/time/issues/236)
* [http-types: Request::query should match Tide's behavior](https://github.com/http-rs/http-types/issues/154)
* [http-types: Status should take TryInto<StatusCode/>](https://github.com/http-rs/http-types/issues/155)
* [http-types: Expose method shorthands for Request constructor](https://github.com/http-rs/http-types/issues/156)


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

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for `std::io::{BufReader, BufWriter}::capacity`](https://github.com/rust-lang/rust/issues/68833)
* [disposition: merge] [impl From<[T; N]\> for Box<[T]\>](https://github.com/rust-lang/rust/pull/71095)
* [disposition: merge] [Implement PartialOrd and Ord for SocketAddr*](https://github.com/rust-lang/rust/pull/72239)
* [disposition: merge] [Stabilize AtomicN::fetch_min and AtomicN::fetch_max](https://github.com/rust-lang/rust/pull/72324)
* [disposition: merge] [Resolve overflow behavior for RangeFrom](https://github.com/rust-lang/rust/pull/72368)
* [disposition: merge] [impl Step for char (make Range*<char\> iterable)](https://github.com/rust-lang/rust/pull/72413)
* [disposition: merge] [Stabilize core::panic::Location::caller](https://github.com/rust-lang/rust/issues/72448)
* [disposition: merge] [Stabilize str_strip feature](https://github.com/rust-lang/rust/pull/72466)


## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online

* [May 27. Montréal, QC, CA - Remote - RustMTL May 2020](https://www.meetup.com/Rust-Montreal/events/270635425)
* [May 27. Wrocław, PL - Remote - Rust Wrocław Meetup #20](https://www.meetup.com/Rust-Wroclaw/events/270771184/)
* [May 27. London, UK - Remote - LDN Talks May 2020](https://www.meetup.com/Rust-London-User-Group/events/270526235/)
* [May 28. Berlin, DE - Remote - Rust Hack and Learn](https://www.meetup.com/Rust-London-User-Group/events/270526235/)
* [June 3. Johannesburg, ZA - Remote - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/270827463/)
* [June 8. Auckland, NZ - Remote - Rust AKL](https://www.meetup.com/rust-akl/events/266876685/)
* [June 9. Seattle, WA - Remote - Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybcjbmb/)

### North America

* [June 3. Indianapolis, IN, US - Indy.rs Meetup](https://www.meetup.com/indyrs/events/dtqwprybcjbfb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

* [Rust Back End Engineer, Core Banking - TrueLayer - Milan, Italy](https://apply.workable.com/truelayer/j/37748BA121/)
* [Sr. SW Engineer (TW or Remote) - NZXT - Taipei, Taiwan](https://nzxt.bamboohr.com/jobs/view.php?id=233)
* [Rust Graphics Engineer - Elektron - Gotheburg, Sweden](https://www.elektron.se/rust-graphics-engineer/)


# Quote of the Week

> The whole motivation behind exceptions is to allow one to write ones business logic, concentrate on what one likes to think ones program will do, without having lots of fiddly error checking and handling code obscuring that logic. Error situations are therefore swept under the carpet with "try" and kept out of sight with "catch".
>
> However in my world view failure is not exceptional, it is a common happening, it's too important to be hidden away. Therefor failure handling should be in ones face in the code you write. Certainly in the face of those that read it.

– [ZiCog on rust-users](https://users.rust-lang.org/t/did-rust-make-the-right-choice-about-error-handling/41736/29)

Thanks to [Lzutao](https://users.rust-lang.org/t/twir-quote-of-the-week/328/872) for the suggestions!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/gmyv8h/this_week_in_rust_339/).</small>
