use diesel::{prelude::*, Queryable, Insertable};
use log::error;

use crate::{
    controller::database::{ConnMgrPool, CRUDController},
    controller::database::recipe_ingredient::CRUDRecipeIngredient,
    schema::*
};

#[derive(Insertable)]
#[table_name="recipes"]
pub struct NewRecipe {
    pub name: String,
    pub description: String
}

impl NewRecipe {
    pub fn new(name: &str, description: &str) -> Self {
        NewRecipe { name: name.to_owned(), description: description.to_owned() }
    }
}

#[derive(AsChangeset, Queryable, Debug)]
#[table_name="recipes"]
pub struct Recipe {
    pub id: Option<i32>,
    pub name: String,
    pub description: String
}

impl Recipe {
    pub fn new(id: i32, name: &str, description: &str) -> Self {
        Recipe { id: Some(id), name: name.to_owned(), description: description.to_owned() }
    }
}

pub struct CRUDRecipe { }

impl CRUDController for CRUDRecipe {
    type NewItem = NewRecipe;
    type Item = Recipe;

    fn create(conn_mgr: &ConnMgrPool, new_item: &NewRecipe) -> bool {
        match diesel::insert_into(recipes::table)
        .values(new_item)
        .execute(conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert meal: {}", e);
                false
            }
        }
    }

    fn read(conn_mgr: &ConnMgrPool, item_id: i32) -> Option<Recipe> {
        use crate::schema::recipes::dsl::*;

        match recipes
            .filter(id.eq(item_id))
            .load::<Recipe>(conn_mgr)
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

    fn update(conn_mgr: &ConnMgrPool, item_id: i32, item: Recipe) -> bool {
        use crate::schema::recipes::dsl::*;

        match diesel::update(
            recipes.filter(id.eq(item_id)))
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
        use crate::schema::recipes::dsl::*;

        CRUDRecipeIngredient::delete_by_recipe_id(conn_mgr, item_id);
        match diesel::delete(
            recipes.filter(id.eq(item_id)))
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
            let item = NewRecipe::new("testitem", "testdescription");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDRecipe::create(&conn_mgr, &item);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let item = NewRecipe::new("testitem", "testdescription");
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDRecipe::create(&conn_mgr, &item);
            assert!(ret_val, "could not create meal");
        })
    }

    #[test]
    fn create_creates_item_correct_parameters() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = NewRecipe::new("createditem", "createddescription");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDRecipe::create(&conn_mgr, &item);
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM recipes WHERE id=(select max(id) from recipes);")
                .output()
                .expect("Failed to execute process");
            let expected = "3|createditem|createddescription\n";
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }

    #[test]
    fn read_returns_correct_item() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let ret_val = CRUDRecipe::read(&conn_mgr, 1).unwrap();
            assert_eq!(ret_val.id, Some(1));
            assert_eq!(ret_val.name, "testitem1");
            assert_eq!(ret_val.description, "testdescription1");
        })
    }

    #[test]
    fn update_with_sane_id_updates_as_expected() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let item = Recipe::new(1, "updatedname", "updateddescription");
            let conn_mgr = setup_conn_mgr();
            let _ = CRUDRecipe::update(&conn_mgr, 1, item);
            let expected = "1|updatedname|updateddescription\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM recipes WHERE id=1;")
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
            let _ = CRUDRecipe::delete(&conn_mgr, 1);
            let expected = "2|testitem2|testdescription2\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM recipes;")
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
            let _ = CRUDRecipe::delete(&conn_mgr, 1);
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
            let inserted = CRUDRecipe::check(&conn_mgr, 1);
            assert_eq!(inserted, true)
        })
    }

    #[test]
    fn check_returns_false_if_ingredient_not_available() {
        run_db_test(|| {
            let conn_mgr = setup_conn_mgr();
            let inserted = CRUDRecipe::check(&conn_mgr, 3);
            assert_eq!(inserted, false)
        })
    }
}
