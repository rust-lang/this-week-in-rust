Title: This Week in Rust 353
Number: 353
Date: 2020-08-25
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://rustacean-station.org/episode/028-twir-352/)

# Updates from Rust Community

### Official

### Tooling
* [Contributing to the IntelliJ Rust plugin](https://kobzol.github.io/rust/intellij/2020/07/31/contributing-0-setup.html)

### Newsletters

### Observations/Thoughts

* [A Story of Rusty Containers, Queues, and the Role of Assumed Identity](https://dev.to/pnehrer/a-story-of-rusty-containers-queues-and-the-role-of-assumed-identity-kl2)
* [As above, so below: Bare metal Rust generics](https://www.ecorax.net/as-above-so-below-1/)

### Learn Standard Rust

### Learn More Rust

* [PL] [CrabbyBird #0 Pierwsza przygoda z Rustem i Godotem](https://postacnormalna.pl/pierwsza-przygoda-z-rustem-i-godotem/)
* [video][Build a Smart Bookmarking Tool with Rust and Rocket Video Series](https://www.youtube.com/playlist?list=PLzIwronG0sE56c6hDYOKW3-rPxmIyttoe)

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [pdf](https://github.com/pdf-rs/pdf), a crate for reading PDF files.

Thanks to [S3bk](https://users.rust-lang.org/t/crate-of-the-week/2704/806) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

292 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-17..2020-08-24

* [remove fast path in reallocation for same layout sizes](https://github.com/rust-lang/rust/pull/75621)
* [missing doc examples lint improvements](https://github.com/rust-lang/rust/pull/75776)
* [promote missing_fragment_specifier to hard error](https://github.com/rust-lang/rust/pull/75516)
* [polymorphize: if any param in a predicate is used, then all are used](https://github.com/rust-lang/rust/pull/75595)
* [make `OnceCell<T>` transparent to dropck](https://github.com/rust-lang/rust/pull/75648)
* [don't panic in `Vec::shrink_to_fit`](https://github.com/rust-lang/rust/pull/75677)
* [improve codegen for `align_offset`](https://github.com/rust-lang/rust/pull/75600)
* [add `Arc::new_cyclic`](https://github.com/rust-lang/rust/pull/75505)
* [new zeroed slice](https://github.com/rust-lang/rust/pull/75171)
* [make `<*const T>::is_null` const fn](https://github.com/rust-lang/rust/pull/74940)
* [stabilize `ptr_offset_from`](https://github.com/rust-lang/rust/pull/74238)
* [use `min_specialization` in libcore](https://github.com/rust-lang/rust/pull/73565)
* [const floating point bitcasts and classification](https://github.com/rust-lang/rust/pull/72449)
* [compiler-builtins: add mips/mips64 compiler-rt fallbacks so that libgcc is not required](https://github.com/rust-lang/compiler-builtins/pull/341)
+ [pin-utils: deprecate unsafe pin projection macros](https://github.com/rust-lang/pin-utils/pull/33)
* [git2: fix dangling pointer in format_email](https://github.com/rust-lang/git2-rs/pull/614)
* [git2: add support for zlib-ng](https://github.com/rust-lang/git2-rs/pull/612)
* [cargo: remove unnecessary allocations](https://github.com/rust-lang/cargo/pull/8641)
* [rust-bindgen: do generate unnamed enums, as they can be referred to by members and others](https://github.com/rust-lang/rust-bindgen/pull/1882)

## Rust Compiler Performance Triage

* [2020-08-24](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-24.md):
  1 regression, 4 improvements.
  
  This week included a major speedup on optimized builds of real-world crates (up to 5%) as a result of the [upgrade to LLVM 11](https://github.com/rust-lang/rust/pull/73526#issuecomment-679374070).

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

*No RFCs are currently in the final comment period.*

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize Range[Inclusive]::is_empty](https://github.com/rust-lang/rust/pull/75132)
* [disposition: merge][ stabilize ptr_offset_from](https://github.com/rust-lang/rust/pull/74238)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [August 19. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out Night](https://www.meetup.com/Vancouver-Rust/events/vcgsvrybclbzb/)
* [August 20. RustConf](https://rustconf.com/)
* [September 08. Saarbrücken, DE - Rust-Saar Meetup `3u16.map_err(...)`](https://www.meetup.com/Rust-Saar/events/272522454/)

### North America
* [August 25. Dallas, TX, US - Dallas Rust - Last Tuesday](https://www.meetup.com/Dallas-Rust/events/nppvrrybclbhc/)

### Asia Pacific
* [August 18. Seoul, KR - Rust Meetup - Rust last 6 months review (also available on Zoom)](https://www.meetup.com/Rust-Seoul-Meetup/events/qfkdvrybclbxb/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a very different beast for me. It is a *much* bigger and *much* more capable language. However, I've found that it is, in many ways, a lot more restrictive in how you can approach problems. I frequently find myself being perplexed at how to eloquently solve a problem. When I discover the idiomatic way of doing it I'm usually both blown away by the brilliance of it and a bit disheartened by how difficult it would be to come up with that solution by myself :-).

- [mikekchar on /r/rust](https://reddit.com/r/rust/comments/id8n8d/are_some_of_you_coming_from_javascript_ts/g27d3ni/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/931) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
