Title: This Week in Rust 200
Number: 200
Date: 2017-09-19
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

* [impl Future for Rust](https://blog.rust-lang.org/2017/09/18/impl-future-for-rust.html). An update on 2017 roadmap.
* [Josh Triplett joins the Cargo team](https://internals.rust-lang.org/t/please-welcome-josh-triplett-to-the-cargo-team/5898).
* [Discover the world of microcontrollers through Rust](https://japaric.github.io/discovery/).
* [Rust By Example has been ported to mdBook](https://github.com/rust-lang/rust-by-example/pull/897).
* [Rust is one of the most energy efficient languages](https://sites.google.com/view/energy-efficiency-languages).
* [This week in Rust docs 73](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-73).
* [podcast] [New Rustacean - crates you show know: Rayon](http://www.newrustacean.com/show_notes/cysk/rocket/). Safe, threaded, parallel code in Rust!

# Crate of the Week

This week's crate is [rug](https://crates.io/crates/rug), a crate providing arbitrary-precision integers, rationals and floating-point numbers, using GMP, MPFR and MPC.
Thank you, [Trevor Spiteri](https://users.rust-lang.org/u/tspiteri) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Findwork: Find something Rusty to work on](https://www.rustaceans.org/findwork/). An entry point to a number of open issues across the Rust project.
* [Help us libz blitz this impl period](https://www.reddit.com/r/rust/comments/70zi34/help_us_libz_blitz_this_impl_period/)!
* [Libz blitz: Out-of-band crate evaluation for 2017-10-09: semver](https://internals.rust-lang.org/t/out-of-band-crate-evaluation-for-2017-10-09-semver/5929).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

160 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-09-04..2017-09-11

* [accept underscores in unicode escapes](https://github.com/rust-lang/rust/pull/43716)
* [evaluate fixed-length array length expressions lazily](https://github.com/rust-lang/rust/pull/44275)
* [remove overzealous Box<ZeroSizeType> optimization](https://github.com/rust-lang/rust/pull/44553)
* [fix end region emission order](https://github.com/rust-lang/rust/pull/44129)
* [fix regression in promotion of rvalues referencing a static](https://github.com/rust-lang/rust/pull/44442)
* [fix "new trace_macros doesn't work if there's an error during expansion"](https://github.com/rust-lang/rust/pull/44088)
* [remove `HirId` from queries](https://github.com/rust-lang/rust/pull/44435)
* [remove `Session::dep_graph`](https://github.com/rust-lang/rust/pull/44502)
* [get `allow(unused_mut)` to work on `let` bindings](https://github.com/rust-lang/rust/pull/44590)
* [Fix mispositioned error indicators](https://github.com/rust-lang/rust/pull/44386)
* [run the miri test suite on the aux builder and travis](https://github.com/rust-lang/rust/pull/43628) (will make future type system work easier)
* [autodetect the type of allocator crate used](https://github.com/rust-lang/rust/pull/44133)
* [preallocate when building the dep graph](https://github.com/rust-lang/rust/pull/44586) (good memory savings)
* [remove rustc_bitflags; use the bitflags crate](https://github.com/rust-lang/rust/pull/44441)
* [specialize `<FlatMap as Iterator>::fold`](https://github.com/rust-lang/rust/pull/44577) (great speedup)
* [`Ipv4Addr` and `Ipv6Addr` convenience constructors](https://github.com/rust-lang/rust/pull/44395)
* [add `_.or_default()` to `Entry` APIs](https://github.com/rust-lang/rust/pull/44344)
* [additional traits for `std::mem::ManuallyDrop`](https://github.com/rust-lang/rust/pull/44310)
* [implement `<Rc<Any>>::downcast`](https://github.com/rust-lang/rust/pull/44273)
* [implement `Arc`/`Rc` raw pointer conversions for `?Sized`](https://github.com/rust-lang/rust/pull/44073)
* [`impl Hasher for` {`&mut Hasher`, `Box<Hasher>`}](https://github.com/rust-lang/rust/pull/44015)
* [remove `Borrow` bound from `SliceExt::binary_search`](https://github.com/rust-lang/rust/pull/43989)
* [implement unsafe pointer methods](https://github.com/rust-lang/rust/pull/43964)
* [individualize feature gates for `const fn` invocation](https://github.com/rust-lang/rust/pull/43017)
* [remove deprecated lang items](https://github.com/rust-lang/rust/pull/44526)
* [stabilize `iterator_for_each`](https://github.com/rust-lang/rust/pull/44567)
* [stabilize `drop_types_in_const`](https://github.com/rust-lang/rust/pull/44456)
* [stabilize `tcpstream_connect_timeout`](https://github.com/rust-lang/rust/pull/44563)
* [stabilize `compiler_fences`](https://github.com/rust-lang/rust/pull/44595)
* [stabilize `ord_max_min`](https://github.com/rust-lang/rust/pull/44593)
* [rustdoc: Compile fail stable](https://github.com/rust-lang/rust/pull/43949)
* [cargo: Infer targets from subdirectories](https://github.com/rust-lang/cargo/pull/4496)
* [crates.io sends an email to a user to confirm their email address](https://github.com/rust-lang/crates.io/pull/1045)

## New Contributors

* 42triangles
* David Adler
* Gauri Kholkar
* Ixrec
* J. Cliff Dyer
* Michal Budzynski
* rwakulszowa
* smt923
* Trevor Merrifield

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [RFC 2052: Evolving Rust through Epochs](https://github.com/rust-lang/rfcs/pull/2052).
* [RFC 2000: Const generics](https://github.com/rust-lang/rfcs/pull/2000).
* [RFC 1977: Introduce a public/private distinction to crate dependencies](https://github.com/rust-lang/rfcs/pull/1977).
* [RFC 2126: Clarify and streamline paths and visibility](https://github.com/rust-lang/rfcs/pull/2126).
* [RFC 2071: Named existentials and impl Trait variable declarations](https://github.com/rust-lang/rfcs/pull/2071).
* [RFC 1826: Change the default URL of doc.rust-lang.org](https://github.com/rust-lang/rfcs/pull/1826).
* [RFC 2103: Attributes for tools, 2.0](https://github.com/rust-lang/rfcs/pull/2103).
* [RFC 2113: `dyn Trait` syntax for trait objects: Take 2](https://github.com/rust-lang/rfcs/pull/2113).

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Non-lexical lifetimes](https://github.com/rust-lang/rfcs/pull/2094).
* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: postpone] [Infer function signatures from trait declaration into 'impl's](https://github.com/rust-lang/rfcs/pull/2063).
* [disposition: merge] [`cfg!(target_feature)` and `#[target_feature]`](https://github.com/rust-lang/rfcs/pull/2045).
* [disposition: merge] [Make the `assert!` macro generic to all expressions, and extend the readability of debug dumps](https://github.com/rust-lang/rfcs/pull/2011).
* [disposition: merge] [Add external doc attribute to rustc](https://github.com/rust-lang/rfcs/pull/1990).
* [disposition: postpone] [Tuple-based variadic generics](https://github.com/rust-lang/rfcs/pull/1935).
* [disposition: merge] [Unsized Rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: postpone] [`'fn` lifetime ascription](https://github.com/rust-lang/rfcs/pull/1847). Add a `'fn` lifetime that is bound to the scope of the body of the current innermost function or closure.
* [disposition: postpone] [Default struct field values](https://github.com/rust-lang/rfcs/pull/1806).
* [disposition: postpone] [Introduce `Option::<&T>::borrowed`](https://github.com/rust-lang/rfcs/pull/1792).
* [disposition: merge] [Support defining C-compatible variadic functions in Rust](https://github.com/rust-lang/rfcs/pull/2137).
* [disposition: merge] [Add support to Cargo for alternative registries](https://github.com/rust-lang/rfcs/pull/2141).

## New RFCs

* [Raw Identifiers](https://github.com/rust-lang/rfcs/pull/2151). Add a raw identifier format `r#ident`, so crates written in future language epochs/versions can still use an older API that overlaps with new keywords.
* [new `rand_core` crate, `rand` adaptations](https://github.com/rust-lang/rfcs/pull/2152).
* [Debuginfo-based panic locations](https://github.com/rust-lang/rfcs/pull/2154).

# Upcoming Events

* [Sep 21. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Sep 23: Rust Hungary Kickoff](https://www.meetup.com/Rust-Hungary-Meetup/events/242955063/).
* [Sep 24. Rust Indonesia - Level up Your Knowledge on Rust](https://www.eventbrite.com/e/level-up-your-knowledge-on-rust-tickets-36755777520).
* [Sep 25. Rust Paris Meetup #38](https://www.meetup.com/Rust-Paris/events/243110057/).
* [Sep 25. Rust Durham, NC - Becoming a Contributor / Chris Krycho](https://www.meetup.com/triangle-rustaceans/events/243047099/).
* [Sep 27. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Sep 27. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Sep 28. Mozilla Community Dresden - Rust Meetup](https://www.meetup.com/Mozilla-Community-Dresden/events/242610304/).
* **[Sep 30 - Oct 1. RustFest Zürich](http://zurich.rustfest.eu).**
* [Sep 30. Rust Mexico #8: Taller Introductorio a Rust y Rocket](https://www.meetup.com/Rust-MX/events/243334902/).
* [Oct  2 - Oct 3. Impl Days at RustFest Zürich](https://github.com/RustFestEU/blog.rustfest.eu/issues/29).
* [Oct  4. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Oct  4. Rust Atlanta - Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/243164851/).
* [Oct  4. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Oct  4. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/243084182/).
* [Oct  5. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust developers at Æternity](https://blog.aeternity.com/join-the-t%C3%A6m-rust-or-erlang-devs-wanted-31908daba788).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> \<heycam\> one of the best parts about stylo has been how much easier it has been to implement these style system optimizations that we need, because Rust
> \<heycam\> can you imagine if we needed to implement this all in C++ in the timeframe we have
> \<bholley\> heycam: yeah srsly
> \<bholley\> heycam: it's so rare that we get fuzz bugs in rust code
> \<bholley\> heycam: considering all the complex stuff we're doing
> * heycam remembers getting a bunch of fuzzer bugs from all kinds of style system stuff in gecko
> \<bholley\> heycam: think about how much time we could save if each one of those annoying compiler errors today was swapped for a fuzz bug tomorrow :-)
> \<njn\> you guys sound like an ad for Rust

— [Conversation between some long-time Firefox developers](http://logs.glob.uno/?c=mozilla%23servo&s=13+Sep+2017&e=13+Sep+2017#c751661).

Thanks to [Josh Matthews](https://users.rust-lang.org/t/twir-quote-of-the-week/328/452) for the suggestion.

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
