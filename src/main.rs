use linkedList::{custom_list};

fn main() {
    let mut linkl = custom_list::new();
    
    linkl.print_list();
    linkl.insert_node(10);
    linkl.insert_node(20);
    linkl.insert_node(30);
    linkl.print_list();
    linkl.insert_node(40);

    let k = linkl.get_node_value(9);
    match k {
        Ok(x) => println!("value we got is {}",x),
        Err(x) => println!("Err we got is {}",x),
    };  
    linkl.insert_node(50);
    linkl.print_list();
}
