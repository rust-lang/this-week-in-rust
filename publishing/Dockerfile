FROM ubuntu:20.04

# Must run from base TWIR directory... makefile takes care of this.
WORKDIR /usr/twir

RUN apt-get update
RUN apt-get install -qq -y curl python3.8 python3-pip language-pack-en

# pelican+friends
ENV LANG='en_US.UTF-8'
ENV LC_ALL='en_US.UTF-8'
RUN ln -s /usr/bin/python3.8 /usr/bin/python
COPY requirements.txt .
RUN pip3 install -r requirements.txt
RUN curl https://files.stork-search.net/releases/v1.5.0/stork-ubuntu-20-04 -o stork \
    && chmod +x stork \
    && mv ./stork /usr/bin/stork

# sass/juice
RUN apt-get install -y curl
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g sass
RUN npm install -g juice

# pelican setup
COPY content content
COPY plugins plugins
COPY themes themes
COPY pelicanconf.py pelicanconf.py

COPY publishing/*.sh ./
RUN chmod 777 *.sh

CMD ["pelican", "--delete-output-directory", "content"]