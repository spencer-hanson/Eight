use crate::eight::expressions::secondary::SecondaryExpression;


#[derive(Debug)]
pub struct FuncArgs {
    pub(crate) arglist: Vec<SecondaryExpression>,
}

impl FuncArgs {
    pub fn new(v: Vec<SecondaryExpression>) -> Self {
        FuncArgs { arglist: v }
    }

    pub fn to_vec(self) -> Vec<SecondaryExpression> {
        return self.arglist;
    }
    pub fn get_arglist(&self) -> &Vec<SecondaryExpression> {
        return &self.arglist;
    }

    pub fn grab_arg(&mut self, idx: usize) -> SecondaryExpression {
        self.arglist.remove(idx)
    }

    pub fn empty() -> Self {
        FuncArgs { arglist: vec![] }
    }
}
