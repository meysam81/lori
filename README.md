# ruby

*This package is named after my dog which is very dear to me 😍🐶*

## Intro

This is a simple SMTP server that receives mails and send it to SendGrid.

An improved version might include different incoming protocols and different
outgoing integrations.

Note that SendGrid already supports receiving messages from SMTP, but I didn't
find it before writing this app.

The format for [sending an email through SMTP to SendGrid][sendgrid-smtp] is as below:

```plaintext
smtps://apikey:SENDGRID_API_KEY@smtp.sendgrid.net:465/
```

## How to install it?

### Using cargo

```bash
cargo install ruby
```

### Download compiled binary

You can download the latest release from the [releases page][release] for your
platform.

### Docker

```bash
docker run -p 2525:2525 -e SENDGRID_API_KEY=your-api-key meysam81/ruby
```

## How to use it?

```bash
export SENDGRID_API_KEY=your-api-key
ruby  # listening on localhost:2525
```

[sendgrid-smtp]: https://docs.sendgrid.com/for-developers/sending-email/getting-started-smtp
[release]: https://github.com/meysam81/ruby/releases/latest
