use jiff::ToSpan;
use jiff::Zoned;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, PartialEq, Copy)]
enum Category {
    Food,
    Transport,
    Rent,
    Entertainment,
}

struct Expense {
    id: i32,
    amount: f64,
    category: Category,
    description: String,
    date: Zoned,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Category::Food => "Food",
            Category::Transport => "Transport",
            Category::Rent => "Rent",
            Category::Entertainment => "Entertainment",
        };
        write!(f, "{}", s)
    }
}

fn input() -> String {
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Failed");
    x.trim().to_string()
}
fn prompt(st: String) {
    print!("{}", st);
    io::stdout().flush().expect("Failed");
}

fn mk_expense(
    xpenselist: &mut Vec<Expense>,
    id: i32,
    amount: f64,
    category: Category,
    description: String,
    date: Zoned,
) {
    let expense = Expense {
        id,
        amount,
        category,
        description,
        date,
    };
    xpenselist.push(expense);
    println!("Successfully entered expense");
}

fn list_all_expenses(xpenselist: &Vec<Expense>) {
    let mut sum: f64 = 0.0;
    if xpenselist.len() > 0 {
        println!("Expenses");
        println!("--------------------");
        for i in xpenselist {
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            sum += i.amount;
        }
        println!("Sum: {}", sum)
    } else {
        println!("No expenses found, spoil yourself to get some ;)");
    }
}

fn list_expense_by_id(xpenselist: &Vec<Expense>, id: i32) {
    let mut found_one = false;
    for i in xpenselist {
        if i.id == id {
            println!("Expense");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
        }
    }
    if !found_one {
        println!("Couldnt find any expenses with id: {}", id);
    }
}

fn list_expense_by_amount(xpenselist: &Vec<Expense>, amount: f64) {
    let mut found_one = false;
    let mut sum = 0.0;
    for i in xpenselist {
        if i.amount == amount {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            sum += i.amount;
            found_one = true;
        }
    }
    if found_one {
        println!("Sum: {}", sum);
    }
    if !found_one {
        println!("Couldnt find any expenses with amount: {}", amount);
    }
}

fn list_expense_by_category(xpenselist: &Vec<Expense>, category: Category) {
    let mut sum = 0.0;
    let mut found_one = false;
    for i in xpenselist {
        if i.category == category {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
            sum += i.amount;
        }
    }

    if found_one {
        println!("Sum: {}", sum);
    }

    if !found_one {
        println!("No expense found with category: {}", category);
    }
}

fn list_expense_by_desc(xpenselist: &Vec<Expense>, description: String) {
    let mut found_one = false;
    let mut sum = 0.0;
    for i in xpenselist {
        if i.description.contains(&description) {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
            sum += i.amount;
        }
    }
    if found_one {
        println!("Sum: {}", sum);
    }
    if !found_one {
        println!(
            "Couldnt find any expenses containing any keywords such as {} in their description",
            description
        );
    }
}

fn list_last_7_days(xpenselist: &Vec<Expense>) {
    let now = Zoned::now();
    let mut sum = 0.0;
    let cutoff = (&now - 7.days());
    let mut found_one = false;
    for i in xpenselist {
        if i.date >= cutoff {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
            sum += i.amount;
        }
    }
    if found_one {
        println!("Sum: {}", sum);
    }
    if !found_one {
        println!("No expenses found on the last 7 days");
    }
}
fn list_last_month(xpenselist: &Vec<Expense>) {
    let mut sum = 0.0;
    let now = Zoned::now();
    let cutoff = (&now - 1.month());
    let mut found_one = false;
    for i in xpenselist {
        if i.date >= cutoff {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
            sum += i.amount;
        }
    }
    if found_one {
        println!("Sum: {}", sum);
    }
    if !found_one {
        println!("No expenses found on the last month");
    }
}

fn list_last_year(xpenselist: &Vec<Expense>) {
    let mut sum = 0.0;
    let now = Zoned::now();
    let cutoff = (&now - 1.year());
    let mut found_one = false;
    for i in xpenselist {
        if i.date >= cutoff {
            println!("Expense(s)");
            println!("--------------------");
            println!("Id: {}", i.id);
            println!("Amount: {}", i.amount);
            println!("Category: {}", i.category);
            println!("Description: {}", i.description);
            println!("Date: {}", i.date);
            println!("--------------------");
            found_one = true;
            sum += i.amount;
        }
    }
    if found_one {
        println!("Sum: {}", sum);
    }
    if !found_one {
        println!("No expenses found on the last year");
    }
}

