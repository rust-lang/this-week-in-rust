Title: This Week in Rust 269
Number: 269
Date: 2019-01-15
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

### #Rust2019

Find all #Rust2019 posts at [Read Rust](https://readrust.net/rust-2019/).

# Crate of the Week

This week's crate is [ropey](https://github.com/cessen/ropey), an editable text buffer data structure. Thanks to [Vikrant Chaudhary](https://users.rust-lang.org/t/crate-of-the-week/2704/477) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [A call for #Rust2019 Roadmap blog posts](https://blog.rust-lang.org/2018/12/06/call-for-rust-2019-roadmap-blogposts.html).
* [A call for #RustWasm2019 Roadmap blog posts](https://rustwasm.github.io/2018/12/06/reflecting-on-rust-and-wasm-in-2018.html).
* [medium] [Mundane: Run tests with ASan and MSan](https://github.com/google/mundane/issues/15). Mundane is a Rust cryptography library backed by BoringSSL.
* [medium] [Mundane: Test BoringSSL refcounting](https://github.com/google/mundane/issues/14).
* [easy] [Mundane: CONTRIBUTING.md: Document that you need to pull from googlesource.com](https://github.com/google/mundane/issues/12).
* [Tetra: Black screen/shader issues on MacOS](https://github.com/17cupsofcoffee/tetra/issues/54). Tetra is a 2D game framework written in Rust.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

189 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2019-01-07..2019-01-14

* [add miri to rustup](https://github.com/rust-lang/rustup.rs/pull/1606)
* [fix undefined behavior](https://github.com/rust-lang/rust/pull/57511)
* [resolve: mark extern crate items as used in more cases](https://github.com/rust-lang/rust/pull/57557)
* [clarify resolve typo suggestion](https://github.com/rust-lang/rust/pull/57477)
* [privacy: fix private-in-public check for existential types](https://github.com/rust-lang/rust/pull/57556)
* [tweak output of type mismatch between "then" and `else` `if` arms](https://github.com/rust-lang/rust/pull/57381)
* [use structured suggestion when casting a reference](https://github.com/rust-lang/rust/pull/57493)
* [use structured suggestions for nonstandard style lints](https://github.com/rust-lang/rust/pull/57387)
* [point at match discriminant on type error in match arm pattern](https://github.com/rust-lang/rust/pull/57366)
* [const-stabilize `const_int_ops` + `const_ip`](https://github.com/rust-lang/rust/pull/57234)
* [don't actually create a full MIR stack frame when not needed](https://github.com/rust-lang/rust/pull/57351)
* [speed up item_bodies for large match statements involving regions](https://github.com/rust-lang/rust/pull/57494)
* [change `String` to `&'static str` in `ParseResult::Failure`](https://github.com/rust-lang/rust/pull/57461)
* [parallelize and optimize parts of HIR map creation](https://github.com/rust-lang/rust/pull/57232)
* [stabilize cfg_target_vendor](https://github.com/rust-lang/rust/pull/57465)
* [stabilize cfg_attr_multi](https://github.com/rust-lang/rust/pull/57332)
* [stabilize core::convert::identity](https://github.com/rust-lang/rust/pull/57322)
* [stabilize `let` bindings and destructuring in constants and const fn](https://github.com/rust-lang/rust/pull/57175)
* [clean up and optimize OpenTask / read_index](https://github.com/rust-lang/rust/pull/57114)
* [NLL: add union justifications to conflicting borrows](https://github.com/rust-lang/rust/pull/57102)
* [fix and optimize query profiling](https://github.com/rust-lang/rust/pull/57095)
* [make `TokenStream` less recursive](https://github.com/rust-lang/rust/pull/57004)
* [replace LockCell with atomic types](https://github.com/rust-lang/rust/pull/56614)
* [make more passes incremental](https://github.com/rust-lang/rust/pull/51487)
* [librustc_mir: fix ICE with slice patterns](https://github.com/rust-lang/rust/pull/57538)
* [don't unwrap unexpected tokens in `format!`](https://github.com/rust-lang/rust/pull/57522)
* [stabilize `uniform_paths`](https://github.com/rust-lang/rust/pull/56759)
* [stabilize irrefutable if-let and while-let patterns](https://github.com/rust-lang/rust/pull/57535)
* [stabilize `if_while_or_patterns`](https://github.com/rust-lang/rust/pull/57532)
* [std: render large exit codes as hex on Windows](https://github.com/rust-lang/rust/pull/57473)
* [add `#[must_use]` to `Iterator` and `Future`](https://github.com/rust-lang/rust/pull/57549)
* [std: force `Instant::now()` to be monotonic](https://github.com/rust-lang/rust/pull/56988)
* [optimise floating point `is_finite` (2x) and `is_infinite` (1.6x)](https://github.com/rust-lang/rust/pull/57353)
* [`cargo --`{`example`,`bin`,`bench`,`test`} with no argument now lists all available targets](https://github.com/rust-lang/cargo/pull/6505)
* [rustup: fix `utils::copy_file` for symlink](https://github.com/rust-lang/rustup.rs/pull/1521)
* [rustdoc: allow inlining of reexported crates and crate items](https://github.com/rust-lang/rust/pull/57508)

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

* [disposition: merge] [Implement Debug, Eq, PartialEq, and Hash for libc structs](https://github.com/rust-lang/rfcs/pull/2235).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Summary issue for const-stabilizing `const_int_overflowing`](https://github.com/rust-lang/rust/issues/57237).
* [disposition: merge] [Const-stabilize `const_int_ops` + `const_ip`](https://github.com/rust-lang/rust/pull/57234).
* [disposition: merge] [Stabilize `let` bindings and destructuring in constants and const fn](https://github.com/rust-lang/rust/pull/57175).
* [disposition: merge] [Stablilize const_int_{rotate,wrapping,sign}](https://github.com/rust-lang/rust/pull/57105).
* [disposition: merge] [Stabilize `uniform_paths`](https://github.com/rust-lang/rust/pull/56759).
* [disposition: merge] [Stabilize the `integer_atomics` feature: Atomic{I,U}{8,16,32,64}](https://github.com/rust-lang/rust/issues/56753).
* [disposition: merge] [Stabilization proposal for #![feature(if_while_or_patterns)]](https://github.com/rust-lang/rust/issues/56212).
* [disposition: merge] [Tracking issue for RFC 2306, "Add core::convert::identity"](https://github.com/rust-lang/rust/issues/53500).
* [disposition: merge] [Tracking issue for write_all_at/read_exact_at convenience methods](https://github.com/rust-lang/rust/issues/51984).
* [disposition: merge] [Tracking issue for non-panicking pow](https://github.com/rust-lang/rust/issues/48320).
* [disposition: merge] [Tracking Issue for Result<Option> and Option<Result> Conversion](https://github.com/rust-lang/rust/issues/47338).

## New RFCs

* [Type Ascribed Coercions](https://github.com/rust-lang/rfcs/pull/2623).

# Upcoming Events

### Online

* [Jan 16. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).
* [Jan 23. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).

### Europe

* [Jan 10. Brno, CZ - Rust meetup at Masaryk University](https://rust-brno.github.io/).
* [Jan 14. Cologne, DE - Rust Cologne Meetup](https://www.meetup.com/RustCologne/events/vnwndpyzcbdb/).
* [Jan 15. Rome, IT - Rust Rome Meetup](https://www.meetup.com/Rust-Roma/events/257921654/).
* [Jan 22. Lyon, FR - TupperRust](https://tupperrust.github.io).
* [Jan 23. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzcbfc/).

### North America

* [Jan 10. Columbus, US - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dbcfrpyzcbnb/).
* [Jan 10. Utah, US - Utah Rust monthly meetup](https://www.meetup.com/utahrust/events/255209742/).
* [Jan 10. Arlington, US - Rust DC—Mid-month Rustful](https://www.meetup.com/RustDC/events/256380444).
* [Jan 13. Mountain view, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbrb/).
* [Jan 15. Los Angeles, US - Los Angeles Rust Meetup](https://www.meetup.com/Rust-Los-Angeles/events/257872752/).
* [Jan 20. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbbc/).
* [Jan 23. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzcbfc/).
* [Jan 23. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyzcbfc/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Intern (Summer 2019) at Mozilla, San Francisco, US](https://careers.mozilla.org/position/gh/1480831/).
* [Kernel Engineer at System76, Denver, US](https://system76.com/careers#kernel-engineer).
* [Senior Software Engineer at Prevoty, Los Angeles, US](https://www.prevoty.com/about/careers?gh_jid=4032159002).
* [Rust Developer at Parity, Berlin, DE](https://paritytech.io/jobs/).
* [volunteer] [UX Developer at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-ux-developers/321).
* [volunteer] [Team Artist at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-artists/319).
* [volunteer] [Core Developer at Amethyst](https://community.amethyst-engine.org/t/position-available-showcase-team-core-developers/320).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Right.  I've never even used this impl, but my first thought upon seeing the question "I have an `Iterator` of `X` and need a `Y`" was to look at the `FromIterator` impls of `Y`.
>
> If that impl *didn't* exist, I'd then look for the following:
>
> * Other `FromIterator<X>` impls for `String` to see if any of those `X` can easily be produced from `char` (and then I would call `map` before `.collect()`).
> * `impl FromIterator<char> for Vec<u8>`.  If this existed I would use `String::from_utf8(iterator.collect())`.
> * `impl Add<char> for String`.  If this existed, I would use `.fold(String::new(), |s, c| s + c)`
> * methods of [char](https://doc.rust-lang.org/std/primitive.char.html) to see if there's anything that lets you obtain the UTF8 bytes.  Indeed, there is `encode_utf8`, which even gives a `&mut str`, so one can write
>   ```rust
>   .fold(String::new(), |s, c| {
>       let mut buffer = [u8; 4];
>       s += &*c.encode_utf8(&mut buffer);
>       s
>   })
>   ```
> * idly check the [inherent methods of `String`](https://doc.rust-lang.org/std/string/struct.String.html) for whatever pops out at me
>
> and if I could still find nothing after all of that I'd slam my head into a wall somewhere.

– Michael Lamparski [on rust-users](https://users.rust-lang.org/t/iterator-of-char-into-string/24003/4)

Thanks to [Cauê Baasch De Souza](https://users.rust-lang.org/t/twir-quote-of-the-week/328/593) for the suggestion!

[Please submit your quotes for next week](http://users.rust-lang.org/t/twir-quote-of-the-week/328)!

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42), [llogiq](https://github.com/llogiq), and [Flavsditz](https://github.com/Flavsditz).*

<small>[Discuss on r/rust]().</small>
