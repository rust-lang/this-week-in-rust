State of Rust 0.11.0
====================

:date: 2014-07-15 15:40
:category: Rust
:slug: state-of-rust-0.11.0

Hello and welcome to the *State of Rust*.  `Rust`_ is a systems
language pursuing the trifecta: safe, concurrent, and fast.

Over the past 6 months since the last one of these updates was
written, Rust has evolved significantly: the standard library was
refactored to make Rust more convenient to use in embedded or
bare-metal platforms, the language has been greatly simplified (moving
most pointer types into libraries) and the package ecosystem has been
thriving under a new package manager.

The Rust Project
----------------

Rust is now its `own organization`_ on GitHub! This reflects that Rust
is a major project with its own community and culture, and not simply
another project under the Mozilla umbrella. Additionally, the `meeting
minutes`_ now live in that organization, if you're interested in
watching those. We have also introduced a `Discourse forum`_ for
discussing the design and implementation of Rust and its standard
libraries.  An `RFC process`_ has been introduced for proposing
changes to the language.

We now have `official nightlies`_ and a `script that installs them`_, which
provides a dead-simple way to install Rust without waiting over half an hour
for a build. Our `online sandbox`_ also provides instant gratification, and
all doc examples are runnable in it.

Moving on from meta-topics, many changes have happened in the language
itself. Many features have been removed.  These and other less-major
changes make the language simpler, more consistent, and more powerful.
An incomplete list:

* ``do`` expressions
* The ``Freeze`` kind
* The ``priv`` keyword (everything is private-by-default)
* All variations of ``@``-pointers. They are replaced by the library
  type, ``Gc<T>``, which shares many of the same problems ``@`` did,
  implementation-wise.
* All variations of ``~``-pointers. They are replaced by the library
  types, ``Box<T>`` and ``Vec<T>`` for ``~T`` and ``~[T]``
  respectively.
* Dereferencing is now overloadable, via the ``Deref`` and
  ``DerefMut`` types. This makes creating and using smart pointers
  much more painless!
* A "placement new", or more accurately, "placement box" feature has
  been added. This replaces ``~expr`` and will be extensible to user
  types. ``box 42i`` has type ``Box<int>``. This feature is still
  baking.

The libraries have also seen a lot of love. Rather than having a
single, monolithic ``libextra``, there are now multiple more focused
libraries. A list is available on our fancy new `documentation index
page`_. Of particular note, a huge amount of effort was spent
splitting ``libstd`` itself into smaller pieces. There is a (rather
large!) subset, which lives in ``libcore`` that uses no dynamic
allocation or other such runtime features. ``libcore`` is suitable for
use in embedded, bare-metal, and other resource constrained
environments. ``libstd`` still presents a unified interface to all of
the individual pieces, and is the stable entry point to the entire
standard library.

The compiler has also grown a plugin system, that allows for
user-defined syntax extensions, lint passes, and exportable macros.
This is not well documented yet, but the new `regex crate`_ takes
great advantage of this to provide compile-time checked regular
expressions that can compile *directly* into Rust code, rather than
always running them in a virtual machine at runtime or requiring a
complex JIT compiler for good performance.

Steve Klabnik has been hired on a 6 month contract to improve our
documentation. He has been busy filling in examples and writing our `new
guide`_. Our documentation will be vastly improved as time goes on, in
preparation for the big 1.0.

.. _Rust: http://www.rust-lang.org/
.. _We love contributions: https://github.com/rust-lang/rust/wiki/Note-guide-for-new-contributors
.. _own organization: https://github.com/rust-lang/
.. _online sandbox: http://play.rust-lang.org/
.. _official nightlies: http://www.rust-lang.org/install.html
.. _script that installs them: http://www.rust-lang.org/rustup.sh
.. _meeting minutes: https://github.com/rust-lang/meeting-minutes
.. _Discourse forum: http://internals.rust-lang.org/
.. _RFC process: https://github.com/rust-lang/rfcs/
.. _documentation index page: http://doc.rust-lang.org/#libraries
.. _regex crate: http://doc.rust-lang.org/regex/
.. _new guide: http://doc.rust-lang.org/guide.html

The Rust Ecosystem
------------------

Perhaps the most marked change in Rust is its community. It is growing
faster than I could have anticipated. There are many projects being
undertaken by groups of people who aren't also working on the compiler
or standard library. This is a significant shift from the past, where
trying to do a project in Rust almost forced you to improve Rust or
abandon it. In particular:

* The Zinc_ project is building a bare-metal ARM stack aiming at
  writing a completely safe RTOS toolkit. It uses a loadable syntax
  extension to describe the hardware features of a particular
  hardware platform.
* The Piston_ project is a growing community of game developers
  building libraries and an engine to make developing interactive
  graphical applications painless and safe.
* The Iron_ project has created a Rack-inspired web framework that
  allows easily composable "middleware" to stack and build a robust
  web application.

There are many other smaller libraries and projects the community is
undertaking, however these three represent major areas that Rust is
great for and are the largest (in terms of developers/maturity) in
their respective spaces.

Making things even better, the official package manager, Cargo_, has
finally been released! It is only in alpha, but is maturing quickly.
The magnificent `Rust CI`_ is tracking which repositories have Cargo
support. Uptake has been rapid, which bodes very well for its
continued success.

Looking Forward ("Is Rust ready yet?")
--------------------------------------

Rust is *not* ready yet. It still has a few more release cycles before 1.0. In
particular, the "unboxed closure" and "dynamically sized types" work is not
yet complete. There are also many minor changes to improve ergonomics in the
pipeline. That said, it is becoming more stable, and many major features are
complete. If you want to, give it a spin, and let us know how it goes!

.. _Zinc: http://zinc.rs/
.. _Piston: http://www.piston.rs/
.. _Iron: http://ironframework.io/
.. _Cargo: http://crates.io/
.. _Rust CI: http://rust-ci.org/
