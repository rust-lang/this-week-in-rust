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
    # Get Events list.
    event_list = get_test_events()

    # Format date and location data.
    format_data(event_list)

    # Get closest Wednesday for date range.
    start_date = date.today()
    while start_date.weekday() != 2:
        start_date = start_date + timedelta(days=1)

    # Sort Events within date range.
    # Sorted into virtual or by continent.
    # Ordered by date, then city.
    event_list = sort_and_filter_events(event_list, start_date)

    # Output Sorted Event List
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


def sort_and_filter_events(event_list, start_date) -> List[Event]:
    # Remove Events that are outside current date window.
    for event in event_list:
        if not (start_date <= event.date <= start_date + timedelta(weeks=5)):
            event_list.remove(event)
    
    # Sort Events by date and location.
    sort_events(event_list)

    # Flag potential duplicate events.
    potential_duplicate(event_list)

    # Return 2D list of Events sorted by Virtual/Continent.
    return sort_virtual_continent(event_list)


def sort_events(event_list):
    for i in range(1, len(event_list)):
        # Assigns current event to sort.
        current_event = event_list[i]

        # Initialise comparison index
        j = i - 1
        while j >= 0 and event_list[j].date > current_event.date:
            # Shifts all events with lower dates to the left.
            event_list[j+1] = event_list[j]
            j -= 1

        if event_list[j].date == current_event.date:
            # If current event date is equal to comparison date, compares location.
            while j >= 0 and current_event.location[0:3] < event_list[j].location[0:3]:
                # Shifts events with a location alphabetically higher to the left.
                event_list[j+1] = event_list[j]
                j -= 1

        # Places current event in correct index.
        event_list[j+1] = current_event


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
