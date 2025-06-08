#[macro_export]
macro_rules! match_driver {
    ($driver: expr, $db_driver: ident -> $instr: block) => {
        match $driver {
            #[cfg(target_os = "android")]
            DbProvider::Sqlite($db_driver) => $instr,
            #[cfg(target_os = "linux")]
            DbProvider::Postgres($db_driver) => $instr,
            _ => {}
        }
    };
}
