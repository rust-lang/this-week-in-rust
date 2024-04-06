### Event Class:
**Required Variables**
- name (string): Title of the event.
- location (string): Location of the event either in full detail (e.g. `"111 test st, city, country, postcode"`) to be formatted by [geopy](https://geopy.readthedocs.io/en/stable/#) module and `format_location()` in event sink, or in `"city, state/territory, country"` format with state/territory details included if the location is in Australia, Canada or United States. See code sample included below for an example of location formatting.
    - Note: If location string is in `"city, state, country"` format, for both state/territory and country ISO alpha-2 codes must be used (e.g. AU for Australia, CA for California).
- date (date or datetime): Date of event in the locations local time (NOT the local time of where the program is being run).
- url (string): Unique URL for event page details.
- virtual (boolean): If event is online.
- organizerName (string): Name of event organiser.
- organizerUrl (string): URL for event organiser webpage.

**Additional Variable(s) for Internal Use:**
- duplicate (boolean): Flag for potential event duplicate based on evaluation during event sink. 

**Code Sample for Creating an Event:**
```py
Event(name="Test Event", location="Melbourne, VIC, AU", date=date.today(), url="website3.com", virtual=True, organizerName="Test Organizer", organizerUrl="testorg.com")
```

### Event Sink:
**Within Scope**:
- The event sink will take a list of event objects (see `test_events.py` for example data), format the date and location data (if not done already), filter out events that are outside of the pre-determined 'date window' for the current TWiR issue then sort the events by date and then location alphabetically. After this process the list is then split via virtual or continent, and any potential duplicate events within the list are flagged (through comparison of event data). Finally, the event sink will output the details of the finalised list of events in a pre-determined markdown format, complete with virtual/continent headers.
- Note that potential duplicate events will be flagged with a `** NOTE POTENTIAL DUPLICATE: **` warning immediately preceding the event information.

**Out of Scope**:
- The purpose of the event sink is to cross-reference and curate data from various sources. It shouldn't be responsible for gathering or adding required fields of data into the Event class. Any edge cases should be managed by the event sources.

### How to Add a New Event Source Module:
- To write a new event source module, it must be written in python. Event sources should be a function that passes no parameters and returns a list of events with required variables detailed above. If the event source has additional requirements, that should be added to `requirements.txt`. The event source should detail any specific run instructions in it's own documentation. Look at `test_events.py` for bare-minimum event list output.
- To run a new event source, import the new module from new event source and add the function call to `event_list`.

### Requirements to run this code:
- Event sink requires Python installation.
- For specific module requirements: `pip install -r requirements.txt`

### How to use this code:
- In order to use the Event Sink code please ensure that all requirements listed above have been met/installed on your device.
- Check that all Event Source module function calls are included in `event_list` (function calls should concatenate into a single list of event objects).

### Architecture of the Event Sink:
- `main()`: Gathers a list of events from imported Event Source modules, formats the event date and location data, identifies a start date for the date-window filter, creates a 2D sorted list of events from `sort_and_filter_events()`, outputs to screen Event details in markdown format with section headers and duplicate warnings.
- `format_data(event_list)`: Formats event date and location data into pre-determined format (as described above) using Event Class functions `format_date()` and `format_location()`.
    - `format_date()`: Identifies if date variable is a datetime object and if so, converts to date object.
    - `format_location()`: Identifies if location data is already formatted. If not, parses location data through geopy module, extracts the required information, then formats the data.
- `sort_and_filter_events(event_list, start_date)`: Removes all events that are outside of the determined date window (start_date + 5 weeks), sorts the list of events first by date then by location, identifies potential duplicate events to be manually checked on completion, separates events by virtual/continent into a 2D list that is returned to `main()`. 
- `sort_events(event_list)`: Uses an insertion sort algorithm to sort the list of events by date, then by location.
- `potential_duplicate(event_list)`: Flags potential duplicate events by comparing date, url, name, and organizerName variables (in that order).
- `sort_virtual_continent(event_list)`: Separates the event list into a 2D list by virtual/continent. See function code comments for index key.

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
