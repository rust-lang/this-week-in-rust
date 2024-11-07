import os
import re
import json
# Retrieves URLs of all past meetups

# Directory where the files are located (adjusted to be one level up from /tools/events)
draft_dir = os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'content')
print("Draft directory: {}".format(draft_dir))

# Updated Regex pattern to match URLs and allow for "_" and "-" in the event name
url_pattern = r"www\.meetup\.com/([a-zA-Z0-9_-]+)/events/?"

# Set to store unique URLs
unique_urls = set()

# Traverse through all files in the draft directory
for root, dirs, files in os.walk(draft_dir):
    for file in files:
        if file.endswith(".md"):  # Ensure we are only processing .md files
            file_path = os.path.join(root, file)
            print("Processing file: {}".format(file_path))
            try:
                # Open file in binary mode to handle encoding issues
                with open(file_path, 'rb') as f:
                    content = f.read().decode('utf-8', errors='replace')  # Replace invalid characters
                    # Find all URLs matching the pattern
                    matches = re.findall(url_pattern, content)
                    for match in matches:
                        # Reconstruct the full URL and lowercase the event name
                        full_url = "https://www.meetup.com/{}/events/".format(match.lower())
                        unique_urls.add(full_url)
            except Exception as e:
                print(f"Error processing file {file_path}: {e}")

# Sort URLs based on the alphanumeric part of the URL (case-insensitive sorting)
sorted_urls = sorted(unique_urls, key=lambda url: re.search(url_pattern, url).group(1).lower())

# Output to meetups.json
output_file = os.path.join(os.path.dirname(os.path.abspath(__file__)), 'meetups.json')
with open(output_file, 'w', encoding='utf-8') as json_file:
    json.dump(sorted_urls, json_file, indent=4)

print("URLs have been saved to {}".format(output_file))
