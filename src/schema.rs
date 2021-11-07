table! {
    ingredient_macros (id) {
        id -> Nullable<Integer>,
        ingredient_id -> Integer,
        proteins -> Float,
        carbs -> Float,
        fats -> Float,
        alcohols -> Float,
        calories -> Float,
    }
}

table! {
    ingredients (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    meal_ingredients (id) {
        id -> Nullable<Integer>,
        meal_id -> Integer,
        ingredient_id -> Integer,
        mass -> Integer,
    }
}

table! {
    meals (id) {
        id -> Nullable<Integer>,
        name -> Text,
        date -> Date,
        time -> Time,
    }
}

table! {
    recipe_ingredients (id) {
        id -> Nullable<Integer>,
        recipe_id -> Integer,
        ingredient_id -> Integer,
        mass -> Integer,
    }
}

table! {
    recipes (id) {
        id -> Nullable<Integer>,
        name -> Text,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredient_macros,
    ingredients,
    meal_ingredients,
    meals,
    recipe_ingredients,
    recipes,
);
