Title: This Week in Rust 210
Number: 210
Date: 2017-11-28
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

* [Nightly Rust compiler and Cargo now run on Redox](https://www.redox-os.org/news/this-week-in-redox-32/).
* [Announcing Failure](https://boats.gitlab.io/blog/post/2017-11-16-announcing-failure/).
* [Speed up your Python using Rust](https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/).
* [Evolving Rust with Milksnake](https://blog.sentry.io/2017/11/14/evolving-our-rust-with-milksnake).
* [Crates.io ecosystem not ready for embedded Rust](https://www.tockos.org/blog/2017/crates-are-not-safe/).
* [This week in Rust docs 82](https://guillaumegomez.github.io/this-week-in-rust-docs/blog/this-week-in-rust-docs-82).
* Notes from Rust+GNOME Hackfest in Berlin - by [Federico](https://people.gnome.org/~federico/blog/rust-gnome-hackfest-berlin.html), [Guillaume](https://blog.guillaume-gomez.fr/articles/2017-11-18+Rust%2BGNOME+Hackfest+in+Berlin), and [Antoyo](http://antoyo.ml/rust-gnome-hackfest-november).
* [podcast] [Rusty Spike Podcast - episode 8](https://rusty-spike.blubrry.net/2017/11/16/episode-8-nov-15-2017/). Firefox Quantum, Lin’s Quantum post, fearless concurrency, incremental typecheck, better wasm support, and Cargo on Redox.
* [podcast] [New Rustacean News: Rust 1.21 and 1.22](http://www.newrustacean.com/show_notes/news/rust_1_21_1_22/index.html). Quality of life improvements, Failure, wasm, and rustdoc fun – or, a bunch of highlights from the new releases *and* the community since 1.20.

# Crate of the Week

Sadly, this week saw no nomination, so it remains crateless.

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [Contribute to Rust's 2017 impl period](https://www.rustaceans.org/findwork/impl).
* [Molten - "a style-preserving TOML parser" has some easy and accessible issues for beginners](https://github.com/LeopoldArkham/Molten/issues).
* [easy] [mdbook: Select default theme](https://github.com/rust-lang-nursery/mdBook/issues/95).

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

118 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2017-11-20..2017-11-27

* [rustbuild: Enable WebAssembly backend by default](https://github.com/rust-lang/rust/pull/46115)
* [rustc: Add support for some more x86 SIMD ops](https://github.com/rust-lang/rust/pull/45367)
* [rustc: don't mark lifetimes as early-bound in the presence of impl Trait](https://github.com/rust-lang/rust/pull/46191)
* [implement in-band lifetime bindings](https://github.com/rust-lang/rust/pull/46051)
* [impl Trait Lifetime Handling](https://github.com/rust-lang/rust/pull/45701)
* [Display negative traits implementation](https://github.com/rust-lang/rust/pull/46134)
* [Properly handle reexport of foreign items](https://github.com/rust-lang/rust/pull/46129)
* [Make accesses to fields of packed structs unsafe](https://github.com/rust-lang/rust/pull/44884)
* [support `::crate` in paths](https://github.com/rust-lang/rust/pull/45771)
* [allocators: don’t assume MIN_ALIGN for small sizes](https://github.com/rust-lang/rust/pull/46117)
* [Kill the storage for all locals on returning terminators](https://github.com/rust-lang/rust/pull/46100)
* [incr.comp.: Make sure we don't lose unused green results from the query cache](https://github.com/rust-lang/rust/pull/46111)
* [`InstCombine Len([_; N]) => const N` in MIR](https://github.com/rust-lang/rust/pull/46264)
* [do match-check for consts](https://github.com/rust-lang/rust/pull/46033)
* [rustc_trans: don't apply noalias on returned references](https://github.com/rust-lang/rust/pull/46253)
* [allow filtering analysis by reachability](https://github.com/rust-lang/rust/pull/46011)
* [typeck aggregate rvalues in MIR type checker](https://github.com/rust-lang/rust/pull/46054)
* [add a MIR pass to lower 128-bit operators to lang item calls](https://github.com/rust-lang/rust/pull/46093)
* [add a MIR-borrowck-only output mode](https://github.com/rust-lang/rust/pull/46106)
* [MIR Borrowck: Parity with Ast for E0384 (Cannot assign twice to immutable)](https://github.com/rust-lang/rust/pull/46022)
* [add structured suggestions for various "use" suggestions](https://github.com/rust-lang/rust/pull/46035)
* [be more obvious when suggesting dereference](https://github.com/rust-lang/rust/pull/45947)
* [add hints for the case of confusing enum with its variants](https://github.com/rust-lang/rust/pull/45942)
* [dead code lint to say "never constructed" for variants](https://github.com/rust-lang/rust/pull/46103)
* [add process::parent_id](https://github.com/rust-lang/rust/pull/46092)
* [impl From for Mutex and RwLock](https://github.com/rust-lang/rust/pull/46082)
* [optimize `read_to_end`](https://github.com/rust-lang/rust/pull/46050)
* [make float::from_bits transmute](https://github.com/rust-lang/rust/pull/46012)
* [implement `Rc`/`Arc` conversions for string-like types](https://github.com/rust-lang/rust/pull/45990)
* [add Box::leak<'a>(Box<T>) -> &'a mut T where T: 'a](https://github.com/rust-lang/rust/pull/45881)
* [move closure kind, signature into `ClosureSubsts`](https://github.com/rust-lang/rust/pull/45879)
* [add RefCell<T>::replace_with](https://github.com/rust-lang/rust/pull/45819)
* [rustdoc: Fix path search](https://github.com/rust-lang/rust/pull/46081)
* [show in docs whether the return type of a function impls Iterator/Read/Write](https://github.com/rust-lang/rust/pull/45039)
* [rustdoc: include external files in documentation](https://github.com/rust-lang/rust/pull/44781) (RFC [#1990](https://rust-lang.github.io/rfcs/1990-external-doc-attribute.html))

## New Contributors

* Alexey Orlenko
* Benjamin Hoffmeyer
* Chris Vittal
* Collin Anderson
* Dan Gohman
* Jeff Crocker
* Laurentiu Nicola
* loomaclin
* Martin Lindhe
* Michael Lamparski
* Ramana Venkata
* Ritiek Malhotra
* Robert T Baldwin

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments)
process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

*No RFCs were approved this week.*

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now. [This week's FCPs][fcp] are:

[fcp]: https://github.com/rust-lang/rfcs/labels/final-comment-period

* [disposition: merge] [Fallible collection allocation 1.0](https://github.com/rust-lang/rfcs/pull/2116).
* [disposition: merge] [Implicit caller location (third try to the unwrap/expect line info problem)](https://github.com/rust-lang/rfcs/pull/2091).
* [disposition: merge] [Unsized rvalues](https://github.com/rust-lang/rfcs/pull/1909).
* [disposition: merge] [eRFC: Cargo build system integration](https://github.com/rust-lang/rfcs/pull/2136).
* [disposition: merge] [Type privacy and private-in-public lints](https://github.com/rust-lang/rfcs/pull/2145).

## New RFCs

* [Guard Clause Flow Typing](https://github.com/rust-lang/rfcs/pull/2221).

# Upcoming Events

* [Nov 24. Monkey Tech Days Toulouse, France - Explore Languages (Go Vs Rust) - MKTD#5](https://www.meetup.com/Monkey-Tech-Days/events/237545492/).
* [Nov 25. Rust Bangalore - Rust Concurrency (part 2 of 2)](https://www.meetup.com/rustox/events/244782966/).
* [Nov 27. Triangle Rustaceans Durham, NC - Algebraic Data Types in Practice and Theory](https://www.meetup.com/triangle-rustaceans/events/kkjnpnywpbkc).
* [Nov 29. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Nov 29. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Nov 29. OpenTechSchool Berlin - Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/krnczlywpbmc).
* [Nov 30. Rust Munich: Rust Machine Learning with Juice](https://www.meetup.com/rust-munich/events/244580709/).
* [Nov 30. Rust Detroit - Introducing Tock OS 1.0](https://www.meetup.com/rust-detroit/events/244855856/).
* [Nov 30. Rust release triage](https://internals.rust-lang.org/t/release-cycle-triage-proposal/3544).
* [Dec  6. Rust Community Team Meeting at #rust-community on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-community).
* [Dec  6. Rust Documentation Team Meeting at #rust-docs on irc.mozilla.org](https://chat.mibbit.com/?server=irc.mozilla.org&channel=%23rust-docs).
* [Dec  6. Rust Cologne: impl Glühwein](https://www.meetup.com/RustCologne/events/244487721/).
* [Dec  6. Rust Atlanta: Grab a beer with fellow Rustaceans](https://www.meetup.com/Rust-ATL/events/rhvgrmywqbjb/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Contract opportunity @ Mozilla: Distributed compilation cache written in Rust](https://users.rust-lang.org/t/contract-opportunity-mozilla-distributed-compilation-cache-written-in-rust/13898).

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> Rust's abstraction layers feel both transparent and productive. It's like being on a glass-bottomed boat, you see the sharks, but they can't get you.
> It's like a teaching language that you can also use in production. Rust helped me understand C.
> Also Rust people are amazing.

— [@gibfahn on Twitter](https://twitter.com/gibfahn/status/931187143686393863).

Thanks to [@sebasmagri](https://twitter.com/sebasmagri/status/931246295439650816) for the suggestion!

[Submit your quotes for next week][submit]!

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328

*This Week in Rust is edited by: [nasa42](https://github.com/nasa42) and [llogiq](https://github.com/llogiq).*
