#!/bin/sh

# TODO: Make sure running from latest "main" branch commit
# TODO: Remove `BLOG_DOWNLOAD` variable requirement. Adjust "email" logic to:
# 			- Run generate container
#			- Curl website
#  			- Run HTML-friendly container

# Typical flows:
# 
# 1. The first workflow will generate the desired website, landing
#    the end contents in the local "output/" directory. This is the 
#    equivalent of running `pelican --delete-output-directory content`
#    from a properly instantantiated environment.
#
#    $ make build && make generate-website && make host-content
#   
#    Then, visit the printed URL from a browser to verify.
# 
# 2. This workflow will generate the desired email template, landing
#    the end contents in the local "output/" directory. This is the 
#    equivalent of running `USE_EMAIL_THEME=1 pelican --delete-output-directory content`
#    from a properly instantantiated environment.   
# 
#    $ make build && make generate-email && make host-content
#
# 	Then, visit the printed URL from a browser to verify.
#   Once verified, one can adjust the "BLOG_DOWNLOAD" variable below, and 
#   we can create an email-friendly HTML version of the latest Blog page. 
#   **While the above container is still running**, open another terminal.
#   This is the equivalent of running 
#   `curl ${BLOG_DOWNLOAD} > juice/in.html && juice --web-resources-images false /juice/in.html /juice/out.html`
#   
#    $ make optimize-email
#
#   This results in the desired email-friendly HTML version: "juice/out.html".

# TODO: The BLOG_DOWNLOAD is only needed because one dockerfile will have to communicate
#       with another, and I don't want to deal with building out the networking logic... yet...
# CHANGE ME!  vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
BLOG_DOWNLOAD=http://localhost:8000/blog/2023/01/25/this-week-in-rust-479/

website-workflow: build clean generate-website host-content

email-workflow-1: build clean generate-email host-content
email-workflow-2: optimize-email

build:
	cd .. && docker build -t twir -f publishing/Dockerfile . && cd -

clean:
	rm -rf output/

generate-website: clean
	docker run -it \
		-v $(shell pwd)/output:/usr/twir/output \
		twir:latest 

generate-email: clean
	docker run -it \
		-e USE_EMAIL_THEME=1 \
		-v $(shell pwd)/output:/usr/twir/output \
		twir:latest

host-content:
	docker run -it \
		-p 8000:8000 \
	 	-v $(shell pwd)/output:/usr/twir/output:ro \
		-it \
		twir:latest \
		bash run_server.sh

optimize-email:
	@echo -n "Is this '${BLOG_DOWNLOAD}' your desired blog? [y/N] " && read ans && [ $${ans:-N} = y ]
	rm -rf juice
	mkdir juice
	curl ${BLOG_DOWNLOAD} > juice/in.html
	docker run \
		-v $(shell pwd)/juice:/usr/twir/juice \
		twir:latest \
		bash create_html_friendly_page.sh
