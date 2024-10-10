#!/bin/bash

source utils.sh

printf "\n\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\nHEY TWiR PUBLISHER..."
printf "\nLatest blog found at: ${YELLOW_FONT}'${LATEST_BLOG_URL}'${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}\n\n"

cd output
python3 -m http.server
