use std::path::PathBuf;

use structopt::StructOpt;

use dftk_conference_hall::ConferenceHallConfig;
use dftk_database::MongodbConfig;
use dftk_hugo_site::SiteConfig;
use dftk_server::ServerConfig;

#[derive(Debug, Clone, StructOpt)]
pub struct CliOpt {
    #[structopt(subcommand)]
    command: Command,
}

impl CliOpt {
    pub fn command(&self) -> Command {
        self.command.clone()
    }
}

#[derive(Debug, Clone, StructOpt)]
pub enum Command {
    /// Synchronize Conference Hall data to the Database
    Synchronize {
        #[structopt(flatten)]
        conference_hall: ConferenceHallOpts,
        #[structopt(flatten)]
        mongodb: MongodbOpts,
    },
    /// Generate site data from Database
    Generate {
        #[structopt(flatten)]
        site_dir: SiteDirOpts,
        #[structopt(flatten)]
        mongodb: MongodbOpts,
    },
    /// Run the server
    Serve {
        #[structopt(flatten)]
        site_dir: SiteDirOpts,
        #[structopt(flatten)]
        conference_hall: ConferenceHallOpts,
        #[structopt(flatten)]
        mongodb: MongodbOpts,
        #[structopt(flatten)]
        server: ServerOpts,
    },
    /// Cleaning some data
    Clean {
        /// The output site directory
        site_dir: Option<PathBuf>,
    },
}

#[derive(StructOpt, Debug, Clone)]
pub struct SiteDirOpts {
    /// The output site directory
    #[structopt(short, long, env, parse(from_os_str))]
    pub(crate) site_dir: PathBuf,
}

impl From<SiteDirOpts> for SiteConfig {
    fn from(opts: SiteDirOpts) -> Self {
        let SiteDirOpts { site_dir } = opts;

        Self::new(site_dir)
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct ConferenceHallOpts {
    /// The conference hall site
    #[structopt(long, env = "CH_URL", default_value = "http://conference-hall.io")]
    pub(crate) url: String,

    /// The conference_hall event id
    #[structopt(short, long, env)]
    pub(crate) event_id: String,

    /// The conference_hall API key id
    #[structopt(short = "k", long, env)]
    pub(crate) api_key: String,
}

impl Into<ConferenceHallConfig> for ConferenceHallOpts {
    fn into(self) -> ConferenceHallConfig {
        let ConferenceHallOpts {
            url,
            event_id,
            api_key,
        } = self;

        ConferenceHallConfig::new(url, event_id, api_key)
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct MongodbOpts {
    /// The MongoDB URI
    #[structopt(
        short = "u",
        long,
        env = "MONGODB_URI",
        default_value = "mongodb://localhost:27017/"
    )]
    pub(crate) uri: String,

    /// The MongoDB database
    #[structopt(short = "d", long, env = "MONGODB_DATABASE", default_value = "devfest")]
    pub(crate) database: String,
}

impl Into<MongodbConfig> for MongodbOpts {
    fn into(self) -> MongodbConfig {
        let MongodbOpts {
            uri: mongodb_uri,
            database: mongodb_database,
        } = self;

        MongodbConfig::new(mongodb_uri, mongodb_database)
    }
}

#[derive(StructOpt, Debug, Clone)]
pub struct ServerOpts {
    /// The server host
    #[structopt(short, long, env = "HTTP_HOST", default_value = "0.0.0.0")]
    pub(crate) host: String,

    /// The server port
    #[structopt(short, long, env = "HTTP_PORT", default_value = "8080")]
    pub(crate) port: u32,

    /// The graphql api path
    #[structopt(long, env = "GRAPHQL_PATH", default_value = "graphql")]
    pub(crate) graphql_path: String,

    /// The REST api path
    #[structopt(long, env = "REST_PATH", default_value = "api")]
    pub(crate) rest_path: String,
}

impl Into<ServerConfig> for ServerOpts {
    fn into(self) -> ServerConfig {
        let ServerOpts {
            host,
            port,
            graphql_path,
            rest_path,
        } = self;

        ServerConfig::new(host, port, graphql_path, rest_path)
    }
}
