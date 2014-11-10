Version 1.3
===========

* Next and previous article navigation is placed below comments section so that article's content and comments appear together
* Article title and site name in `<title>` tag is separated by ` · ` which is cleaner and more subtle than ` -  ` 
* Subtitle of articles and pages is added in `<title>` tag along with main title
* Description meta tag on Home Page uses `SITE_DESCRIPTION`
* Bug fix: Expand comments section if URL points to a comment
* Bug fix: CSS style of links in an unordered list inside article content is different from article links

Version 1.2
===========

* RSS and Atom feed links
* CSS style for permanent links added. It is visible only user hovers over the heading
* Block quote is indented towards left
* Bug fix: Hyperlink dashed underline is not visible on Chrome
* Bug fix: Text in list goes beyond list marker when text is long and overflows to next line
* Bug fix: Disqus comment count is always 0

Version 1.1
===========

* Add template for pages. Pages do not have tags, category and Disqus comments
* Keep style of a hyperlink in `modified` meta data consisted with the theme
* Add `keywords` meta tag that uses keywords, tags and category attribute of articles and pages
* Validate search form for empty strings
* If `RECENT_ARTICLES_COUNT` is undefined, set it to 10. So that Pelican does not throw critical error
* Bug fix: Path of search.html in search form action should always be absolute
* Bug fix: Copyright meta tag should be set to the author, instead of the license
* Bug fix: Close meta tags
* Bug fix: ID of search form in 404 page should be different from the ID of search form in main navigation
* Bug fix: Links in ordered list in an article do not conform to the link style in rest of the article

Version 1.0
===========

* Initial release
