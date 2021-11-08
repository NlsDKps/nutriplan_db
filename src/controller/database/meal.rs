use diesel::{
    prelude::*,
    Queryable, Insertable,
};
use log::error;

use crate::{
    controller::database::{ConnMgrPool, CRUDController},
    controller::database::meal_ingredient::CRUDMealIngredient,
    schema::*
};

#[derive(Insertable)]
#[table_name="meals"]
pub struct NewMeal {
    pub name: String,
    pub date: String,
    pub time: String
}

impl NewMeal {
    fn new(name: &str, date: &chrono::NaiveDate, time: &chrono::NaiveTime) -> Self {
        NewMeal {
            name: name.to_owned(),
            date: date.to_string(),
            time: time.to_string()
        }
    }
}

#[derive(AsChangeset, Queryable, Debug)]
#[table_name="meals"]
pub struct Meal {
    pub id: Option<i32>,
    pub name: String,
    pub date: String,
    pub time: String
}

impl Meal {
    fn new(id: i32, name: &str, date: &chrono::NaiveDate, time: &chrono::NaiveTime) -> Self {
        Meal {
            id: Some(id),
            name: name.to_owned(),
            date: date.to_string(),
            time: time.to_string()
        }
    }
}

pub struct CRUDMeal { }

impl CRUDMeal {}

impl CRUDController for CRUDMeal {
    type NewItem = NewMeal;
    type Item = Meal;

    fn create(conn_mgr: &ConnMgrPool, new_item: &NewMeal) -> bool {
        match diesel::insert_into(meals::table)
        .values(new_item)
        .execute(conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert meal: {}", e);
                false
            }
        }
    }

    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<Meal> {
        use crate::schema::meals::dsl::*;

        match meals
            .filter(id.eq(item_id))
            .load::<Meal>(conn_mgr)
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
    
    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: Meal) -> bool {
        use crate::schema::meals::dsl::*;

        match diesel::update(
            meals.filter(id.eq(item_id)))
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
        use crate::schema::meals::dsl::*;

        CRUDMealIngredient::delete_by_meal_id(conn_mgr, item_id);
        match diesel::delete(
            meals.filter(id.eq(item_id)))
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
    fn create_accepts_ingredient_as_parameter() {
        run_db_test(|| {
            let item = NewMeal::new("created",
                &chrono::NaiveDate::from_ymd(2020, 01, 02),
                &chrono::NaiveTime::from_hms(9, 10, 11));
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMeal::create(&conn_mgr, &item);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let item = NewMeal::new("created",
                &chrono::NaiveDate::from_ymd(2020, 01, 02),
                &chrono::NaiveTime::from_hms(9, 10, 11));
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMeal::create(&conn_mgr, &item);
            assert!(ret_val, "could not create meal");
        })
    }

    #[test]
    fn create_creates_item_correct_parameters() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = NewMeal::new("created",
                &chrono::NaiveDate::from_ymd(2020, 01, 02),
                &chrono::NaiveTime::from_hms(9, 10, 11));
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMeal::create(&conn_mgr, &item);
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT name FROM meals WHERE id=(select max(id) from meals);")
                .output()
                .expect("Failed to execute process");
            let expected = "created\n";
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn read_with_sane_id_returns_correct_item() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDMeal::read(&conn_mgr, 1).unwrap();
            assert_eq!(ret_val.name, "testmeal1");
            assert_eq!(ret_val.id, Some(1));
        })
    }

    #[test]
    fn update_with_sane_id_updates_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = Meal::new(1, "updated",
                &chrono::NaiveDate::from_ymd(2020, 12, 31),
                &chrono::NaiveTime::from_hms(12, 13, 14));
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMeal::update(&conn_mgr, 1, item);
            let expected = "1|updated|2020-12-31|12:13:14\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meals WHERE id=1;")
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
            let _ = CRUDMeal::delete(&conn_mgr, 1);
            let expected = "2|testmeal2|2000-02-02|20:00:00\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM meals;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_also_removes_meal_ingredient_entry() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDMeal::delete(&conn_mgr, 1);
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
    fn check_returns_true_if_ingredient_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDMeal::check(&conn_mgr, 1);
            assert_eq!(inserted, true)
        })
    }

    #[test]
    fn check_returns_false_if_ingredient_not_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDMeal::check(&conn_mgr, 3);
            assert_eq!(inserted, false)
        })
    }
}

