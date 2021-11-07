use diesel::{prelude::*, Queryable, Insertable};
use log::error;

use crate::{
    controller::database::{ConnMgrPool, CRUDController},
    controller::database::ingredient::CRUDIngredient,
    controller::database::meal::CRUDMeal,
    schema::*
};

#[derive(Insertable)]
#[table_name="meal_ingredients"]
pub struct NewMealIngredient {
    pub meal_id: i32,
    pub ingredient_id: i32,
    pub mass: i32
}

impl NewMealIngredient {
    fn new(meal_id: i32, ingredient_id: i32, mass: i32) -> Self {
        NewMealIngredient { meal_id, ingredient_id, mass }
    }
}

#[derive(AsChangeset, Queryable, Debug)]
#[table_name="meal_ingredients"]
pub struct MealIngredient {
    pub id: Option<i32>,
    pub meal_id: i32,
    pub ingredient_id: i32,
    pub mass: i32
}

impl MealIngredient {
    fn new(id: i32, meal_id: i32, ingredient_id: i32, mass: i32) -> Self {
        MealIngredient { id: Some(id), meal_id, ingredient_id, mass }
    }
}

pub struct CRUDMealIngredient { }

impl CRUDMealIngredient {
    pub fn delete_by_ingredient_id(conn_mgr: &ConnMgrPool, iid: i32) -> bool {
        use crate::schema::meal_ingredients::dsl::*;

        match diesel::delete(
            meal_ingredients.filter(ingredient_id.eq(iid)))
            .execute(conn_mgr) {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not delete item with ingredient id {}: {}", iid, e);
                    false
                }
            }

    }
    
    pub fn delete_by_meal_id(conn_mgr: &ConnMgrPool, mid: i32) -> bool {
        use crate::schema::meal_ingredients::dsl::*;

        match diesel::delete(
            meal_ingredients.filter(meal_id.eq(mid)))
            .execute(conn_mgr) {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not delete item with meal id {}: {}", mid, e);
                    false
                }
            }

    }
}

impl CRUDController for CRUDMealIngredient {
    type NewItem = NewMealIngredient;
    type Item = MealIngredient;

    fn create(conn_mgr: &ConnMgrPool, new_item: &NewMealIngredient) -> bool {
        let meal_avail = CRUDMeal::check(conn_mgr, new_item.meal_id);
        let ingredient_avail = CRUDIngredient::check(conn_mgr, new_item.ingredient_id);
        match meal_avail & ingredient_avail {
            true => (),
            false => return false
        }

        match diesel::insert_into(meal_ingredients::table)
        .values(new_item)
        .execute(conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert meal: {}", e);
                false
            }
        }
    }

    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<MealIngredient> {
        use crate::schema::meal_ingredients::dsl::*;

        match meal_ingredients
            .filter(id.eq(item_id))
            .load::<MealIngredient>(conn_mgr)
        {
            Ok(mut entities) => {
                if entities.len() == 0 {
                    error!("Could not find item with id: {}", item_id);
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

    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: MealIngredient) -> bool {
        use crate::schema::meal_ingredients::dsl::*;

        match diesel::update(
            meal_ingredients.filter(id.eq(item_id)))
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
        use crate::schema::meal_ingredients::dsl::*;

        match diesel::delete(
            meal_ingredients.filter(id.eq(item_id)))
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
    fn create_accepts_item_as_parameter() {
        run_db_test(|| {
            let item = NewMealIngredient::new(1, 1, 123);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMealIngredient::create(&conn_mgr, &item);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let item = NewMealIngredient::new(1, 1, 123);
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMealIngredient::create(&conn_mgr, &item);
            assert!(ret_val, "could not create item");
        })
    }

    #[test]
    fn create_creates_item_correct_parameters() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = NewMealIngredient::new(1, 1, 123);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMealIngredient::create(&conn_mgr, &item);
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meal_ingredients WHERE id=(select max(id) from meal_ingredients);")
                .output()
                .expect("Failed to execute process");
            let expected = "3|1|1|123\n";
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn create_returns_false_on_unknown_meal() {
        run_db_test(|| {
            let item = NewMealIngredient::new(3, 1, 123);
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMealIngredient::create(&conn_mgr, &item);
            assert_eq!(ret_val, false);
        })
    }

    #[test]
    fn create_returns_false_on_unknown_ingredient() {
        run_db_test(|| {
            let item = NewMealIngredient::new(1, 3, 123);
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMealIngredient::create(&conn_mgr, &item);
            assert_eq!(ret_val, false);
        })
    }

    #[test]
    fn read_returns_correct_item() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMealIngredient::read(&conn_mgr, 1).unwrap();
            assert_eq!(ret_val.id, Some(1));
            assert_eq!(ret_val.meal_id, 1);
            assert_eq!(ret_val.ingredient_id, 1);
            assert_eq!(ret_val.mass, 111);
        })
    }

    #[test]
    fn update_with_sane_id_updates_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = MealIngredient::new(1, 2, 3, 456);
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMealIngredient::update(&conn_mgr, 1, item);
            let expected = "1|2|3|456\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meal_ingredients WHERE id=1;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_by_item_id_with_sane_id_deletes_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMealIngredient::delete(&conn_mgr, 1);
            let expected = "2|2|2|222\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meal_ingredients;")
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
            let _ = CRUDMealIngredient::delete_by_ingredient_id(&conn_mgr, 1);
            let expected = "2|2|2|222\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meal_ingredients;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_by_meal_id_with_sane_id_deletes_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMealIngredient::delete_by_meal_id(&conn_mgr, 1);
            let expected = "2|2|2|222\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meal_ingredients;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_by_meal_id_with_sane_id_returns_true() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMealIngredient::delete_by_meal_id(&conn_mgr, 1);
            assert_eq!(ret_val, true);
        })
    }

    #[test]
    fn check_returns_true_if_ingredient_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDMealIngredient::check(&conn_mgr, 1);
            assert_eq!(inserted, true)
        })
    }

    #[test]
    fn check_returns_false_if_ingredient_not_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDMealIngredient::check(&conn_mgr, 3);
            assert_eq!(inserted, false)
        })
    }
}

