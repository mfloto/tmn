# Ticketmaster Notifier
![Docker Image Version (latest by date)](https://img.shields.io/docker/v/mfloto/tmn)
![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/mfloto/tmn)
![Docker Pulls](https://img.shields.io/docker/pulls/mfloto/tmn)
![Docker Stars](https://img.shields.io/docker/stars/mfloto/tmn)
<br>
Get notified about resale tickets on ticketmaster (EU)

## TODO
- [X] make url variable for different TM_regions
- [ ] better error handling
- [X] rewrite discord notification

## How to use
Befor running the binary, set the following environment variables:

| var             | description                                                                               |
|-----------------|-------------------------------------------------------------------------------------------|
| EVENT_ID        | Ticketmaster id of the event. Format depends on your Ticketmaster region.                 |
| COUNTRY_CODE    | Country code of your Ticketmaster region (e.g. DE for Germany or NL for the Netherlands). |
| DISCORD_WEBHOOK | Webhook url for the channel you want use for ticket-notifications.                        |
| THRESHOLD_PRICE | Maximum price for a ticket in euro. Tickets above that price won't trigger a notification.|
