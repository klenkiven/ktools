use clap::{Subcommand, ValueEnum};

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(author = "KlenKiven <wzl709@outlook.com>")]
    #[command(version = "0.1.0")]
    #[command(about = "Dynamic DNS, use Cloudfare client API v4")]
    #[command(long_about = "Dynamic DNS, use Cloudfare client API v4\n\
                        Only you use Cloudfare for name resolving, that tool is valid")]
    CfDdns {
        /// Cloudfare API Key, eg. afejhwuoi2hj-ehufiawhfuiewfdHUIHIhuifa
        #[arg(value_name="CFTOKEN")]
        token: String,

        /// Zone name, eg. example.org
        #[arg(value_name="ZONE_NAME")]
        zone_name: String,

        /// Record Name, eg. home.example.org
        #[arg(value_name="RECORD_NAME")]
        record_name: String,

        /// Record Type, default type is A type
        #[arg(short, long, value_enum, value_name="RECORD_TYPE", default_value_t = RecordType::A)]
        #[arg(help = "Record Type, default type is A type\n\
                        - a Type: IPv4 Address\n\
                        - aaaa Type: IPv6 Address\n")]
        record_type: RecordType,

        /// Cloudflare TTL for record, between 120 and 86400 seconds
        #[arg(short, long, value_name="TTL", default_value_t = 3600)]
        ttl: i32,

        /// Cloudfare Proxy
        #[arg(short, long, value_name="PROXIED", default_value_t = false)]
        proxied: bool, 

        /// Dose update to Cloudfare directly
        #[arg(short, long, default_value_t = false)]
        force: bool
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum RecordType {
    /// IPv4 Address Record
    A,

    /// IPv6 Address Record
    AAAA
}