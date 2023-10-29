mod with_hash_map;
mod with_num_list;
mod with_string;

fn main() {
    println!("\n****** with_num_list ******\n");
    with_num_list::main();
    println!("\n****** with_string ******\n");
    with_string::main();
    println!("\n****** with_hash_map ******\n");
    with_hash_map::main();
}
