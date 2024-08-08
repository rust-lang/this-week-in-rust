# Event Sink
The maintainer of the Events section is faced with manual work; this process automates generation of a draft events list using various Event Source Modules that gather event information, and an Event Sink that sorts, groups and formats the events into a pre-determined markdown format.

## Getting Started:
### Pre-Requisites:
- Event sink requires Python3 installation.
- For specific module requirements: `pip install -r requirements.txt`
- See https://geopy.readthedocs.io/en/stable/# for `geopy` module documentation.

### Running:
Before running please check that all Event Source module function calls are included in `event_list` (function calls should concatenate into a single list of event objects).

To run this code:

```py
pip install -r requirements.txt
python3 main.py
```

### How to Add a New Event Source Module:
- To write a new event source module, it must be written in python. Event sources should be a function that passes no parameters and returns a list of events with required variables detailed above. If the event source has additional requirements, that should be added to `requirements.txt`. The event source should detail any specific run instructions in it's own documentation. Look at `test_events.py` for bare-minimum event list output.
- To run a new event source, import the new module from new event source and add the function call to `event_list`.

## Architecture:
### Event Class:
**Required Variables**
- `name` (string): Title of the event.
- `location` (string): Location of the event either in full detail (e.g. `"111 test st, city, country, postcode"`) to be formatted by `geopy` module and `format_location()` in event sink, or in `"city, state/territory, country"` format with state/territory details included if the location is in Australia, Canada or United States. See code sample included below for an example of location formatting.
    - Note: If location string is in `"city, state, country"` format, for both state/territory and country ISO alpha-2 codes must be used (e.g. AU for Australia, CA for California).
- `date` (date or datetime): Date of event in the locations local time (NOT the local time of where the program is being run).
- `url` (string): Unique URL for event page details.
- `virtual` (boolean): If event is online.
- `organizerName` (string): Name of event organiser.
- `organizerUrl` (string): URL for event organiser webpage.

**Additional Variable(s) for Internal Use:**
- `duplicate` (boolean): Flag for potential event duplicate based on evaluation during event sink. 

**Code Sample for Creating an Event:**
```py
Event(name="Test Event", location="Melbourne, VIC, AU", date=date.today(), url="website3.com", virtual=True, organizerName="Test Organizer", organizerUrl="testorg.com")
```

### Within Scope of Event Sink:
- The event sink will take a list of event objects (see `test_events.py` for example data), format the date and location data (if not done already), filter out events that are outside of the pre-determined 'date window' for the current TWiR issue then sort the events by date and then location alphabetically. After this process the list is then split via virtual or continent, and any potential duplicate events within the list are flagged (through comparison of event data). Finally, the event sink will output the details of the finalised list of events in a pre-determined markdown format, complete with virtual/continent headers.
- Note that potential duplicate events will be flagged with a `** NOTE POTENTIAL DUPLICATE: **` warning immediately preceding the event information.


### Out of Scope:
- The purpose of the event sink is to cross-reference and curate data from various sources. It shouldn't be responsible for gathering or adding required fields of data into the Event class. Any edge cases should be managed by the event sources.

### Expected Output:
Example Output from `test_events.py` data:
```
### Virtual:

* 2024-04-12 | Virtual (Dublin, IE) | [Test Organizer](testorg.com)
        *[**Test Event 1**](website1.com)

### North America:

* 2024-04-03 | New York, NY, US | [Test Organizer](testorg.com)
        *[**Test Event 7**](website7.com)
* 2024-04-04 | San Francisco, CA, US | [Test Organizer](testorg.com)
        *[**Test Event 6**](website6.com)
* 2024-04-18 | Indianapolis, IN, US | [Test Organizer](testorg.com)
        *[**Test Event 2**](website2.com)

### Oceania:

* 2024-04-04 | Sydney, NSW, AU | [Test Organizer](testorg.com)
        *[**Test Event 4**](website4.com)
```
