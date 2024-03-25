
class Event():
  def __init__(self, name, location, date, url, virtual, organizerName, maybeSpam, popularity=None, recurring=None, inPast=None, duplicate=False) -> None:
    self.name = name
    self.location = location
    self.date = date
    self.url = url
    self.virtual = virtual
    self.organizerName = organizerName
    self.popularity = popularity
    self.recurring = recurring
    self.inPast = inPast
    self.duplicate = duplicate

  def to_markdown_string(self) -> str:
    if self.virtual:
      return f'* {self.date} | Virtual ({self.location}) | [{self.organizerName}](TODO: ORGANISER URL HERE)\n\t*[**{self.name}**]({self.url})'
    else:
      return f'* {self.date} | {self.location} | [{self.organizerName}](TODO: ORGANISER URL HERE)\n\t*[**{self.name}**]({self.url})'
    
