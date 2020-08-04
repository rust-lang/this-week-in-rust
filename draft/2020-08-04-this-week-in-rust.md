Title: This Week in Rust 350
Number: 350
Date: 2020-08-04
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/cmr/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/024-twir-349/)

# Updates from Rust Community

### Official
* [Announcing Rust 1.45.1](https://blog.rust-lang.org/2020/07/30/Rust-1.45.1.html)
* [Announcing Rust 1.45.2](https://blog.rust-lang.org/2020/08/03/Rust-1.45.2.html)

### Tooling
* [Rust Analyzer Changelog #36](https://www.reddit.com/r/rust/comments/i2wic3/rustanalyzer_changelog_36/)
* [IntelliJ Rust: Updates for the 2020.2 Release](https://blog.jetbrains.com/clion/2020/08/intellij-rust-updates-for-the-2020-2-release/)
* [Headcrab: July 2020 progress report](https://headcrab.rs/2020/07/31/july-update.html)

### Newsletters
* [This Month in Rust OSDev (July 2020)](https://rust-osdev.com/this-month/2020-07/)

### Observations/Thoughts

### Learn
* [Learning Rust: Mindsets and Expectations](https://ferrous-systems.com/blog/mindsets-and-expectations/)
* [Understanding the Rust borrow checker](https://blog.logrocket.com/introducing-the-rust-borrow-checker/)
* [Beginner's guide to Error Handling in Rust](http://www.sheshbabu.com/posts/rust-error-handling/)
* [Rust Hashmaps Tutorial](https://frogtok.com/very-simple-guide-to-rust-hashmaps/)
* [Blue Team Rust: What is "Memory Safety", really?](https://tiemoko.com/blog/blue-team-rust/)
* [How to use the Rust compiler as your integration testing framework](https://blog.logrocket.com/using-the-rust-compiler-as-your-integration-testing-framework/)
* [A Comprehensive Tutorial to Rust Operators for Beginners](https://towardsdatascience.com/a-comprehensive-tutorial-to-rust-operators-for-beginners-11554b2c64d4)
* [What You Should Know About Unsigned, Signed Integers and Casting in Rust](https://towardsdatascience.com/unsinged-signed-integers-and-casting-in-rust-9a847bfc398f)
* [Memory safety in Rust - part 2](https://hashrust.com/blog/memory-safety-in-rust-part-2/)
* [A gentle introduction to assembly with rust](https://lfn3.net/2020/08/03/a-gentle-intro-to-assembly-with-rust/)
* [Creating Linux Packages for Rust Projects (1/2)](https://ebbflow.io/blog/vending-linux-1)
* [Prevent Breaking Code Changes in Future Releases using `non exhaustive` enums in Rust](https://blog.knoldus.com/prevent-breaking-code-changes-in-future-releases-using-non-exhaustive-enums-in-rust/)
* [Operating System development tutorials in Rust on the Raspberry Pi](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials#operating-system-development-tutorials-in-rust-on-the-raspberry-pi)
* [Reverse Engineering a USB Device with Rust](https://gill.net.in/posts/reverse-engineering-a-usb-device-with-rust/)
* [A Heaping Helping of Stacks](https://deislabs.io/posts/a-heaping-helping-of-stacks/)
* [A Simple Crud on Rust (With Rocket.rs and Diesel.rs)](https://medium.com/swlh/a-simple-crud-on-rust-with-rocket-rs-and-diesel-rs-e885672cb23d)
* [Some Learnings from Implementing a Normalizing Rust Representer](https://seanchen1991.github.io/posts/rust-representer/)
* [Countdown problem in Rust](https://amitdev.github.io/posts/2020-07-27-countdown-rust/)
* [A Curious Tale of Rust TLS and Postgres in the Cloud](https://dev.to/pnehrer/a-curious-tale-of-rust-tls-and-postgres-in-the-cloud-434k)
* [video][Learning Rust by Working Through the Rustlings Exercises](https://egghead.io/playlists/learning-rust-by-solving-the-rustlings-exercises-a722)
* [video][Hypercore Protocol in Rust](https://www.youtube.com/watch?v=2JCblJf9hFg&list=PL7sG5SCUNyeYx8wnfMOUpsh7rM_g0w_cu)
* [video][Reasonable Coding 030 - HotStuff, a composable, no-nonsense document compiler (part 1)](https://www.youtube.com/watch?v=kURv5ZbkEss)
* [video][Rusty Days 2020 - all videos](https://www.youtube.com/playlist?list=PLf3u8NhoEikhTC5radGrmmqdkOK-xMDoZ)

### Project Updates

* [Rust Language Cheat Sheet 2019 -> 2020](https://github.com/ralfbiedert/cheats.rs/issues/100)

### Miscellaneous

* [Video recording technology at RustFest](https://estada.ch/2020/7/30/video-recording-technology-at-rustfest/)
* [audio][The State of Rust 2 with Alex Chrichton](https://anchor.fm/the-virtual-world/episodes/Ep-7--The-State-of-Rust-2-with-Alex-Crichton-ehjpsq)
* [audio][The State of Rust with Steve Klabnik](https://anchor.fm/the-virtual-world/episodes/Ep-6--The-State-of-Rust-with-Steve-Klabnik-ehf8mk)

# Crate of the Week

This week's crate is [polyfuse](https://github.com/ubnt-intrepid/polyfuse), a library for writing FUSE file systems using async code.

Thanks to [Ivan Kozik](https://users.rust-lang.org/t/crate-of-the-week/2704/795) for the suggestion!

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

347 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-07-20..2020-07-27

* [AVR: correctly set the pointer address space when constructing pointers to functions](https://github.com/rust-lang/rust/pull/73270)
* [detect turbofish missing surrounding angle brackets](https://github.com/rust-lang/rust/pull/74687)
* [serialize span hygiene data](https://github.com/rust-lang/rust/pull/72121)
* [proc_macro: add API for tracked access to environment variables](https://github.com/rust-lang/rust/pull/74653)
* [correctly deal with unsorted generic parameters](https://github.com/rust-lang/rust/pull/74676)
* [normalize bounds fully when checking defaulted types](https://github.com/rust-lang/rust/pull/74670)
* [disallow non-static lifetimes in const generics](https://github.com/rust-lang/rust/pull/74051)
* [forbid generic parameters in anon consts inside of type defaults](https://github.com/rust-lang/rust/pull/74487)
* [add a system for creating diffs across multiple mir optimizations](https://github.com/rust-lang/rust/pull/74715)
* [optimize away `BitAnd` and `BitOr` when possible](https://github.com/rust-lang/rust/pull/74491)
* [make more primitive integer methods const](https://github.com/rust-lang/rust/pull/73858)
* [impl Default for ranges](https://github.com/rust-lang/rust/pull/73197)
* [remove needless unsafety from `BTreeMap::drain_filter`](https://github.com/rust-lang/rust/pull/74677)
* [hashbrown: refactor probing logic into an external iterator](https://github.com/rust-lang/hashbrown/pull/181)
* [rustlings: add ability to run rustlings on repl.it](https://github.com/rust-lang/rustlings/pull/471)

## Rust Compiler Performance Triage

* [2020-07-28](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-07-28.md).
  2 regressions, 1 improvement, none in rollups.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC: 'C unwind' ABI](https://github.com/rust-lang/rfcs/pull/2945)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

*No Tracking Issues or PRs are currently in the final comment period.*

## New RFCs

* [Procedural vtables and wide ptr metadata](https://github.com/rust-lang/rfcs/pull/2967)
* [Edition 2021 and beyond](https://github.com/rust-lang/rfcs/pull/2966)

# Upcoming Events

### Online
* [August 5. Johannesburg, ZA - Johannesburg Rust Meetup - Monthly Joburg Rust Chat](https://www.meetup.com/Johannesburg-Rust-Meetup/events/271875886/)
* [August 5. Dublin, IE - Rust Dublin - August Remote Meetup](https://www.meetup.com/Rust-Dublin/events/272162980/)
* [August 5. Buffalo, NY, US - Buffalo Rust Meetup - Rust User Group](https://www.meetup.com/Buffalo-Rust-Meetup/events/271511557/)
* [August 5. Indianapolis, IN, US - Indy Rust - Indy.rs with Social Distancing](https://www.meetup.com/indyrs/events/jhfstrybclbhb/)
* [August 6. Linz, AT - Rust Meetup Linz - Kick Off](https://www.meetup.com/de-DE/Rust-Linz/events/271857182/)
* [August 6. Berlin, DE - Berline.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybclbjb/)
* [August 11. Seattle, WA, US - Seattle Rust Meetup - Monthly meetup](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrybclbpb/)
* [August 11. Saarbrücken, DE - Rust-Saar Meetup `3u16`](https://www.meetup.com/Rust-Saar/events/272044845/)
* [August 13. San Diego, CA, US - San Diego Rust - August 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/272060817/)

### North America
* [August 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybclbrb/)

### Asia Pacific

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

Sadly, we had no quote suggestions this week.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/i094wo/this_week_in_rust_349/)</small>
