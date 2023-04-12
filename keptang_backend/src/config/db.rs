
use mysql::*;
    

pub fn conDB() -> std::result::Result<PooledConn, Box<dyn std::error::Error>> {
    let url = "mysql://root:@localhost:3306/keptang";
    let pool = Pool::new(url)?;
    let conn = pool.get_conn()?;
    Ok(conn)
}
