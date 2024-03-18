# import all the event sources & event sink
# collect all the events from the event sources
# call event sink with our collected events
# print to console / output to file formatted markdown

"""
Example Markdown format:
* 2024-03-06 | Virtual (Dublin, IE) | [Rust Dublin](https://www.meetup.com/rust-dublin/)
    * [**An intro to `nom`, parsing made easy for Rustaceans**](https://www.meetup.com/rust-dublin/events/299358988/)
* 2024-03-06 | Virtual (Indianapolis, IN, US) | [Indy Rust](https://www.meetup.com/indyrs/)
    * [**Indy.rs - with Social Distancing**](https://www.meetup.com/indyrs/events/299047891/)
* 2024-03-07 | Virtual (Charlottesville, NC, US) | [Charlottesville Rust Meetup](https://www.meetup.com/charlottesville-rust-meetup/)
    * [**Crafting Interpreters in Rust Collaboratively**](https://www.meetup.com/charlottesville-rust-meetup/events/298368787/)

Sorted by:
 - Date
 - City (location)
"""

from test_events import get_test_events

def main():
    event_list = get_test_events()
    sort_and_filter_events(event_list)
    for event in event_list:
        print(event.to_markdown_string())


def sort_and_filter_events(event_list):
    # Sort Events by date and location.
    sort_events(event_list)

    # Sort Events by Virtual/In-Person.
    sort_virtual(event_list)

    # TODO Filter out Potential Spam Events.
    # TODO Filter out duplicates.


def sort_events(event_list):
    # Sorts event_list date and location.
    for i in range(len(event_list)):
        # Assigns a current event to be sorted and removes it from list.
        current_event = event_list[i]
        event_list.remove(event_list[i])
        #TODO put filter in here for dates not in current window.
        for j in range(len(event_list)):
            if current_event.date < event_list[j].date:
                # If current event date is earlier than comparison, inserts back into list.
                event_list.insert(j, current_event)
                break
            elif current_event.date == event_list[j].date:
                # If current event date is equal to comparison, compares location.
                if current_event.location[0] < event_list[j].location[0]:
                    # If current event location is alphabetically higher, inserts back into list.
                    event_list.insert(j, current_event)
                    break
        if current_event not in event_list:
            # If current event has not already been inserted, appends to the end of the list.
            event_list.append(current_event)


def sort_virtual(event_list):
    # Orders event_list into virtual, then in-person events.
    virtual_events = []

    for event in event_list:
        # Filters and removes any virtual events from event_list.
        if event.virtual:
            virtual_events.append(event)
            event_list.remove(event)
    
    for i in range(len(virtual_events)-1, -1, -1):
        # Inserts virtual events back at the top of event_list.
        event_list.insert(0, virtual_events[i])


def filter_potential_spam():
    # Filters out potential spam events.
    pass


if __name__ == "__main__":
    main()
