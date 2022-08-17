#!/usr/bin/python3

"""
Inspect a set of markdown files, and warn if there are:
- duplicate links
- malformed links
"""

import argparse
import bs4
import logging
import markdown
import os
import re
import sys
import urllib

LOG = logging.getLogger(__name__)
LOG.setLevel(logging.INFO)


class Warnings:
    """ A singleton object for gathering warnings to be printed later. """

    def __init__(self):
        self.warnings = []
        self.silent = False

    def silence(self, val):
        self.silent = val

    def warn(self, msg):
        if not self.silent:
            self.warnings.append(msg)

    def get(self):
        return self.warnings


# The singleton object that gathers warnings, for later reporting.
warnings = Warnings()

# A regex that matches filenames to inspect.
RE_FILENAME = re.compile(r'\d\d\d\d-\d\d-\d\d-this-week-in-rust.md$')

# A block-list of tracking parameters
TRACKING_PARAMETERS = set([
    'utm_source',
    'utm_campaign',
    'utm_medium',
    'utm_content',
])

# A list of section titles that will trigger duplicate-tag detection.
STRICT_TITLES = [
    'updates from rust community',
]


def is_strict_title(title):
    """ Return True if this title is one that needs strict checks. """
    title = str(title)
    # .lower() doesn't necessarily handle unicode in a robust way,
    # but the set of strings we care about is tiny, and use only ascii.
    return title.lower() in STRICT_TITLES


def check_truncated_title(tag):
    """ Flag any links that have a probably-truncated title.

    Links collected from Discord may be truncated to a length of exactly
    70 characters, including a "..." suffix.

    If we're unlucky enough to trigger this warning by mistake, here are
    some workarounds:
    - Make any change to the title so that it's not exactly 70 characters
      (e.g. add an extra space between words)
    - Replace the "..." with unicode "…"
    """
    title = tag.string
    LOG.debug(f'link title: {repr(title)}')
    if title and title.endswith('...') and len(title) == 70:
        warnings.warn(f'truncated link title: {repr(title)}')


def extract_links(html):
    """ Return a list of links from this file.

    Links will only be returned if they are within a section deemed "strict".
    This allows us to ignore links that are deliberately repeated (to this
    github repo and twitter account, for example).

    Side-effects:
    - If links are malformed, warnings may be recorded. See `parse_url`
      for details.

    """
    strict_mode = False
    tags = ['a', 'h1', 'h2', 'h3', 'h4']
    urls = []

    # Remember the header level (h2, h3, etc) when we turned on
    # strict_mode.
    header_level = None

    for tag in bs4.BeautifulSoup(html, 'html.parser').find_all(tags):
        if tag.name == 'a':
            link = tag.get('href')
            LOG.debug(f'found link tag: {link}')
            if strict_mode:
                check_truncated_title(tag)
                trimmed_url = parse_url(link)
                urls.append(trimmed_url)
        else:
            level = tag.name
            if header_level and level > header_level:
                LOG.debug(f'skipping {tag}, overridden at {header_level}')
                continue

            # This is the title of a section. If this title is "strict",
            # we will check for any duplicate links inside it.

            strict_mode = is_strict_title(tag.string)
            if strict_mode:
                header_level = level
            else:
                header_level = None
            LOG.debug(f'found heading tag: {tag} (strict={strict_mode})')

    return urls


def scrub_parameters(url, query):
    """ Strip tracking parameters from the URL """
    query_dict = urllib.parse.parse_qs(query)

    filtered_dict = {}
    found_tracking = []
    for k, v in query_dict.items():
        if k in TRACKING_PARAMETERS:
            found_tracking.append(k)
        else:
            filtered_dict[k] = v

    # Store a warning if
    if found_tracking:
        warnings.warn(f'found tracking parameters on {url}: {found_tracking}')

    # If there are no query parameters left, return the empty string.
    if not filtered_dict:
        return ''

    # Re-encode remaining URL paramaters
    return urllib.parse.urlencode(filtered_dict, doseq=True)


