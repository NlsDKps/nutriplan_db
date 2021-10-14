use crate::controller::database::CRUDController;

struct CRUDIngredient { }

impl CRUDController for CRUDIngredient {

}

#[cfg(test)]
mod test {
    use super::*;
    use crate::controller::util::test::run_db_test;


    #[test]
    fn it_works() {
        run_db_test(|| {
            assert!(true)
        })
    }
}
