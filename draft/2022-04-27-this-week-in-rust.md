Title: This Week in Rust 440
Number: 440
Date: 2022-04-27
Category: This Week in Rust

Hello and welcome to another issue of *This Week in Rust*!
[Rust](https://www.rust-lang.org/) is a programming language empowering everyone to build reliable and efficient software.
This is a weekly summary of its progress and community.
Want something mentioned? Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) or [send us a pull request](https://github.com/rust-lang/this-week-in-rust).
Want to get involved? [We love contributions](https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md).

*This Week in Rust* is openly developed [on GitHub](https://github.com/rust-lang/this-week-in-rust).
If you find any errors in this week's issue, [please submit a PR](https://github.com/rust-lang/this-week-in-rust/pulls).

## Updates from Rust Community

<!--

Dear community contributors:
Please read README.md for guidance on submissions.
Each submitted link should be of the form:

* [Title of the Linked Page](https://example.com/my_article)

If you don't know which category to use, feel free to submit a PR anyway
and just ask the editors to select the category.

-->

### Official

### Foundation

### Newsletters

### Project/Tooling Updates

### Observations/Thoughts

### Rust Walkthroughs

### Research

### Miscellaneous

## Crate of the Week

This week's crate is [czkawka](https://github.com/qarmin/czkawka), a GTK-based duplicate finder.

Despite a lack of nominations, llogiq is pleased with his pick.

[Please submit your suggestions and votes for next week][submit_crate]!

[submit_crate]: https://users.rust-lang.org/t/crate-of-the-week/2704

## Call for Participation

Always wanted to contribute to open-source projects but didn't know where to start?
Every week we highlight some tasks from the Rust community for you to pick and get started!

Some of these tasks may also have mentors available, visit the task page for more information.

If you are a Rust project owner and are looking for contributors, please submit tasks [here][guidelines].

[guidelines]: https://users.rust-lang.org/t/twir-call-for-participation/4821

## Updates from the Rust Project

278 pull requests were [merged in the last week][merged]

[merged]: https://github.com/search?q=is%3Apr+org%3Arust-lang+is%3Amerged+merged%3A2022-04-18..2022-04-25

* [debuginfo: emit ZST struct debuginfo for unit type when CPP-like debuginfo is enabled](https://github.com/rust-lang/rust/pull/96316)
* [better error message for `_` in function signature in `impl Trait for Ty`](https://github.com/rust-lang/rust/pull/95395)
* [fix an invalid error for a suggestion to add a slice in pattern-matching](https://github.com/rust-lang/rust/pull/96122)
* [improve span for `consider adding an explicit lifetime bound` suggestions under NLL](https://github.com/rust-lang/rust/pull/96352)
* [improve diagnostic on failure to meet send bound on future in a foreign crate](https://github.com/rust-lang/rust/pull/94493)
* [make the lifetime accurate which is used in the region constraints part](https://github.com/rust-lang/rust/pull/96315)
* [miri: allow to track multiple alloc-ids, call-ids and pointer tags](https://github.com/rust-lang/miri/pull/2075)
* [miri: do not consider thread-local allocations read-only](https://github.com/rust-lang/miri/pull/2074)
* [interpret: fix writing uninit to an allocation](https://github.com/rust-lang/rust/pull/96162)
* [micro-optimize `ty::relate::relate_substs` by avoiding `match`](https://github.com/rust-lang/rust/pull/96020)
* [optimize `const_prop` mir-opt by accessing `local_decls` through `ecx`](https://github.com/rust-lang/rust/pull/96281)
* [remove visibility information from HIR](https://github.com/rust-lang/rust/pull/93970)
* [speed up `TokenCursor`](https://github.com/rust-lang/rust/pull/96210)
* [`alloc`: make `vec!` unavailable under `no_global_oom_handling`](https://github.com/rust-lang/rust/pull/96089)
* [unstably constify `impl<I: Iterator> IntoIterator for I`](https://github.com/rust-lang/rust/pull/90602)
* [add `as_slice` to slice iterator](https://github.com/rust-lang/rust/pull/92287)
* [improve Windows path prefix parsing](https://github.com/rust-lang/rust/pull/94887)
* [reduce allocations for path conversions on Windows](https://github.com/rust-lang/rust/pull/96314)
* [futures: create `copy_buf_abortable`, which enables to stop copying in the middle](https://github.com/rust-lang/futures-rs/pull/2507)
* [codegen\_gcc: don't emit `.intel_syntax` for non-x86 targets](https://github.com/rust-lang/rustc_codegen_gcc/pull/164)
* [cargo: prefer `key.workspace = true` to `key = { workspace = true }`](https://github.com/rust-lang/cargo/pull/10584)
* [rustdoc: optimize `IdMap`](https://github.com/rust-lang/rust/pull/96260)
* [rustdoc: optimize and refactor doc link resolution](https://github.com/rust-lang/rust/pull/96135)
* [rustdoc: resolve some more doc links early](https://github.com/rust-lang/rust/pull/96261)
* [rustdoc: unindent doc fragments on `Attributes` construction](https://github.com/rust-lang/rust/pull/96282)
* [rustdoc: make primitive synthetic impls for correct doc module](https://github.com/rust-lang/rust/pull/96301)
* [clippy: add `large_include_file` lint](https://github.com/rust-lang/rust-clippy/pull/8727)
* [clippy: add macro export exemption to `redundant_pub_crate`](https://github.com/rust-lang/rust-clippy/pull/8736)
* [clippy: fix missing whitespace in `collapsible_else_if` suggestion](https://github.com/rust-lang/rust-clippy/pull/8729)
* [clippy: fix `needless_match` false positive for if-let when the else block doesn't match to given expr](https://github.com/rust-lang/rust-clippy/pull/8700)
* [clippy: new lint bytes count to len](https://github.com/rust-lang/rust-clippy/pull/8711)
* [clippy: `manual_split_once`: lint manual iteration of `SplitN`](https://github.com/rust-lang/rust-clippy/pull/8717)
* [clippy: add `empty_drop`](https://github.com/rust-lang/rust-clippy/pull/8571)
* [clippy: `mistyped_literal_suffix`: improve integer suggestions, avoid wrong float suggestions](https://github.com/rust-lang/rust-clippy/pull/8742)
* [clippy: `wrong_self_convention` allows `is_*` to take `&mut self`](https://github.com/rust-lang/rust-clippy/pull/8738)
* [rust-analyzer: fix const generic panic in `dyn trait`](https://github.com/rust-lang/rust-analyzer/pull/12054)
* [rust-analyzer: reduce priority of `flyimport` completions](https://github.com/rust-lang/rust-analyzer/pull/12074)
* [rust-analyzer: restart proc-macro client when server reload](https://github.com/rust-lang/rust-analyzer/pull/12007)
* [rust-analyzer: display signature help when applying "Add `::<>`" assist](https://github.com/rust-lang/rust-analyzer/pull/12032)
* [rust-analyzer: prefer core/alloc over std in auto-imports if `#[no_std]` is conditional](https://github.com/rust-lang/rust-analyzer/pull/12041)
* [rust-analyzer: improve parameter completion](https://github.com/rust-lang/rust-analyzer/pull/12040)
* [rust-analyzer: index the correct `CargoWorkspace` with `rustc_private`](https://github.com/rust-lang/rust-analyzer/pull/12044)

### Rust Compiler Performance Triage

<!-- Perf results go here -->

### [Approved RFCs](https://github.com/rust-lang/rfcs/commits/master)

Changes to Rust follow the Rust [RFC (request for comments) process](https://github.com/rust-lang/rfcs#rust-rfcs). These
are the RFCs that were approved for implementation this week:

### Final Comment Period

Every week [the team](https://www.rust-lang.org/team.html) announces the
'final comment period' for RFCs and key PRs which are reaching a
decision. Express your opinions now.

#### [RFCs](https://github.com/rust-lang/rfcs/labels/final-comment-period)

#### [Tracking Issues & PRs](https://github.com/rust-lang/rust/issues?q=is%3Aopen+label%3Afinal-comment-period+sort%3Aupdated-desc)

### [New and Updated RFCs](https://github.com/rust-lang/rfcs/pulls)


## Upcoming Events

Rusty Events between 2022-04-27 - 2022-05-25 🦀

### Virtual

### Europe

### North America

### Oceania

If you are running a Rust event please add it to the [calendar] to get
it mentioned here. Please remember to add a link to the event too.
Email the [Rust Community Team][community] for access.

[calendar]: https://www.google.com/calendar/embed?src=apd9vmbc22egenmtu5l6c5jbfc%40group.calendar.google.com
[community]: mailto:community-team@rust-lang.org

# Rust Jobs

<!--

New jobs can be posted here.

They should be of the form:

**Company Name**

* [Job Title (Location)](https://example.com/my-job-link)

-->

*Tweet us at [@ThisWeekInRust](https://twitter.com/ThisWeekInRust) to get your job offers listed here!*

# Quote of the Week

> This is the most fundamental philosophy of both the Rust language and the Rust project: we don't think it's sufficient to build robust systems by only including people who don't make mistakes; we think it's better to provide tooling and process to catch and prevent mistakes.

– [Jane Lusby on the inside Rust blog](https://blog.rust-lang.org/inside-rust/2022/04/19/imposter-syndrome.html)

Thanks to [farnbams](https://users.rust-lang.org/t/twir-quote-of-the-week/328/1220) for the suggestion!

[Please submit quotes and vote for next week!](https://users.rust-lang.org/t/twir-quote-of-the-week/328)

*This Week in Rust is edited by: [nellshamrell](https://github.com/nellshamrell), [llogiq](https://github.com/llogiq), [cdmistman](https://github.com/cdmistman), [ericseppanen](https://github.com/ericseppanen), [extrawurst](https://github.com/extrawurst), [andrewpollack](https://github.com/andrewpollack), [U007D](https://github.com/U007D), [kolharsam](https://github.com/kolharsam), [joelmarcey](https://github.com/joelmarcey), [mariannegoldin](https://github.com/mariannegoldin).*

*Email list hosting is sponsored by [The Rust Foundation](https://foundation.rust-lang.org/)*

<small>[Discuss on r/rust](REDDIT_LINK_HERE)</small>
