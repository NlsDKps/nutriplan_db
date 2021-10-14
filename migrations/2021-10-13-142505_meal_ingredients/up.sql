-- Your SQL goes here
CREATE TABLE meal_ingredients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    meal_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    mass FLOAT NOT NULL
)
