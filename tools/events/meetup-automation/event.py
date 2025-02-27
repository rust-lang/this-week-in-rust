import logging
import string
from dataclasses import dataclass
from datetime import datetime


logger = logging.getLogger(__name__)


@dataclass
class Location:
  city: None | str
  state: None | str
  country: None | str

  def __post_init__(self):
    """ Normalize our location strings here """
    if self.city:
      # capwords does the "heavy lifting" of our formatting here https://docs.python.org/3/library/string.html#string.capwords
      self.city = string.capwords(self.city)
    else:
      self.city = None

    if self.state:
      self.state = self.state.strip()
      self.state = self.state.upper()
    else:
      self.state = None

    if self.country:
      self.country = self.country.strip()
      self.country = self.country.upper()
    else:
      self.country = None

    if self.country == "GB":
      # looks like in GB meetup considers part of the post code as the "state", which is not a common way to write
      # locations in GB (or that's my understanding at least)
      self.state = None


  def fields_present(self) -> int:
    """ Check how many fields are present, used to determine which Location has more information when comparing """
    c = 0

    if self.city:
      c += 1
    if self.state:
      c += 1
    if self.country:
      c += 1

    return c

  def to_str(self) -> str:
    s = ''

    if self.city:
      s += self.city
    if self.state:
      s += ', '
      s += self.state
    if self.country:
      s += ', '
      s += self.country

    return s


@dataclass
class Event:
  name: str
  location: Location
  date: datetime
  url: str
  virtual: bool
  organizer_name: str
  organizer_url: str

  def __post_init__(self):
    """ Normalize the event data here """
    self.name = self.name.strip()
    self.organizer_name = self.organizer_name.strip()

  def to_dict(self) -> dict:
    """ Method for serializing to a dict which can be further serialized to json """
    return {
      "name": self.name,
      "location": self.location.to_str(),
      "date": self.date.strftime("%Y-%m-%d"),
      "url": self.url,
      "virtual": self.virtual,
      "organizer_name": self.organizer_name,
      "organizer_url": self.organizer_url
    }

  def to_markdown_string(self) -> str:
    location = f"Virtual ({self.location.to_str()})" if self.virtual else self.location.to_str()

    return f'* {self.date.date()} | {location} | [{self.organizer_name}]({self.organizer_url})\n    * [**{self.name}**]({self.url})'


@dataclass
class RawGqlEvent:
  """
  Dataclass for our GQL responses for upcomingEvents. Maps very closely to the raw API response with minimal field parsing
  """
  title: str
  group_name: str
  group_location: Location
  date_time_str: str
  event_url_str: str
  venue_type: None | str
  event_location: Location
  lat: float
  long: float

  def __init__(self, **kwargs) -> None:
    logger.debug(f"Constructing RawGqlEvent from: {kwargs}")
    # TODO: add some validation here, these error messages will be not useful currently
    node = kwargs["node"]
    self.title = node["title"]

    group = node["group"]
    self.group_name = group["name"]
    self.group_location = Location(group["city"], group["state"], group["country"])
    
    self.date_time_str = node["dateTime"]
    self.event_url_str = node["eventUrl"]

    venue = node["venue"]
    self.venue_type = venue["venueType"]
    # TODO: do we need these lat longs?
    self.lat = venue["lat"]
    self.long = venue["lng"]
    self.event_location = Location(venue["city"], venue["state"], venue["country"])

  def to_event(self, group_url: str) -> Event:
    is_virtual = self.venue_type == "online"

    # this is a bit weird because we want a naive datetime object that just contains the year/month/day because we get
    # timestamps with tz info like "2025-01-16T19:00+01:00", just strip the time and tz info before parsing
    date = datetime.strptime(self.date_time_str.split('T')[0], '%Y-%m-%d')

    # prefer the event specific location, otherwise fall back to the group's location
    if self.event_location.fields_present() > self.group_location.fields_present():
      location = self.event_location
    else:
      location = self.group_location

    return Event(
      name=self.title,
      location=location,
      date=date,
      url=self.event_url_str,
      virtual=is_virtual,
      organizer_name=self.group_name,
      organizer_url=group_url
    )

