# import all the event sources & event sink
# collect all the events from the event sources
# call event sink with our collected events
# print to console / output to file formatted markdown

from typing import List
from event import Event
from test_events import get_test_events
from datetime import date, timedelta
from country_code_to_continent import country_code_to_continent

def main():
    # Get Events list from Event Sources.
    event_list = get_test_events()

    # Format date and location data.
    format_data(event_list)

    # Remove events outside of date range.
    date_window_filter(event_list)

    # Sort remaining events by date, then location.
    event_list.sort(key=lambda event: (event.date, event.location))

    # Flag potential duplicate events.
    potential_duplicate(event_list)
    
    # Sort into virtual or by continent.
    event_list = sort_virtual_continent(event_list)

    # Output Sorted Event List
    output_to_screen(event_list)


def output_to_screen(event_list):
    # Prints sorted Event List to terminal screen.
    for continent in event_list:
        if len(continent) == 0:
            continue
        else:
            country_code = continent[0].location[-2:]
            # Output Section Header
            if continent[0].virtual:
                print(f'### Virtual:\n')
            else:
                print(f'### {country_code_to_continent(country_code)}:\n')
            
            # Output event details
            for event in continent:
                if event.duplicate:
                    print("** NOTE POTENTIAL DUPLICATE: **")
                print(event.to_markdown_string())
            print()


def format_data(event_list):
    # Formats date and location data into specified format.
    for event in event_list:
        event.format_date()
        event.format_location()


def date_window_filter(event_list):
    # Removes Events that are outside current date window.
    # Date window = closest wednesday + 5 weeks.
    start_date = date.today()
    while start_date.weekday() != 2:
        start_date = start_date + timedelta(days=1)
        
    for event in event_list:
        if not (start_date <= event.date <= start_date + timedelta(weeks=5)):
            event_list.remove(event)


def sort_virtual_continent(event_list) -> List[Event]:
    # Return 2D list of events separated in virtual and by continent.
    # Index Key: [[0=Virtual], [1=Africa], [2=Asia],
    #             [3=Europe], [4=North America],
    #             [5=Oceania], [6=South America]]
    separated_event_list = [[], [], [], [], [], []]

    for event in event_list:
        # Separates Events by Virtual or by Continent
        if event.virtual:
            separated_event_list[0].append(event)
        else:
            continent = country_code_to_continent(event.location[-2:])
            if continent == "Africa":
                separated_event_list[1].append(event)
            elif continent == "Asia":
                separated_event_list[2].append(event)
            elif continent == "Europe":
                separated_event_list[3].append(event)
            elif continent == "North America":
                separated_event_list[4].append(event)
            elif continent == "Oceania":
                separated_event_list[5].append(event)
            elif continent == "South America":
                separated_event_list[6].append(event)

    return separated_event_list


def potential_duplicate(event_list):
    # Identifies possible duplicate Events within Event List.
    for i in range(len(event_list)):
        for j in range(i+1, len(event_list)):
            if event_list[i].date == event_list[j].date:
                if event_list[i].url == event_list[j].url:
                    if event_list[i].name == event_list[j].name:
                        if event_list[i].organizerName == event_list[j].organizerName:
                            event_list[i].duplicate = True


if __name__ == "__main__":
    main()
