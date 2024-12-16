import requests
import datetime
import logging

from jwt_auth import generate_signed_jwt
from geopy.geocoders import Nominatim
from event import Event, RawGqlEvent
from utils import MeetupGroupUrl
from typing import List

logger = logging.getLogger(__name__)


class TwirMeetupClient:
    AUTH_ENDPOINT = "https://secure.meetup.com/oauth2/access"
    GQL_ENDPOINT = "https://api.meetup.com/gql"

    def __init__(self) -> None:
        self._access_token = None
        self._refresh_token = None
        self._geolocator = Nominatim(user_agent="TWiR")

    def _authenticate(self):
        """
        Handles the OAuth 2.0 authentication process.
        Sets access and refresh tokens from the Meetup API
        """
        logger.info("Fetching auth tokens...")

        headers = {
            "Content-Type": "application/x-www-form-urlencoded"
        }
        body = {
            "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
            "assertion": generate_signed_jwt()
        }

        # Make a request for access and refresh tokens
        response = requests.post(url=self.AUTH_ENDPOINT, headers=headers, data=body)
        if response.status_code == 200:
            access_token = response.json().get("access_token")
            refresh_token = response.json().get("refresh_token")
            self._access_token = access_token
            self._refresh_token = refresh_token
        else:
            response.raise_for_status()

        logger.info("Done fetching auth tokens!")

    def _get_access_token(self):
        if not self._access_token:
            self._authenticate()

        return self._access_token

    def _build_event_listing_gql_query(self, group_url_name: str) -> dict:
        return {
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
                                group {
                                    name
                                    city
                                    state
                                    country
                                }
                                title
                                dateTime
                                eventUrl
                                venue {
                                    city
                                    state
                                    country
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
                "urlName": group_url_name,
                "searchEventInput": {
                    # TODO: see if we need this limit or not
                    "first": 20
                }
            }
        }

    def _parse_event_listing_gql_response(self, response: dict) -> List[RawGqlEvent]:
        edges = response["groupByUrlname"]["upcomingEvents"]["edges"]

        events = []
        # TODO: maybe move this validation somewhere else?
        for edge_kwargs in edges:
            if not edge_kwargs["node"]["venue"]:
                logger.error(f"Event response missing venue: {edge_kwargs}")
                continue

            events.append(RawGqlEvent(**edge_kwargs))

        return events

    def fetch_groups(self, endCursor=""):
        """
        Returns the response from the API call, which includes data on groups matching the criteria specified in the GraphQL query.
        :type endCursor: An optional string parameter used for pagination, indicating the starting point of the query for fetching subsequent pages of results
        :rtype: requests.Response
        """

        # Sets the content type to application/json for the request body.
        headers = {
            "Authorization": f"Bearer {self._get_access_token()}",
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
        return requests.post(url=self.GQL_ENDPOINT, headers=headers, json=data)

    def get_rust_groups(self) -> dict:
        """
        Returns a dictionary where each key represents the unique ID of a group, and the corresponding value is another dictionary containing details about the group such as name, link, URL name, latitude, and longitude
        :rtype: dict
        """
        endCursor = None
        groups = dict()
        while True:
            response = self.fetch_groups(endCursor).json()
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

    def get_raw_events_for_group(self, group: MeetupGroupUrl) -> List[RawGqlEvent]:
        headers = {
            "Authorization": f"Bearer {self._get_access_token()}",
            "Content-Type": "application/json",
        }

        logger.info(f"Fetching events for group {group}")
        query = self._build_event_listing_gql_query(group.url_name)
        response = requests.post(url=self.GQL_ENDPOINT, headers=headers, json=query)
        data = response.json()["data"]
        logger.debug(data)

        if data["groupByUrlname"] is None:
            logger.error(f"Group {group} not valid, skipping")
            return []

        return self._parse_event_listing_gql_response(data)

