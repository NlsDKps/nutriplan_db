#[macro_use]
extern crate diesel;

pub mod controller;
pub mod schema;

use controller::database::ConnMgrPool;
use controller::util::setup_conn_mgr;


use controller::database::{
    ingredient::{NewIngredient, Ingredient},
    ingredient_macro::{NewIngredientMacro, IngredientMacro},
    meal::{NewMeal, Meal},
    meal_ingredient::{NewMealIngredient, MealIngredient},
    recipe::{NewRecipe, Recipe},
    recipe_ingredient::{NewRecipeIngredient, RecipeIngredient},
};

pub trait INutriplanDbIngredient {
    /* Ingredients */
    fn create(&self, item: NewIngredient) -> bool;
    fn read(&self,id: i32) -> Option<Ingredient>;
    fn update(&self, item: Ingredient) -> bool;
    fn delete(&self, id: i32) -> bool;
}
pub trait INutriplanDbIngredientMacro {
    /* Ingredient Macros */
    fn create(&self, item: NewIngredientMacro) -> bool;
    fn read(&self, id: i32) -> Option<IngredientMacro>;
    fn update(&self, item: IngredientMacro) -> bool;
    fn delete(&self, id: i32) -> bool;
}
pub trait INutriplanDbMeal {
    /* Meals */
    fn create(&self, item: NewMeal) -> bool;
    fn read(&self, id: i32) -> Option<Meal>;
    fn update(&self, item: Meal) -> bool;
    fn delete(&self, id: i32) -> bool;
}
pub trait INutriplanDbMealIngredient {
    /* Meal Ingredients */
    fn create(&self, item: NewMealIngredient) -> bool;
    fn read(&self, id: i32) -> Option<MealIngredient>;
    fn update(&self, item: MealIngredient) -> bool;
    fn delete(&self, id: i32) -> bool;
}
pub trait INutriplanDbRecipe {
    /* Recipes */
    fn create(&self, item: NewRecipe) -> bool;
    fn read(&self, id: i32) -> Option<Recipe>;
    fn update(&self, item: Recipe) -> bool;
    fn delete(&self, id: i32) -> bool;
}
pub trait INutriplanDbRecipeIngredient {
    /* Recept Ingredients */
    fn create(&self, item: NewRecipeIngredient) -> bool;
    fn read(&self, id: i32) -> Option<RecipeIngredient>;
    fn update(&self, item: RecipeIngredient) -> bool;
    fn delete(&self, id: i32) -> bool;
}

pub struct NutriplanSqliteDbIngredient {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbIngredient {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbIngredient{ conn_mgr }
    }
}

impl INutriplanDbIngredient for NutriplanSqliteDbIngredient {
    fn create(&self, item: NewIngredient) -> bool {
        CRUDIngredient::create(&self.conn_mgr, &item)
    }

    fn read(&self, id: i32) -> Option<Ingredient> {
        CRUDIngredient::read(&self.conn_mgr, id)
    }
    fn update(&self, item: Ingredient) -> bool {
        match item.id {
            Some(id) => CRUDIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete(&self, id: i32) -> bool {
        CRUDIngredient::delete(&self.conn_mgr, id)
    }
}

pub struct NutriplanSqliteDbIngredientMacro {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbIngredientMacro {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbIngredientMacro{ conn_mgr }
    }
}

impl INutriplanDbIngredientMacro for NutriplanSqliteDbIngredientMacro {
    fn create(&self, item: NewIngredientMacro) -> bool {
        CRUDIngredientMacro::create(&self.conn_mgr, &item)
    }
    fn read(&self, id: i32) -> Option<IngredientMacro> {
        CRUDIngredientMacro::read(&self.conn_mgr, id)
    }
    fn update(&self, item: IngredientMacro) -> bool {
        match item.id {
            Some(id) => CRUDIngredientMacro::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete(&self, id: i32) -> bool {
        CRUDIngredientMacro::delete(&self.conn_mgr, id)
    }
}

pub struct NutriplanSqliteDbMeal {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbMeal {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbMeal{ conn_mgr }
    }
}

impl INutriplanDbMeal for NutriplanSqliteDbMeal {
    fn create(&self, item: NewMeal) -> bool {
        CRUDMeal::create(&self.conn_mgr, &item)
    }
    fn read(&self, id: i32) -> Option<Meal> {
        CRUDMeal::read(&self.conn_mgr, id)
    }
    fn update(&self, item: Meal) -> bool {
        match item.id {
            Some(id) => CRUDMeal::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete(&self, id: i32) -> bool {
        CRUDMeal::delete(&self.conn_mgr, id)
    }
}

pub struct NutriplanSqliteDbMealIngredient {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbMealIngredient {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbMealIngredient{ conn_mgr }
    }
}

impl INutriplanDbMealIngredient for NutriplanSqliteDbMealIngredient {
    fn create(&self, item: NewMealIngredient) -> bool {
        CRUDMealIngredient::create(&self.conn_mgr, &item)
    }

    fn read(&self, id: i32) -> Option<MealIngredient> {
        CRUDMealIngredient::read(&self.conn_mgr, id)
    }

    fn update(&self, item: MealIngredient) -> bool {
        match item.id {
            Some(id) => CRUDMealIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }

    fn delete(&self, id: i32) -> bool {
        CRUDMealIngredient::delete(&self.conn_mgr, id)
    }
}

pub struct NutriplanSqliteDbRecipe {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbRecipe {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbRecipe{ conn_mgr }
    }
}

impl INutriplanDbRecipe for NutriplanSqliteDbRecipe {
    fn create(&self, item: NewRecipe) -> bool {
        CRUDRecipe::create(&self.conn_mgr, &item)
    }

    fn read(&self, id: i32) -> Option<Recipe> {
        CRUDRecipe::read(&self.conn_mgr, id)
    }

    fn update(&self, item: Recipe) -> bool {
        match item.id {
            Some(id) => CRUDRecipe::update(&self.conn_mgr, id, item),
            None => false
        }
    }

    fn delete(&self, id: i32) -> bool {
        CRUDRecipe::delete(&self.conn_mgr, id)
    }
}

pub struct NutriplanSqliteDbRecipeIngredient {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqliteDbRecipeIngredient {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqliteDbRecipeIngredient{ conn_mgr }
    }
}

use controller::database::{
    CRUDController,
    ingredient::CRUDIngredient,
    ingredient_macro::CRUDIngredientMacro,
    meal::CRUDMeal,
    meal_ingredient::CRUDMealIngredient,
    recipe::CRUDRecipe,
    recipe_ingredient::CRUDRecipeIngredient,
};

impl INutriplanDbRecipeIngredient for NutriplanSqliteDbRecipeIngredient {
    fn create(&self, item: NewRecipeIngredient) -> bool {
        CRUDRecipeIngredient::create(&self.conn_mgr, &item)
    }
    fn read(&self, id: i32) -> Option<RecipeIngredient> {
        CRUDRecipeIngredient::read(&self.conn_mgr, id)
    }
    fn update(&self, item: RecipeIngredient) -> bool {
        match item.id {
            Some(id) => CRUDRecipeIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete(&self, id: i32) -> bool {
        CRUDRecipeIngredient::delete(&self.conn_mgr, id)
    }
}
