🚧 Because of changes to the Ticketmaster API requiring a `reese84` token, this project does not work at the moment! [Issue](https://github.com/mfloto/tmn/issues/5) 🚧
# Ticketmaster Notifier
![Docker Image Version (latest by date)](https://img.shields.io/docker/v/mfloto/tmn)
![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/mfloto/tmn)
![Docker Pulls](https://img.shields.io/docker/pulls/mfloto/tmn)
![Docker Stars](https://img.shields.io/docker/stars/mfloto/tmn)
<br>
Get notified about resale tickets on ticketmaster (EU)

![](https://github.com/mfloto/tmn/blob/main/assets/tmn_mt_sample.png?raw=true)

## How to use
To get the project up and running quickly, use the `docker-compose.yaml` and edit the `.env`, then run `docker compose up -d`.

| var             | description                                                                               |
|-----------------|-------------------------------------------------------------------------------------------|
| EVENT_ID        | Ticketmaster id of the event. Format depends on your Ticketmaster region.                 |
| COUNTRY_CODE    | Country code of your Ticketmaster region (e.g. DE for Germany or NL for the Netherlands). |
| DISCORD_WEBHOOK | Webhook url for the channel you want use for ticket-notifications.                        |
| THRESHOLD_PRICE | Maximum price for a ticket in euro. Tickets above that price won't trigger a notification.|

## How to build
To build the binary for the image you need to have rust and the `x86_64-unknown-linux-musl` toolchain.

```bash
cargo build --release --target x86_64-unknown-linux-musl
```

```bash
docker build -t tmn . --no-cache
```

## TODO
- [ ] `listingId` is now a string
- [X] make url variable for different TM_regions
- [X] better error handling
- [X] rewrite discord notification