def parse_url(link):
    """ Parse a URL and return it in a stripped-down form.

    In an effort to detect duplicate URLs, some information is removed:
    - tracking parameters are removed (see `scrub_parameters`)
    - "http" and "https" URLs are considered the same.
    - consecutive slashes and trailing slashes are ignored.

    Warnings may be issued if unnecessary tracking parameters are found,
    or if the URL contains consecutive slashes.
    """
    parsed_url = urllib.parse.urlsplit(link)

    scheme = parsed_url.scheme
    if scheme not in ('mailto', 'http', 'https'):
        warnings.warn(f'possibly malformed link: {link}')
    if scheme == 'http':
        scheme = 'https'

    # If there are query parameters present, give them a cleanup pass to remove irrelevant ones.
    query = parsed_url.query
    if query:
        LOG.debug(f'{parsed_url.geturl()} found query parameters: {query}')
        query = scrub_parameters(link, query)
        if query:
            LOG.debug(
                f'{parsed_url.geturl()} keeping query parameters: {query}')

    # Remove consecutive slashes, because https://path/to////file and http://path/to/file are the same.
    path = parsed_url.path
    path_components = path.split('/')
    trailing_slash = path_components[-1] == ''
    path_components = [s for s in path_components if s]
    path = '/'.join(path_components)
    if trailing_slash:
        path += '/'

    # Re-constitute the URL with any simplifications that should trigger a warning.
    (sch, loc, _, _, frag) = parsed_url
    reconstituted = urllib.parse.urlunsplit((sch, loc, path, query, frag))
    if reconstituted != link:
        LOG.debug(f'reconstituted: {reconstituted}')
        warnings.warn(f'link can be simplified: {link} -> {reconstituted}')

    # Strip any trailing slashes, again.
    path = path.rstrip('/')

    # Re-constitute a second time, including more simplifications that we don't
    # need to warn about
    reconstituted = urllib.parse.urlunsplit((scheme, loc, path, query, frag))

    return reconstituted


def inspect_file(filename):
    LOG.info(f'inspecting file {filename}')
    md_text = open(filename).read()
    html = markdown.markdown(md_text)
    links = extract_links(html)
    LOG.debug(f'examining {len(links)} links')
    return links


def get_recent_files(dirs, count):
    """ return a list of the N most recent markdown files in `dir`.

    We assume the files are named "YYYY-MM-DD-this-week-in-rust-md".
    """
    LOG.debug(f'searching for {count} recent files in "{dirs}"')

    listing = []
    for dir in dirs.split(':'):
        files = os.listdir(path=dir)
        if not files:
            raise Exception(f'No files found in {dir}')
        files = list(filter(RE_FILENAME.match, files))
        if not files:
            raise Exception(f'No matching files found in {dir}')

        # create a tuple (file, file+path) so we can sort by filename
        file_tuples = [(f, os.path.join(dir, f)) for f in files]
        listing.extend(file_tuples)

    listing.sort()
    listing = listing[-count:]

    # return the file+path.
    listing = [tup[1] for tup in listing]

    LOG.info(f'recent files: {listing}')
    return listing


def inspect_files(file_list, num_warn):
    """ Inspect a set of files, storing warnings about duplicate links. """
    linkset = {}

    # If we inspect 5 files (enumerated 0-4), and want to warn on 2,
    # then the warnings start at N=3 (length - 1 - num_warn).
    warn_index = len(file_list) - 1 - num_warn

    for index, file in enumerate(file_list):
        warnings.silence(index < warn_index)
        links = inspect_file(file)
        LOG.debug(f'found links: {links}')
        for link in links:
            collision = linkset.get(link)
            if collision:
                warnings.warn(
                    f"possible duplicate link {link} in file {file} (also found in {collision}")
            else:
                linkset[link] = file


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--paths', default='content:draft',
                        help="Directory paths to inspect (colon separated)")
    parser.add_argument('--num-recent', default=25, type=int,
                        help="Number of recent files to inspect")
    parser.add_argument('--num-warn', default=1, type=int,
                        help="Number of recent files to warn about")
    parser.add_argument('--debug', action='store_true')
    args = parser.parse_args()
    if args.debug:
        LOG.setLevel(logging.DEBUG)
    LOG.debug(f'command-line arguments: {args}')
    file_list = get_recent_files(args.paths, args.num_recent)
    inspect_files(file_list, args.num_warn)


def setup_logging():
    log_stdout = logging.StreamHandler(sys.stdout)
    logging.getLogger('').addHandler(log_stdout)


if __name__ == "__main__":
    setup_logging()
    main()

    warns = warnings.get()
    if warns:
        print("warnings exist:")
        for w in warns:
            print(w)
        sys.exit(1)
    else:
        print("everything is ok!")
