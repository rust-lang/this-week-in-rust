# Mapping of known cities to their state codes
CITY_STATE_MAP = {
    # New South Wales
    "Sydney": "NSW",
    "Newcastle": "NSW",
    "Wollongong": "NSW",
    "Central Coast": "NSW",

    # Victoria
    "Melbourne": "VIC",
    "Geelong": "VIC",
    "Ballarat": "VIC",
    "Bendigo": "VIC",

    # Queensland
    "Brisbane": "QLD",
    "Gold Coast": "QLD",
    "Sunshine Coast": "QLD",
    "Townsville": "QLD",
    "Cairns": "QLD",

    # Western Australia
    "Perth": "WA",
    "Fremantle": "WA",
    "Bunbury": "WA",

    # South Australia
    "Adelaide": "SA",
    "Mount Gambier": "SA",

    # Tasmania
    "Hobart": "TAS",
    "Launceston": "TAS",

    # Northern Territory
    "Darwin": "NT",
    "Alice Springs": "NT",

    # Australian Capital Territory
    "Canberra": "ACT",
}

def update_state_from_city(events, city_state_map=CITY_STATE_MAP):
    """
    Iterate over all events and set event.location.state
    based on event.location.city using the provided mapping.
    """
    for key, value in events.items():
        for event in value:
            city = event.location.city
            state = city_state_map.get(city)
            if state:
                event.location.state = state
            else:
                print(f"No state mapping found for city: {city}")

    return events