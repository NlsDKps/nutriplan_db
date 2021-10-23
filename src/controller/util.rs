#[cfg(test)]
pub mod test {
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

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

    /** Mutex, necessary to prevent colliding access to database */
    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(Mutex::default);

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
        assert!(result.is_ok());
        db_teardown();
    }
}
