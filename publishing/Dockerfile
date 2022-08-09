FROM ubuntu:18.04

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

# pelican setup
COPY content content
COPY plugins plugins
COPY themes themes
COPY pelicanconf.py pelicanconf.py

# sass/juice
RUN apt-get install -y curl
RUN curl -sL https://deb.nodesource.com/setup_14.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g sass
RUN npm install -g juice

COPY publishing/run_server.sh run_server.sh
RUN chmod 777 run_server.sh
COPY publishing/create_html_friendly_page.sh create_html_friendly_page.sh
RUN chmod 777 create_html_friendly_page.sh

CMD ["pelican", "--delete-output-directory", "content"]