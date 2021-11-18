use std::env::args;

#[derive(Debug, PartialEq)]
enum CmdPattern {
    Unknown,
    Ingredient,
    IngredientMacro,
    Meal,
    MealIngredient,
    Recipe,
    RecipeIngredient,
    Help,
}

#[derive(Debug, PartialEq)]
enum SubCmdPattern {
    Unknown,
    Create,
    Read,
    Update,
    Delete,
    Help
}

fn usage() {
    println!("Usage: {} <cmd> <subcmd> ", args().nth(0).unwrap());
    println!("Where cmd holds the table to insert to:");
    println!("\t* ingredient\t* ingredient_macro");
    println!("\t* meal\t\t* meal_ingredient");
    println!("\t* recipe\t* recipe_ingredient");
    println!("\t* help");
    println!("And cmd is one of");
    println!("\t* create\t* read");
    println!("\t* update\t* delete");
    println!("For each command a help method is available, which gives a deeper understanding.");
}

fn decode_cmd(cmd_str: &str) -> CmdPattern {
    match cmd_str {
        "ingredient" => CmdPattern::Ingredient,
        "ingredient_macro" => CmdPattern::IngredientMacro,
        "meal" => CmdPattern::Meal,
        "meal_ingredient" => CmdPattern::MealIngredient,
        "recipe" => CmdPattern::Recipe,
        "recipe_ingredient" => CmdPattern::RecipeIngredient,
        "help" => CmdPattern::Help,
        _ => CmdPattern::Unknown
    }
}

fn decode_subcmd(subcmd_str: &str) -> SubCmdPattern {
    match subcmd_str {
        "create" => SubCmdPattern::Create,
        "read" => SubCmdPattern::Read,
        "update" => SubCmdPattern::Update,
        "delete" => SubCmdPattern::Delete,
        "help" => SubCmdPattern::Help,
        _ => SubCmdPattern::Unknown
    }
}

fn decode_i32(pos: usize) -> i32 {
    match args().nth(pos) {
        Some(id) => {
            match id.parse::<i32>() {
                Ok(id) => id,
                Err(e) => {
                    panic!("Could not parse id: {}", e)
                }
            }
        }
        None => {
            usage();
            panic!("No argument provided on position {}", pos)
        }
    }
}

fn decode_f32(pos: usize) -> f32 {
    match args().nth(pos) {
        Some(id) => {
            match id.parse::<f32>() {
                Ok(id) => id,
                Err(e) => {
                    panic!("Could not parse id: {}", e)
                }
            }
        }
        None => {
            usage();
            panic!("No argument provided on position {}", pos)
        }
    }
}

fn decode_string(pos: usize) -> String {
    match args().nth(pos) {
        Some(name) => name,
        None => {
            usage();
            panic!("No argument provided on position {}", pos)
        }
    }
}

fn decode_date(pos: usize) -> chrono::NaiveDate {
    match args().nth(pos) {
        Some(s) => match chrono::NaiveDate::parse_from_str(&s, "%Y-%M-%D") {
            Ok(date) => date,
            Err(e) => panic!("Could not parse date: {}", e)
        }
        None => {
            usage();
            panic!("No argument provided on position {}", pos)
        }
    }
}

fn decode_time(pos: usize) -> chrono::NaiveTime {
    match args().nth(pos) {
        Some(s) => match chrono::NaiveTime::parse_from_str(&s, "%H:%M") {
            Ok(time) => time,
            Err(e) => panic!("Could not parse time: {}", e)
        },
        None => {
            usage();
            panic!("No argument provided on position {}", pos)
        }
    }
}

