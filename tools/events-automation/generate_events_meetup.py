import requests
import datetime
import concurrent.futures
import csv

from jwt_auth import generate_signed_jwt
from urllib.parse import urlsplit
from geopy.geocoders import Nominatim
from event import Event

def authenticate():
    """
    Handles the OAuth 2.0 authentication process.
    Returns obtaining access and refresh tokens from the Meetup API
    """
    # API Configuration:
    URL = "https://secure.meetup.com/oauth2/access"
    headers = {
        "Content-Type": "application/x-www-form-urlencoded"
    }
    body = {
        "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
        "assertion": generate_signed_jwt()
    }

    # Make a request for access and refresh tokens
    response = requests.post(url=URL, headers=headers, data=body)
    if response.status_code == 200:
        access_token = response.json().get("access_token")
        refresh_token = response.json().get("refresh_token")
        return access_token, refresh_token
    else:
        print("Failed to obtain access token")
        print("Status Code:", response.status_code)
        print("Response:", response.text)
        return None, None

# Initialize variables for querying and formatting data:
#ACCESS_TOKEN, REFRESH_TOKEN = authenticate()
# initialize Nominatim API 
GEOLOCATOR = Nominatim(user_agent="TWiR")

def fetch_groups(endCursor=""):
    """
    Returns the response from the API call, which includes data on groups matching the criteria specified in the GraphQL query.
    :type endCursor: An optional string parameter used for pagination, indicating the starting point of the query for fetching subsequent pages of results
    :rtype: requests.Response
    """

    # API Configuration: 
    # Sets the API endpoint and constructs headers using an access token for authentication.
    URL = "https://api.meetup.com/gql"
    access_token, refresh_token = ACCESS_TOKEN, REFRESH_TOKEN

    if not access_token:
        print("Authentication failed, cannot proceed to fetch events.")
        return

    # Sets the content type to application/json for the request body.
    headers = {
        "Authorization": f"Bearer {access_token}",
        "Content-Type": "application/json",
    }

    # GraphQL Query:
    # Below is a GraphQL query that requests information about groups such as ID, name, link, URL name, latitude, and longitude. 
    data = {
        "query": """
        query (
            $searchGroupInput: ConnectionInput!, 
            $searchGroupFilter: SearchConnectionFilter!,
            $sortOrder: KeywordSort!
        ) {
            keywordSearch(
                input: $searchGroupInput, 
                filter: $searchGroupFilter,
                sort: $sortOrder
            ) {
                pageInfo {
                    hasNextPage
                    endCursor
                }
                edges {
                    node {
                        result {
                            ... on Group {
                                id
                                name
                                link
                                urlname
                                latitude
                                longitude
                            }
                        }
                    }
                }
            }
        }
        """,
        # The query filters results based on the keyword "Rust" and sorts them by relevance
        "variables": {
            "searchGroupFilter": {
                "query": "Rust",
                "lat": 0.0,
                "lon": 0.0,
                "radius": 20000,
                "source": "GROUPS"
            },
            "searchGroupInput": {
                "first": 200,
                "after": endCursor
            },
            "sortOrder":{
                "sortField": "RELEVANCE"
            }
        }
    }
    return requests.post(url=URL, headers=headers, json=data)

def get_rush_groups() -> dict:
    """
    Returns a dictionary where each key represents the unique ID of a group, and the corresponding value is another dictionary containing details about the group such as name, link, URL name, latitude, and longitude
    :rtype: dict
    """
    endCursor = None
    groups = dict()
    while True:
        response = fetch_groups(endCursor).json()
        data = response['data']
        edges = data['keywordSearch']['edges']
        pageInfo = data['keywordSearch']['pageInfo']
        for node in edges:
            group = node["node"]["result"]
            if not (group["id"] in groups):
                groups[group["id"]] = group
        if pageInfo['hasNextPage']:
            endCursor = pageInfo['endCursor']
        else:
            break
    return groups

