FROM python:3.13.7-slim

# Must run from base TWIR directory... makefile takes care of this.
WORKDIR /usr/twir

RUN apt-get update && apt-get install -y curl build-essential

# pelican+friends
ENV LANG='en_US.UTF-8'
ENV LC_ALL='en_US.UTF-8'
COPY requirements.txt .
RUN pip3 install -r requirements.txt
RUN curl https://files.stork-search.net/releases/v1.5.0/stork-ubuntu-20-04 -o stork \
    && chmod +x stork \
    && mv ./stork /usr/bin/stork

# sass/juice
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - 
RUN apt-get install -y nodejs
RUN npm install -g sass juice

# pelican setup
COPY content content
COPY plugins plugins
COPY themes themes
COPY pelicanconf.py pelicanconf.py

COPY publishing/*.sh ./
RUN chmod 777 *.sh

CMD ["pelican", "--delete-output-directory", "content"]
