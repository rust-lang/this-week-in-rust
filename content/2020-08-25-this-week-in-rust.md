Title: This Week in Rust 353
Number: 353
Date: 2020-08-26
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/emberian/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/emberian/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Tooling
* [Contributing to the IntelliJ Rust plugin](https://kobzol.github.io/rust/intellij/2020/07/31/contributing-0-setup.html)
* [IntelliJ Rust Changelog #129](https://intellij-rust.github.io/2020/08/24/changelog-129.html)
* [Rust Analyzer Changelog #39](https://rust-analyzer.github.io/thisweek/2020/08/24/changelog-39.html)

### Newsletters
* [This Week in Veloren 81](https://veloren.net/devblog-81/)
* [The Embedded Working Group Newsletter - 25](https://rust-embedded.github.io/blog/newsletter-25/)

### Observations/Thoughts
* [A Story of Rusty Containers, Queues, and the Role of Assumed Identity](https://dev.to/pnehrer/a-story-of-rusty-containers-queues-and-the-role-of-assumed-identity-kl2)
* [As above, so below: Bare metal Rust generics](https://www.ecorax.net/as-above-so-below-1/)
* [First thoughts on Rust vs OCaml](https://blog.darklang.com/first-thoughts-on-rust-vs-ocaml/)
* [The problem of safe FFI bindings in Rust](https://www.abubalay.com/blog/2020/08/22/safe-bindings-in-rust)
* [That's so Rusty!](https://dev.to/imaculate3/that-s-so-rusty-3akm)
* [Profiling Doesn't Always Have To Be Fancy](https://www.justanotherdot.com/posts/profiling-doesnt-always-have-to-be-fancy.html)
* [Why I like Piston, A Rust game engine](https://kai.coding.blog/why-i-like-piston)
* [The CXX Debate](https://steveklabnik.com/writing/the-cxx-debate)
* [video] [Bending the curve: A personal tutor at your fingertips](https://youtu.be/Z6X7Ada0ugE)

### Learn Standard Rust
* [Why can't I early return in an if statement in Rust?](https://www.christopherbiscardi.com/why-can-t-i-early-return-in-an-if-statement-in-rust)
* [Bleed Less during Runtime with Rust's Lifetime](https://blog.knoldus.com/bleed-less-during-runtime-with-rusts-lifetime/)
* [A Javascript Developer's Cheatsheet for Rust](https://tndl.me/blog/2020/rust-javascript-cheatsheet/)
* [video] [RustConf 2020 - Macros for a More Productive Rust](https://youtu.be/dZiWkbnaQe8)
* [video] [RustConf 2020 - Rust for Non-Systems Programmers](https://youtu.be/BBvcK_nXUEg)

### Learn More Rust
* [Different levels of Async in Rust](https://www.fpcomplete.com/blog/different-levels-async-rust/)
* [As above, so blow: Bare metal Rust generics 1/2](https://www.ecorax.net/as-above-so-below-1/)
* [How to run Rust on Arduino Uno](https://dev.to/creativcoder/how-to-run-rust-on-arduino-uno-40c0)
* [Let's build a single binary gRPC server-client with Rust in 2020 - Part 1](https://dev.to/tjtelan/let-s-build-a-single-binary-grpc-server-client-with-rust-in-2020-part-1-3cnk)
* [Writing a Test Case Generator for a Programming Language](https://fitzgeraldnick.com/2020/08/24/writing-a-test-case-generator.html)
* [Day 12: Write web app with actix-web - 100DaysOfRust](https://dev.to/0xbf/day11-write-web-app-with-actix-web-100dayofrust-1lkn)
* [Kernel printing with Rust](https://not-matthias.github.io/kernel-printing-with-rust/)
* [Running Animation in Amethyst](https://mtigley.dev/posts/running-animation/)
* [PL] [CrabbyBird #0 Pierwsza przygoda z Rustem i Godotem](https://postacnormalna.pl/pierwsza-przygoda-z-rustem-i-godotem/)
* [video] [Build a Smart Bookmarking Tool with Rust and Rocket Video Series](https://www.youtube.com/playlist?list=PLzIwronG0sE56c6hDYOKW3-rPxmIyttoe)
* [video] [RustConf 2020 - Error Handling Isn't All About Errors](https://youtu.be/rAF8mLI0naQ)
* [video] [RustConf 2020 - Under a Microscope: Exploring Fast and Safe Rust for Biology](https://youtu.be/2b8InauuRqw)
* [video] [RustConf 2020 - My First Rust Project: Starting a 2D game with Amethyst](https://youtu.be/GFi_EdS_s_c)
* [video] [RustConf 2020 - Controlling Telescope Hardware with Rust](https://youtu.be/xlVnp7VOxRE)

### Project Updates
* [Rustsim becomes Dimforge](https://www.dimforge.com/blog/2020/08/18/rustsim-becomes-dimforge/)
* [Announcing the Rapier physics enginge](https://www.dimforge.com/blog/2020/08/25/announcing-the-rapier-physics-engine/)
* [defmt, a highly efficient Rust logging framework for embedded devices](https://ferrous-systems.com/blog/defmt/)
* [Scaling Bevy](https://bevyengine.org/news/scaling-bevy/)
* [One year of Nushell](http://www.nushell.sh/blog/2020/08/23/year_of_nushell.html)

### Miscellaneous
* [Rust-style futures in C](https://axelforsman.tk/2020/08/24/rust-style-futures-in-c.html)
* [Porting a Golang and Rust CLI tool to D](https://blog.pingfrommorocco.com/2020/08/porting-golang-and-rust-cli-tool-to-d.html)
* [How to Read First Impression Posts](https://www.possiblerust.com/community/how-to-read-first-impression-posts)
* [audio] [The Virtual World - Embedded Rust & Ferrous Systems - James Munns](https://anchor.fm/the-virtual-world/episodes/Embedded-Rust--Ferrous-Systems---James-Munns-eidv76)
* [audio] [Rust GameDev Podcast - Interview with Herbert Wolverson (Bracket-Lib)](https://rustgamedev.com/episodes/interview-with-herbert-wolverson-bracket-lib)
* [video] [Before Main: How Executables Work on Linux](https://youtu.be/jR2hUhjcAXI)
* [video] [RustConf 2020 - Opening Keynote](https://youtu.be/IwPRu5FhfIQ)
* [video] [RustConf 2020 - How to Start a Solo Project that You'll Stick With](https://youtu.be/yv6L_xmjw5I)
* [video] [RustConf 2020 - Closing Keynote](https://youtu.be/RNsEsZbXE-4)

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

* [Rusty Celery has several issues tagged with "Help Wanted"](https://github.com/rusty-celery/rusty-celery/issues?q=label%3A%22Status%3A+Help+Wanted%22)

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

* [Update the intra-doc link RFC to match the implementation](https://github.com/rust-lang/rfcs/pull/2975)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge][Stabilize deque_make_contiguous](https://github.com/rust-lang/rust/pull/74559)
* [disposition: merge][Tracking issue for hint::spin_loop (renamed sync::atomic::spin_loop_hint)](https://github.com/rust-lang/rust/issues/55002)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [September 2. Johannesburg, ZA - Monthly Joburg Rust Chat! - Johannesburg Rust Meetup](https://www.meetup.com/Johannesburg-Rust-Meetup/events/272786420/)
* [September 2. Dublin, IE - Rust Dublin September Remote Meetup](https://www.meetup.com/Rust-Dublin/events/272781420/?action=rsvp&response=yes)
* [September 2. Indianapolis, IN, US - Indy Rust - Indy.rs - with Social Distancing](https://www.meetup.com/indyrs)
* [September 3. Berlin, DE - Berlin.rs - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/txcprrybcmbfb/)
* [September 8. Saarbrücken, DE - Rust-Saar Meetup - `3u16.map_err(...)`](https://www.meetup.com/Rust-Saar/events/272522454/)

### North America
* [September 9. Atlanta, GA, US - Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/qxqdgrybcmbmb/)

### Asia Pacific
* [September 7. Auckland, NZ - Rust AKL - Finally, good Rust IDE support in VSCode: rust-analyzer](https://www.meetup.com/rust-akl/events/266876702/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Senior Engineer - Embedded Rust at Ockam (Remote)](https://www.ockam.io/team/Senior-Engineer-Embedded-Rust/31b8be44-ca35-5e12-8777-ab16d7b08033)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust is a very different beast for me. It is a *much* bigger and *much* more capable language. However, I've found that it is, in many ways, a lot more restrictive in how you can approach problems. I frequently find myself being perplexed at how to eloquently solve a problem. When I discover the idiomatic way of doing it I'm usually both blown away by the brilliance of it and a bit disheartened by how difficult it would be to come up with that solution by myself :-).

- [mikekchar on /r/rust](https://reddit.com/r/rust/comments/id8n8d/are_some_of_you_coming_from_javascript_ts/g27d3ni/)

Thanks to [Stephan Sokolow](https://users.rust-lang.org/t/twir-quote-of-the-week/328/931) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/ih8uai/this_week_in_rust_353/)</small>
