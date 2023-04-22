# KTools

> a toolkit collection that help you to build your own server.

## Tools

1. Cloudfare DDNS Tool [cf-ddns]
   Following is the tool's usage:
   ```bash
   # Usage as following
   # KEY is get from cloudfare
   # ZONE is your bought domain
   # RECORD is your record wanted Fully qualified domain name (FQND)
   ktools cf-ddns <TOKEN> <ZONE> <RECORD>
   ```
   and there is a lot of optional argumengts:
   ```
   Dynamic DNS, use Cloudfare client API v4

   Usage: ktools cf-ddns [OPTIONS] <CFTOKEN> <ZONE_NAME> <RECORD_NAME>

   Arguments:
   <CFTOKEN>      Cloudfare API Key, eg. afejhwuoi2hj-ehufiawhfuiewfdHUIHIhuifa
   <ZONE_NAME>    Zone name, eg. example.org
   <RECORD_NAME>  Record Name, eg. home.example.org

   Options:
   -r, --record-type <RECORD_TYPE>  Record Type, default type is A type
                                    - a Type: IPv4 Address
                                    - aaaa Type: IPv6 Address
                                       [default: a] [possible values: a, aaaa]
   -t, --ttl <TTL>                  Cloudflare TTL for record, between 120 and 86400 seconds [default: 3600]
   -p, --proxied                    Cloudfare Proxy
   -f, --force                      Dose update to Cloudfare directly
   -h, --help                       Print help (see more with '--help')
   -V, --version                    Print version
   ```