Title: This Week in Rust 58
Date: 2014-11-24
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

The big news this week was that the [central crate
repository][crates.io] is [now live][crates.io-blog].  There was much
discussion on [HN][crates.io-hn], [/r/rust][crates.io-reddit1] and
[/r/programming][crates.io-reddit2].

[crates.io]: https://crates.io/
[crates.io-blog]: http://blog.rust-lang.org/2014/11/20/Cargo.html
[crates.io-hn]: http://news.ycombinator.com/item?id=8637493
[crates.io-reddit1]: https://www.reddit.com/r/rust/comments/2mwice/cratesio_has_shipped/
[crates.io-reddit2]: https://www.reddit.com/r/programming/comments/2mwidh/rusts_central_package_repository_is_up/

We also have a new [guide to error handling][error].

[error]: http://doc.rust-lang.org/guide-error-handling.html

# What's cooking on master?

xxx pull requests were [merged in the last week][1].

[1]: https://github.com/rust-lang/rust/pulls?q=is%3Apr+is%3Amerged+updated%3A2014-11-17..2014-11-23

## Breaking Changes

* Enum variants are now [namespaced by their type name][enumns], a
  major change. Enums in the standard library have been reexported in
  their old locations, for now at
  least. [RFC][enumns-rfc]. [Reddit][enumns-reddit].
* As part of ongoing [DST-ification][dst], `AsSlice` has been
  [modified to work with unsized types][unsized-asslice]. This breaks
  existing code because `AsSlice` type vars should now be taken by
  reference. At the same time, some of the ops have been extended to
  work with unsized types.
* The `str` method `slice_shift_char`'s return type [has changed
  slightly][slice_shift_char].
* The `find_copy` and `get_copy` methods of `HashMap` [are
  deprecated][cloned].  `find_copy` can be performed with
  `map.get(&key).cloned()`, which converts the `Option<&T>` returned
  by `get` to `Option<T>`, and `get_copy` to simply
  `map[key].clone()`, which calls `.clone()` on the value returned by
  reference from the indexing operator.
* [Runtime removal continues][rt]. Breaking changes here are mostly
  to code invoking the runtime directly.
* The `overloaded_calls` and `unboxed_closure_sugar` feature gates
  [have been combined][gates] into a single `unboxed_closures` gate.
* Formatting has [seen a stability pass][fmt] with some minor breaking
  changes. [RFC][fmt-rfc].
* The little-used `col!` macro is [renamed to `column!`][column].
* Non-ASCII lifetime identifiers are [behind the `non_ascii_idents`
  feature gate][ascii] as intended.
* [Struct variants can not be matched as if they were tuple
  variants][varmatch].

[enumns]: https://github.com/rust-lang/rust/pull/18973
[enumns-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0390-enum-namespacing.md
[enumns-reddit]: https://www.reddit.com/r/rust/comments/2ml4oo/switch_to_purely_namespaced_enums/
[dst]: http://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/
[unsized-asslice]: https://github.com/rust-lang/rust/pull/18638
[slice_shift_char]: https://github.com/rust-lang/rust/pull/18911
[cloned]: https://github.com/rust-lang/rust/pull/18914
[rt]: https://github.com/rust-lang/rust/pull/18967
[gates]: https://github.com/rust-lang/rust/pull/18993
[fmt]: https://github.com/rust-lang/rust/pull/19040
[fmt-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0380-stabilize-std-fmt.md
[column]: https://github.com/rust-lang/rust/pull/19071
[ascii]: https://github.com/rust-lang/rust/pull/19073
[varmatch]: https://github.com/rust-lang/rust/pull/19087

## Other Changes

* Rust now [supports higher-ranked trait bounds][hrtb]. This is
  necessary to make unboxed closures work, but the implications are
  quite complex. Read the [RFC][hrtb-rfc] for details.
* `Vec` [implements `Writer`][vec-writer] and `MemWriter` is deprecated.
* All statics now [support the `#[linkage]` attribute][linkage], which
  is behind the `linkage` feature gate.
* Parts of rustc have been [pulled into a new `rustc_trans`
  crate][rustc_trans] to reduce memory pressure.
* All idents following literals are [tokenized specially][litid] now
  as future proofing. [RFC][litid-rfc].
* [Unboxed closures can be written with the sugared syntax][sugar].

[hrtb]: https://github.com/rust-lang/rust/pull/18993
[hrtb-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0387-higher-ranked-trait-bounds.md
[vec-writer]: https://github.com/rust-lang/rust/pull/18885
[linkage]: https://github.com/rust-lang/rust/pull/18890
[rustc_trans]: https://github.com/rust-lang/rust/pull/19070
[litid]: https://github.com/rust-lang/rust/pull/19103
[litid-rfc]: https://github.com/rust-lang/rfcs/blob/master/text/0463-future-proof-literal-suffixes.md
[sugar]: https://github.com/rust-lang/rust/pull/19113

## New Contributors



# Approved RFC's



# New RFC's



# Community

Karen Rustad found a wild [rustacean](https://twitter.com/whoisaldeka/status/535679593353854976).

## From the Team

* [Weekly-meetings/2014-11-18][mtg]: cmp/ops; TLS; future-proofing
  literal parsing; ungating tuple indexing, if/while let; naming
  conventions; struct variants matching; for syntax for higher-order
  lifetimes; macros; type parameter grammar; better shepherding
  [Reddit][mtg-reddit].
* [Cargo: Rust's community crate host][cargo].

[mtg]: https://github.com/rust-lang/meeting-minutes/blob/master/weekly-meetings/2014-11-18.md
[mtg-reddit]: https://www.reddit.com/r/rust/comments/2mwdhn/weekly_meeting_20141118/
[cargo]: http://blog.rust-lang.org/2014/11/20/Cargo.html

## Blog Posts



## Discussions



## New Projects



## Project Updates



## Upcoming Meetups