def get_known_rush_groups(fileName="rust_meetup_groups.csv") -> dict:
    """
    Returns a dictionary represents all groups from a specified CSV file 
    :type fileName: Name or Path of the CSV file that contains the URLs and locations of the groups.
    """

    # Reads the CSV file, specifically extracting data from the 'url' and 'location' columns
    groups = dict() # main dictionary that stores all information of different groups
    df = pd.read_csv(fileName, header=0, usecols=['url', 'location'])

    # Extracting the url name of known Rust groups
    # Format of extracting the URL:
    # [source](https://stackoverflow.com/questions/35616434/how-can-i-get-the-base-of-a-url-in-python)
    # https://www.meetup.com/seattle-rust-user-group/
    # split_url.scheme   "http"
    # split_url.netloc   "www.meetup.com" 
    # split_url.path     "/seattle-rust-user-group/"
    for index, row in df.iterrows():
        group = {}
        group["link"] = row["url"]
        split_url = urlsplit(group["link"])
        group["urlname"] = (split_url.path).replace("/", "")
        group["location"] = row["location"]
        groups[index] = group
    return groups

def get_20_events(groups) -> list[Event]:
    """
    Returns a list where each element is an instance of the Event class, representing event data from the Meetup API 
    :type groups: A dictionary of groups where each entry contains the group's URL name to make an API request
    :rtype: dict
    """
    events = [] # main list to store data about each fetched event.

    # API Configuration:
    URL = "https://api.meetup.com/gql"
    access_token, refresh_token = ACCESS_TOKEN, REFRESH_TOKEN

    if not access_token:
        print("Authentication failed, cannot proceed to fetch events.")
        return

    headers = {
        "Authorization": f"Bearer {access_token}",
        "Content-Type": "application/json",
    }

    # Constructs and sends a GraphQL query for each group to fetch up to 20 upcoming events from the Meetup API using the group's URL name
    data = {}
    for group in groups.values():
        urlName = group["urlname"]
        data = {
            "query": """
            query ($urlName: String!, $searchEventInput: ConnectionInput!) {
                groupByUrlname(urlname: $urlName) {
                    upcomingEvents(input: $searchEventInput, sortOrder: ASC) {
                        pageInfo {
                            hasNextPage
                            endCursor
                        }
                        edges {
                            node {
                                id
                                title
                                dateTime
                                eventUrl
                                venue {
                                    venueType
                                    lat
                                    lng
                                }
                            }
                        }
                    }
                }
            }
            """,
            "variables": {
                "urlName": urlName,
                "searchEventInput": {
                    "first": 20
                }
            }
        }
        response = requests.post(url=URL, headers=headers, json=data)
        data = response.json()["data"]

        # Constructs Event with attributes such as title, location, date, URL, and organizer details
        if data:
            searchGroupByUrlname = data["groupByUrlname"]
            if searchGroupByUrlname:
                edges = searchGroupByUrlname["upcomingEvents"]["edges"]
                if edges:
                    for edge in edges:
                        node = edge["node"]
                        if node:
                            venue = node["venue"]
                            # TODO: Handle events don't have venue:
                            # 1. Flagging the events and they will have to be check manually, 
                            # 2. Putting them in separate list to check
                            # (for now ignore those events) 
                            if venue:
                                name = node["title"]
                                virtual = True
                                if venue["venueType"] != "online":
                                    virtual = False

                                # Convert obtained latitude and longitude of an event to formatted location 
                                address = (GEOLOCATOR.reverse(str(venue["lat"]) +","+ str(venue["lng"]))).raw["address"]
                                location = format_location(address)
                                date = datetime.datetime.fromisoformat(node["dateTime"]).date()
                                url = node["eventUrl"]
                                organizerName = group.get("name", urlName)
                                organizerUrl = group["link"]
                                events.append(Event(name, location, date, url, virtual, organizerName, organizerUrl))
    return events

def format_location(address) -> str:
    """
    Helper method to format address of events with required components for a location
    :rtype: string
    """
    if not address:
        return "No location"
    
    # All components for a location
    components = ['road', 'city', 'state', 'postcode', 'country']

    # Get available components, otherwise replace missing component with an empty string
    location = [address.get(component, "") for component in components]


    return ','.join(location) if location else "No location"

def get_events() -> list[Event]:
    """
    Returns a list of Event objects querying from known, and Meetup API Rust groups
    :rtype: list[Event]
    """
    # TODO: once the handling events without venue successful, get events_meetup_groups = get_20_events(get_rush_groups())
    events_known_groups = get_20_events(get_known_rush_groups())
    return events_known_groups
