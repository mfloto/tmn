FROM alpine:latest
LABEL Name=tmn Version=0.0.1
COPY target/x86_64-unknown-linux-musl/release/ticketmaster_notifier .
RUN echo "*/5 * * * * /ticketmaster_notifier" >> /var/spool/cron/crontabs/root
WORKDIR /data
VOLUME [ "/data" ]
CMD [ "crond", "-f" ]