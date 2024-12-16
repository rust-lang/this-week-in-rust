# import all the event sources & event sink
# collect all the events from the event sources
# call event sink with our collected events
# print to console / output to file formatted markdown

import argparse
import logging
from geopy.geocoders import Nominatim
from typing import List

from datetime import date, timedelta
from country_code_to_continent import country_code_to_continent
from generate_events_meetup import TwirMeetupClient
from event import Event
from utils import read_meetup_group_urls

# TODO: Flagged events list handling.

logger = logging.getLogger(__name__)

def main():
    args = parse_args()

    log_level = logging.DEBUG if args.debug else logging.INFO
    logging.basicConfig(level=log_level)

    logger.info("Starting...")

    meetup_client = TwirMeetupClient()
    geolocator = Nominatim(user_agent="TWiR")

    # get our known rust meetup groups
    group_urls = read_meetup_group_urls(args.groups_file)

    events = []
    for group_url in group_urls:
        group_raw_events = meetup_client.get_raw_events_for_group(group_url)

        events += [raw_event.to_event(geolocator, group_url.url) for raw_event in group_raw_events]

    # Remove events outside of date range.
    events = date_window_filter(events, args.weeks)

    # Sort remaining events by date, then location.
    events.sort(key=lambda event: (event.date, event.location.to_str()))

    # for event in events:
    #     print(event.to_markdown_string())

    # Remove potential duplicate events.
    events = remove_duplicate_events(events)
    
    # Group by virtual or by continent.
    events = group_virtual_continent(events)

    # Output Sorted Event List.
    output_to_screen(events)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description='Fetches meetup events for TWIR')
    parser.add_argument("-d", "--debug", action="store_true", dest="debug", help="Enable debug logging")
    parser.add_argument("-g", "--groups", action="store", type=str, dest="groups_file", required=True, help="File with a JSON array of meetup group URLS")
    parser.add_argument("-w", "--weeks", action="store", type=int, dest="weeks", default=5, help="Number of weeks to search for events from, starting next Wednesday")

    return parser.parse_args()


def output_to_screen(event_list):
    # Prints sorted Event List to terminal screen.
    for key, value in event_list.items():
        if len(value) == 0:
            continue
        else:
            print(f'### {key}:\n')
            
            # Output event details
            for event in value:
                print(event.to_markdown_string())
            print()


def date_window_filter(events: List[Event], weeks: int) -> List[Event]:
    # Removes Events that are outside current date window.
    # Date window = closest wednesday + 5 weeks.
    start_date = date.today()
    while start_date.weekday() != 2:
        start_date = start_date + timedelta(days=1)
        
    valid = []
    for event in events:
        if not (start_date <= event.date.date() <= start_date + timedelta(weeks=weeks)):
            logger.debug(f"Removed event outside of date range: {event}")
        else:
            valid.append(event)

    return valid


def group_virtual_continent(event_list):
    # Return dictionary of events separated in virtual and by continent.
    separated_event_list = {}

    for event in event_list:
        # Separates Events by Virtual or by Continent
        key = "Virtual" if event.virtual else country_code_to_continent(event.location.country)
        separated_event_list.setdefault(key, []).append(event)
    
    return separated_event_list


def remove_duplicate_events(events: List[Event]) -> List[Event]:
    # Identifies possible duplicate Events within Event List.
    seen_event_urls = set()
    checked = []

    for event in events:
        if event.url in seen_event_urls:
            logger.warning(f"Found duplicate event: {event}")
        else:
            seen_event_urls.add(event.url)
            checked.append(event)

    return checked
            

if __name__ == "__main__":
    main()
