use crate::eight::expressions::primary::Expression;
use crate::eight::expressions::primary::ExpressionRelations;
use log::{debug, trace};


pub struct EightAST {
    exprs: Vec<Expression>,
}

impl EightAST {
    pub fn new(v: Vec<Expression>) -> Self {
        EightAST { exprs: v }
    }

    pub fn expand_relation_entry(&self, rel: &mut RelationEntry) -> Vec<RelationEntry> {
        let mut rels = Vec::new();

        if rel.is_empty() {
            return rels;
        }

        let mut crs = vec![];
        let mut refs = vec![];
        crs.append(&mut rel.create);
        refs.append(&mut rel.reference);

        rels.push(RelationEntry::new(crs, refs, vec![]));

        for entry in rel.entries.iter_mut() {
            let mut rel_expanded = self.expand_relation_entry(entry);
            for _ in 0..rel_expanded.len() {
                let t = rel_expanded.pop().unwrap();
                if !t.is_empty() {
                    rels.push(t);
                }
            }
        }
        rels
    }

    pub fn reduce_relation_entry(&self, rel: &mut RelationEntry) -> RelationEntry {
        let mut r = RelationEntry::new(vec![], vec![], vec![]);
        r.create.append(&mut rel.create);
        r.reference.append(&mut rel.reference);

        for rr in rel.entries.iter_mut() {
            let mut entry = self.reduce_relation_entry(rr);
            r.create.append(&mut entry.create);
            r.reference.append(&mut entry.reference);
        }

        r
    }

    pub fn get_first_order_relations(
        &self,
        orig_exprs: &Vec<Vec<RelationEntry>>,
    ) -> Vec<Vec<Option<RelationEntry>>> {
        // Get the expressions that only have a create field defined, with no reference or sub-entries
        let mut first_order = Vec::new();
        for vec_expr in orig_exprs {
            let mut sub_relations = Vec::new();

            for expr in vec_expr {
                if expr.create.len() > 0 && expr.reference.is_empty() && expr.entries.is_empty() {
                    sub_relations.push(Some(expr.clone()));
                } else {
                    sub_relations.push(None);
                }
            }
            first_order.push(sub_relations);
        }
        return first_order;
    }

    pub fn optimize_ast(&mut self) -> Vec<Vec<Expression>> {
        // TODO optimize stats here
        debug!("----------OPTIMIZE AST START----------");
        let mut relation_entries: Vec<Box<RelationEntry>> = Vec::new();
        let mut orig_exprs: Vec<Vec<RelationEntry>> = Vec::new();
        let mut expanded_exprs: Vec<Vec<RelationEntry>> = Vec::new();
        let mut reduced_exprs: Vec<RelationEntry> = Vec::new();
        for expr in &self.exprs {
            let mut reduce_expr_rels = expr.get_expr_references();
            let mut expand_expr_rels = expr.get_expr_references();

            orig_exprs.push(expr.get_expr_references());

            for _ in 0..reduce_expr_rels.len() {
                // Reduced
                let mut reduc = reduce_expr_rels.pop().unwrap();
                relation_entries.push(Box::from(reduc.clone()));
                reduced_exprs.push(self.reduce_relation_entry(&mut reduc));

                // Expanded
                let mut expd = expand_expr_rels.pop().unwrap();
                expanded_exprs.push(self.expand_relation_entry(&mut expd));
            }
        }

        let reduced_entry =
            self.reduce_relation_entry(&mut RelationEntry::new(vec![], vec![], relation_entries));
        let available_refs = reduced_entry.create;

        trace!("Created {:?}", available_refs);

        trace!("Orig");
        for rr in &orig_exprs {
            trace!("{:?}", rr);
        }

        trace!("Reduced");
        for rr in &reduced_exprs {
            trace!("{:?}", rr);
        }

        trace!("Expanded");
        for rr in &expanded_exprs {
            trace!("{:?}", rr);
        }

        let first_order_rels = self.get_first_order_relations(&orig_exprs);
        // let mut first_order_exprs = Vec::new();
        let mut first_order_idxs = Vec::new();

        trace!("First Order Expression Relations:");
        for i in 0..first_order_rels.len() {
            trace!("{:?} -> {:?}", self.exprs[i], first_order_rels[i]);
        }

        for i in 0..first_order_rels.len() {
            for j in 0..first_order_rels[i].len() {
                match first_order_rels[i][j] {
                    Some(_) => first_order_idxs.push(i),
                    None => (),
                }
            }
        }

        trace!("First order idxs: '{:?}'", first_order_idxs);

        // for i in (0..first_order_idxs.len()).rev() {
        //     println!("Removing at index: '{}'", i);
        //     first_order_exprs.push(self.exprs.remove(first_order_idxs[i]));
        // }

        trace!("First order Expressions:");
        // for rr in &first_order_exprs {
        //     println!("{:?}", rr);
        // }

        trace!("Leftover Expressions:");
        for rr in &self.exprs {
            trace!("{:?}", rr);
        }
        // TODO Finish optimization
        let mut v = Vec::new();
        let mut vv = Vec::new();

        for d in 0..self.exprs.len() {
            vv.push(self.exprs.pop().unwrap());
        }
        vv.reverse();
        v.push(vv);
        debug!("----------OPTIMIZE AST END----------");
        return v;
    }
}

#[derive(Debug)]
pub struct RelationEntry {
    create: Vec<String>,                         // create at this entry
    pub(crate) reference: Vec<String>,           // reference at this entry
    pub(crate) entries: Vec<Box<RelationEntry>>, // Other sub-entries, don't include their data in this node
}

impl RelationEntry {
    pub fn new(
        create: Vec<String>,
        reference: Vec<String>,
        entrs: Vec<Box<RelationEntry>>,
    ) -> Self {
        let mut entries = Vec::new();
        for en in entrs {
            if en.entries.len() > 0 || en.create.len() > 0 || en.reference.len() > 0 {
                entries.push(en)
            }
        }
        RelationEntry {
            create,
            reference,
            entries,
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.create.len() == 0 && self.entries.len() == 0 && self.reference.len() == 0 {
            return true;
        }
        return false;
    }

    pub fn clone(&self) -> Self {
        let cr = self.create.clone();
        let refer = self.reference.clone();
        let mut ents = Vec::new();

        for ent in &self.entries {
            ents.push(Box::from((*ent).clone()));
        }

        RelationEntry {
            create: cr,
            reference: refer,
            entries: ents,
        }
    }
}
