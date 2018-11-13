Title: This Week in Rust 3
Date: 2013-06-22 05:21
Category: This Week in Rust

Hello and welcome to the third issue of *This Week in Rust*, a weekly overview
of Rust and its community.

It has come to my attention that Github does not categorize some
merged-by-bors pull requests as "merged" and instead categorizes it as
"closed". This skews the numbers and also the PRs that I looked through for
inclusion into twir. I'll no longer be including issue churn/PR numbers, and I
hope I didn't miss any import PRs in the last two issues.

<!-- more -->

# What's cooking in incoming?

The last of the tree breakage has been cleaned up. The mysterious stack
corruption was traced down by Blei to occur in jemalloc. It has been disabled
until the real cause of the error is found. A bunch more buildbot
configurations have been turned on for auto, to fend off more breakage. The
tree is open, and aatch got out a new snapshot!

Most of the work this week is cleanup and preparation for 0.7, but since the
tree has been closed (and the extra auto buildbots lengthen the time it takes
for a PR to be tested for the day it has been open), not much has landed these
past few days, but a bunch happened earlier this week.

## Notable additions, bugfixes, and cleanup

- doener made compiles faster by emitting less useless copies and allocations
  in [7259](https://github.com/mozilla/rust/pull/7259)
- aatch landed [part 1](https://github.com/mozilla/rust/pull/7124) and [part
  2](https://github.com/mozilla/rust/pull/7182) of his huge trans refactor
  effort, which make trans faster and less terrible.
- sully got [default methods](https://github.com/mozilla/rust/pull/7203)
  less broken
- yichoi landed a [bunch](https://github.com/mozilla/rust/pull/7128) of
  Android fixes.
- vadimcn has [fixed debuginfo](https://github.com/mozilla/rust/pull/7134),
  which is super amazing. This makes it a lot easier for the GSoC student (mw)
  to get started.
- doener has [fixed](https://github.com/mozilla/rust/pull/7186) some
  pathological behavior in how codegen creates cleanup blocks. This makes the
  IR better, reducing compile time, and also allowing better optimization,
  reducing binary size.

{% blockquote @dotdash https://github.com/mozilla/rust/pull/7259 %}
They reduce compile times by about 10% in total.
{% endblockquote %}

{% blockquote @dotdash https://github.com/mozilla/rust/pull/7186 %}
Reduces the size of librustc by about 5% and the time required to build
it by about 10%.
{% endblockquote %}

{% blockquote @dotdash https://github.com/mozilla/rust/pull/7154 %}
The resulting code for rustc is about 13% faster (measured up to and
including the "trans" pass) and the resulting librustc is about 5%
smaller.
{% endblockquote %}

## Breaking changes

strcat [continues](https://github.com/mozilla/rust/pull/7263)
[work](https://github.com/mozilla/rust/pull/7162) with iterators. The changes
that landed are vector cleanups. Probably most importantly, the `each` and
`eachi` methods are being removed. The `eachi` removal landed but the `each`
one broke bors, so it's currently in limbo (**UPDATE** 6/23/2013: it landed).
The current replacement is:

```
// each
for your_vec.iter().advance |element| {
  ...
}
// eachi
for your_vec.iter().enumerate().advance |(i, element)| {
  ...
}
```

Once the rest of the iterator work is hashed out and lands, it will just be

```
// each
for your_vec |element| {
  ...
}
// eachi
for your_vec.enumerate() |(i, element)| {
  ...
}
```

although the syntax might be slightly different (`for element in your_vec` is
my favorite proposal).

# Meetings

The [Tuesday
meeting's](https://github.com/mozilla/rust/wiki/Meeting-weekly-2013-06-18)
main topics were once fn's and how namespaces work. For once fn's, graydon
says "I'm concerned with adding new features and I want to see if we can live
without it, I understand it is common," not yet making a decision to include
them in the language.

# Meetups

- tjc's pre-talk and talk, "Rust: A Friendly Introduction" went very well. The
  [slides](http://catamorphism.org/Writing/Rust-Tutorial-tjc.pdf) are up, and
  a recording is coming soon (hopefuly). tjc says the slides aren't as
  understanable without the audio of the talk.
- nmatsakis has expressed willingness/interest in a Boston meetup sometime. If
  you're interested, contact him on IRC or the ML.

# Notable discourse and external projects

- [mmap and the Rust FFI](http://maniagnosis.crsr.net/2013/06/mmap-and-rust-foreign-function-interface.html)
  (indutny is working on mman bindings in libc, for the record. will mention
  in next week's twir when it lands)
- [code generation and rustc speed](https://mail.mozilla.org/pipermail/rust-dev/2013-June/004480.html)
- [Rust for game development?](http://www.reddit.com/r/rust/comments/1gs93k/rust_for_game_development/)
- [Feed us some low hanging fruit!](http://www.reddit.com/r/rust/comments/1grj61/feed_us_some_low_hanging_fruit/)
- [Paying Technical Debt in rustc](http://aatch.github.io/blog/2013/06/19/paying-technical-debt-in-rustc/)
- [What issues in Rust today effect (sic) you most?](http://www.reddit.com/r/rust/comments/1gpbcs/what_issues_in_rust_today_effect_you_most/)
- [rustdoc rewrite and redesign](https://mail.mozilla.org/pipermail/rust-dev/2013-June/004520.html)
- [rust-bench: a tool for profiling memory usage](http://www.reddit.com/r/rust/comments/1gmac5/linux_rustbench_a_tool_for_profiling_memory_usage/)

# Other announcements

- Michael Woerister (mw), the GSoC student working on debug info, has begun
  work. His [project log](http://michaelwoerister.github.io/) will be updated
  weekly. I'm looking forward to a much better debug experience.
