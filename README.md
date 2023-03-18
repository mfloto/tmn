# Ticketmaster Notifier
Get notified about resale tickets on ticketmaster.de

## TODO
- [X] make url variable for different TM_regions
- [ ] better error handling
- [ ] rewrite discord notification

## How to use
Befor running the binary, set the following environment variables:

| var             | description                                                                               |
|-----------------|-------------------------------------------------------------------------------------------|
| EVENT_ID        | Ticketmaster id of the event. Format depends on your Ticketmaster region.                 |
| COUNTRY_CODE    | Country code of your Ticketmaster region (e.g. DE for Germany or NL for the Netherlands). |
| DISCORD_WEBHOOK | Webhook url for the channel you want use for ticket-notifications.                        |
| THRESHOLD_PRICE | Maximum price for a ticket. Tickets above that price won't trigger a notification.        |
