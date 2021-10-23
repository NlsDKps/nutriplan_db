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
