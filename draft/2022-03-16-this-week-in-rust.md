Title: This Week in Rust 434
Number: 434
Date: 2022-03-16
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Miscellaneous

## Crate of the Week

This week's crate is [noline](https://crates.io/crates/noline), a small no-std compatible readline-like line editor.

A lack of suggestions notwithstanding, llogiq is pretty pleased with his choice.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

302 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-03-07..2022-03-14

* [diagnostics: do not spuriously claim something is "not an iterator"](https://github.com/rust-lang/rust/pull/94870)
* [diagnostics: single colon within `<>` probably, not type ascription](https://github.com/rust-lang/rust/pull/94865)
* [improve suggestion when casting `usize` to (possibly) wide pointer](https://github.com/rust-lang/rust/pull/92150)
* [warn users about `||` in `let` chain expressions](https://github.com/rust-lang/rust/pull/94754)
* [suggest `if let`/`let_else` for refutable pat in `let`](https://github.com/rust-lang/rust/pull/94739)
* [suggest using double colon when a struct field type include single colon](https://github.com/rust-lang/rust/pull/94839)
* [miri: implement `simd_`{`shuffle`, `gather`, `scatter`}](https://github.com/rust-lang/miri/pull/2013)
* [CTFE/Miri: detect out-of-bounds pointers in `offset_from`](https://github.com/rust-lang/rust/pull/94827)
* [change several `HashMap`s to `IndexMap` to improve incremental hashing performance](https://github.com/rust-lang/rust/pull/90253)
* [improve `AdtDef` interning](https://github.com/rust-lang/rust/pull/94733)
* [optimize `ascii::escape_default`](https://github.com/rust-lang/rust/pull/94776)
* [make some `Clone` impls `const`](https://github.com/rust-lang/rust/pull/91804)
* [remove argument from closure in `thread::Scope::spawn`](https://github.com/rust-lang/rust/pull/94559)
* [use `MaybeUninit` in `VecDeque` to remove the undefined behavior of slice](https://github.com/rust-lang/rust/pull/94472)
* [constify `Index`{,`Mut`} for `[T]`, `str`, and `[T; N]`](https://github.com/rust-lang/rust/pull/94657)
* [fix soundness issue in scoped threads](https://github.com/rust-lang/rust/pull/94644)
* [implement `BITS` constant for non-zero integers](https://github.com/rust-lang/rust/pull/93292)
* [implement `MIN`/`MAX` constants for non-zero integers](https://github.com/rust-lang/rust/pull/93293)
* [add `Result::`{`ok`, `err`, `and`, `or`, `unwrap_or`} as `const`](https://github.com/rust-lang/rust/pull/92385)
* [add `Atomic`*`::get_mut_slice`](https://github.com/rust-lang/rust/pull/94816)
* [add `core::hint::must_use`](https://github.com/rust-lang/rust/pull/94723)
* [unix: reduce the size of `DirEntry`](https://github.com/rust-lang/rust/pull/94750)
* [portable-simd: add `.min` and `.max` for integers](https://github.com/rust-lang/portable-simd/pull/260)
* [compiler-builtins: add support for Apple watchOS](https://github.com/rust-lang/compiler-builtins/pull/456)
* [futures: add `Mutex::lock_owned` and `Mutex::try_lock_owned`](https://github.com/rust-lang/futures-rs/pull/2571)
* [rustfmt: improve mod resolution error for mods with multiple candidate files](https://github.com/rust-lang/rustfmt/pull/5243)
* [clippy: improve styles of filtering options for Clippy's lint list](https://github.com/rust-lang/rust-clippy/pull/8070)
* [clippy: new lint that detects useless match expression](https://github.com/rust-lang/rust-clippy/pull/8471)
* [clippy: new lint: `only_used_in_recursion`](https://github.com/rust-lang/rust-clippy/pull/8422)
* [clippy: allow `single_component_path_imports` for all macros](https://github.com/rust-lang/rust-clippy/pull/8537)
* [clippy: make `search_is_some`s suggestion `MachineApplicable`](https://github.com/rust-lang/rust-clippy/pull/8536)

### Rust Compiler Performance Triage

A pretty smooth week. All three regressions were small, and two were isolated to rustdoc alone.

Triage done by **@pnkfelix**.
Revision range: [f0c4da49..10dccdc7](https://perf.rust-lang.org/?start=f0c4da49983aa699f715caf681e3154b445fb60b&end=10dccdc7fcbdc64ee9efe2c1ed975ab8c1d61287&absolute=false&stat=instructions%3Au)

3 Regressions, 3 Improvements, 1 Mixed; 2 of them in rollups
53 comparisons made in total

[Full report here](https://github.com/rust-lang/rustc-perf/blob/master/triage/2022-03-09.md)

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* *No new or updated RFCs were submitted this week.*

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [disposition: merge] [Cargo alternative registry auth](https://github.com/rust-lang/rfcs/pull/3139)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

* [disposition: merge] [Consistently present absent stdio handles on Windows as NULL handles.](https://github.com/rust-lang/rust/pull/93263)
* [disposition: merge] [Stabilize ADX target feature](https://github.com/rust-lang/rust/pull/93745)
* [disposition: merge] [proc-macro: Stop wrapping ident matchers into groups](https://github.com/rust-lang/rust/pull/92472)
* [disposition: merge] [Tracking Issue for RFC #2972: Constrained Naked Functions](https://github.com/rust-lang/rust/issues/90957)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)

* [new] [Edition Based Method Disambiguation: Preventing inference ambiguity breakages with extension trait methods](https://github.com/rust-lang/rfcs/pull/3240)
* [update RFC #2991] [RFC: Add target configuration](https://github.com/rust-lang/rfcs/pull/3239)

## Upcoming Events

Rusty Events between 2022-03-09 - 2022-04-06 🦀

### Virtual

* 2022-03-09 | Boulder, CO, US | [Boulder Elixir and Rust](https://www.meetup.com/boulder-elixir-rust/)
    * [**Monthly Meetup**](https://www.meetup.com/boulder-elixir-rust/events/283985719/)
* 2022-03-09 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284379686/)
* 2022-03-09 | München, DE | [Rust Munich](https://www.meetup.com/rust-munich/)
    * [**Rust Munich Remote(?) #10**](https://www.meetup.com/rust-munich/events/283790509/)
* 2022-03-09 | Selangor, MY | [Rust Malaysia](https://t.me/golangmalaysia)
    * [**Rust Meetup**](https://forms.gle/35pipPdsKm1VFzCa9)
* 2022-03-09 | Stuttgart, DE | [Rust Community Stuttgart](https://www.meetup.com/Rust-Community-Stuttgart/)
    * [**Rust-Meetup**](https://www.meetup.com/Rust-Community-Stuttgart/events/284068315)
* 2022-03-10 | Charlottesville, VA, US | [Charlottesville Rust Meetup](https://www.meetup.com/Charlottesville-Rust-Meetup/)
    * [**Bluetoothing with Rust using BlueR**](https://www.meetup.com/Charlottesville-Rust-Meetup/events/284152560/)
* 2022-03-15 | Berlin, DE | [OpenTechSchool Berlin](https://www.meetup.com/de-DE/opentechschool-berlin/)
    * [**Rust Hack and Learn**](https://www.meetup.com/de-DE/opentechschool-berlin/events/284205132/)
* 2022-03-15 | Dublin, IE | [Rust Dublin](https://www.meetup.com/Rust-Dublin/)
    * [**Rust Dublin March Meetup**](https://www.meetup.com/Rust-Dublin/events/283613905)
* 2022-03-15 | Rome, IT | [Rust-Roma](https://www.meetup.com/Rust-Roma/)
    * [**Packaging and deploying a rust application with cargo-deb #Aperitech**](https://www.meetup.com/Rust-Roma/events/284425486/)
* 2022-03-15 | Washington, DC, US | [Rust DC](https://www.meetup.com/RustDC/)
    * [**Rust and Tell Lightning Talks! ⚡🦀**](https://www.meetup.com/RustDC/events/283374540/)
* 2022-03-16 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284221983/)
* 2022-03-16 | Seattle, WA, US | [Seattle Rust Meetup](https://www.meetup.com/Seattle-Rust-Meetup/)
    * [**Monthly meetup**](https://www.meetup.com/Seattle-Rust-Meetup/events/gskksrydcfblb/)
* 2022-03-16 | Vancouver, BC, CA | [Vancouver Rust](https://www.meetup.com/Vancouver-Rust/)
    * [**Building a Randomizer**](https://www.meetup.com/Vancouver-Rust/events/283910183/)
* 2022-03-23 | Egg Harbor City, NJ, US | [Neighborhood Math Club](https://www.meetup.com/neighborhood-math-club/)
    * [**The Early Rustacean Gets The Worm!**](https://www.meetup.com/neighborhood-math-club/events/284379020/)
* 2022-04-05 | Buffalo, NY, US | [Buffalo Rust Meetup](https://www.meetup.com/Buffalo-Rust-Meetup/)
    * [**Buffalo Rust User Group, First Tuesdays**](https://www.meetup.com/Buffalo-Rust-Meetup/events/284342808/)
* 2022-04-06 | Indianapolis, IN, US | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/284244527/)

### Europe

* 2022-03-13 | Kyiv, UA | [Вивчаємо Rust Разом / Learn Rust Together](https://t.me/learn_rust_together_ukr)
    *[**Ukrainian Rusty Dinner**](https://t.me/learn_rust_together_ukr)
* 2022-03-15 | Sofia, BG | [Rust Meetup Sofia](https://www.meetup.com/rust-meetup-sofia/)
    * [**Rust Meetup Sofia - 1st Edition**](https://www.meetup.com/rust-meetup-sofia/events/284141594)

### North America

* 2022-03-14 | Atlanta, GA, US | [Atlanta Rustaceans](https://twitter.com/atl_rustaceans/)
    * [**_New_ Atlanta Rust Meetup**](https://twitter.com/atl_rustaceans/status/148958647136758989) 
* 2022-03-15 | San Francisco, CA, US | [San Francisco Rust Study Group](https://www.meetup.com/san-francisco-rust-study-group/)
    * [**Rust Hacking in Person**](https://www.meetup.com/san-francisco-rust-study-group/events/284215958/)

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> protip: the rust extern keyword has a --help flag
>
> ```text
> error[E0703]: invalid ABI: found `--help`
>  --> ext.rs:1:8
>   |
> 1 | extern "--help" {}  fn main() {}
>   |        ^^^^^^^^ invalid ABI
>   |
>   = help: valid ABIs: Rust, C, C-unwind, cdecl, stdcall, stdcall-unwind, fastcall, vectorcall, thiscall, thiscall-unwind, aapcs, win64, sysv64, ptx-kernel, msp430-interrupt, x86-interrupt, amdgpu-kernel, efiapi, avr-interrupt, avr-non-blocking-interrupt, C-cmse-nonsecure-call, wasm, system, system-unwind, rust-intrinsic, rust-call, platform-intrinsic, unadjusted
>
> error: aborting due to previous error
>
> For more information about this error, try `rustc --explain E0703`.
> ```

– [Aria the Cat (with some help from rustc) on twitter](https://twitter.com/Gankra_/status/1501307407292641280)

Thanks to [Jacob Pratt](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1188) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
