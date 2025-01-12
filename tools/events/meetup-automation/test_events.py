from event import Event
from typing import List
from datetime import date, timedelta


def get_test_events() -> List[Event]:
  return [Event(name="Test Event 1", location="Dublin, IE", date=date.today() + timedelta(days=2), url="website1.com", virtual=True, organizerName="Test Organizer", organizerUrl="testorg.com"), 
          Event(name="Test Event 2", location="Indianapolis, IN, US", date=date.today() + timedelta(weeks=2), url="website2.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 3", location="Melbourne, VIC, AU", date=date.today(), url="website3.com", virtual=True, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 4", location="Sydney, NSW, AU", date=date.today(), url="website4.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 5", location="Melbourne, VIC, AU", date=date.today(), url="website5.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 6", location="San Francisco, CA, US", date=date.today(), url="website6.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 7", location="New York, NY, US", date=date.today() - timedelta(days=1), url="website7.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com"),
          Event(name="Test Event 7", location="New York, NY, US", date=date.today() - timedelta(days=1), url="website7.com", virtual=False, organizerName="Test Organizer", organizerUrl="testorg.com")
          ]
