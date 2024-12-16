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
    events = date_window_filter(events)

    # Sort remaining events by date, then location.
    events.sort(key=lambda event: (event.date, event.location))

    for event in events:
        print(event.to_markdown_string())

    # Flag potential duplicate events.
    # potential_duplicate(events)
    
    # Group by virtual or by continent.
    # events = group_virtual_continent(events)

    # Output Sorted Event List.
    # output_to_screen(events)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description='Fetches meetup events for TWIR')
    parser.add_argument("-d", "--debug", action="store_true", dest="debug", help="Enable debug logging")
    parser.add_argument("-g", "--groups", action="store", type=str, dest="groups_file", required=True, help="File with a JSON array of meetup group URLS")

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
                if event.duplicate:
                    print("** NOTE POTENTIAL DUPLICATE: **")
                print(event.to_markdown_string())
            print()


def date_window_filter(events: List[Event]) -> List[Event]:
    # Removes Events that are outside current date window.
    # Date window = closest wednesday + 5 weeks.
    start_date = date.today()
    while start_date.weekday() != 2:
        start_date = start_date + timedelta(days=1)
        
    valid = []
    for event in events:
        if not (start_date <= event.date.date() <= start_date + timedelta(weeks=5)):
            logger.debug(f"Removed event outside of date range: {event}")
        else:
            valid.append(event)

    return valid


def group_virtual_continent(event_list):
    # Return dictionary of events separated in virtual and by continent.
    separated_event_list = {}

    for event in event_list:
        # Separates Events by Virtual or by Continent
        key = "Virtual" if event.virtual else country_code_to_continent(event.location[-2:])
        separated_event_list.setdefault(key, []).append(event)
    
    return separated_event_list


def potential_duplicate(event_list):
    # Identifies possible duplicate Events within Event List.
    for i in range(len(event_list)):
        for j in range(i+1, len(event_list)):
            if event_list[i].date == event_list[j].date:
                if event_list[i].url == event_list[j].url:
                    if event_list[i].name == event_list[j].name:
                        if event_list[i].organizerName == event_list[j].organizerName:
                            if event_list[i].location == event_list[j].location:
                                event_list[i].duplicate = True


if __name__ == "__main__":
    main()
