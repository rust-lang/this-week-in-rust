Title: This Week In Rust 1
Date: 2013-06-07 18:46
Tags: this-week-in-rust, rust, programming
Category: This Week in Rust

Hello and welcome to the first issue of *This Week In Rust*, a weekly overview
of Rust and its community. I'll be covering what's cooking in incoming,
meeting summaries, meetups, and anything else pertinent. Any ideas, email them
to me, <mailto:corey+rust@octayn.net>.

The Rust interns arrived this week and have got cracking right away. Big hello
to Aaron Todd, Ben Blum, and Michael Sullivan! We can look forward to work all
over the place, especially in the RT and debug-info.

<!-- more -->

# What's cooking in incoming?

There were 30 pull requests merged this week. A scattering of doc fixes and a
bunch of code cleanups and optimization work as usual. Total issue churn
(excludes pull requests) this week was +6.

## Notable additions, bugfixes, and cleanups

- Ben Striegel added the `as_c_str` string function as a method as part of the
  overall methodization covered by [6045][is6045].
- James Miller landed a much better optimization pipeline in [6881][is6881],
  fixing a bunch of nascent optimization problems, especially with inlining,
  and also fixing an earlier (huge) regression (that he introduced,
  admittedly).
- James also fixed [6977][is6977], which allowed nonsensical expressions like `[0,
  ..-1]`. Whoops!
- I introduced terminfo handling to `extra::term` in [6826][is6826], bringing
  rustc's beautiful colors to a wider audience.  Unfortunately, it isn't
  complete yet and, most notably, does not handle `xterm-256color` correctly
  yet.
- Daniel Micay landed jemalloc as the default allocator in the runtime,
  leading to nice allocation performance boosts on all platforms, as well as
  much improved multithreaded performance. It also has the benefit of
  cross-platform tuning and instrumentation.

## Breaking changes

It would be silly not to mention these changes, even though they weren't
strictly this week:

- `libcore` was renamed to `libstd` and `libstd` was renamed to `libextra`, to
  better reflect their purpose. Confusingly, the code in rustc still uses the
  old names. Something to watch out for!
- All of the module reexports were removed from the prelude, so if you use,
  for example, `io::foo`, `vec::foo`, etc, you will find yourself having to
  add a lot of extra imports. `use std::*` to regain the old behavior,
  more or less.
- Patrick fixed the unsafe checker to safe code can no longer call unsafe
  methods.

This week:

- `pub impl` was removed by Patrick Walton as part of [6944][is6944]. What
  this did was have all `fn`s in a `pub impl` be `pub` by default. Now, you
  must explicitly specify `pub` on all `fn`s in the impl if you want them
  public.
- Also in [6944][is6944], Patrick removed the ability to have multiple
  patterns appear in "let" declarations. For example: `let a = 4, b = 2;`
  becomes `let (a, b) = (4, 2);`
- Daniel renamed the `Ptr` trait to `RawPtr` in [6913][is6913]
- Daniel and Huon Wilson have been working on iterators a lot. In
  [6999][is6999], they start removing the `vec::each_*` functions, as the new
  iterator code in `std::iterator` is now mature enough for use.

# Meetings

There were two main meetings this week. Mostly discussion about DST, closures,
and the GC. Lots of issues and details remain to be worked out, I suspoect it
will still be a bit before anything final-looking comes up in a PR. See the
[meeting][mtg1] [notes][mtg2] for more details.

# Meetups

- Erick Tryzelaar has a meetup planned in Mountain View on Wednesday, June 12,
  at 7pm. See the [ML thread][sanfran] for more details.
- Tim Chevalier will be giving a talk titled "Rust: A Friendly Introduction"
  on Monday, June 17, 6-9pm in Portland. See [Calagator][rafi] for more details.

# Prominent blog posts and ML threads

- <https://mail.mozilla.org/pipermail/rust-dev/2013-June/004364.html>
- <http://blog.pnkfx.org/blog/2013/06/07/detective-work-on-rust-closures/>
- <http://smallcultfollowing.com/babysteps/blog/2013/06/03/more-on-fns/>
- <http://smallcultfollowing.com/babysteps/blog/2013/06/06/reducing-dst-annotation/>
- <http://pcwalton.github.io/blog/2013/06/02/removing-garbage-collection-from-the-rust-language/>

# Other announcements

- 10gen has some interns working on a MongoDB driver for Rust, which will be
  very nice to have. Good luck to them!
- Brendan Zabarauskas has fixed `lmath`. It now works on incoming. Yay!

Brendan sent in a correction:

{% blockquote %}
Unfortunately whilst it builds on incoming, due to a bug you can't use it in
external crates. moonchrome and I am are working on fixing this but it will
require us to remove the trait heirachy and use macros to generate each type
(Vec3f, Vec3f32, ... etc.) individually instead. Integer and Boolean vector
types (present in GLSL) will also be removed.
{% endblockquote %}

[is6045]: https://github.com/mozilla/rust/issues/6045
[is6881]: https://github.com/mozilla/rust/pull/6881
[is6977]: https://github.com/mozilla/rust/issues/6977
[is6826]: https://github.com/mozilla/rust/pull/6826
[is6944]: https://github.com/mozilla/rust/pull/6944
[is6913]: https://github.com/mozilla/rust/pull/6913
[mtg1]: https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-06-04
[mtg2]: https://github.com/mozilla/rust/wiki/Meeting-2013-06-07
[rafi]: http://calagator.org/events/1250464376
[sanfran]: https://mail.mozilla.org/pipermail/rust-dev/2013-June/004356.html
[is6999]: https://github.com/mozilla/rust/pull/6999
