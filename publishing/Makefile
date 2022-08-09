#!/bin/sh

# Typical flows:
# 
# 1. The first workflow will generate the desired website, landing
#    the end contents in the local "output/" directory. This is the 
#    equivalent of running `pelican --delete-output-directory content`
#    from a properly instantantiated environment.
#
#    $ make build && make generate-website && make host-content
#   
#    Then, visit "http://127.0.0.1:8000/blog" from a browser to verify.
# 
# 2. This workflow will generate the desired email template, landing
#    the end contents in the local "output/" directory. This is the 
#    equivalent of running `TWIR_NEWSLETTER_THEME=1 pelican --delete-output-directory content`
#    from a properly instantantiated environment.   
# 
#    $ make build && make generate-email && make host-content
#
# 	Then, visit "http://127.0.0.1:8000/blog" from a browser to verify.
#   Once verified, one can adjust the "BLOG_DOWNLOAD" variable below, and
#   create an email-friendly HTML version of the "BLOG_DOWNLOAD" page. 
#   This is the equivalent of running `juice --web-resources-images false /tmp/in.html /tmp/out.html`
#   
#   $ make optimize-email
#
#   This results in the desired email-friendly HTML version: "juice/out.html".

# CHANGE ME!  vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
BLOG_DOWNLOAD=http://127.0.0.1:8000/blog/2022/07/27/this-week-in-rust-453/

build:
	cd .. && docker build -t twir -f publishing/Dockerfile . && cd -

clean:
	rm -rf output/

generate-website: clean
	docker run \
		-v $(shell pwd)/output:/usr/twir/output \
		twir:latest 

generate-email: clean
	docker run \
		-e TWIR_NEWSLETTER_THEME=1 \
		-v $(shell pwd)/output:/usr/twir/output \
		twir:latest

host-content:
	docker run \
		-p 8000:8000 \
	 	-v $(shell pwd)/output:/usr/twir/output:ro \
		-it \
		twir:latest \
		bash run_server.sh

optimize-email:
	rm -rf juice
	mkdir juice
	curl ${BLOG_DOWNLOAD} > juice/in.html
	docker run \
		-v $(shell pwd)/juice:/usr/twir/juice \
		-e BLOG_DOWNLOAD=${BLOG_DOWNLOAD} \
		twir:latest \
		bash create_html_friendly_page.sh
