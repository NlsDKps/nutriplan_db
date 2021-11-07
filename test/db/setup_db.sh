timeout=0

sqlite3 test.db <<EOF
CREATE TABLE IF NOT EXISTS ingredients (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO ingredients (name) VALUES ("test1");
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO ingredients (name) VALUES ("test2");
EOF

sqlite3 test.db <<EOF
CREATE TABLE IF NOT EXISTS ingredient_macros (id INTEGER PRIMARY KEY AUTOINCREMENT, ingredient_id INTEGER NOT NULL, proteins FLOAT NOT NULL, carbs FLOAT NOT NULL, fats FLOAT NOT NULL, alcohols FLOAT NOT NULL, calories FLOAT NOT NULL);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO ingredient_macros (ingredient_id, proteins, carbs, fats, alcohols, calories) VALUES (1, 1.0, 1.0, 1.0, 1.0, 1.0);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO ingredient_macros (ingredient_id, proteins, carbs, fats, alcohols, calories) VALUES (2, 2.0, 2.0, 2.0, 2.0, 2.0);
EOF

sqlite3 test.db <<EOF
CREATE TABLE IF NOT EXISTS meals (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL, date
DATE NOT NULL, time TIME NOT NULL);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO meals (name, date, time) VALUES ("testmeal1", "2000-01-01", "08:00:00");
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO meals (name, date, time) VALUES ("testmeal2", "2000-02-02", "20:00:00");
EOF

sqlite3 test.db <<EOF
CREATE TABLE meal_ingredients ( \
    id INTEGER PRIMARY KEY AUTOINCREMENT, \
    meal_id INTEGER NOT NULL, \
    ingredient_id INTEGER NOT NULL, \
    mass INTEGER NOT NULL \
)
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO meal_ingredients (meal_id, ingredient_id, mass)\
    VALUES (1, 1, 111);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO meal_ingredients (meal_id, ingredient_id, mass)\
    VALUES (2, 2, 222);
EOF

sqlite3 test.db <<EOF
CREATE TABLE recipes (\
    id INTEGER PRIMARY KEY AUTOINCREMENT,\
    name TEXT NOT NULL,\
    description TEXT NOT NULL\
)
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO recipes (name, description)\
    VALUES ("testitem1", "testdescription1");
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO recipes (name, description)\
    VALUES ("testitem2", "testdescription2");
EOF

sqlite3 test.db <<EOF
CREATE TABLE recipe_ingredients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    recipe_id INTEGER NOT NULL,
    ingredient_id INTEGER NOT NULL,
    mass INTEGER NOT NULL
)
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO recipe_ingredients (recipe_id, ingredient_id, mass)\
    VALUES (1, 1, 111);
EOF

sqlite3 test.db <<EOF
.timeout $timeout
INSERT INTO recipe_ingredients (recipe_id, ingredient_id, mass)\
    VALUES (2, 2, 222);
EOF

