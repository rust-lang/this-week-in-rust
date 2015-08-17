Title: This Week in Rust 92
Number: 92
Date: 2015-08-17
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safety, concurrency, and speed. This is a weekly summary of its progress and
community. Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us an
email](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)!
Want to get involved? [We love
contributions](https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors).

*This Week in Rust* is openly developed [on GitHub](https://github.com/cmr/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/cmr/this-week-in-rust/pulls).

# From the Blogosphere

* [Rust in 2016](http://blog.rust-lang.org/2015/08/14/Next-year.html).
* [video] [RustCamp 2015 videos are now online](http://confreaks.tv/events/rustcamp2015).
* [slides] [Rethinking Systems Programming](http://thoughtram.io/rust-and-nickel/).
* [A Rusting Rubyist V](https://medium.com/@mfpiccolo/a-rusting-rubyist-v-496fb7b1cbbf). Rust Parallel HTTP Requests in Ruby.
* [An Unfortunate Coercion](https://llogiq.github.io/2015/08/14/coercion.html).
* [Testing With Unused Arguments](https://llogiq.github.io/2015/08/17/test.html).
* [Rust and the most elegant FSM](http://blog.benjaminfry.com/2015/08/rust-and-most-elegant-fsm.html).
* [Why Rust Appeals to Me](https://cmcenroe.me/2015/08/08/why-rust.html).

# New Releases & Project Updates

* [Are we (I)DE yet?](http://areweideyet.com/). An overview about the state of Rust development environments and their integrated bretheren.
* The Rust by Example [lifetime section](http://rustbyexample.com/scope/lifetime.html) is updated by [mdinger](https://github.com/mdinger).
* [cargo-outdated](https://github.com/kbknapp/cargo-outdated). A cargo subcommand for displaying when Rust dependencies are out of date.
* [Cargo Darf](https://bitbucket.org/joshmorin/cargo-darf). Develop, test, run, and deploy Rust as a scripted language.
* [rustaceans.com](https://www.rustaceans.com/). Rust user groups & mailing lists.

# New Contributors

* Alex Ozdemir
* Chris Krycho
* Dylan McKay
* Elaine "See More" Nemo
* Jonas Schievink
* llogiq
* Nathan Kleyn
* Nicholas Seckar
* Niranjan Padmanabhan
* Tim Neumann
* w00ns
* Without Boats

# Subteam reports

Every week [The Rust Team](http://www.rust-lang.org/team.html) release
a report on what is going on in their corner of the project. Here are
the highlights from [this week's report](https://internals.rust-lang.org/t/subteam-reports-2015-08-14/2509).

## Compiler team

[Full report](https://github.com/rust-lang/subteams/blob/master/compiler/reports/2015-08-14.md).

Perhaps the most exciting thing going on right now is @huonw's PR
[implementing the groundwork for SIMD][1]. We've also decided
to merge the [MIR RFC][2].

[1]: https://github.com/rust-lang/rust/pull/27169
[2]: https://github.com/rust-lang/rfcs/pull/1211

## Lang team

[Full report](https://github.com/rust-lang/subteams/blob/master/lang/reports/2015-08-14.md).


The following RFCs were promoted to **final comment period**:

- [RFC #1229](https://github.com/rust-lang/rfcs/pull/1229), which
  specifies that when the compiler is doing constant evaluation as a
  form of optimization, it should not report compilation errors for
  overflows or other problems that are encountered (warnings are ok).
  Otherwise, improvements in the constant evaluator can become
  breaking changes.
- [RFC #1234](https://github.com/rust-lang/rfcs/pull/1234), which
  modifies the DST coercion rules to permit `PhantomData`. The
  discussion has primarily focused on whether we can indeed make the
  rules even more accepting than that; conclusion was to leave this
  for future work.

[Full list of T-lang RFCs in FCP.](https://github.com/rust-lang/rfcs/issues?q=is%3Aopen+label%3AT-lang+label%3Afinal-comment-period)

Some fine fellow (*ahem*) also started a scintillating discussion on
the interaction of type parameter fallback with integral fallback
started on internals, which would benefit from more eyeballs:

- [Interaction of user-defined and integral fallbacks with inference][1]

[1]: https://internals.rust-lang.org/t/interaction-of-user-defined-and-integral-fallbacks-with-inference/2496

Last week, we had a number of RFCs enter FCP. Unfortunately, we did
not reach final decisions on very many of them, due to Felix Klock
being out for some well-earned R&R. However, we did merge a few
noncontroversial items:

- [RFC #1209](https://github.com/rust-lang/rfcs/pull/1209) updates
  the follow set for types in macros to include `;`.
- [RFC #1189](https://github.com/rust-lang/rfcs/pull/1189) fixes
  some typos.
  
We also decided to close several RFCs for prioritization reasons, even
if the ideas espoused had merit:

- [RFC 886](https://github.com/rust-lang/rfcs/pull/886), allowing
  "must use" on functions (by @huonw).
- [RFC 890](https://github.com/rust-lang/rfcs/pull/890), custom
  preludes (by @aturon).
- [RFC 1216](https://github.com/rust-lang/rfcs/pull/1216), promote `!`
  to a type (by @canndrew).

## Libs team

[Full report](https://github.com/rust-lang/subteams/blob/master/libs/reports/2015-08-14.md).

Starting with the current release cycle (for 1.4), we are making some
changes to the stabilization process:

* All unstable APIs
  [now map to tracking issues](https://internals.rust-lang.org/t/psa-stabilization-tracking-on-the-libs-team/2493). The
  APIs will be tagged with the issue tracking them, so when you get an
  error that you need a feature gate, the compiler can also tell you
  which issue to subscribe to for stabilization. We'll use these
  issues as a centralized place for discussing questions/concerns
  about APIs before they are stabilized.

* Stabilization will be tied to our 6 week release cycles: at the
  beginning of the cycle, the team will announce a number of unstable
  features undergoing "final comment period" (heading either toward
  stabilization or deprecation). The comment period will last for the
  entire release cycle. One week before the release is cut, the team
  will make final decisions and land a PR accordingly.

This new process such give much more visibility into the feature
pipeline and make the stabilization process easier to follow.

With that in mind, here are the features undergoing FCP for this cycle:

- [FCP Issue #27277](https://github.com/rust-lang/rust/issues/27277):
  Stabilisation of `result_expect`
- [FCP Issue #27736](https://github.com/rust-lang/rust/issues/27736):
  Tracking issue for f{32,64}::from_str_radix
- [FCP Issue #27764](https://github.com/rust-lang/rust/issues/27764):
  Tracking issue for CStr => str conversions
- [FCP Issue #27765](https://github.com/rust-lang/rust/issues/27765):
  Tracking issue for collection append methods
- [FCP Issue #27766](https://github.com/rust-lang/rust/issues/27766):
  Tracking issue for collection split_off methods
- [FCP Issue #27767](https://github.com/rust-lang/rust/issues/27767):
  Tracking issue for VecDeque::retain
- [FCP Issue #27768](https://github.com/rust-lang/rust/issues/27768):
  Tracking issue for Box::{into_raw, from_raw}
- [FCP Issue #27769](https://github.com/rust-lang/rust/issues/27769):
  Tracking issue for CString::{from_ptr, into_ptr}
- [FCP Issue #27771](https://github.com/rust-lang/rust/issues/27771):
  Tracking issue for some Duration-taking functions
- [FCP Issue #27773](https://github.com/rust-lang/rust/issues/27773):
  Tracking issue for the socket timeout functions
- [FCP Issue #27775](https://github.com/rust-lang/rust/issues/27775):
  Tracking issue for converting slice iterators to slices
- [FCP Issue #27776](https://github.com/rust-lang/rust/issues/27776):
  Tracking issue for viewing Result/Option as slices
- [FCP Issue #27785](https://github.com/rust-lang/rust/issues/27785):
  Tracking issue for Box<str>/String conversions
- [FCP Issue #27792](https://github.com/rust-lang/rust/issues/27792):
  Tracking issue for str::split_at
- [FCP Issue #27795](https://github.com/rust-lang/rust/issues/27795):
  Tracking issue for creating BTree{Map,Set} with a B
- [FCP Issue #27797](https://github.com/rust-lang/rust/issues/27797):
  Tracking issue for consumption into OS handles

In addition, this week we made the following decisions:

- [FCP PR #1195](https://github.com/rust-lang/rfcs/pull/1195):
  ordered query API
  - Close, with the hope that we can find a way to achieve these goals
    with a smaller API surface
- [FCP PR #1192](https://github.com/rust-lang/rfcs/pull/1192):
  RFC for inclusive ranges with ...
  - Merge, with the extra field for `Iterator` being kept unstable for
    the time being.

Finally, the following RFC is entering FCP this week:

- [FCP PR #1198](https://github.com/rust-lang/rfcs/pull/1198):
  pretty print `Debug` of tuples, tuple structs and enum variants in a single line

# Tool team

[Full report](https://github.com/rust-lang/subteams/blob/master/tools/reports/2015-08-14.md).

These past two weeks saw a number of exciting events!

* Rust 1.2 [was released][rust-12], thanks to @brson for doing the release
  process!
* The main site is now [hosted over https][https] thanks to @edunham's move from
  GitHub pages to Cloudfront!
* [`make check` is now running][make-check] for 64-bit MSVC. This means that all
  changes to the compiler must pass all tests on MSVC to land! Note that this is
  enabled by [turning on unwinding][unwinding] for 64-bit MSVC which is in turn
  enabled by @vadimcn's [awesome work][winexn] on unwinding.
* The [`cargo install` RFC][cargo-install] has been merged.
* @brson has [released a tool][crusader] for testing whether publishing your
  crate would cause a regression on reverse dependencies.
* multirust has gained [a `which` command][mr-which] to locate where binaries
  are, thanks to @shaleh!

[rust-12]: http://blog.rust-lang.org/2015/08/06/Rust-1.2.html
[cargo-install]: http://github.com/rust-lang/rfcs/issues/1200
[make-check]: https://github.com/rust-lang/rust/pull/27786
[unwinding]: https://github.com/rust-lang/rust/pull/27676
[https]: https://github.com/rust-lang/rust-www/issues/148
[winexn]: https://github.com/rust-lang/rust/pull/27210
[crusader]: https://github.com/brson/cargo-crusader
[mr-which]: https://github.com/brson/multirust/pull/84

# Upcoming Events

* [8/18. Sydney](http://www.meetup.com/Rust-Sydney/).
* [8/19. Los Angeles](http://www.meetup.com/Rust-Los-Angeles/events/224231575/).
* [8/20. Berlin](http://www.meetup.com/Rust-Berlin/events/224141638/).
* [8/26. Columbus Rust Society](http://www.meetup.com/columbus-rs/).
* [8/31. Paris](http://www.meetup.com/Rust-Paris).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Email [Erick Tryzelaar][erickt] or [Brian
Anderson][brson] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[erickt]: mailto:erick.tryzelaar@gmail.com
[brson]: mailto:banderson@mozilla.com

# fn work(on: RustProject) -> Money

*No jobs listed for this week. Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

*"Any sufficiently advanced macro is indistinguishable from magic."* — [barosl](https://www.reddit.com/user/barosl) at [reddit](https://www.reddit.com/r/rust/comments/3fq3o2/embed_c_directly_inside_your_rust_code/ctqzi26).

Thanks to [nasa42](https://users.rust-lang.org/users/nasa42) for the tip. [Submit your quotes for next week!][submit].

[submit]: http://users.rust-lang.org/t/twir-quote-of-the-week/328
