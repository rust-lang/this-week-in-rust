# Default OUTPUT_PREFIX, but allow overriding for email workflows.
if [ -z "$OUTPUT_PREFIX" ]; then
  OUTPUT_PREFIX="output"
fi

LATEST_YEAR=$(ls ${OUTPUT_PREFIX}/blog/ | sort | tail -2 | head -1)
LATEST_MONTH=$(ls ${OUTPUT_PREFIX}/blog/${LATEST_YEAR} | sort | tail -1)
LATEST_DAY=$(ls ${OUTPUT_PREFIX}/blog/${LATEST_YEAR}/${LATEST_MONTH} | sort | tail -1)
LATEST_ISSUE_NUMBER=$(ls ${OUTPUT_PREFIX}/blog/${LATEST_YEAR}/${LATEST_MONTH}/${LATEST_DAY}/ | awk -F'-' '{print $5}')
LATEST_ISSUE_FULL_PATH="${OUTPUT_PREFIX}/blog/${LATEST_YEAR}/${LATEST_MONTH}/${LATEST_DAY}/this-week-in-rust-${LATEST_ISSUE_NUMBER}/index.html"
LATEST_BLOG_URL="http://localhost:8000/blog/${LATEST_YEAR}/${LATEST_MONTH}/${LATEST_DAY}/this-week-in-rust-${LATEST_ISSUE_NUMBER}/"

YELLOW_FONT='\033[1;32m'
CYAN_FONT='\033[0;36m'
PURPLE_FONT='\033[1;35m'
NC='\033[0m' # No Color
