Title: The State of Rust 0.9
Date: 2014-01-12 21:13
Category: Rust

[Rust](http://www.rust-lang.org/) is a systems language pursuing the trifecta:
safe, concurrent, and fast. This is my post-mortem of the past 3 months, the
current status of Rust, and its future.

<!-- more -->

# Compiler

The compiler saw a fair bit of work this release cycle. Some polish has gone
into `mut` in patterns. For example, `let (mut x, y) = foo();` now works as
expected. We now have "feature gates", or feature flags, that let us mark
certain features as either deprecated or experimental and subject to change.
`@mut` has been removed. Slice representation has been optimized (we now store
number of elements, not number of bytes). Soundness bugs have been fixed, and
some bugs in our name resolution have been patched up. We have a dead code
warning, stability annotations, crate introspection, and lots more.

On top of all that, we also have support for static linking and LTO. Compiler
performance has further improved. From the 112ms compiling `fn main() { }` in
0.8, we now do 91ms with static linking (the default) and 68ms for dynamic
linking (`-Z prefer-dynamic`). Our debuginfo is in a much better state. The
entire codebase (compiler + standard library + tools) compiles with it, and
[Servo](https://github.com/mozilla/servo/) compiles with it by default.

# Runtime

The runtime has seen tons of work this release. `std::io` has been swapped
over to the new runtime, which is written entirely in Rust. We now support
both 1:1 and M:N threading models, their respective runtimes supplied by
"libnative" and "libgreen". libgreen (the "old new runtime") has seen some
performance improvements. Chris Morgan reports that the scalability of his
[rust-http](https://github.com/chris-morgan/rust-http/) benchmarks has
improved from 1.25x to 1.75x from 1 to 8 concurrent request handlers. On my
machine, using libnative gives an impressive performance boost, pushing us
ahead of go's performance (graph below). libnative has yet to see any optimization work.
David Renshaw reports roughly a 2x performance increase on his [capn proto
benchmark](http://dwrensha.github.io/capnproto-rust/2013/11/16/benchmark.html)
when using libnative for I/O.

<noscript>
	<img src="https://docs.google.com/a/octayn.net/spreadsheet/oimg?key=0ArjzeYh7LqL2dENRREdRbDljajR4LWt1RlozM2YyY0E&oid=1&zx=xnqmqanhavt" />
	<p>(This would be interactive if you had JavaScript enabled)</p>
</noscript>

<script type="text/javascript" src="//ajax.googleapis.com/ajax/static/modules/gviz/1.0/chart.js"> {"dataSourceUrl":"//docs.google.com/a/octayn.net/spreadsheet/tq?key=0ArjzeYh7LqL2dENRREdRbDljajR4LWt1RlozM2YyY0E&transpose=0&headers=1&range=A1%3AE6&gid=0&pub=1","options":{"titleTextStyle":{"bold":true,"color":"#000","fontSize":16},"series":{"0":{"errorBars":{"errorType":"none","magnitude":10}}},"animation":{"duration":500},"width":600,"hAxis":{"title":"Concurrency (\"threads\")","useFormatFromData":true,"minValue":null,"viewWindowMode":null,"viewWindow":null,"maxValue":null},"vAxes":[{"useFormatFromData":true,"title":"Requests / second","minorGridlines":{"count":"4"},"minValue":null,"gridlines":{"count":"5"},"logScale":false,"viewWindow":{"max":null,"min":null},"maxValue":null},{"useFormatFromData":true,"minValue":null,"logScale":false,"viewWindow":{"max":null,"min":null},"maxValue":null}],"booleanRole":"certainty","title":"rust-http comparison benchmark","height":371,"domainAxis":{"direction":1},"legend":"right","focusTarget":"series","useFirstColumnAsDomain":true,"isStacked":false,"tooltip":{"trigger":"none"}},"state":{},"view":{"columns":[{"calc":"stringify","type":"string","sourceColumn":0},1,2,3,4]},"isDefaultVisualization":false,"chartType":"ColumnChart","chartName":"Chart 1"} </script>

You can run the benchmarks yourself. `cd comparisons; python run.py`. You'll
need to build rust-http first (`make`), and have go, nodejs, and ab (apache
bench) installed.

# Documentation

Our documentation hasn't been in the best state in the past. It still isn't
where it should be, but [they have been
reorganized](http://static.rust-lang.org/doc/master/index.html), on top of
seeing the usual work. We're in the process of moving all of the documentation
from the wiki onto that site, to make it easier to find and search. Steve
Klabnik gave a [critical but very constructive
presentation](https://air.mozilla.org/rust-meetup-december-2013/) at the Bay
Area meetup about what we are doing wrong and how we can improve. At the same
meetup, Chris Morgan talked about the technologies we use in our documentation
stack. In the long run, the consensus seems to be that reStructuredText and
Sphinx are they way forward. Thanks to the rustdoc rewrite, it will be able to
have first-class status as a documentation backend.

# The Future

For 0.10, we have [Dynamically Sized Types
(DST)](http://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/) to
look forward to, as well as the removal of `@` pointers. `box`, aka "placement
new", along with smart pointer sugar (overloadable dereferencing/borrowing),
should also be in by then. We may also see an actual garbage collector.

# Is Rust Ready Yet?

Nope. It still has some work to do. 1.0 is estimated before the end of 2014,
though that may slip depending on how things land. An early estimate puts the
release over the summer! We still need a robust package manager. We now have
[rust-ci](http://rust-ci.org/), which makes it easy to keep code up to date,
and know if a library is up to date.

The breaking changes, especially language changes, are slowing down, besides
bug fixes. The standard library is also starting to shake itself out, though
it still has a lot of work before they will be stable. There are currently two
known uses of Rust in production: [Tilde](http://www.tilde.io/) is using it in
[Skylight](https://www.skylight.io/), and [OpenDNS](http://opendns.com) is
using it for [real-time data
processing](http://labs.umbrella.com/2013/10/04/zeromq-helping-us-block-malicious-domains/).

It has been a good release, and the next will be even better. Want to get
involved? [We love
contributions](https://github.com/mozilla/rust/wiki/Note-guide-for-new-contributors).
Want to follow development? I post a [weekly
newsletter](http://cmr.github.io/blog/categories/this-week-in-rust/)
summarizing the important changes. There is also a
[subreddit](http://reddit.com/r/rust). Here's to an awesome 0.10!
