## Guide to Generating Events Using Meetup APIs

### Introduction
This guide provides step-by-step instructions on how to use Meetup APIs to fetch event data related to Rust programming groups. It outlines the installation of necessary packages, setting up environment variables, and executing the script to obtain event data.

### Prerequisites
Before you start, ensure you have the following:
- Python installed on your system (Python 3.8 or later recommended).
- `pip` for managing Python packages.
- Access to terminal or command line.

### Tips for Managing Python Packages

- **Virtual Environment**: Create a virtual environment using `venv` (built into Python 3) or `virtualenv`. Here's how to activate a virtual environment:
  ```bash
  # Creating a virtual environment (for Linux/macOS)
  python3 -m venv myenv
  # Activating the virtual environment (for Linux/macOS)
  source myenv/bin/activate

  or

  # Creating a virtual environment (for Windows)
  python -m venv myenv
  # Activating the virtual environment (for Windows)
  myenv\Scripts\activate
  ```
- **Upgrade `pip`**: Ensure your `pip` is up-to-date to avoid installation issues with newer packages:
  ```bash
  pip install --upgrade pip
  ```

### Installation
1. **Open Terminal or Command Prompt**: 
   Navigate to the directory `.../tools/events-automation` where `requirements.txt` file is located.

2. **Run the Installation Command**: 
   Execute the following command to install all the packages listed in `requirements.txt`:
   ```bash
   pip install -r requirements.txt
   ```

3. **Set Up Environment Variables**:
   - Set the following environment variables with your actual values:
     ```
     MEETUP_AUTHORIZED_MEMBER_ID=<meetup authorized member id>
     MEETUP_CLIENT_KEY=<meetup client key value>
     MEETUP_PRIVATE_KEY=<path to RSA pem file>
     ```
   These values are used for authentication with the Meetup API and to generate JWT tokens securely.

### Running the Script
To fetch events, run the following command from the project directory `.../tools/events-automation`:
```bash
python3 main.py -g <path to meetup group JSON array file>
```
This script performs the following operations:
- Authenticates with the Meetup API using JWT.
- Filters and formats the event data into a standardized structure.
- Outputs the details of upcoming events.

### Example Output Format
The script outputs event details in the following format:
```
* 2025-05-08T19:00+02:00 | Virtual (,,Tuvalu) | [rust-noris](TODO: ORGANISER URL HERE)
    *[**Rust Nürnberg online**](https://www.meetup.com/rust-noris/events/gmkpctyhchblb)
```
This format includes the date and time of the event, the location (virtual in this case), and links to both the organizer's profile and the event itself.

### Challenges and Considerations
- **Data Accuracy**: The script uses "Rust" as a keyword for searching groups and events. This can sometimes pull in events that merely mention Rust but aren't directly related to Rust programming in the event descriptions.
- **Event Data Completeness**: Not all events have complete venue information, especially for virtual events. The script currently ignores handling missing data, but the ideal is flagging such events for manual review or excluding them from the final output.
- **API Limitations**: The Meetup API has rate limits and other constraints that may affect how frequently you can fetch data.
