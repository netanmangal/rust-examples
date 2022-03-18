// mod table;

// fn main() {
//     table::print_table(12);
// }



// ---- Way 2 ----
// mod table;
// use crate::table::print_table;

// fn main() {
//     print_table(12);
// }


// ---- Way 3 ----
mod table {
    pub fn print_table (num: u32) {
        for i in 1..11 {
            println!("{} * {} = {}", num, i, num * i);
        }
    }
}

fn main() {
    table::print_table(15);
}