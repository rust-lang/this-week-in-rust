
class Event():
  def __init__(self, name, location, date, url, virtual, organizerName, maybeSpam, popularity=None, recurring=None, inPast=None) -> None:
    self.name = name
    self.location = location
    self.date = date
    self.url = url
    self.virtual = virtual
    self.organizerName = organizerName
    self.popularity = popularity
    self.recurring = recurring
    self.inPast = inPast
    self.maybeSpam = maybeSpam
