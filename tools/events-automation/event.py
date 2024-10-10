import datetime
from geopy.geocoders import Nominatim
from state_territory_to_abbrev import au_state_territory_to_abbrev, us_state_to_abbrev, ca_state_territory_to_abbrev

class Event():
  def __init__(self, name, location, date, url, virtual, organizerName, organizerUrl, duplicate=False) -> None:
    self.name = name
    self.location = location
    self.date = date
    self.url = url
    self.virtual = virtual
    self.organizerName = organizerName
    self.organizerUrl = organizerUrl
    self.duplicate = duplicate

  def to_markdown_string(self) -> str:
    if self.virtual:
      return f'* {self.date} | Virtual ({self.location}) | [{self.organizerName}]({self.organizerUrl})\n\t*[**{self.name}**]({self.url})'
    else:
      return f'* {self.date} | {self.location} | [{self.organizerName}]({self.organizerUrl})\n\t*[**{self.name}**]({self.url})'
    
  def format_date(self):
      # Formats datetime data into date.
      if isinstance(self.date, datetime.datetime):
        self.date = self.date.date()

  def format_location(self):
    # Formats location data into (city, +/-state, country).
    geocoder = Nominatim(user_agent="TWiR", timeout=5)
    locationData = geocoder.geocode(self.location, language="en", addressdetails=True).raw["address"]

    country_code, city = locationData["country_code"].upper(), locationData.get("city", locationData.get("town", locationData.get("village", "**NO CITY DATA**")))
    if country_code in ["AU", "CA", "US"]:
      state = locationData.get("state", locationData.get("territory", "**NO STATE DATA**"))
      if state == "**NO STATE DATA**":
        state_abbrev = state
      elif country_code == "AU":
        state_abbrev = au_state_territory_to_abbrev(state)
      elif country_code == "CA":
        state_abbrev = ca_state_territory_to_abbrev(state)
      elif country_code == "US":
        state_abbrev = us_state_to_abbrev(state)
      self.location = f'{city}, {state_abbrev}, {country_code}'
    else:
      self.location = f'{city}, {country_code}'


