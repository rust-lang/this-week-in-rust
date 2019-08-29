# -*- coding: utf-8 -*-
"""
Neighbor Articles Plugin for Pelican
====================================

This plugin adds ``next_article`` (newer) and ``prev_article`` (older)
variables to the article's context.

Adapted from: https://github.com/getpelican/pelican-plugins/blob/master/neighbors/neighbors.py
"""
from collections import namedtuple

from pelican import signals

Neighbors = namedtuple('Neighbors', ['next', 'current', 'previous'])


def iter_neighbors(seq):
    """Yields Neighbors for every article."""
    iterator = iter(seq)
    nxt = None
    current = next(iterator)
    for previous in iterator:
        yield Neighbors(next=nxt, current=current, previous=previous)
        nxt, current = current, previous
    yield Neighbors(next=nxt, current=current, previous=None)


def get_translation(article, preferred_language):
    if not article:
        return None
    for translation in article.translations:
        if translation.lang == preferred_language:
            return translation
    return article


def set_neighbors(articles, next_name, prev_name):
    for neighbor in iter_neighbors(articles):
        setattr(neighbor.current, next_name, neighbor.next)
        setattr(neighbor.current, prev_name, neighbor.previous)
        for translation in neighbor.current.translations:
            setattr(
                translation,
                next_name,
                get_translation(neighbor.next, translation.lang),
            )
            setattr(
                translation,
                prev_name,
                get_translation(neighbor.previous, translation.lang),
            )


def neighbors(generator):
    set_neighbors(generator.articles, 'next_article', 'prev_article')

    for category, articles in generator.categories:
        articles.sort(key=lambda x: x.date, reverse=True)
        set_neighbors(
            articles,
            'next_article_in_category',
            'prev_article_in_category',
        )


def register():
    signals.article_generator_finalized.connect(neighbors)