use nutriplan_db_sqlite::INutriplanDbIngredient;
use nutriplan_db_sqlite::INutriplanDbIngredientMacro;
use nutriplan_db_sqlite::INutriplanDbMeal;
use nutriplan_db_sqlite::INutriplanDbMealIngredient;
use nutriplan_db_sqlite::INutriplanDbRecipe;
use nutriplan_db_sqlite::INutriplanDbRecipeIngredient;
use nutriplan_db_sqlite::controller::database::ingredient::NewIngredient;
use nutriplan_db_sqlite::controller::database::ingredient::Ingredient;
use nutriplan_db_sqlite::controller::database::ingredient_macro::NewIngredientMacro;
use nutriplan_db_sqlite::controller::database::ingredient_macro::IngredientMacro;
use nutriplan_db_sqlite::controller::database::meal::NewMeal;
use nutriplan_db_sqlite::controller::database::meal::Meal;
use nutriplan_db_sqlite::controller::database::meal_ingredient::NewMealIngredient;
use nutriplan_db_sqlite::controller::database::meal_ingredient::MealIngredient;
use nutriplan_db_sqlite::controller::database::recipe::NewRecipe;
use nutriplan_db_sqlite::controller::database::recipe::Recipe;
use nutriplan_db_sqlite::controller::database::recipe_ingredient::NewRecipeIngredient;
use nutriplan_db_sqlite::controller::database::recipe_ingredient::RecipeIngredient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = match args().nth(1) {
        Some(cmd) => decode_cmd(cmd.as_str()),
        None => CmdPattern::Unknown
    };
    let subcmd = match args().nth(2) {
        Some(subcmd) => decode_subcmd(subcmd.as_str()),
        None => SubCmdPattern::Unknown
    };
    if subcmd == SubCmdPattern::Unknown {
        usage()
    }
    match cmd {
        CmdPattern::Ingredient => {
            let ingredient_db = nutriplan_db_sqlite::NutriplanSqliteDbIngredient::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let name = match args().nth(3) {
                        Some(name) => name,
                        None => {
                            usage();
                            return Ok(())
                        }
                    };
                    let ingredient = NewIngredient::new(&name);
                    match ingredient_db.create(ingredient) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let ingredient = match ingredient_db.read(id) {
                        Some(ingredient) => ingredient,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found ingredient with id {}, and name {}", id, ingredient.name);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let name = decode_string(4);
                    let ingredient = Ingredient::new(id, &name);
                    match ingredient_db.update(ingredient) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match ingredient_db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::IngredientMacro => {
            let db = nutriplan_db_sqlite::NutriplanSqliteDbIngredientMacro::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let iid = decode_i32(3);
                    let proteins = decode_f32(4);
                    let carbs = decode_f32(5);
                    let fats = decode_f32(6);
                    let alcohols = decode_f32(7);
                    let item = NewIngredientMacro::new(iid, proteins, carbs, fats, alcohols);
                    match db.create(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let item = match db.read(id) {
                        Some(item) => item,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found ingredient macro with id {}", id);
                    println!("\tproteins: {}", item.proteins);
                    println!("\tcarbs: {}", item.carbs);
                    println!("\tfats: {}", item.fats);
                    println!("\talcohols: {}", item.alcohols);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let iid = decode_i32(4);
                    let proteins = decode_f32(5);
                    let carbs = decode_f32(6);
                    let fats = decode_f32(7);
                    let alcohols = decode_f32(8);
                    let item = IngredientMacro::new(id, iid, proteins, carbs, fats, alcohols);
                    match db.update(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::Meal => {
            let db = nutriplan_db_sqlite::NutriplanSqliteDbMeal::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let name = decode_string(3);
                    let date = decode_date(4);
                    let time = decode_time(5);
                    let item = NewMeal::new(&name, &date, &time);
                    match db.create(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let item = match db.read(id) {
                        Some(item) => item,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found item with id {}", id);
                    println!("\tname: {}", item.name);
                    println!("\tdate: {}", item.date);
                    println!("\ttime: {}", item.time);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let name = decode_string(4);
                    let date = decode_date(5);
                    let time = decode_time(6);
                    let item = Meal::new(id, &name, &date, &time);
                    match db.update(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::MealIngredient => {
            let db = nutriplan_db_sqlite::NutriplanSqliteDbMealIngredient::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let mid = decode_i32(3);
                    let iid = decode_i32(4);
                    let mass = decode_i32(5);
                    let item = NewMealIngredient::new(mid, iid, mass);
                    match db.create(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let item = match db.read(id) {
                        Some(item) => item,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found item with id {}", id);
                    println!("\tdate: {}", item.meal_id);
                    println!("\tname: {}", item.ingredient_id);
                    println!("\ttime: {}", item.mass);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let mid = decode_i32(4);
                    let iid = decode_i32(5);
                    let mass = decode_i32(6);
                    let item = MealIngredient::new(id, mid, iid, mass);
                    match db.update(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::Recipe => {
            let db = nutriplan_db_sqlite::NutriplanSqliteDbRecipe::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let name = decode_string(3);
                    let description = decode_string(4);
                    let item = NewRecipe::new(&name, &description);
                    match db.create(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let item = match db.read(id) {
                        Some(item) => item,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found item with id {}", id);
                    println!("\tname: {}", item.name);
                    println!("\tdescription: {}", item.description);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let name = decode_string(4);
                    let description = decode_string(5);
                    let item = Recipe::new(id, &name, &description);
                    match db.update(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::RecipeIngredient => { 
            let db = nutriplan_db_sqlite::NutriplanSqliteDbRecipeIngredient::new("nutriplan.db");
            match subcmd {
                SubCmdPattern::Create => {
                    let mid = decode_i32(3);
                    let iid = decode_i32(4);
                    let mass = decode_i32(5);
                    let item = NewRecipeIngredient::new(mid, iid, mass);
                    match db.create(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Read => {
                    let id = decode_i32(3);
                    let item = match db.read(id) {
                        Some(item) => item,
                        None => {
                            println!("No ingredient with id {} found", id);
                            return Ok(())
                        }
                    };
                    println!("Found item with id {}", id);
                    println!("\tdate: {}", item.recipe_id);
                    println!("\tname: {}", item.ingredient_id);
                    println!("\ttime: {}", item.mass);
                },
                SubCmdPattern::Update => {
                    let id = decode_i32(3);
                    let rid = decode_i32(4);
                    let iid = decode_i32(5);
                    let mass = decode_i32(6);
                    let item = RecipeIngredient::new(id, rid, iid, mass);
                    match db.update(item) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }

                },
                SubCmdPattern::Delete => {
                    let id = decode_i32(3);
                    match db.delete(id) {
                        true => println!("Success"),
                        false => println!("Failure")
                    }
                },
                SubCmdPattern::Unknown => usage(),
                SubCmdPattern::Help => usage(),
            }
        }
        CmdPattern::Help => { 
            usage()
        }
        CmdPattern::Unknown => { 
            usage()
        }
    }
    Ok(())
}
