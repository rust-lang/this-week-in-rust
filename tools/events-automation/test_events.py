from event import Event
from typing import List
from datetime import date, timedelta


def get_test_events() -> List[Event]:
  return [Event(name="Test Event 1", location="Earth", date=date.today() + timedelta(days=2), url="website1.com", virtual=True, organizerName="Test Organizer", maybeSpam=False), 
          Event(name="Test Event 2", location="Earth", date=date.today() - timedelta(days=2), url="website2.com", virtual=False, organizerName="Test Organizer", maybeSpam=False),
          Event(name="Test Event 3", location="Earth", date=date.today(), url="website3.com", virtual=True, organizerName="Test Organizer", maybeSpam=False),
          Event(name="Test Event 4", location="Moon", date=date.today(), url="website4.com", virtual=False, organizerName="Test Organizer", maybeSpam=False)
          ]