fn sum(xpenselist: &Vec<Expense>) {
    let mut sum = 0.0;
    for i in xpenselist {
        sum += i.amount;
    }
    println!("Sum: {}", sum);
}

fn delete_all(xpenselist: &mut Vec<Expense>) {
    if xpenselist.len() > 0 {
        xpenselist.clear();
        println!("Successfully removed all expenses");
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_id(xpenselist: &mut Vec<Expense>, id: i32) {
    let mut found_one = false;
    for i in 0..xpenselist.len() {
        if xpenselist[i].id == id {
            xpenselist.remove(i);
            found_one = true;
            break;
        }
    }
    if !found_one {
        println!("No expense found with id: {}", id)
    }
}

fn delete_by_amount(xpenselist: &mut Vec<Expense>, amount: f64) {
    let oldlen = xpenselist.len();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| t.amount != amount);
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found with amount {}", amount);
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_category(xpenselist: &mut Vec<Expense>, category: Category) {
    let oldlen = xpenselist.len();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| t.category != category);
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found with category {}", category);
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_desc(xpenselist: &mut Vec<Expense>, description: String) {
    let oldlen = xpenselist.len();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| !t.description.contains(&description));
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found containing {}", description);
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_week(xpenselist: &mut Vec<Expense>) {
    let oldlen = xpenselist.len();
    let now = Zoned::now();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| t.date >= (&now - 7.days()));
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found in 7 days");
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_month(xpenselist: &mut Vec<Expense>) {
    let oldlen = xpenselist.len();
    let now = Zoned::now();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| t.date >= (&now - 1.month()));
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found in a month");
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn delete_by_year(xpenselist: &mut Vec<Expense>) {
    let oldlen = xpenselist.len();
    let now = Zoned::now();
    if xpenselist.len() > 0 {
        xpenselist.retain(|t| t.date >= (&now - 1.year()));
        let lol = oldlen - xpenselist.len();
        match lol {
            0 => {
                println!("No expenses found in a year");
            }
            _ => {
                println!("Successfully removed {} expenses", lol);
            }
        }
    } else {
        println!("There are already no expenses");
    }
}

fn editor(xpenselist: &mut Vec<Expense>) {
    if xpenselist.len() != 0 {
        println!(
            "Welcome to edit mode, commands are limited here, you can still enter ls* though, enter ch-id to change the selected id, by default its 0 so be careful, enter help to see the commands"
        );
        let mut chs: i32 = 0;
        'editorLoop: loop {
            prompt("editor>> ".to_string());
            let inp = input();
            let rinp: &str = &inp;

            match rinp {
                "ls*" => {
                    list_all_expenses(xpenselist);
                }

                "exit" => {
                    break 'editorLoop;
                }

                "help" => {
                    println!("           Xpense-RS-1-Editor-Commands");
                    println!("--------------------------------------------------");
                    println!("ls*       - list all expenses");
                    println!("ch-id     - change the selected id");
                    println!("edit-am   - edit amount of selected expense");
                    println!("edit-ctg  - edit category of selected expense");
                    println!("edit-desc - edit description of selected expense");
                    println!("exit      - leave edit mode");
                    println!("--------------------------------------------------");
                }

                "ch-id" => {
                    prompt("enter id to change the selected id: ".to_string());
                    let chidinpraw = input();
                    let nwid: i32 = chidinpraw.parse().unwrap();
                    chs = nwid;
                    println!("Successfully changed the selected id to {}", chs);
                }

                "edit-am" => {
                    prompt("enter new amount: ".to_string());
                    let strid = input();
                    let am: f64 = strid.parse().unwrap();
                    xpenselist[chs as usize].amount = am;
                    println!("Successfully changed amount to {}", am);
                }

                "edit-ctg" => 'innerLoop: loop {
                    prompt("enter new category (f: food, t: transportation, r: rent, e: entertainment): ".to_string());
                    let strid = input();
                    let inp: &str = &strid;
                    match inp {
                        "f" => {
                            xpenselist[chs as usize].category = Category::Food;
                            println!("Successfully changed category to food");
                            break 'innerLoop;
                        }
                        "t" => {
                            xpenselist[chs as usize].category = Category::Transport;
                            println!("Successfully changed category to transport");
                            break 'innerLoop;
                        }
                        "r" => {
                            xpenselist[chs as usize].category = Category::Rent;
                            println!("Successfully changed category to rent");
                            break 'innerLoop;
                        }
                        "e" => {
                            xpenselist[chs as usize].category = Category::Entertainment;
                            println!("Successfully changed category to entertainment");
                            break 'innerLoop;
                        }
                        _ => {
                            println!("No category as {}", inp);
                        }
                    }
                },

                "edit-desc" => {
                    prompt("enter new desc: ".to_string());
                    let strinp = input();
                    xpenselist[chs as usize].description = strinp;
                    println!("Successfully changed description");
                }

                _ => {
                    println!("Command not found: {}", rinp);
                }
            }
        }
    } else {
        println!("There are no expenses so how do u think u'll edit one?");
    }
}

