from dataclasses import dataclass
from datetime import datetime
from geopy.geocoders import Nominatim


@dataclass
class Location:
  city: None | str
  state: None | str
  country: None | str

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
      s += self.city.lower().capitalize()
    if self.state:
      s += ', '
      s += self.state.upper()
    if self.country:
      s += ', '
      s += self.country.upper()

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

  def to_event(self, geolocator: Nominatim, group_url: str) -> Event:
    # address = (geolocator.reverse(str(self.lat) +","+ str(self.long))).raw["address"]
    # location = self._format_location(address)

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

  # @staticmethod
  # def _format_location(address: dict) -> str:
  #   # TODO: look this over...
  #   if not address:
  #     return "No location"

  #   # All components for a location
  #   components = ['road', 'city', 'state', 'postcode', 'country']

  #   # Get available components, otherwise replace missing component with an empty string
  #   location = [address.get(component, "") for component in components]

  #   return ','.join(location) if location else "No location"

  #   # ????????????
  #   country_code, city = locationData["country_code"].upper(), locationData.get("city", locationData.get("town", locationData.get("village", "**NO CITY DATA**")))
  #   if country_code in ["AU", "CA", "US"]:
  #     state = locationData.get("state", locationData.get("territory", "**NO STATE DATA**"))
  #     if state == "**NO STATE DATA**":
  #       state_abbrev = state
  #     elif country_code == "AU":
  #       state_abbrev = au_state_territory_to_abbrev(state)
  #     elif country_code == "CA":
  #       state_abbrev = ca_state_territory_to_abbrev(state)
  #     elif country_code == "US":
  #       state_abbrev = us_state_to_abbrev(state)
  #     self.location = f'{city}, {state_abbrev}, {country_code}'
  #   else:
  #     self.location = f'{city}, {country_code}'

