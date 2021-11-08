use diesel::{
    prelude::*,
    Queryable, Insertable,
};
use log::error;
use crate::{
    controller::database::{ConnMgrPool, CRUDController},
    controller::database::ingredient_macro::CRUDIngredientMacro,
    controller::database::meal_ingredient::CRUDMealIngredient,
    controller::database::recipe_ingredient::CRUDRecipeIngredient,
    schema::*
};

#[derive(Insertable)]
#[table_name="ingredients"]
pub struct NewIngredient {
    pub name: String
}

impl NewIngredient {
    pub fn new(name: &str) -> Self {
        NewIngredient {
            name: String::from(name)
        }
    }
}

#[derive(AsChangeset, Queryable, Debug)]
#[table_name="ingredients"]
pub struct Ingredient {
    pub id: Option<i32>,
    pub name: String
}

impl Ingredient {
    pub fn new(id: i32, name: &str) -> Self {
        Ingredient {
            id: Some(id),
            name: String::from(name)
        }
    }
}

pub struct CRUDIngredient { }

impl CRUDIngredient {
}

impl CRUDController for CRUDIngredient {
    type NewItem = NewIngredient;
    type Item = Ingredient;

    fn create(conn_mgr: &ConnMgrPool, new_item: &NewIngredient) -> bool {
        match diesel::insert_into(ingredients::table)
        .values(new_item)
        .execute(conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert ingredient: {}", e);
                false
            }
        }
    }

    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<Ingredient> {
        use crate::schema::ingredients::dsl::*;

        match ingredients
            .filter(id.eq(item_id))
            .load::<Ingredient>(conn_mgr)
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

    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: Ingredient) -> bool {
        use crate::schema::ingredients::dsl::*;

        match diesel::update(
            ingredients.filter(id.eq(item_id)))
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
        use crate::schema::ingredients::dsl::*;

        CRUDIngredientMacro::delete_by_ingredient_id(conn_mgr, item_id);
        CRUDMealIngredient::delete_by_ingredient_id(conn_mgr, item_id);
        CRUDRecipeIngredient::delete_by_ingredient_id(conn_mgr, item_id);
        match diesel::delete(
            ingredients.filter(id.eq(item_id)))
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

    use crate::controller::util::test::{setup_conn_mgr, run_db_test};

    #[test]
    fn create_accepts_ingredient_as_parameter() {
        run_db_test(|| {
            let ingredient = NewIngredient::new("test");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredient::create(&conn_mgr, &ingredient);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let ingredient = NewIngredient::new("test");
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDIngredient::create(&conn_mgr, &ingredient);
            assert!(ret_val, "could not create ingredient");
        })
    }

    #[test]
    fn create_creates_item_correct_parameters() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let ingredient = NewIngredient::new("created");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredient::create(&conn_mgr, &ingredient);
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT name FROM ingredients WHERE id=(select max(id) from ingredients);")
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
            let ret_val = CRUDIngredient::read(&conn_mgr, 1).unwrap();
            assert_eq!(ret_val.name, "test1");
            assert_eq!(ret_val.id, Some(1));
        })
    }

    #[test]
    fn update_with_sane_id_updates_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let ingredient = Ingredient::new(1, "updated");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredient::update(&conn_mgr, 1, ingredient);
            let expected = "1|updated\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredients WHERE id=1;")
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
            let _ = CRUDIngredient::delete(&conn_mgr, 1);
            let expected = "2|test2\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredients;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn delete_with_sane_id_also_deletes_corresponding_macro() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredient::delete(&conn_mgr, 2);
            let expected = "1|1|1.0|1.0|1.0|1.0|1.0\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredient_macros;")
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
            let _ = CRUDIngredient::delete(&conn_mgr, 1);
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
    fn delete_also_removes_recipe_ingredient_entry() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDIngredient::delete(&conn_mgr, 1);
            let expected = "2|2|2|222\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM recipe_ingredients;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn check_returns_true_if_ingredient_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDIngredient::check(&conn_mgr, 1);
            assert_eq!(inserted, true)
        })
    }

    #[test]
    fn check_returns_false_if_ingredient_not_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDIngredient::check(&conn_mgr, 3);
            assert_eq!(inserted, false)
        })
    }
}
