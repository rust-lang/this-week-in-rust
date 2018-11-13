Title: This Week In Rust 2
Date: 2013-06-15 22:00
Tags: this-week-in-rust, programming
Category: This Week in Rust

Hello and welcome to the second issue of *This Week In Rust*, a weekly overview
of Rust and its community. I'll be covering what's cooking in incoming,
meeting summaries, meetups, and anything else pertinent.

I've decided to stop using real names and use irc/github names, simply because
that is how I, and most everyone, interacts in the community.

<!-- more -->

# What's cooking in incoming?

There's been a lot of breakage on incoming this week, with jemalloc breaking
32bit cross-compilation as well as random segfaults and stack corruption of
unknown cause.  Some heroics by the core devs have got it mostly cleaned up,
though the tree is still rather chaotic. Meanwhile a handful of performance
improvements have landed, and achricto rewrote `rusti`.

There were 17 pull requests merged this week. Total issue churn (excludes pull
requests) this week was +2 (this excludes the 38 pull requests that were
closed when incoming was killed).

## `incoming` branch annihilated

Goodbye `incoming`, hello `master`! This change, long in coming, unfortunately
closed all open PRs. Start doing your development against `master` rather than
incoming.

## Notable additions, bugfixes, and cleanups

There's a concerted effort to remove duplicate freestanding functions where
possible.

- In [6986][is6986] bjz and jensnockert have cleaned up the numeric code some
  more, adding methods for existing things like `sin`, as well as adding a
  bunch of interpolation stuff.
- steven_is_false added prototype dynamic library loading support in
  [7027][is7027], which should remove a lot of pain for people looking for
  easy dynamic loading. It currently doesn't work on Windows, so if you can
  sling Windows code, help would be appreciated!
- In [7029][is7029] luqmana allows having multiple impl's add static methods,
  which previously did not work.
- Eridius stepped up to [fix the terminfo code][tinfo], colors should be
  arriving to more people soon.
- SiegeLord [improved the CSS][css] used by rustdoc with *huge* improvements.
- sully has gotten default methods working for the most part, he is still
  testing cross-crate edge casses.
- vadimcn [has fixed debuginfo][debug], and supposedly the GSoC intern is
  getting started on improving it next week.
- doener has got [some nice][inline] [performance][cache] PRs in place.
- aatch is working on [cleaning up trans][trans]. Huge thanks to him!

## Breaking changes

- dbaupp and strcat continue their cleanup of the standard library, removing
  the ad-hoc iterator functions where `std::iterator` can replace them.
- All of the string functions that could be reasonably converted to methods
  have been.
- If you're working in the stdlib, acrichto has toggled most of the lint
  settings to "deny" for std/extra, so watch out.

# Meetings

The [Tuesday meeting][tues] talked about bblum's [Effect proposal][eff],
removing the master/incoming split, and "alloc expressions", a replacement for
@-sigils.

The consensus on the effect proposal is that it needs investigation and
wouldn't be landing in 1.0.

Discussion about master/incoming mostly centered on "master isn't always
green, how can we add better coverage to bors' tests?" Consensus seems to be
that removing incoming would be beneficial, but enabling more OS and valgrind
coverage on bors would harmfully impact development speed.

The proposed syntax for alloc expressions is `new (provider) expr`, with `new
expr` becoming the replacement for the current `~expr`. This would allow
custom smart pointers. pcwalton ended the meeting with a huge cliff hanger

{% blockquote %}
I've been meaning to talk a little bit today about simplifying the
mut-borrowing story in regards to this, we may be able to effect a large
simplification on the language
{% endblockquote %}

Personally, I think [kimundi's proposal][kim] has a lot of promise, and the
syntax is more pleasing to me. It wasn't brought up at the meeting, though.

# Meetups

- The Mountain View meetup was a great success. 18 showed up. erickt is
  planning for another SF Bay area meetup in July. If you want to give a
  presentation, send him your proposal and how long you need to put it
  together.
- Tim Chevalier will be giving a talk titled "Rust: A Friendly Introduction"
  on Monday, June 17, 6-9pm in Portland. See [Calagator][rafi] for more
  details. This is a preview of a talk he will be giving at [Open Source
  Bridge][osb], also in Portland.

# Notable discourse

- Still more discussion about [iterators][iter], this time focusing around
  [changing the semantics][for] of the `for` loop.
- Some discussion about [list comprehensions][listcomp], including initial
  proofs-of-concept.
- Graydon explains hashing and versioning
  https://botbot.me/mozilla/rust/msg/3792753/
- Principal author of 0install evaluates rust among other languages as a
  python replacement
  http://roscidus.com/blog/blog/2013/06/09/choosing-a-python-replacement-for-0install/
- Niko thinks about parallelism
  http://smallcultfollowing.com/babysteps/blog/2013/06/11/data-parallelism-in-rust/
- http://smallcultfollowing.com/babysteps/blog/2013/06/11/on-the-connection-between-memory-management-and-data-race-freedom/

# Other announcements

- bjz tells me lmath is *actually* fixed now, and is usable

[is6986]: https://github.com/mozilla/rust/pull/6986
[is7027]: https://github.com/mozilla/rust/pull/7027
[is7029]: https://github.com/mozilla/rust/pull/7029
[tues]: https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-06-11
[kim]: https://gist.github.com/Kimundi/5744578
[iter]: https://mail.mozilla.org/pipermail/rust-dev/2013-June/004364.html
[rafi]: http://calagator.org/events/1250464376
[for]: https://mail.mozilla.org/pipermail/rust-dev/2013-June/004465.html
[listcomp]: http://www.reddit.com/r/rust/comments/1gag3t/list_comprehensions_in_rust_iterator/
[css]: https://github.com/mozilla/rust/pull/7077
[eff]: https://github.com/mozilla/rust/wiki/Proposal-for-effects
[tinfo]: https://github.com/mozilla/rust/pull/7133
[osb]: http://opensourcebridge.org/sessions/970
[debug]: https://github.com/mozilla/rust/pull/7134
[inline]: https://github.com/mozilla/rust/pull/7154
[cache]: https://github.com/mozilla/rust/pull/7144
[trans]: https://github.com/mozilla/rust/pull/7124
