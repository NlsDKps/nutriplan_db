#[cfg(test)]
pub mod test {

    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use crate::controller::database::ConnMgrPool;

    /** Mutex, necessary to prevent colliding access to database */
    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

    /**
     * Setup test environment.
     * NOTE: Currently we allow unused_must_use, since we want to enable the logger. If it is
     * already enabled, it would return an Err, which will occur on the second test we execute.
     * This is not critical and so we do not care.
     */
    #[allow(unused_must_use)]
    pub fn db_setup() {
        env_logger::builder().is_test(true).try_init();
        std::process::Command::new("sh")
            .arg("./test/db/setup_db.sh")
            .output()
            .expect("Failed to build database");
    }

    /**
     * Teardown test environment.
     */
    pub fn db_teardown() {
        std::process::Command::new("sh")
            .arg("./test/db/teardown_db.sh")
            .output()
            .expect("Failed to delete database");
    }

    /**
     * Run test method.
     * This method is used to run tests instead of the rust test method to allow setup and teardown
     * of each testrun.
     */
    pub fn run_db_test<T>(test: T) -> ()
        where T: FnOnce() -> () + std::panic::UnwindSafe
    {
        let _shared = TEST_MUTEX.lock();
        db_setup();
        let result = std::panic::catch_unwind(|| {
            test()
        });
        db_teardown();
        assert!(result.is_ok());
    }

    /**
     * Setup connection manager necessary to connect to the local database.
     */
    pub fn setup_conn_mgr() -> ConnMgrPool {
        use crate::controller::database::connect_database;
        let db_pool = match connect_database("test.db") {
            Some(db_pool) => db_pool,
            None => panic!("No database url provided.")
        };
        match db_pool.get() {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("Could not get a connection manager from database pool!")
        }
    }
}
