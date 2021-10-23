use diesel::{
    prelude::*,
    Queryable, Insertable,
    r2d2::{PooledConnection, ConnectionManager}
};
use log::error;
use crate::{
    controller::database::{CRUDController, connect_database},
    schema::*
};

#[derive(Insertable)]
#[table_name="ingredients"]
struct NewIngredient {
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
struct Ingredient {
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

struct CRUDIngredient {
    conn_mgr: PooledConnection<ConnectionManager<SqliteConnection>>
}

impl CRUDIngredient {
    pub fn new(db_url: &str) -> Self {
        let db_pool = match connect_database(db_url) {
            Some(db_pool) => db_pool,
            None => panic!("No database url provided.")
        };
        let conn_mgr = match db_pool.get() {
            Ok(conn_mgr) => conn_mgr,
            Err(_) => panic!("Could not get a connection manager from database pool!")
        };
        CRUDIngredient { conn_mgr }
    }
}

impl CRUDController for CRUDIngredient {
    type NewItem = NewIngredient;
    type Item = Ingredient;

    fn create(&self, new_item: &NewIngredient) -> bool {
        match diesel::insert_into(ingredients::table)
        .values(new_item)
        .execute(&self.conn_mgr) {
            Ok(_) => true,
            Err(e) => {
                error!("Could not insert ingredient: {}", e);
                false
            }
        }
    }

    fn read(&self, item_id: i32) -> Option<Ingredient> {
        use crate::schema::ingredients::dsl::*;

        match ingredients
            .filter(id.eq(item_id))
            .load::<Ingredient>(&self.conn_mgr)
        {
            Ok(mut ingredient_entities) => {
                if ingredient_entities.len() == 0 {
                    error!("Could not find ingredient with id: {}", item_id);
                    None
                } else {
                    Some(ingredient_entities.remove(0))
                }
            },
            Err(e) => {
                error!("Could not read from database: {}", e);
                None
            }
        }
    }

    fn update(&self, item_id: i32, item: Ingredient) -> bool {
        use crate::schema::ingredients::dsl::*;

        match diesel::update(
            ingredients.filter(id.eq(item_id)))
            .set(item)
            .execute(&self.conn_mgr)
            {
                Ok(_) => true,
                Err(e) => {
                    error!("Could not update item with id {}: {}", item_id, e);
                    false
                }
            }
    }

    fn delete(&self, item_id: i32) -> bool {
        use crate::schema::ingredients::dsl::*;

        match diesel::delete(
            ingredients.filter(id.eq(item_id)))
            .execute(&self.conn_mgr) {
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

    use crate::controller::util::test::run_db_test;


    #[test]
    fn create_accepts_ingredient_as_parameter() {
        run_db_test(|| {
            let ingredient = NewIngredient::new("test");
            let _ = CRUDIngredient::new("test.db").create(&ingredient);
        })
    }

    #[test]
    fn create_returns_ok_on_sane_parameters() {
        run_db_test(|| {
            let ingredient = NewIngredient::new("test");
            let ret_val = CRUDIngredient::new("test.db").create(&ingredient);
            assert!(ret_val, "could not create share");
        })
    }

    #[test]
    fn create_creates_item_correct_parameters() {
        run_db_test(|| {
            use std::process::Command;
            use std::str;
            let ingredient = NewIngredient::new("created");
            let _ = CRUDIngredient::new("test.db").create(&ingredient);
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
    fn read_with_sane_id_returns_correct_ingredient() {
        run_db_test(|| {
            let ret_val = CRUDIngredient::new("test.db").read(1).unwrap();
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
            let _ = CRUDIngredient::new("test.db").update(1, ingredient);
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
            let _ = CRUDIngredient::new("test.db").delete(1);
            let expected = "2|test2\n";
            let output = Command::new("sqlite3")
                .arg("test.db")
                .arg("SELECT * FROM ingredients;")
                .output()
                .expect("Failed to execute process");
            assert_eq!(expected, str::from_utf8(&output.stdout).unwrap());
        })
    }
}
