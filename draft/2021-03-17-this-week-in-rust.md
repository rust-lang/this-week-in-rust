Title: This Week in Rust 382
Number: 382
Date: 2021-03-10
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](http://rust-lang.org) is a systems language pursuing the trifecta: safety, concurrency, and speed.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

# Updates from Rust Community

### Official

### Newsletters

### Project/Tooling Updates

* [MiniWASM - A minimalist Rust WebAssembly project template](https://github.com/thedjinn/MiniWASM)
* [Stepper 0.5 (formerly Step/Dir) - Universal Stepper Motor Interface](https://flott-motion.org/news/stepper-0-5/)

### Observations/Thoughts

* [Why the Starship shell prompt is built in Rust (interview with project creator)](https://console.dev/qa/starship-matan-kushner/)

### Rust Walkthroughs
* [OS in Rust: Incorporate VGA buffer: Part-7](https://blog.knoldus.com/os-in-rust-incorporate-vga-buffer-part-7/)

* [How to send emails with Rust](https://kerkour.com/blog/rust-send-email/)

### Papers and Research Projects

### Miscellaneous
* [rkyv is faster than {bincode, capnp, cbor, flatbuffers, postcard, prost, serde_json}](https://davidkoloski.me/blog/rkyv-is-faster-than/)
* [100ms delays with Rust on Lambda](https://www.peakscale.com/100ms-gotcha/)

# Crate of the Week

This week's crate is [ibig](https://github.com/tczajka/ibig-rs), a crate of fast big integers.

Thanks to [Willi Kappler](https://users.rust-lang.org/t/crate-of-the-week/2704/889) for the suggestion!

[Submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

# Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

* [rust-analyzer - Optimize MBE (macro by example) performance](https://github.com/rust-analyzer/rust-analyzer/issues/7857)
* [Our own "Papers and Research Projects" section needs filling!](https://github.com/rust-lang/this-week-in-rust/)
* [delta-rs has many good first issues for those who want to learn Delta Lake or Rust](https://github.com/delta-io/delta-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22)

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

# Updates from Rust Core

365 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2021-03-08..2021-03-15

* [expand: do not allocate `Lrc` for `allow_internal_unstable` list unless necessary](https://github.com/rust-lang/rust/pull/82422)
* [account for `if (let pat = expr) {}`](https://github.com/rust-lang/rust/pull/82854)
* [introduce `proc_macro_back_compat` lint, and emit for `time-macros-impl`](https://github.com/rust-lang/rust/pull/83127)
* [eagerly construct bodies of THIR](https://github.com/rust-lang/rust/pull/82495)
* [store HIR attributes in a side table](https://github.com/rust-lang/rust/pull/79519)
* [add `StatementKind::CopyNonOverlapping`](https://github.com/rust-lang/rust/pull/77511)
* [tweaks to stable hashing](https://github.com/rust-lang/rust/pull/83064)
* [`rustc_query_system`: simplify `QueryCache::iter`](https://github.com/rust-lang/rust/pull/83069)
* [mir-opt-level 4 is the new 3](https://github.com/rust-lang/miri/pull/1737)
* [miri: ensure we catch incorrectly unwinding calls](https://github.com/rust-lang/miri/pull/1744)
* [miri: check callee ABI when Miri calls closures](https://github.com/rust-lang/miri/pull/1743)
* [don't implement `mem::replace` with `mem::swap`](https://github.com/rust-lang/rust/pull/83022)
* [fix `io::copy` specialization using `copy_file_range` when writer was opened with `O_APPEND`](https://github.com/rust-lang/rust/pull/82417)
* [added `#[repr(transparent)]` to `core::cmp::Reverse`](https://github.com/rust-lang/rust/pull/81879)
* [add `Option::get_or_default`](https://github.com/rust-lang/rust/pull/82849)
* [implement `Extend` and `FromIterator` for `OsString`](https://github.com/rust-lang/rust/pull/82121)
* [improve `sift_down` performance in `BinaryHeap`](https://github.com/rust-lang/rust/pull/81127)
* [fix leak in `Vec::extend_from_within`](https://github.com/rust-lang/rust/pull/82760)
* [regex: substantially reduce regex stack size](https://github.com/rust-lang/regex/pull/752)
* [clippy: implement new lint: `if_then_some_else_none`](https://github.com/rust-lang/rust-clippy/pull/6859)

## Rust Compiler Performance Triage

A generally positive albeit quiet week though many of the perf improvements were gaining performance back from previous regressions. We'll need to continue to keep an eye on rollups as there were two that caused small performance changes.

Triage done by **@rylev**.
Revision range: [edeee..86187](https://perf.rust-lang.org/?start=edeee915b1c52f97411e57ef6b1a8bd46548a37a&end=861872bc453bde79b83ff99d443d035225f10e87&absolute=false&stat=instructions%3Au)

1 Regression, 4 Improvements, 1 Mixed

2 of them in rollups

## Approved RFCs

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

* [Linking modifiers for native libraries](https://github.com/rust-lang/rfcs/pull/2951)

## Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

* [Change visibility scoping rules for macro_rules macros](https://github.com/rust-lang/rfcs/pull/3067)
* [Make the authors field optional](https://github.com/rust-lang/rfcs/pull/3052)

### [Tracking Issues & PRs](https://github.com/rust-lang/rust/labels/final-comment-period)

* [disposition: merge] [rustdoc: allow list syntax for `#[doc(alias)`] attributes](https://github.com/rust-lang/rust/pull/82846)
* [disposition: merge] [slice: Stabilize `IterMut::as_slice`.](https://github.com/rust-lang/rust/pull/82771)
* [disposition: merge] [Implement Extend and FromIterator for OsString](https://github.com/rust-lang/rust/pull/82121)
* [disposition: merge] [Emit errors/warns on some wrong uses of rustdoc attributes](https://github.com/rust-lang/rust/pull/80300)
* [disposition: close] [resolve: allow super in module in block to refer to block items](https://github.com/rust-lang/rust/pull/79309)
* [disposition: close] [[Edition vNext] Consider deprecating weird nesting of items](https://github.com/rust-lang/rust/issues/65516)

## New RFCs

*No new RFCs were proposed this week.*

# Upcoming Events

### Online
* [March 11, Columbus, OH, US - Monthly Meeting - Columbus Rust Society](https://www.meetup.com/columbus-rs/events/dpkhgryccfbpb/)
* [March 11, Lehi, UT, US - Show and Tell: `rustdoc` - Utah Rust](https://www.meetup.com/utah-rust/events/276845098/)
* [March 11, San Diego, CA, US - March 2021 Tele-Meetup - San Diego Rust](https://www.meetup.com/San-Diego-Rust/events/276759349/)
* [March 13th, DE - Chemnitzer Linux Tage - Talk on Rust and its ecosystem](https://chemnitzer.linux-tage.de/2021/en/programm/beitrag/135)
* [March 16, Washington, DC, US - Rust and Tell Lightning Talks - Rust DC](https://www.meetup.com/RustDC/events/kcfpzryccfbpb/)
* [March 17, Vancouver, BC, US - Rust Study/Hack/Hang-out night](https://www.meetup.com/Vancouver-Rust/events/npqfbsyccfbwb/)
* [March 18, Manchester, UK - Rust Manchester Opening Night - Rust Manchester](https://www.meetup.com/rust-manchester/events/276567843/)
* [March 18, Berlin, DE - Rust Hack and Learn - Berline.rs](https://www.meetup.com/opentechschool-berlin/events/txcprryccfbxb/)
* [March 23, Berlin, DE - Rust and Tell - 2021 Kickoff - Berline.rs](https://berline.rs/2021/03/23/rust-and-tell.html)
* [March 25. Barcelona, ES - BcnRust Meetup](https://www.meetup.com/es-ES/BcnRust/events/276796209/).

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

**Oso**

*[Software Engineer (New York, NY, US or Remote)](https://www.osohq.com/jobs/eng)
*[Engineering Manager (New York, NY, US or Remote](https://www.osohq.com/jobs/eng-mgr)
*[Developer Advocate (New York, NY, US or Remote](https://www.osohq.com/jobs/dev-advocate)

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> I think the security of the internet is incredibly important obviously and I want it to be secure and I think bringing rust there is absolutely going to help it. Just by default it eliminates some of the most classic types of vulnerabilities.
>
> But I don't think that's the most exciting part. I think the most exciting part is that the set of people for whom it is possible to implement these types of things, like who writes coreutils, who writes curl, who does those things. That used to be a really small pool of people. That had to be people who knew the dark arts, and only them and only their buddies or something.
>
> **And it's the goal of rust to empower that to be a larger group of people** and ultimately I think that that is what is going to happen which means the sheer number of people will be larger, and also the diversity of that set of people is going to grow. And I that that that will probably actually do more for the security and usefulness of these tools than eliminating underfined behaviour.

– [Ashley Williams on twitch](https://www.twitch.tv/videos/946905598) (quote starts at 46:48)

Thanks to [Nixon Enraght-Moony](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1014) for the suggestion.

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), and [cdmistman](https://github.com/cdmistman).*

<small>[Discuss on r/rust](https://www.reddit.com/r/rust/comments/k5nsab/this_week_in_rust_367/)</small>
