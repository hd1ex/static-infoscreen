# Static Infoscreen

This project realizes an info screen using a static web page.

A client is expected to run a fullscreen browser and connect to the generated
website through a web server.

## Static site generation

The static website can be generated using the provided rust project.

### Config file

The rust program reads a config file `infoscreen.conf` which has to contain
lines specifying a display time in seconds and a ressource link. A resource link
can either be a reference to a local file which gets served or an uri a web
browser can use as resource reference and a client can access. Note that local
file names have to be unique in order for the browser caching to work and that
remote files rely on the config of other web servers to be proper for caching.

```
# Static Infoscreen config file
#
# display-time ressource-link
15 https://wttr.in
10 https://imgs.xkcd.com/comics/compiling.png
0 https://media.giphy.com/media/VbnUQpnihPSIgIXuZv/giphy.mp4 
```

### Running the site generation

If you have [Rust](https://www.rust-lang.org/) installed and cloned this repo,
you can give it a quick test run, by running
```shell
cargo run
```

The html files will be generated into a `html/` folder. Now you can give the
infoscreen a quick test run by opening `html/0.html` with a web browser. Please
note that [Chromium](https://www.chromium.org/) usually has better transitions
at website changes than [Firefox](https://www.mozilla.org/en-US/firefox).

Please also note if you want to show *local files* without a web server, you
have to symlink them into the `html/` folder. For example, if you have your
files in a `files/` folder, you can symlink it by running
```shell
ln -sf $(realpath files/) html/files/
# or
ln -sf ../files html/files/
```

## Webserver and caching

You most probably want to show your own files (posters, videos, ...) at your
infoscreen(s): For this we use a web server.

### Nginx
Install and configure [nginx](https://nginx.org/) on your server.
<details>
  <summary>Here is a sample config</summary>

```nginx
user http;
worker_processes auto;
worker_cpu_affinity auto;

events {
    multi_accept on;
    worker_connections 1024;
}

http {
    charset utf-8;
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    server_tokens off;
    log_not_found off;
    types_hash_max_size 4096;
    client_max_body_size 16M;

    # MIME
    include mime.types;
    default_type application/octet-stream;

    # logging
    access_log /var/log/nginx/access.log;
    error_log /var/log/nginx/error.log warn;

    # load configs
    include /etc/nginx/conf.d/*.conf;
    include /etc/nginx/sites-enabled/*;
}

```
</details>

Now you can proceed by cloning this repo to your web server/host system (note
that nginx requires **root** privileges):
```shell
cd /srv
git clone https://github.com/hd1ex/static-infoscreen.git
cd static-infoscreen/
```

The core of a smooth infoscreen experience is web browser caching. For this the
web server sets file expiration headers when transmitting files. Look at the
*media* section in the [nginx.conf](nginx.conf) for the configuration of this.

Now lets add some local files to serve:
```shell
wget -P files https://imgs.xkcd.com/comics/{compiling,hell,masks}.png
```

Now we can create a quick infoscreen config file for the files:
```shell
for file in $(ls files/*); do echo "10 $file"; done > infoscreen-local.conf
```

Lets update the website to show these files:
```shell
cargo run -- --config infoscreen-local.conf
```

Now we can link our nginx config and start nginx:
```shell
ln -sf $(realpath nginx.conf) /etc/nginx/sites-enabled/static-infoscreen
systemctl start nginx
# or
nginx
```

You can now access your static infoscreen by opening the address of your server
in a web browser. If you run this on your local host, this would be
[127.0.0.1](https://127.0.0.1/).
