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

trait NutriplanDBInterface {
    /* Ingredients */
    fn create_ingredient(&self, item: NewIngredient) -> bool;
    fn read_ingredient(&self,id: i32) -> Option<Ingredient>;
    fn update_ingredient(&self, item: Ingredient) -> bool;
    fn delete_ingredient(&self, id: i32) -> bool;
    /* Ingredient Macros */
    fn create_ingredient_macro(&self, item: NewIngredientMacro) -> bool;
    fn read_ingredient_macro(&self, id: i32) -> Option<IngredientMacro>;
    fn update_ingredient_macro(&self, item: IngredientMacro) -> bool;
    fn delete_ingredient_macro(&self, id: i32) -> bool;
    /* Meals */
    fn create_meal(&self, item: NewMeal) -> bool;
    fn read_meal(&self, id: i32) -> Option<Meal>;
    fn update_meal(&self, item: Meal) -> bool;
    fn delete_meal(&self, id: i32) -> bool;
    /* Meal Ingredients */
    fn create_meal_ingredient(&self, item: NewMealIngredient) -> bool;
    fn read_meal_ingredient(&self, id: i32) -> Option<MealIngredient>;
    fn update_meal_ingredient(&self, item: MealIngredient) -> bool;
    fn delete_meal_ingredient(&self, id: i32) -> bool;
    /* Recipes */
    fn create_recipe(&self, item: NewRecipe) -> bool;
    fn read_recipe(&self, id: i32) -> Option<Recipe>;
    fn update_recipe(&self, item: Recipe) -> bool;
    fn delete_recipe(&self, id: i32) -> bool;
    /* Recept Ingredients */
    fn create_recipe_ingredient(&self, item: NewRecipeIngredient) -> bool;
    fn read_recipe_ingredient(&self, id: i32) -> Option<RecipeIngredient>;
    fn update_recipe_ingredient(&self, item: RecipeIngredient) -> bool;
    fn delete_recipe_ingredient(&self, id: i32) -> bool;
}

struct NutriplanSqlite {
    conn_mgr: ConnMgrPool
}

impl NutriplanSqlite {
    pub fn new(database_path: &str) -> Self {
        let conn_mgr = setup_conn_mgr(database_path);
        NutriplanSqlite{ conn_mgr }
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

impl NutriplanDBInterface for NutriplanSqlite {
    fn create_ingredient(&self, item: NewIngredient) -> bool {
        CRUDIngredient::create(&self.conn_mgr, &item)
    }

    fn read_ingredient(&self, id: i32) -> Option<Ingredient> {
        CRUDIngredient::read(&self.conn_mgr, id)
    }
    fn update_ingredient(&self, item: Ingredient) -> bool {
        match item.id {
            Some(id) => CRUDIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete_ingredient(&self, id: i32) -> bool {
        CRUDIngredient::delete(&self.conn_mgr, id)
    }

    fn create_ingredient_macro(&self, item: NewIngredientMacro) -> bool {
        CRUDIngredientMacro::create(&self.conn_mgr, &item)
    }
    fn read_ingredient_macro(&self, id: i32) -> Option<IngredientMacro> {
        CRUDIngredientMacro::read(&self.conn_mgr, id)
    }
    fn update_ingredient_macro(&self, item: IngredientMacro) -> bool {
        match item.id {
            Some(id) => CRUDIngredientMacro::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete_ingredient_macro(&self, id: i32) -> bool {
        CRUDIngredientMacro::delete(&self.conn_mgr, id)
    }

    fn create_meal(&self, item: NewMeal) -> bool {
        CRUDMeal::create(&self.conn_mgr, &item)
    }
    fn read_meal(&self, id: i32) -> Option<Meal> {
        CRUDMeal::read(&self.conn_mgr, id)
    }
    fn update_meal(&self, item: Meal) -> bool {
        match item.id {
            Some(id) => CRUDMeal::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete_meal(&self, id: i32) -> bool {
        CRUDMeal::delete(&self.conn_mgr, id)
    }

    fn create_meal_ingredient(&self, item: NewMealIngredient) -> bool {
        CRUDMealIngredient::create(&self.conn_mgr, &item)
    }

    fn read_meal_ingredient(&self, id: i32) -> Option<MealIngredient> {
        CRUDMealIngredient::read(&self.conn_mgr, id)
    }

    fn update_meal_ingredient(&self, item: MealIngredient) -> bool {
        match item.id {
            Some(id) => CRUDMealIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }

    fn delete_meal_ingredient(&self, id: i32) -> bool {
        CRUDMealIngredient::delete(&self.conn_mgr, id)
    }

    fn create_recipe(&self, item: NewRecipe) -> bool {
        CRUDRecipe::create(&self.conn_mgr, &item)
    }

    fn read_recipe(&self, id: i32) -> Option<Recipe> {
        CRUDRecipe::read(&self.conn_mgr, id)
    }

    fn update_recipe(&self, item: Recipe) -> bool {
        match item.id {
            Some(id) => CRUDRecipe::update(&self.conn_mgr, id, item),
            None => false
        }
    }

    fn delete_recipe(&self, id: i32) -> bool {
        CRUDRecipe::delete(&self.conn_mgr, id)
    }

    fn create_recipe_ingredient(&self, item: NewRecipeIngredient) -> bool {
        CRUDRecipeIngredient::create(&self.conn_mgr, &item)
    }
    fn read_recipe_ingredient(&self, id: i32) -> Option<RecipeIngredient> {
        CRUDRecipeIngredient::read(&self.conn_mgr, id)
    }
    fn update_recipe_ingredient(&self, item: RecipeIngredient) -> bool {
        match item.id {
            Some(id) => CRUDRecipeIngredient::update(&self.conn_mgr, id, item),
            None => false
        }
    }
    fn delete_recipe_ingredient(&self, id: i32) -> bool {
        CRUDRecipeIngredient::delete(&self.conn_mgr, id)
    }
}

#[cfg(test)]
mod tests {

}