fn banner() {
    println!(
        r###"
        ___    ___ ________  _______   ________   ________  _______                  ________  ________    _____
       |\  \  /  /|\   __  \|\  ___ \ |\   ___  \|\   ____\|\  ___ \                |\   __  \|\   ____\  / __  \
       \ \  \/  / | \  \|\  \ \   __/|\ \  \\ \  \ \  \___|\ \   __/|   ____________\ \  \|\  \ \  \___|_|\/_|\  \
        \ \    / / \ \   ____\ \  \_|/_\ \  \\ \  \ \_____  \ \  \_|/__|\____________\ \   _  _\ \_____  \|/ \ \  \
         /     \/   \ \  \___|\ \  \_|\ \ \  \\ \  \|____|\  \ \  \_|\ \|____________|\ \  \\  \\|____|\  \   \ \  \
        /  /\   \    \ \__\    \ \_______\ \__\\ \__\____\_\  \ \_______\              \ \__\\ _\ ____\_\  \   \ \__\
       /__/ /\ __\    \|__|     \|_______|\|__| \|__|\_________\|_______|               \|__|\|__|\_________\   \|__|
       |__|/ \|__|                                  \|_________|                                 \|_________|
       Made by agcar8940-cloud on github
       V1.0
       type help to see the commands
    "###
    )
}

