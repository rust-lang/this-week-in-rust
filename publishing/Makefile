#!/bin/sh

# TODO: Make sure running from latest "main" branch commit

# Typical flows:
#
# 1. `make website`
# 	 The first workflow will generate the desired website, landing
#    the end contents in the local "output-website/" directory. This is the 
#    equivalent of running `pelican --delete-output-directory content`
#    from a properly instantantiated environment.
#
#    $ generate-website host-website
#
#    Then, visit the printed URL from a browser to verify.
#
#    Output: `output-website/`
#
# 2. `make copy-website-contents`
#    This workflow will sync the `output-website/` directory from above, and sync
#    it with the directory passed to it. Used for syncing state with
#    this-week-in-rust.github.io repo.
#
# 3. `make email`
#    This workflow will generate the desired email template, landing
#    the end contents in the local "email/" directory. This is the 
#    equivalent of running `USE_EMAIL_THEME=1 pelican --delete-output-directory content`
#    from a properly instantantiated environment, and running
#    `juice --web-resources-images false /juice/in.html /juice/out.html` on the latest content post.
# 
#    $ build clean generate-email optimize-email
#
#	 Output: `email/<NUMBER>-<YEAR>-<MONTH>-<DAY>-email.html`
#

website: generate-website host-website
copy-website-contents:
	@./copy_website_content_to_repo.sh
email: generate-email optimize-email

build:
	cd .. && docker build -t twir -f publishing/Dockerfile . && cd -

clean-website:
	@rm -rf output/ && rm -rf output-website/ && rm -rf juice/
clean-email:
	@rm -rf output/ && rm -rf output-email-format/ && rm -rf email/ && rm -rf juice/

generate-website: build clean-website
	@echo "Generating website..."
	@docker run -it \
		-v $(shell pwd)/output-website:/usr/twir/output \
		twir:latest 
	@echo "Finished generating website."

host-website:
	@echo "Hosting website..."
	@docker run -it \
		-p 8000:8000 \
	 	-v $(shell pwd)/output-website:/usr/twir/output:ro \
		-it \
		twir:latest \
		bash run_server.sh
	@echo "Finished hosting website."
	@echo ""
	@echo "To sync contents with your local 'this-week-in-rust.github.io' repo, run \033[1;33m'make copy-website-contents'\033[0m"

generate-email: build clean-email
	@echo "Generating email..."
	@docker run -it \
		-e USE_EMAIL_THEME=1 \
		-v $(shell pwd)/output-email-format:/usr/twir/output \
		twir:latest

optimize-email:
	@echo "Generating optimized email..."
	@OUTPUT_PREFIX=output-email-format ./create_optimized_email.sh
