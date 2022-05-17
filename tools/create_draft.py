#!/usr/bin/python3

"""
Create a draft of a new issue.
"""

import argparse
import datetime
import logging
import os
import string
import sys


LOG = logging.getLogger(__name__)
LOG.setLevel(logging.INFO)

DRAFT_PATH = 'draft'
TEMPLATE_PATH = os.path.join('draft', 'DRAFT_TEMPLATE')
PREVIOUS_ISSUE_SEARCH_PATHS = ['draft', 'content']
EVENTS_LIST_PLACEHOLDER = '<!-- Events list goes here -->'


def path_from_root(path):
    """ Returns a path computed relative to the repository root.

    This is determined by computing the path to this script, then
    traversing up one directory and appending the `path` argument.
    """

    self_path = os.path.abspath(__file__)
    self_dir = os.path.dirname(self_path)
    root_path, _ = os.path.split(self_dir)
    return os.path.join(root_path, path)


def get_template_path():
    """ Returns the path to the template file. """
    return path_from_root(TEMPLATE_PATH)


def default_draft_path():
    """ Returns the default path where the draft issue should be written. """
    return path_from_root(DRAFT_PATH)


def issue_filename(date):
    """ Compute the issue filename, given the date. """
    return date.isoformat() + '-this-week-in-rust.md'


def default_date():
    """ Returns a datetime.date, that is the estimated next issue date. """
    # If no input date is specified, we assume that this will be run ~7 days
    # before the next issue is released. Just in case an issue comes out early
    # or late, we'll pick the Wednesday that is between 4 and 10 days in the
    # future.
    starting_day = datetime.date.today() + datetime.timedelta(4)
    # Compute the number of days until the next Wednesday.
    # weekday() returns 0(Monday)..6(Sunday). We want 2.
    delta = (2 - starting_day.weekday()) % 7
    date = starting_day + datetime.timedelta(delta)
    return date


def compute_issue_number(date):
    # 2022-01-05 is issue 424; we can calculate number of weeks since then.
    ref_date = datetime.date(2022, 1, 5)
    delta = date - ref_date
    if delta.days % 7 == 0:
        return 424 + int(delta.days / 7)
    raise Exception('failed to compute issue number')


def read_previous_issue(date):
    """ Read the previous issue, and return the file contents. """

    previous = date - datetime.timedelta(7)
    filename = issue_filename(previous)
    for path in PREVIOUS_ISSUE_SEARCH_PATHS:
        full_path = os.path.join(path_from_root(path), filename)
        try:
            # Read and return the file contents
            return open(full_path).read()
        except Exception as e:
            # Note: this error doesn't necessarily indicate something
            # is wrong, because we're searching multiple paths.
            LOG.debug(f'failed to read {full_path}: {e}')
            pass
    raise Exception(f'failed to read previous issue {filename}')


def read_previous_events(date):
    """ Attempt to copy the events list from the previous issue. """

    EVENTS_START = 'Rusty Events between'
    EVENTS_END = 'If you are running a Rust event'

    previous_issue = read_previous_issue(date)
    events_list = ''
    found_events = False
    for line in previous_issue.splitlines():
        if found_events:
            if line.strip().startswith(EVENTS_END):
                # We found the end of the events list.
                # Strip blank lines at the beginning and end.
                return events_list.strip('\n')
            else:
                # Copy this line
                events_list += line + '\n'
        else:
            if line.strip().startswith(EVENTS_START):
                # Start copying lines after this one.
                found_events = True

    # If we made it here, something went wrong with the extraction.
    # Most likely, the previous issue doesn't have the strings we are
    # searching for (EVENTS_START, EVENTS_END).
    raise Exception(f'failed to extract the previous events list')


def create_draft(date):
    """ Return a new issue draft based on the template file. """
    # Events listing ends on issue_date + 28 days.
    end_date = date + datetime.timedelta(28)

    try:
        events_list = read_previous_events(date)
    except Exception as e:
        LOG.warning(f'failed to copy previous events: {e}')
        events_list = EVENTS_LIST_PLACEHOLDER

    params = {
        'twir_issue_number': compute_issue_number(date),
        'twir_issue_date': date.isoformat(),
        'twir_events_end_date': end_date.isoformat(),
        'twir_events_list': events_list,
    }

    template_path = get_template_path()
    template = open(template_path).read()
    template = string.Template(template)
    return template.substitute(params)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--date', default=None, metavar='YYYY-MM-DD',
                        help='draft date (defaults to Wednesday ~7 days away)')
    parser.add_argument('--draft-path', default=None, metavar='DIR',
                        help='directory to write the new draft to')
    parser.add_argument('--dry-run', action='store_true',
                        help="don't write the file; print it to stdout")
    parser.add_argument('--debug', action='store_true',
                        help='be more verbose')
    args = parser.parse_args()
    if args.debug:
        LOG.setLevel(logging.DEBUG)
    LOG.debug(f'command-line arguments: {args}')

    # Compute the issue date.
    if args.date:
        date = datetime.date.fromisoformat(args.date)
    else:
        date = default_date()
    LOG.debug(f'issue date {date}')

    # Create the draft filename: draft/YYYY-MM-DD-this-week-in-rust.md
    if args.draft_path:
        draft_path = args.draft_path
    else:
        draft_path = default_draft_path()
    filename = os.path.join(draft_path, issue_filename(date))

    # Create the draft text, and either write it to a file, or to stdout.
    draft = create_draft(date)
    if args.dry_run:
        print(draft)
    else:
        open(filename, 'x').write(draft)


def setup_logging():
    log_stdout = logging.StreamHandler(sys.stdout)
    logging.getLogger('').addHandler(log_stdout)


if __name__ == "__main__":
    setup_logging()
    main()
