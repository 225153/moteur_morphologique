
pub struct Derive {
    pub mot: String,
    pub schema: String,
}

pub struct RacineNode {
    pub racine: [char; 3],              
    pub left: Option<Box<RacineNode>>,  
    pub right: Option<Box<RacineNode>>, 
}


pub fn morphologic_cmp(tree_racine : [char;3], racine : [char; 3])-> i8 {
    let mut i = 0 ; 
    while i < 3 {
        if tree_racine[i] == racine[i] {
            i = i + 1 ;
        }
        else {
            if tree_racine[i] > racine[i]{
                return -1 ;
            }
            else {
                return 1 ;
            }
        }
    }
    return 0 ;
}


pub struct Tree{Option<Box<RacineNode>>};
impl RacineNode{
    pub fn new(racine: [char; 3]) -> Self {
        RacineNode{
            racine,
            left: none ,
            right : none,
        }
    }
}
impl Tree {
    pub fn new() -> Self {
        Tree { racine: None }
    }
    pub fn verify()
}
