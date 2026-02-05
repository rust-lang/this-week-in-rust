FROM python:3.8.16-slim

# Must run from base `twir` directory... justfile takes care of this.
WORKDIR /usr/twir

# Install deps and set locales
RUN apt-get update && apt-get install -y curl build-essential locales \
    && sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen \
    && locale-gen en_US.UTF-8 \
    && update-locale LC_ALL=en_US.UTF-8 LANG=en_US.UTF-8 LANGUAGE=en_US.UTF-8

# Set explicit locale environment variables
ENV LC_ALL=en_US.UTF-8
ENV LANG=en_US.UTF-8
ENV LANGUAGE=en_US.UTF-8

# PELICAN+FRIENDS
# install python reqs
COPY requirements.txt .
RUN pip3 install -r requirements.txt

# install sass/juice
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash -
RUN apt-get install -y nodejs
RUN npm install -g sass juice

RUN curl -L -o pagefind.tar.gz https://github.com/Pagefind/pagefind/releases/download/v1.4.0/pagefind_extended-v1.4.0-aarch64-unknown-linux-musl.tar.gz
RUN tar -xvf pagefind.tar.gz

# pelican setup
COPY content content
COPY plugins plugins
COPY themes themes
COPY pelicanconf.py pelicanconf.py

# just website && just email needs these scripts
COPY publishing/*.sh ./
RUN chmod +x *.sh

CMD ["pelican", "--delete-output-directory", "content"]
