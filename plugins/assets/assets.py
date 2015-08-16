# -*- coding: utf-8 -*-
"""
Asset management plugin for Pelican
===================================

This plugin allows you to use the `webassets`_ module to manage assets such as
CSS and JS files.

The ASSET_URL is set to a relative url to honor Pelican's RELATIVE_URLS
setting. This requires the use of SITEURL in the templates::

    <link rel="stylesheet" href="{{ SITEURL }}/{{ ASSET_URL }}">

.. _webassets: https://webassets.readthedocs.org/

"""
from __future__ import unicode_literals

import os
import logging

from pelican import signals
logger = logging.getLogger(__name__)

try:
    import webassets
    from webassets import Environment
    from webassets.ext.jinja2 import AssetsExtension
except ImportError:
    webassets = None

def add_jinja2_ext(pelican):
    """Add Webassets to Jinja2 extensions in Pelican settings."""

    pelican.settings['JINJA_EXTENSIONS'].append(AssetsExtension)


def create_assets_env(generator):
    """Define the assets environment and pass it to the generator."""

    theme_static_dir = generator.settings['THEME_STATIC_DIR']
    assets_destination = os.path.join(generator.output_path, theme_static_dir)
    generator.env.assets_environment = Environment(
        assets_destination, theme_static_dir)

    if 'ASSET_CONFIG' in generator.settings:
        for item in generator.settings['ASSET_CONFIG']:
            generator.env.assets_environment.config[item[0]] = item[1]

    if 'ASSET_BUNDLES' in generator.settings:
        for name, args, kwargs in generator.settings['ASSET_BUNDLES']:
            generator.env.assets_environment.register(name, *args, **kwargs)

    if 'ASSET_DEBUG' in generator.settings:
        generator.env.assets_environment.debug = generator.settings['ASSET_DEBUG']
    elif logging.getLevelName(logger.getEffectiveLevel()) == "DEBUG":
        generator.env.assets_environment.debug = True

    for path in (generator.settings['THEME_STATIC_PATHS'] +
                 generator.settings.get('ASSET_SOURCE_PATHS', [])):
        full_path = os.path.join(generator.theme, path)
        generator.env.assets_environment.append_path(full_path)


def register():
    """Plugin registration."""
    if webassets:
        signals.initialized.connect(add_jinja2_ext)
        signals.generator_init.connect(create_assets_env)
    else:
        logger.warning('`assets` failed to load dependency `webassets`.'
                       '`assets` plugin not loaded.')
