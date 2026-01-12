use crate::token_type;

struct Scanner{
    source:String,
    // tokens=Vec::new()
}

impl Scanner{
    fn new(&mut self,source:String){
        self.source=source;
    }
}