# Komodo E-Mail Alerter

A simple http server that forwards all alerts to a smtp server.

## Installation

Using the following docker compose configuration:

```yaml
services:
  alerter:
    image: git.bs.informatik.uni-siegen.de/breitweiser/komodo-email-alerter:latest
    container_name: komodo-email-alerter
    environment:
      SMTP_HOST: my.mail.server
      SMTP_PORT: 587
      SMTP_USER: user
      SMTP_PASS: pass
      SMTP_REPLY_TO: noreply@example.com
      SMTP_SENDER: "Sender <sender@example.com>"
      RECEIVER: "Receiver <receiver@example.com>"
    ports:
      - "80:80" # customize your host port here
```

Edit the environment variables according to your needs.
In Komodo, configure a new alerter of type "Custom" with the url `http://<ip-or-hostname-of-alerter>:<port>/`.

## Configurable Templates

Default templates for the E-Mail subject and body reside in the `/templates` folder inside the container. To customize
the templates, mount a volume into `/templates` and edit the two template files (`body.html` and `subject.txt`)
according to your own needs.

Templates are rendered using the [`minijinja`](https://docs.rs/minijinja/latest/minijinja/) crate and largely follow [
`jinja2`](https://jinja.palletsprojects.com/) syntax.
The context given for both the subject and body templates is a serialized version of the komodo [
`Alert`](https://docs.rs/komodo_client/latest/komodo_client/entities/alert/struct.Alert.html) type. Example alerts can
be found in the `fixtures/` folder.

The subject template is cleaned up after rendering by replacing all occurrences of whitespace (including newlines) with
a single space character, and whitespace at the beginning and end are trimmed.
The body template is cleaned up after rendering using the [`tidier`](https://docs.rs/tidier/latest/tidier/) crate.