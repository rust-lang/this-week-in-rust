Title: This Week in Rust 55
Date: 2014-08-18 22:28
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is a weekly summary of its progress and
community. Want something mentioned? [Send me an
email!](mailto:corey@octayn.net?subject=This%20Week%20in%20Rust%20Suggestion)
Want to get involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).

It's been a long while since a TWiR, and I apologize for that. This TWiR is
going to be much more abbreviated than usual. The normal pace will resume next
week.

<!-- more -->

# What's cooking on master?

There were 418 pull requests merged in the past two months, modulo any rollups
(which are usually a combination of 10-20 pull requests).

## Breaking Changes

88 commits contained breaking changes. Since this is a completely unreasonable
number (and I'm sure ancient breaking changes aren't interesting), I'll just
cover the last week's:

- A [bunch of changes](https://github.com/rust-lang/rust/pull/16332) happened
  to `core::slice`, including some trait renames. Most code shouldn't be
  affected by this, these traits are all in the prelude.
- A `Duration` type [has been
  added](https://github.com/rust-lang/rust/pull/15934), and many functions
  which logically take a duration have been changed to use it.
- Imports and items are [no longer allowed to
  shadow](https://github.com/rust-lang/rust/pull/16482).

## Other Changes

Far too many for me to list! Impressively, pcwalton has been knocking down
backwards incompatible changes left and right. Currently, only [11
issues](https://github.com/rust-lang/rust/issues?q=is%3Aopen+is%3Aissue+label%3AP-backcompat-lang+milestone%3A1.0)
backwards incompatible language changes are tagged for 1.0. He has also
implemented [unboxed
closures](https://github.com/rust-lang/rfcs/blob/master/active/0044-closures.md), fixed a bunch of soundness issues, a large
portion of associated types, basic [`where`
clauses](https://github.com/rust-lang/rfcs/pull/135), [lifetime
elision](https://github.com/rust-lang/rfcs/blob/master/active/0039-lifetime-elision.md), and various smaller
issues.

There's been tons of library work, including stabilization, and cargo has
really taken off.

## New Contributors

- Adrien Brault
- Alexis Beingessner
- Alisdair Owens
- Andreas Tolfsen
- Andrew Poelstra
- Angus Lees
- Anton Lofgren
- Ben Gamari
- Bheesham Persaud
- Chris Nixon
- Chuck Ries
- DJUrsus
- Daniel Hofstetter
- David Vazgenovich Shakaryan
- Derecho
- Derek Harland
- Dzmitry Malyshau
- Eduardo Bautista
- Gioele Barabucci
- Hugo Jobling
- Ilya Dmitrichenko
- Jack Heizer
- Jake Scott
- James Hurst
- James Lal
- James Rowe
- Jason Thompson
- John Kåre Alsaker
- Jonas Hietala
- Kevin Walter
- LemmingAvalanche
- Matej Lach
- Mathijs van de Nes
- Michael Matuzak
- Michael Sproul
- Mike Robinson
- Nathan Froyd
- Paolo Falabella
- Patrick Yevsukov
- Peer Aramillo Irizar
- Peter Atashian
- Phil Dawes
- Philipp Gesang
- Prudhvi Krishna Surapaneni
- Robert Clipsham
- Russell
- Samuel Neves
- Simon Persson
- Stuart Pernsteiner
- Tim Joseph Dumol
- Tshepang Lekhonkhobe
- Yazhong Liu
- Yuri Albuquerque
- Zbigniew Siciarz
- dgoon
- donkopotamus
- kwantam
- masklinn
- Mitchell Nordine
- Nick Hamann
