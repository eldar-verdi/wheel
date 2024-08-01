FROM debian:bookworm

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    # Remove imagemagick due to https://security-tracker.debian.org/tracker/CVE-2019-10131
    && apt-get purge -y imagemagick imagemagick-6-common
RUN apt-get install git -y
RUN apt-get install nodejs npm -y 
RUN apt-get install gnupg2 pinentry-tty -y
RUN npm install -g pnpm