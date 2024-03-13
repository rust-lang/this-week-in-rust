from event import Event
from typing import List
from datetime import datetime


def get_test_events() -> List[Event]:
  return [Event(name="Test Event 1", location="Earth", date=datetime.now(), url="website1.com", virtual=False, organizerName="Test Organizer", maybeSpam=False), 
          Event(name="Test Event 2", location="Earth", date=datetime.now(), url="website2.com", virtual=False, organizerName="Test Organizer", maybeSpam=False),
          Event(name="Test Event 3", location="Earth", date=datetime.now(), url="website3.com", virtual=False, organizerName="Test Organizer", maybeSpam=False),
          Event(name="Test Event 4", location="Earth", date=datetime.now(), url="website4.com", virtual=False, organizerName="Test Organizer", maybeSpam=False)
          ]