fn main() {
    banner();
    let mut xpenselist: Vec<Expense> = Vec::new();
    let mut id = 0;
    'mainLoop: loop {
        prompt("xpense-rs1>> ".to_string());
        let rawinp = input();
        let inp = rawinp.trim();
        match inp {
            "exit" => {
                break 'mainLoop;
            }

            "mkxp" => {
                prompt("enter amount: ".to_string());
                let rwamount = input();
                let amount: f64 = rwamount.parse().unwrap();

                let category;
                'ctgrLoop: loop {
                    prompt("enter category (f for food, t for transport, r for rent, e for entertainment): ".to_string());
                    let ctgr = input();
                    let rctgr: &str = &ctgr;

                    match rctgr {
                        "f" => {
                            category = Category::Food;
                            break 'ctgrLoop;
                        }
                        "t" => {
                            category = Category::Transport;
                            break 'ctgrLoop;
                        }
                        "r" => {
                            category = Category::Rent;
                            break 'ctgrLoop;
                        }
                        "e" => {
                            category = Category::Entertainment;
                            break 'ctgrLoop;
                        }

                        _ => {
                            println!("{} is not a valid category", rctgr);
                        }
                    }
                }
                prompt("Enter description: ".to_string());
                let dsc = input();
                let desc = dsc.to_string();
                let now = Zoned::now();
                mk_expense(&mut xpenselist, id, amount, category, desc, now);
                id += 1;
            }

            "ls*" => {
                list_all_expenses(&xpenselist);
            }

            "help" => {
                println!("            Xpense-RS-1 Commands");
                println!("--------------------------------------------------");
                println!("mkxp    - create a new expense");
                println!("ls*     - list all expenses");
                println!("lsid    - list expense(s) by id");
                println!("lsam    - list expense(s) by amount");
                println!("lsctg   - list expense(s) by category");
                println!("lsdsc   - list expense(s) by description");
                println!("ls7     - list expenses from the last 7 days");
                println!("ls30    - list expenses from the last month");
                println!("ls365   - list expenses from the last year");
                println!("sum     - show total sum of all expenses");
                println!("rm*     - delete all expenses");
                println!("rm      - delete expense by id");
                println!("rmam    - delete expense(s) by amount");
                println!("rmctg   - delete expense(s) by category");
                println!("rmdsc   - delete expense(s) by description keyword");
                println!("rm7     - delete expenses from the last 7 days");
                println!("rm30    - delete expenses from the last month");
                println!("rm365   - delete expenses from the last year");
                println!("editor  - enter edit mode to modify an expense");
                println!("exit    - quit the program");
                println!("--------------------------------------------------");
            }

            "lsid" => {
                prompt("enter id: ".to_string());
                let rawinpp = input();
                let inpi: i32 = rawinpp.parse().unwrap();
                list_expense_by_id(&xpenselist, inpi);
            }

            "lsam" => {
                prompt("enter amount: ".to_string());
                let rawinpp = input();
                let inpi: f64 = rawinpp.parse().unwrap();
                list_expense_by_amount(&xpenselist, inpi);
            }

            "dbg" => {
                let descs = ["a", "b", "c", "d", "e"];
                let amounts = [1.0, 10.0, 34.0, 37.0, 44.0];
                let categories = [
                    Category::Entertainment,
                    Category::Food,
                    Category::Rent,
                    Category::Transport,
                    Category::Food,
                ];

                for i in 0..5 {
                    mk_expense(
                        &mut xpenselist,
                        id,
                        amounts[i as usize],
                        categories[i as usize],
                        descs[i as usize].to_string(),
                        Zoned::now(),
                    );
                    id += 1;
                }
            }
            "lsctg" => 'innerLoop: loop {
                prompt(
                    "enter category (f: food, t: transportation, r: rent, e: entertainment): "
                        .to_string(),
                );
                let strid = input();
                let inp: &str = &strid;
                match inp {
                    "f" => {
                        list_expense_by_category(&xpenselist, Category::Food);
                        break 'innerLoop;
                    }

                    "t" => {
                        list_expense_by_category(&xpenselist, Category::Transport);

                        break 'innerLoop;
                    }

                    "r" => {
                        list_expense_by_category(&xpenselist, Category::Rent);
                        break 'innerLoop;
                    }

                    "e" => {
                        list_expense_by_category(&xpenselist, Category::Entertainment);

                        break 'innerLoop;
                    }

                    _ => {
                        println!("No category as {}", inp);
                    }
                }
            },

            "lsdsc" => {
                prompt("enter description: ".to_string());
                let strinp = input();
                list_expense_by_desc(&xpenselist, strinp);
            }

            "ls7" => {
                list_last_7_days(&xpenselist);
            }

            "ls30" => {
                list_last_month(&xpenselist);
            }

            "ls365" => {
                list_last_year(&xpenselist);
            }

            "sum" => {
                sum(&xpenselist);
            }

            "rm*" => {
                delete_all(&mut xpenselist);
            }

            "rm" => {
                prompt("enter id: ".to_string());
                let strinp = input();
                let idd: i32 = strinp.parse().unwrap();
                delete_by_id(&mut xpenselist, idd);
            }

            "rmam" => {
                prompt("enter amount: ".to_string());
                let strinp = input();
                let amm: f64 = strinp.parse().unwrap();
                delete_by_amount(&mut xpenselist, amm);
            }

            "rmctg" => 'innerLoop: loop {
                prompt(
                    "enter category (f: food, t: transportation, r: rent, e: entertainment): "
                        .to_string(),
                );
                let strid = input();
                let inp: &str = &strid;
                match inp {
                    "f" => {
                        delete_by_category(&mut xpenselist, Category::Food);
                        break 'innerLoop;
                    }
                    "t" => {
                        delete_by_category(&mut xpenselist, Category::Transport);
                        break 'innerLoop;
                    }
                    "r" => {
                        delete_by_category(&mut xpenselist, Category::Rent);
                        break 'innerLoop;
                    }
                    "e" => {
                        delete_by_category(&mut xpenselist, Category::Entertainment);
                        break 'innerLoop;
                    }
                    _ => {
                        println!("No category as {}", inp);
                    }
                }
            },

            "rmdsc" => {
                prompt("enter description: ".to_string());
                let aainp = input();
                let pinp = aainp.to_string();
                delete_by_desc(&mut xpenselist, pinp);
            }

            "rm7" => {
                delete_by_week(&mut xpenselist);
            }

            "rm30" => {
                delete_by_month(&mut xpenselist);
            }

            "rm365" => {
                delete_by_year(&mut xpenselist);
            }

            "editor" => {
                editor(&mut xpenselist);
            }

            _ => {
                println!("Command not found: {}", inp);
            }
        }
    }
}
