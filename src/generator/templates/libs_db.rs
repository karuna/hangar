pub static TEXT: &'static str = "use r2d2_diesel::ConnectionManager;

use {{db_connection_long}};

use super::conn::Conn;

pub type DbConnection = {{db_connection_short}};

pub type ConnPool = Conn<ConnectionManager<DbConnection>>;
";
