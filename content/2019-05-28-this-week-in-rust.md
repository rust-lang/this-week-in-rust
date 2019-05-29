Title: This Week in Rust 288
Number: 288
Date: 2019-05-28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# Updates from Rust Community

## News & Blog Posts

* 🎈🎉 [Announcing Rust 1.35.0](https://blog.rust-lang.org/2019/05/23/Rust-1.35.0.html). 🎉🎈
* [Update on await syntax](https://boats.gitlab.io/blog/post/await-decision-ii/).
* [Writing a compiler in Rust](http://thume.ca/2019/04/18/writing-a-compiler-in-rust/).
* [Announcing Mockiato - A strict, yet friendly mocking library for Rust 2018](https://blog.myelin.ch/2019/05/24/mockiato-announcement.html).
* [Programming Servo: Zen and the art of removing blocks from your system](https://medium.com/@polyglot_factotum/programming-servo-zen-and-the-art-of-removing-blocks-from-your-system-51c1b7d404e3).
* [Cross compiling and statically linking against Rust libraries](https://medium.com/csis-techblog/cross-compiling-and-statically-linking-against-rust-libraries-2c02ee2c01af).
* [Rebuffing the attack of the clones - a newbie's guide to `clone`](https://thenewwazoo.github.io/clone.html).
* [New Rustacean Meta 3](https://newrustacean.com/show_notes/meta/_3/) - A story and a dream (and the promise of Rust): the final episode of New Rustacean!
* [Erebor](http://erebor.io), a Rust consultancy startup by [Alexander Regueiro](https://github.com/alexreg).

# Crate of the Week

This week's crate is [mockiato](https://github.com/myelin-ai/mockiato), a strict yet friendly mocking library for Rust 2018. Thanks to [Ruben Schmidmeister](https://users.rust-lang.org/t/crate-of-the-week/2704/550) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

*No issues were proposed for CfP*.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

286 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-05-20..2019-05-27

* [Turn turbo 🐟 🍨 into an error](https://github.com/rust-lang/rust/pull/61189)
* [Remove `ObsoleteInPlace`](https://github.com/rust-lang/rust/pull/60803)
* [Make place projections concrete](https://github.com/rust-lang/rust/pull/60441)
* [Simplify use of keyword symbols](https://github.com/rust-lang/rust/pull/60740)
* [Fix overflowing literal lint in loops](https://github.com/rust-lang/rust/pull/61098)
* [Use `Symbol` even more](https://github.com/rust-lang/rust/pull/60815)
* [Use `Symbol` more in lint APIs](https://github.com/rust-lang/rust/pull/60827)
* [Move gensym operations from `Symbol` to `Ident`](https://github.com/rust-lang/rust/pull/60903)
* [Avoid symbol interning in `file_metadata`](https://github.com/rust-lang/rust/pull/60973)
* [Avoid more symbol interning](https://github.com/rust-lang/rust/pull/61035)
* [Don't arena-allocate static symbols](https://github.com/rust-lang/rust/pull/61077)
* [rustc: Improve type size assertions](https://github.com/rust-lang/rust/pull/60959)
* [Allow null-pointer-optimized enums in FFI if their underlying representation is FFI safe](https://github.com/rust-lang/rust/pull/60300)
* [Preserve local scopes in generator MIR](https://github.com/rust-lang/rust/pull/60840)
* [Annotate each `reverse_bits` with `#[must_use]`](https://github.com/rust-lang/rust/pull/61134)
* [Vec: Avoid creating slices to the elements](https://github.com/rust-lang/rust/pull/61114)
* [Fix dangling reference in `Vec::append`](https://github.com/rust-lang/rust/pull/61082)
* [crates.io: Further address performance regression in search](https://github.com/rust-lang/crates.io/pull/1749)
* [rustbuild: Add clippy and fix commands to x.py](https://github.com/rust-lang/rust/pull/56595)

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Tracking issue for RFC 1789: Conversions from `&mut T` to `&Cell<T>`](https://github.com/rust-lang/rust/issues/43038).
* [disposition: merge] [Tracking issue for reversing the bit pattern in an integer](https://github.com/rust-lang/rust/issues/48763).
* [disposition: merge] [Stabilize rustdoc theme options](https://github.com/rust-lang/rust/pull/54733).
* [disposition: merge] [Bors policy question: Auto-reassignment on r+](https://github.com/rust-lang/rust/issues/59489).
* [disposition: merge] [Stabilize RefCell::try_borrow_unguarded](https://github.com/rust-lang/rust/pull/60850).
* [disposition: merge] [Stabilize `std::arch::wasm32::unreachable`](https://github.com/rust-lang/rust/issues/61119).
* [disposition: merge] [Add std::mem::take as suggested in #61129](https://github.com/rust-lang/rust/pull/61130).

## New RFCs

* [Add generalized arity tuples](https://github.com/rust-lang/rfcs/pull/2702).

# Upcoming Events

### Africa

* [Jun  5. Johannesburg, ZA - Johannesburg Rust Meetup - informal discussions on topics related to the language](https://www.meetup.com/Johannesburg-Rust-Meetup/events/gpxrtqyzhbcb/).

### Asia Pacific

* [Jun 10. Auckland, NZ - Rust AKL - WASM - the past, present and future](https://www.meetup.com/rust-akl/events/259480660/).

### Europe

* [Jun  5. Clermont-Ferrand, FR - Clermont'ech: Rust Workshop](https://www.clermontech.org/workshops/workshop-3-rust.html).
* [Jun  6. Wroclaw, PL - Rust Wroclaw Meetup #11](https://www.meetup.com/Rust-Wroclaw/events/261283360/).
* [Jun 12. Berlin, DE - OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/gkkttqyzjbqb/).

### North America

* [Jun  5. Atlanta, GA, US - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/kkzkxqyzjbhb/).
* [Jun  5. Indianapolis, IN, US - Indy.rs](https://www.meetup.com/indyrs/events/mffbtpyzjbhb/).
* [Jun 11. Detroit, MI, US - Detroit Rust - June Detroit Rust at Bamboo](https://www.meetup.com/rust-detroit/events/244855856/).
* [May 14. Redmond, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/nzfspqyzjbpb/).
* [Jun 13. San Diego, CA, US - San Diego Rust May Meetup](https://www.meetup.com/San-Diego-Rust/events/261595821/).
* [Jun 13. Arlington, VA, US - Rust DC — Mid-month Rustful](https://www.meetup.com/RustDC/events/261239650).
* [Jun 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dbcfrpyzjbrb/).
* [Jun 12. Vancouver, BC, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/fzqqwqyzjbqb/).
* [Jun 18. Denver, CO, US - Rust Boulder/Denver - Rust Meetup for June](https://www.meetup.com/Rust-Boulder-Denver/events/259124426/).

### South America

* [Jun 1. Sao Paulo, BR - Rust SP - Encontro Junho 2019](https://www.meetup.com/Rust-Sao-Paulo-Meetup/events/261123153/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [DevOps Storage Engineer at Comcast, Pennsylvania, US](https://career8.successfactors.com/sfcareer/jobreqcareer?jobId=198894).
* [Rust Developer at enhance, London, UK or remote](https://enhance.com/#jobs).
* [Rust Developer at Kaspersky Lab, Moscow, RU](https://careers.kaspersky.com/job/Developer-%D0%BD%D0%B0-Rust-(QA-Team%2C-KasperskyOS)/561880800/?locale=en_EU).
* [Blockchain Runtime Engineer at Parity, Berlin, DE or remote](https://www.parity.io/jobs/#berlin-blockchain-runtime-engineer).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I used to think of programs as execution flowing and think about what the CPU is doing. As I moved to rust I started thinking a lot more about memory: how the data was laid out in memory, and how ownership of different parts of memory is given to different parts of the program at run time.

[Oliver Gould on "The Open Source Show: All About Rust](https://youtu.be/FYGS2q1bljE?t=280)

Thanks to [PrototypeNM1](https://users.rust-lang.org/t/twir-quote-of-the-week/328/643) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/budr02/this_week_in_rust_288/).</small>
