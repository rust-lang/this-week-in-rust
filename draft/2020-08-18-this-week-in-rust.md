Title: This Week in Rust 352
Number: 352
Date: 2020-08-18
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/emberian/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

Check out [this week's *This Week in Rust Podcast*](https://audio.rustacean-station.org/file/rustacean-station/twir-2020-08-11.mp3)

# Updates from Rust Community


### Tooling


### Newsletters


### Observations/Thoughts

* [Why Rust is a great fit for embedded software](https://tweedegolf.nl/blog/39/why-rust-is-a-great-fit-for-embedded-software)
* [Why Rust's Unsafe Works](https://jam1.re/blog/why-rusts-unsafe-works)

### Learn Standard Rust


### Learn More Rust
[Linux Packages For Rust (2/3) - Building with GitHub Actions using Custom Actions and Docker Container Images](https://ebbflow.io/blog/vending-linux-2)

* [Temporarily opt-in to shared mutation](https://ryhl.io/blog/temporary-shared-mutation/)

### Project Updates

### Miscellaneous

# Crate of the Week

This week's crate is [bevy](https://crates.io/crates/bevy), a very capable yet simple game engine.

Thanks to [mmmmib](https://users.rust-lang.org/t/crate-of-the-week/2704/798) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

307 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2020-08-03..2020-08-10

* [add back unwinding support for Sony PSP](https://github.com/rust-lang/rust/pull/75280)
* [fix ICE when using asm! on an unsupported architecture](https://github.com/rust-lang/rust/pull/75227)
* [handle well known traits for more types](https://github.com/rust-lang/chalk/pull/591)
* [resolve `char` as a primitive even if there is a module in scope](https://github.com/rust-lang/rust/pull/75318)
* [forbid `#[track_caller]` on main](https://github.com/rust-lang/rust/pull/75130)
* [remove restriction on type parameters preceding consts w/ feature const-generics](https://github.com/rust-lang/rust/pull/74953)
* [implement the `min_const_generics` feature gate](https://github.com/rust-lang/rust/pull/74877)
* [tweak confusable idents checking](https://github.com/rust-lang/rust/pull/75349)
* [miri: accept some post-monomorphization errors](https://github.com/rust-lang/miri/pull/1503)
* [bubble up errors from `FileDescriptor::as_file_handle`](https://github.com/rust-lang/miri/pull/1501)
* [simplify `array::IntoIter`](https://github.com/rust-lang/rust/pull/75271)
* [polymorphize: unevaluated constants](https://github.com/rust-lang/rust/pull/75260)
* [instance: polymorphize upvar closures/generators](https://github.com/rust-lang/rust/pull/75255)
* [clean up const-hacks in int endianess conversion functions](https://github.com/rust-lang/rust/pull/75253)
* [add `as_mut_ptr` to `NonNull<[T]>`](https://github.com/rust-lang/rust/pull/75248)
* [make `MaybeUninit::as_`(`mut_`)`ptr` const](https://github.com/rust-lang/rust/pull/75250)
* [make `IntoIterator` lifetime bounds of `&BTreeMap` match with `&HashMap`](https://github.com/rust-lang/rust/pull/75203)
* [implement `into_keys` and `into_values` for associative maps](https://github.com/rust-lang/rust/pull/75163)
* [stabilize `Ident::new_raw`](https://github.com/rust-lang/rust/pull/75084)
* [limit I/O vector count on Unix](https://github.com/rust-lang/rust/pull/75005)
* [add `unsigned_abs` to signed integers](https://github.com/rust-lang/rust/pull/74759)
* [BTreeMap: better way to postpone root access in DrainFilter](https://github.com/rust-lang/rust/pull/75257)
* [hashbrown: do not iterate to drop if empty](https://github.com/rust-lang/hashbrown/pull/182)
* [hashbrown: relax bounds on HashSet constructors](https://github.com/rust-lang/hashbrown/pull/185)
* [hashbrown: avoid closures to improve compile times](https://github.com/rust-lang/hashbrown/pull/183)
* [stdarch: add more things that do adds](https://github.com/rust-lang/stdarch/pull/881)
* [futures: avoid writes without any data in write_all_vectored](https://github.com/rust-lang/futures-rs/pull/2187)
* [clean up rustdoc's `main()`](https://github.com/rust-lang/rust/pull/75124)
* [rustdoc: display elided lifetime for non-reference type in doc](https://github.com/rust-lang/rust/pull/75237)

## Rust Compiler Performance Triage

* [2020-08-11](https://github.com/rust-lang/rustc-perf/blob/master/triage/2020-08-11.md).
  1 regression, 1 improvements, 1 of them on rollups. No outstanding nags from last week.

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

*No Tracking Issues or PRs are currently in the final comment period.*

## New RFCs

* [Proposal for POSIX error numbers in `std::os::unix`](https://github.com/rust-lang/rfcs/pull/2973)
* [Standardize methods for leaking containers](https://github.com/rust-lang/rfcs/pull/2969)
* [Introduce '$self' macro metavar for hygienic macro items](https://github.com/rust-lang/rfcs/pull/2968)

# Upcoming Events

### Online
* [August 11. Saarbrücken, DE - Rust-Saar Meetup `3u16`](https://www.meetup.com/Rust-Saar/events/272044845/)
* [August 11. Dallas, TX, US - Dallas Rust - Second Tuesday](https://www.meetup.com/Dallas-Rust/events/mzzfsrybclbpb/)
* [August 13. San Diego, CA, US - San Diego Rust - August 2020 Tele-Meetup](https://www.meetup.com/San-Diego-Rust/events/272060817/)
* [August 19. Vancouver, BC, CA - Vancouver Rust - Rust Study/Hack/Hang-out Night](https://www.meetup.com/Vancouver-Rust/events/vcgsvrybclbzb/)
* [August 20. RustConf](https://rustconf.com/)

### North America
* [August 13. Columbus, OH, US - Columbus Rust Society - Monthly Meeting](https://www.meetup.com/columbus-rs/events/dpkhgrybclbrb/)
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

> You're not allowed to use references in structs until you think Rust is easy. They're the evil-hardmode of Rust that will ruin your day.

- [Kornel on rust-users](https://users.rust-lang.org/t/perpetual-n00b-struggling-with-ownership-again/46920/4)

Thanks to [Tom Phinney](https://users.rust-lang.org/t/twir-quote-of-the-week/328/918) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust]()</small>
