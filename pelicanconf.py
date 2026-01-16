#!/usr/bin/env python
# -*- coding: utf-8 -*- #
from __future__ import unicode_literals
import os

# Adjust `TESTING_LOCALLY` if testing search functionality locally.
TESTING_LOCALLY = False
USE_EMAIL_THEME = True if os.environ.get('USE_EMAIL_THEME') == '1' else False

AUTHOR = u'TWiR Contributors'
SITENAME = u"This Week in Rust"
SITEURL = 'https://this-week-in-rust.org' if not TESTING_LOCALLY else 'http://localhost:8000'

SOURCE_URL = 'https://github.com/rust-lang/this-week-in-rust'

if USE_EMAIL_THEME:
    THEME = 'themes/newsletter'
else:
    THEME = 'themes/rusted'

THEME_STATIC_DIR = THEME + '/static'

TIMEZONE = 'America/New_York'

DEFAULT_LANG = u'en'

FEED_DOMAIN = SITEURL
FEED_ALL_ATOM = 'atom.xml'
FEED_ALL_RSS = 'rss.xml'
FEED_MAX_ITEMS = 4
CATEGORY_FEED_ATOM = 'categories/{slug}/atom.xml'
CATEGORY_FEED_RSS = 'categories/{slug}/rss.xml'
RSS_FEED_SUMMARY_ONLY = False

DEFAULT_PAGINATION = 10

ARTICLE_URL = 'blog/{date:%Y}/{date:%m}/{date:%d}/{slug}/'
ARTICLE_SAVE_AS = 'blog/{date:%Y}/{date:%m}/{date:%d}/{slug}/index.html'

ARCHIVES_SAVE_AS = 'blog/archives/index.html'

CATEGORY_URL = 'categories/{slug}/'
CATEGORY_SAVE_AS = 'categories/{slug}/index.html'

LANDING_PAGE_ABOUT = {
        "title": "Cataloging the Rust community's awesomeness",
        "details": """
A weekly newsletter about Rust and the Rust community, with bonus content
scattered about.
"""
}

PLUGIN_PATHS = ["plugins"]

# Don't add next/previous buttons search functionality for email.
if USE_EMAIL_THEME:
    PLUGINS = ['webassets']
else:
    PLUGINS = ['webassets', 'neighbors']

MARKDOWN = {
    'extension_configs': {
        'markdown.extensions.codehilite': {'css_class': 'highlight'},
        'markdown.extensions.extra': {},
        'markdown.extensions.meta': {},
        'markdown.extensions.toc': {
            'anchorlink': True,
        },
    },
    'output_format': 'html5',
}
