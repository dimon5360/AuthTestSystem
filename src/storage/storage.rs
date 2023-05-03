use futures::TryFutureExt;
use tokio_postgres::*;
use actix::fut;
use actix::prelude::*;

pub struct PostgresHandler {
    pub client: Option<tokio_postgres::Client>,
}

impl Actor for PostgresHandler {
    type Context = Context<Self>;
}

impl PostgresHandler {
    pub fn new() -> Addr<PostgresHandler> {
        let path = std::env::current_dir().unwrap();

        dotenv::from_path(format!("{}/{}", path.display().to_string(), "config/db.env")).unwrap();
        let connection_string = "CONNECTION_STRING";

        let conn_str = match std::env::var(connection_string) {
            Ok(conn_str) => conn_str,
            Err(e) => panic!("${} is not set ({})", connection_string, e),
        };

        println!("{}", conn_str);
        let pg_connection = connect(&conn_str, tokio_postgres::NoTls);

        PostgresHandler::create(move |_ctx| {
          let pg_actor = PostgresHandler {
            client: None,
          };

          let _res = pg_connection
            .map_err(|_| panic!("Can not connect to postgresql"))
            .into_actor(&pg_actor)
            .then(|res, pg_actor, _ctx| {
                let (pg_client, _conn) = res.unwrap();
                pg_actor.client = Some(pg_client);
                fut::ok::<i32,i32>(0)
            });

            pg_actor
        })
    }

    pub fn _start(&mut self) {
        println!("start postgres client");
    }
}

pub struct AddUser {
    pub username: String,
    pub password: String,
  }

  impl Message for AddUser {
    type Result = std::io::Result<()>;
  }

  impl Handler<AddUser> for PostgresHandler {
    type Result = Result<(), std::io::Error>;

    fn handle(&mut self, _data: AddUser, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
  }