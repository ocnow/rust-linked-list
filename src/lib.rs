#[derive(Clone)]
pub struct Node{
    pub value : i32,
    next : Option<Box<Node>>
}

pub struct custom_list{
    head : Option<Node>
}

impl custom_list{

    pub fn new() -> custom_list{
        custom_list { 
            head: None 
        }
    }

    pub fn print_list(&self){
        println!("printing the list----");
        if(self.head.is_none()){
            println!("List is empty");
            return
        }

        let mut head = Some(self.head.as_ref().unwrap());
        while let Some(x) = head {
            println!("{}",x.value);
            head = x.next.as_deref();
        }
    }

    pub fn insert_node(&mut self,val : i32){
        //example node
        let ex = Node{
            value : val,
            next : None
        };

        if(self.head.is_none()){
            self.head = Some(ex);
            return
        }

        let mut head = self.head.as_mut().unwrap();  
        while let Some(_) = head.next{
            head = head.next.as_mut().unwrap();
        }

        head.next = Some(Box::new(ex));
    }

    pub fn get_node_value(&mut self,index : usize) -> Result<i32,String>{
        if(self.head.is_none()){
            return Err("List is empty".to_string());
        }

        if(index == 0){
            return Err("Invalid index".to_string());
        }

        let mut head = self.head.as_mut().unwrap();  
        for _ in 0..(index-1){
            println!("get-step is {}",head.value);

            head = match head.next.as_mut(){
               None => return Err("Invalid index".to_string()),
               Some(x) => x
            }
        }
        Ok(head.value)
    }

    // pub fn delete_node(&mut self,index : usize) -> Result<&Node, String>{
    //     if(self.head.is_none()){
    //         return Err("List is empty".to_string());
    //     }

    //     let mut head = &mut self.head.unwrap();  
    //     
    //     if(index == 1){
    //         return Ok(head)
    //     }

    //     for _ in 0..index{
    //         let k = head.next.as_mut().unwrap();
    //     }
    //     let k = head.next.as_mut().unwrap().next.as_ref().unwrap();
    // }
}

