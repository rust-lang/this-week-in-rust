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

* [Writing an OS in Rust: Introduction to paging](https://os.phil-opp.com/paging-introduction/).
* [Librsvg is almost rustified now](https://people.gnome.org/~federico/blog/librsvg-is-almost-rustified.html).
* [Understanding Rust lifetimes](https://medium.com/nearprotocol/understanding-rust-lifetimes-e813bcd405fa).
* [const types, traits and implementations in Rust](https://varkor.github.io/blog/2019/01/11/const-types-traits-and-implementations-in-Rust.html).
* [Generate Rust tests from data files](https://blog.cyplo.net/posts/2018/12/generate-rust-tests-from-data.html).
* [The evolution of a Rust programmer](http://antoyo.ml/evolution-rust-programmer).

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

* [easy] [Mundane: Rename `Signature::verify` to `is_valid`](https://github.com/google/mundane/issues/16).
* [easy] [Crater: Test Clippy lint](https://github.com/rust-lang-nursery/crater/issues/388).
* [Review source code of yaserde: Yet another serializer/deserializer](https://users.rust-lang.org/t/twir-call-for-participation/4821/226).

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

* [disposition: close] [Using enums like traits](https://github.com/rust-lang/rfcs/pull/2618).
* [disposition: close] [Local `loop` bindings](https://github.com/rust-lang/rfcs/pull/2617).
* [disposition: close] [Item-level blocks (was: Item-level *scopes*)](https://github.com/rust-lang/rfcs/pull/2377).

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [Add signed num::NonZeroI* types](https://github.com/rust-lang/rust/pull/57475).
* [disposition: merge] [Stabilize extern_crate_self](https://github.com/rust-lang/rust/pull/57407).
* [disposition: merge] [Stabilize the `integer_atomics` feature: Atomic{I,U}{8,16,32,64}](https://github.com/rust-lang/rust/issues/56753).

## New RFCs

* [Amend RFC 1242 to specify what “minimal maintenance” means for r-l-deprecated crates](https://github.com/rust-lang/rfcs/pull/2624).

# Upcoming Events

### Online

* [Jan 23. Rust Community Team Meeting on Discord](https://discordapp.com/channels/442252698964721669/443773747350994945).
* [Jan 30. Rust Events Team Meeting on Telegram](https://t.me/joinchat/EkKINhHCgZ9llzvPidOssA).

### Asia

* [Jan 17. Hong Kong, HK - HK Functional Programming Meetup: Fullstack Web Development in Rust](https://www.meetup.com/HK-Functional-programming/events/256805970/).

### Europe

* [Jan 20. St.Petersburg, RU - St. Petersburg Rust Meetup](https://www.meetup.com/spbrust/events/gzjnmqyzcbbc/).
* [Jan 22. Lyon, FR - TupperRust](https://tupperrust.github.io).
* [Jan 23. Berlin, DE - Berlin Rust Hack and Learn](https://www.meetup.com/opentechschool-berlin/events/rjgkhqyzcbfc/).
* [Jan 24. Hamburg, DE - Rust Hack & Learn Hamburg](https://www.meetup.com/Rust-Meetup-Hamburg/events/257153030/).
* [Jan 24. Frankfurt, DE - How to Start in Embedded Rust](https://www.meetup.com/Rust-Rhein-Main/events/257833713/).
* [Jan 30. Toulouse, FR - Rust Toulouse](https://www.meetup.com/fr-FR/Toulouse-Rust-Meetup/events/257926837/).

### North America

* [Jan 20. Mountain View, US - Rust Dev in Mountain View!](https://www.meetup.com/Rust-Dev-in-Mountain-View/events/glnfcpyzcbbc/).
* [Jan 23. Ann Arbor, US - Ann Arbor Rust Meetup](https://www.meetup.com/Ann-Arbor-Rust-Meetup/events/cgsskqyzcbfc/).
* [Jan 23. Vancouver, CA - Vancouver Rust meetup](https://www.meetup.com/Vancouver-Rust/events/rzszlqyzcbfc/).
* [Jan 28. North Carolina, US - Triangle Rustaceans](https://www.meetup.com/triangle-rustaceans/).
* [Jan 30. Chicago, US - Chicago Rust Meetup - Property-Based Testing in Rust](https://www.meetup.com/Chicago-Rust-Meetup/events/257469240/).

### South America

* [Jan 16. Ciudad de México, MX - Rust MX: primera reunión del 2019](https://www.meetup.com/Rust-MX/events/257791793/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

* [Rust Developer at Devolutions, Quebec, CA](https://devolutions.net/careers/openings/rust-developer).
* [Senior Backend Engineer at Kraken, Remote](https://jobs.lever.co/kraken/4c864c8f-bde6-443d-b521-dd90df0e9105).

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

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/agma6t/this_week_in_rust_269/).</small>
