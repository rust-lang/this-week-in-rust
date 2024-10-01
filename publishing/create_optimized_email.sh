#!/bin/sh

source utils.sh

LOCAL_EMAIL_PREFIX="email/${LATEST_ISSUE_NUMBER}-${LATEST_YEAR}-${LATEST_MONTH}-${LATEST_DAY}"
echo "Creating email for ${LATEST_ISSUE_NUMBER}-${LATEST_YEAR}-${LATEST_MONTH}-${LATEST_DAY}"

# Prepare email directory
mkdir -p email
rm -f ${LOCAL_EMAIL_PREFIX}-in.html
cp ${LATEST_ISSUE_FULL_PATH} ${LOCAL_EMAIL_PREFIX}-in.html

docker run \
    -v $(pwd)/email:/usr/twir/email \
    -e LOCAL_EMAIL_PREFIX=${LOCAL_EMAIL_PREFIX} \
    twir:latest \
    bash create_html_friendly_page.sh 

rm ${LOCAL_EMAIL_PREFIX}-in.html

printf "\n\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n📧 HEY TWiR PUBLISHER..."
printf "\nLatest email template found at:  ${YELLOW_FONT}'$(pwd)/${LOCAL_EMAIL_PREFIX}-email.html'${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}"
printf "\n${CYAN_FONT}*****${PURPLE_FONT}*****${CYAN_FONT}*****${PURPLE_FONT}*****${NC}\n\n"
