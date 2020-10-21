Title: This Week in Rust 361
Number: 361
Date: 2020-10-21
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

No newsletters this week.

### Official
* [Marking issues as regressions](https://blog.rust-lang.org/2020/10/20/regression-labels.html)
* [Inside] [Lang team Backlog Bonanza and Project Proposals](https://blog.rust-lang.org/inside-rust/2020/10/16/Backlog-Bonanza.html)

### Tooling
* [Rust-Analyzer Changelog #47](https://rust-analyzer.github.io/thisweek/2020/10/19/changelog-47.html)
* [IntelliJ Rust Changelog #133](https://intellij-rust.github.io/2020/10/19/changelog-133.html)

### Observations/Thoughts
* [Fearless concurrency: how Clojure, Rust, Pony, Erlang and Dart let you achieve that.](https://sites.google.com/a/athaydes.com/renato-athaydes/posts/fearlessconcurrencyhowclojurerustponyerlanganddartletyouachievethat)
* [Shock Result<>?: Rust faster than Python in one test of file parsing](http://www.coralbark.net/blog/technology/2020/10/shock-result-rust-faster-than-python-in-one-test-of-file-parsing/)
* [Building a Recipe Manager - Part 2 - Druid Experience Report](https://bheisler.github.io/post/recipe-manager-part-2-druid-experience-report/)
* [No, C++ still isn't cutting it.](https://da-data.blogspot.com/2020/10/no-c-still-isnt-cutting-it.html)
* [A pitfall of Rust's move/copy/drop semantics and zeroing data](https://benma.github.io/2020/10/16/rust-zeroize-move.html)
* [Proving that 1 + 1 = 10 in Rust](https://tavianator.com/2020/one_plus_one.html)

### Learn Simple Rust
* [Building Your Own Error Type: Part 1](https://yaah.dev/building-your-own-error-type)
* [Lifetimes in Rust](https://blog.thoughtram.io/lifetimes-in-rust/)
* [Piece by Piece: Write Readable Rust Code](https://impl.dev/posts/write-readable-rust-code/)
* [Are out paramters idiomatic in Rust?](https://steveklabnik.com/writing/are-out-parameters-idiomatic-in-rust)
* [Non-Generic Inner Functions](https://www.possiblerust.com/pattern/non-generic-inner-functions)
* [Creating a Snake Clone in Rust, with Bevy](https://mbuffett.com/posts/bevy-snake-tutorial/)
* [Create Your Own PineTime Watch Face in Rust... And Publish on crates.io](https://lupyuen.github.io/pinetime-rust-mynewt/articles/watchface)
* [Either Types for Rust](https://dev.to/sirech/either-types-for-rust-46k4)
* [Rust syntax: What is the questionmark?](https://dev.to/nickymeuleman/rust-syntax-what-the-questionmark-2n58)
* [Private Methods on a Public Trait](https://jack.wrenn.fyi/blog/private-trait-methods/)
* [Learn Rust with Benford's Law](https://gliderkite.github.io/posts/learn-rust-with-benford/)
* [How to Write Hygienic Rust Macros](https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1)
* [video] [Rust Linz, October 2020 - Valentin Tolmer - How not to rely on inheritance](https://youtu.be/m6Gee5kNe7U)

### Learn More Rust
* [Building a runtime reflection system for Rust 🦀️ (Part 2)](https://www.osohq.com/post/runtime-reflection-pt-2)
* [Compile Rust for Raspberry Pi ARM](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050)
* [Basic non-blocking IO using epoll in Rust](https://zupzup.org/epoll-with-rust/)
* [video] [Rust Linz, October 2020 - Matthias Heiden - Writing a Kernel Driver with Rust](https://youtu.be/wREGR7QQHco)

### Project Updates
* [Announcing Tokio 0.3 and the path to 1.0](https://tokio.rs/blog/2020-10-tokio-0-3)

### Miscellaneous
* [A new look, tickets and what's to come](https://blog.rustfest.eu/a-new-look)
* [ICU4X Project Announcement](https://github.com/unicode-org/icu4x/wiki/ICU4X-Project-Announcement)
* [Kata Containers rewritten in Rust gets a major speed boost](https://www.zdnet.com/article/kata-containers-rewritten-in-rust-and-gets-a-major-speed-boost/)
* [Assorted thoughts on zig (and rust)](https://scattered-thoughts.net/writing/assorted-thoughts-on-zig-and-rust/)
* [Flask Creator Armin Ronacher Interview](https://evrone.com/armin-ronacher-interview)
* [A recipe for start using Rust actix-web and launch chrome 🚀](https://itnext.io/a-recipe-for-starting-actix-web-server-and-launch-chrome-b792987935a)
* [Sailfish OS 3.4 Released with Experimental Rust Support, Finally Eyeing 64-bit ARM](https://www.phoronix.com/scan.php?page=news_item&px=Sailfish-OS-3.4-Released)

# Call for Blog Posts

The Rust Core Team wants input from the community!
If you haven't already, [read the official blog](https://blog.rust-lang.org/2020/09/03/Planning-2021-Roadmap.html) and submit a blog post - it will show up here!
Here are the wonderful submissions since the call for blog posts:

# Crate of the Week

This week's crate is [paste](https://crates.io/crates/paste), a macro to concatenate identifiers (which would otherwise be nightly only).

Thanks to [mark-i-m](https://users.rust-lang.org/t/crate-of-the-week/2704/825) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

409 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-10-05..2020-10-12

* [resolve: improve "try using the enum's variant"](https://github.com/rust-lang/rust/pull/77341)
* [Fix `LitKind`'s byte buffer to use refcounted slice](https://github.com/rust-lang/rust/pull/77560)
* [Replace `(Body, DefId)` with `Body` where possible](https://github.com/rust-lang/rust/pull/77552)
* [perf: `UninhabitedEnumBranching` avoid n²](https://github.com/rust-lang/rust/pull/77597)
* [Fix span for unicode escape suggestion](https://github.com/rust-lang/rust/pull/77587)
* [Implement `advance_by`, `advance_back_by` for `iter::Chain`](https://github.com/rust-lang/rust/pull/77594)
* [Add `PartialEq` impls for `Vec` ↔ `slice`](https://github.com/rust-lang/rust/pull/74194)
* [stdsimd: Use xor to implement `Neg::neg` for floats](https://github.com/rust-lang/stdsimd/pull/31)

## Rust Compiler Performance Triage

* [2020-10-13](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-13.md):
0 Regressions, 3 Improvements, 3 Mixed

Overall, fairly busy week, but without major regressions that need to be addressed.

See the [full report](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-10-13.md) for more.

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)
* [RFC: Promote aarch64-unknown-linux-gnu to a Tier-1 Rust target](https://github.com/rust-lang/rfcs/pull/2959)
* [Access to traits' associated functions and constants from trait objects](https://github.com/rust-lang/rfcs/pull/2886)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [stabilize union with 'ManuallyDrop' fields and 'impl Drop for Union'](https://github.com/rust-lang/rust/pull/77547)
* [disposition: merge] [stop promoting union field accesses in 'const'](https://github.com/rust-lang/rust/pull/77526)
* [disposition: merge] [passes: `check_attr` on more targets](https://github.com/rust-lang/rust/pull/77015)
* [disposition: merge] [resolve: Do not put nonexistent crate `meta` into prelude](https://github.com/rust-lang/rust/pull/75802)
* [disposition: postpone][Tracking issue for experiments around coercions, generics, and Copy type ergonomics](https://github.com/rust-lang/rust/issues/44619)

## New RFCs


# Upcoming Events

### Online
* [Octover 15. Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprrybcnbtb/)
* [October 20. Denver, CO, US - Rust Denver - Data Science with Rust](https://www.meetup.com/Rust-Boulder-Denver/events/272996842/)
* [October 21. New York, NY, US - A Journey into the Nucleus at Dropbox with Parker Timmerman - Rust NYC](https://www.meetup.com/Rust-NYC/events/273887563)
* [October 21. Vancover, BC, CA - Rust Study/Hack/Hang-out night - Vancouver Rust](https://www.meetup.com/Vancouver-Rust/events/cxrtxrybcnbcc/)
* [October 22. Edinburgh, UK - Fluence: interface-types for server-side WebAssembly modules - Rust Edinburgh](https://www.meetup.com/rust-edi/events/273685985)
* [October 27. Dallas, TX, US - Last Tuesday - Dallas Rust](https://www.meetup.com/Dallas-Rust/events/jqxqwrybcnbkc/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Just because Rust allows you to write super cool non-allocating zero-copy algorithms safely, doesn’t mean every algorithm you write should be super cool, zero-copy and non-allocating.

- [trentj on rust-users](https://users.rust-lang.org/t/feeling-rust-is-so-difficult/29962/15)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/948) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/iu3ge0/this_week_in_rust_356/)</small>
