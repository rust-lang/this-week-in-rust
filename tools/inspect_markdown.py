#!/usr/bin/python3

"""
Render one or more markdown files, and warn if:
- an odd number of ` characters appear in a line, possibly indicating
  a broken code escape.
- any html tags appear that are unrecognized, as this might indicate
  some < > characters in the markdown were not properly escaped.
"""

import argparse
import bs4
import markdown
import sys
from inspect_links import get_recent_files, setup_logging, warnings

"""
This is a collection of html tags that have appeared in past issues.
If a new tag appears, please add to the list.
"""
VALID_TAGS = ['p', 'a', 'h1', 'h2', 'h3', 'h4', 'h5', 'strong', 'hr',
              'li', 'ul', 'ol', 'em', 'code', 'blockquote', 'small', 'br']


def render_file(filename):
    """ Render markdown to html. """
    md_text = open(filename).read()

    # Warn if there are an odd number of backticks.
    for line in md_text.splitlines():
        # Ignore ``` and ````, since they will throw off the count
        line = line.replace('````', '')
        line = line.replace('```', '')

        # Empty backticks don't make much sense.
        if line.count('``') > 0:
            warnings.warn(f'{filename}: found empty backticks: "{line}"')

        c = line.count('`')
        if c % 2 != 0:
            warnings.warn(f'{filename}: found odd backticks: "{line}"')

    html = markdown.markdown(md_text)
    return html


def check_tags(html, file):
    """ Render markdown to html. """
    prev_tag = None
    for tag in bs4.BeautifulSoup(html, 'html.parser').find_all():
        if tag.name not in VALID_TAGS:
            tag_str = str(tag)[:50]
            warnings.warn(
                f'{file}: unrecognized tag {tag.name} in "{tag_str}"')
        if tag.name == 'li':
            if tag.get_text() == '':
                warnings.warn(f'{file}: empty <{tag.name}> tag after {prev_tag}')
        prev_tag = tag

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--file', help="Markdown file to inspect")
    parser.add_argument('--paths', default='content:draft',
                        help="Directory paths to inspect (colon separated)")
    parser.add_argument('--num-recent', default=10, type=int,
                        help="Number of recent files to inspect")
    args = parser.parse_args()

    if args.file:
        check_tags(render_file(args.file), args.file)
    else:
        file_list = get_recent_files(args.paths, args.num_recent)
        for file in file_list:
            check_tags(render_file(file), file)


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
