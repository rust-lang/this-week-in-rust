import requests
import datetime
import logging

from jwt_auth import generate_signed_jwt
from event import Event, RawGqlEvent
from utils import MeetupGroupUrl
from typing import List

logger = logging.getLogger(__name__)

_EVENT_LISTING_QUERY = """\
query($first: Int = 20, $urlName: String!) {
  groupByUrlname(urlname: $urlName) {
    events(first: $first, sort: ASC) {
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
          venues {
            lat
            lon
            city
            state
            country
            venueType
          }
        }
      }
    }
  }
}
"""

class TwirMeetupClient:
    AUTH_ENDPOINT = "https://secure.meetup.com/oauth2/access"
    GQL_ENDPOINT = "https://api.meetup.com/gql-ext"

    def __init__(self) -> None:
        self._access_token = None
        self._refresh_token = None

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
        return { "query": _EVENT_LISTING_QUERY, "variables": { "urlName": group_url_name } }

    def _parse_event_listing_gql_response(self, response: dict) -> List[RawGqlEvent]:
        edges = response["groupByUrlname"]["events"]["edges"]

        events = []
        # TODO: maybe move this validation somewhere else?
        for edge_kwargs in edges:
            if not edge_kwargs["node"]["venues"]:
                logger.error(f"Event response missing venues: {edge_kwargs}")
                continue

            events.append(RawGqlEvent(**edge_kwargs))

        return events

    def get_raw_events_for_group(self, group: MeetupGroupUrl) -> List[RawGqlEvent]:
        headers = {
            "Authorization": f"Bearer {self._get_access_token()}",
            "Content-Type": "application/json",
        }

        logger.info(f"Fetching events for group {group}")
        query = self._build_event_listing_gql_query(group.url_name)
        response = requests.post(url=self.GQL_ENDPOINT, headers=headers, json=query)
        response.raise_for_status()
        data = response.json()["data"]
        logger.debug(data)

        if data["groupByUrlname"] is None:
            logger.error(f"Group {group} not valid, skipping")
            return []

        return self._parse_event_listing_gql_response(data)
