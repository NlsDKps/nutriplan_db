CREATE TABLE ingredient_macros (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ingredient_id INTEGER NOT NULL,
    proteins FLOAT NOT NULL,
    carbs FLOAT NOT NULL,
    fats FLOAT NOT NULL,
    alcohols FLOAT NOT NULL,
    calories FLOAT NOT NULL
)
