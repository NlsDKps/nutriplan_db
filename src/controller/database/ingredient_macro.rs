use diesel::{
    prelude::*,
    Queryable, Insertable
};
use log::error;
use crate::{
    controller::database::{ConnMgrPool, CRUDController},
    schema::*
};

#[derive(Insertable)]
#[table_name="ingredient_macros"]
pub struct NewIngredientMacro {
    pub ingredient_id: i32,
    pub proteins: f32,
    pub carbs: f32,
    pub fats: f32,
    pub alcohols: f32,
    pub calories: f32,
}

impl NewIngredientMacro {
    pub fn new(ingredient_id: i32, proteins: f32, carbs: f32, fats: f32, alcohols: f32) -> Self
    {
        let mut calories = proteins * 4.0;
        calories += carbs * 4.0;
        calories += fats * 4.0;
        calories += alcohols * 4.0;
        NewIngredientMacro {
            ingredient_id,
            proteins,
            carbs,
            fats,
            alcohols,
            calories
        }
    }
}

#[derive(AsChangeset, Queryable, Debug)]
#[table_name="ingredient_macros"]
pub struct IngredientMacro {
    pub id: Option<i32>,
    pub ingredient_id: i32,
    pub proteins: f32,
    pub carbs: f32,
    pub fats: f32,
    pub alcohols: f32,
    pub calories: f32,
}

impl IngredientMacro {
    pub fn new(id: i32, ingredient_id: i32, proteins: f32, carbs: f32, fats: f32, alcohols: f32
        ) -> Self
    {
        let mut calories = proteins * 4.0;
        calories += carbs * 4.0;
        calories += fats * 4.0;
        calories += alcohols * 4.0;
        IngredientMacro {
            id: Some(id),
            ingredient_id,
            proteins,
            carbs,
            fats,
            alcohols,
            calories
        }
    }
}

pub struct CRUDIngredientMacro { }

impl CRUDIngredientMacro {
    pub fn delete_by_ingredient_id(conn_mgr: &ConnMgrPool, item_id: i32) -> bool {
        use crate::schema::ingredient_macros::dsl::*;

        match diesel::delete(
            ingredient_macros.filter(ingredient_id.eq(item_id)))
            .execute(conn_mgr) {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not delete item with id {}: {}", item_id, e);
                    false
                }
            }
    }
}

impl CRUDController for CRUDIngredientMacro {
    type NewItem = NewIngredientMacro;
    type Item = IngredientMacro;

    fn create(conn_mgr: &ConnMgrPool, new_item: &NewIngredientMacro) -> bool {
        use crate::controller::database::ingredient::CRUDIngredient;
        let avail = CRUDIngredient::check(conn_mgr, new_item.ingredient_id);
        match avail {
            true => (),
            false => return false
        }
        match diesel::insert_into(ingredient_macros::table)
        .values(new_item)
        .execute(conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert ingredient: {}", e);
                false
            }
        }
    }

    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<IngredientMacro> {
        use crate::schema::ingredient_macros::dsl::*;

        match ingredient_macros
            .filter(id.eq(item_id))
            .load::<IngredientMacro>(conn_mgr)
        {
            Ok(mut entities) => {
                if entities.len() == 0 {
                    error!("Could not find ingredient with id: {}", item_id);
                    None
                } else {
                    Some(entities.remove(0))
                }
            },
            Err(e) => {
                error!("Could not read from database: {}", e);
                None
            }
        }
    }

    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: IngredientMacro) -> bool {
        use crate::schema::ingredient_macros::dsl::*;

        match diesel::update(
            ingredient_macros.filter(id.eq(item_id)))
            .set(item)
            .execute(conn_mgr)
            {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not update item with id {}: {}", item_id, e);
                    false
                }
            }
    }

    fn delete(conn_mgr: &ConnMgrPool, item_id: i32) -> bool {
        use crate::schema::ingredient_macros::dsl::*;

        match diesel::delete(
            ingredient_macros.filter(id.eq(item_id)))
            .execute(conn_mgr) {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not delete item with id {}: {}", item_id, e);
                    false
                }
            }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::controller::util::test::{run_db_test, setup_conn_mgr};

    #[test]
    fn create_accepts_ingredient_macro_as_parameter() {
        run_db_test(|| {
            let item = NewIngredientMacro::new(1, 2.0, 3.0, 4.0, 5.0);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredientMacro::create(&conn_mgr, &item);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let item = NewIngredientMacro::new(1, 2.0, 3.0, 4.0, 5.0);
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDIngredientMacro::create(&conn_mgr, &item);
            assert!(ret_val, "could not create share");
        })
    }

    #[test]
    fn read_with_sane_id_returns_correct_ingredient() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDIngredientMacro::read(&conn_mgr, 1).unwrap();
            assert_eq!(ret_val.id, Some(1));
            assert_eq!(ret_val.ingredient_id, 1);
        })
    }

    #[test]
    fn update_with_sane_id_updates_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = IngredientMacro::new(1, 1, 4.0, 5.0, 6.0, 7.0);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredientMacro::update(&conn_mgr, 1, item);
            let expected = "1|1|4.0|5.0|6.0|7.0|88.0\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredient_macros WHERE id=1;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_with_sane_id_deletes_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredientMacro::delete(&conn_mgr, 1);
            let expected = "2|2|2.0|2.0|2.0|2.0|2.0\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredient_macros;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_by_ingredient_id_with_sane_id_deletes_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredientMacro::delete_by_ingredient_id(&conn_mgr, 1);
            let expected = "2|2|2.0|2.0|2.0|2.0|2.0\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredient_macros;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn create_does_not_create_on_wrong_ingredient_id() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = NewIngredientMacro::new(3, 2.0, 3.0, 4.0, 5.0);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredientMacro::create(&conn_mgr, &item);
            let expected = "1|1|1.0|1.0|1.0|1.0|1.0\n2|2|2.0|2.0|2.0|2.0|2.0\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredient_macros;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn check_returns_true_if_ingredient_macro_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDIngredientMacro::check(&conn_mgr, 1);
            assert_eq!(inserted, true)
        })
    }

    #[test]
    fn check_returns_false_if_ingredient_macro_not_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDIngredientMacro::check(&conn_mgr, 3);
            assert_eq!(inserted, false)
        })
    }
}

