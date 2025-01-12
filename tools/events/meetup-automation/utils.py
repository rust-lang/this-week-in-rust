from dataclasses import dataclass
from typing import List
from urllib.parse import urlparse
import json

@dataclass
class MeetupGroupUrl:
    MEETUP_HOSTNAME = "www.meetup.com"

    url: str
    url_name: str

    def __init__(self, url_str: str) -> None:
        parsed = urlparse(url_str)

        if parsed.hostname != self.MEETUP_HOSTNAME:
            raise ValueError(f"Invalid hostname in URL {url_str}, expected  {self.MEETUP_HOSTNAME}")            

        path_split = parsed.path.split('/')

        if len(path_split) < 2:
            raise ValueError(f"Unable to parse meetup group name from URL {url_str}")

        self.url = url_str
        self.url_name = path_split[1]
        

def read_meetup_group_urls(meetups_json: str) -> List[MeetupGroupUrl]:
    with open(meetups_json, "r") as f:
        group_urls = json.loads(f.read())

        parsed_groups = [MeetupGroupUrl(url) for url in group_urls]
        return parsed_groups
