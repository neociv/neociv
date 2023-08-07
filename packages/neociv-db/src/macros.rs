#[macro_export]
macro_rules! db_stmt {
    ($db:ident, $id:expr, $sql:expr) => {
        $db.prep_stmt($id, $sql)
    };
}
pub(crate) use db_stmt;

#[macro_export]
macro_rules! exec_stmt  {
    ($db:expr, $id:expr, $( $args:expr ),*) => {
        $db.exec_stmt($id, rusqlite::params![$( $args ),*])
    };
}
pub(crate) use exec_stmt;
