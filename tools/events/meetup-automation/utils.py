from dataclasses import dataclass
from enum import Enum
from typing import List, Optional
from urllib.parse import urlparse
import json

class LocationOverride(str, Enum):
    HYBRID = "hybrid"
    VIRTUAL = "virtual"


@dataclass
class MeetupGroupUrl:
    MEETUP_HOSTNAME = "www.meetup.com"

    url: str
    url_name: str
    location_override: Optional[LocationOverride]

    def __init__(self, url_str: str, location_override: Optional[str]) -> None:
        parsed = urlparse(url_str)

        if parsed.hostname != self.MEETUP_HOSTNAME:
            raise ValueError(f"Invalid hostname in URL {url_str}, expected  {self.MEETUP_HOSTNAME}")            

        path_split = parsed.path.split('/')

        if len(path_split) < 2:
            raise ValueError(f"Unable to parse meetup group name from URL {url_str}")

        self.url = url_str
        self.url_name = path_split[1]

        if location_override:
            self.location_override = LocationOverride(location_override)
        else:
            self.location_override = None


def read_meetup_group_urls(meetups_json: str) -> List[MeetupGroupUrl]:
    with open(meetups_json, "r") as f:
        group_urls = json.loads(f.read())
        parsed_groups = []

        for url, metadata in group_urls.items():
            location_override = metadata.get("location_override")
            parsed = MeetupGroupUrl(url, location_override)
            parsed_groups.append(parsed)

        return parsed_groups
