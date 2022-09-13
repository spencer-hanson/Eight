// // use crate::eight::common::context::{Context};
// // use std::process::exit;
// // use crate::eight::valarative;
// // use crate::eight::tokens::number::NumberToken;
// // use crate::eight::tokens::string::StringToken;
// // use crate::eight::common::whitespace::CodeWhitespace;
// // use std::collections::HashMap;
//
//
// // pub fn test_contextframes() {
// //     let mut index: usize = 0;
// //     let mut s = "llllllllllllllllll";
// //     let mut ps = CodeWhitespace::new("ppppppppp");
// //
// //     let mut a = Context::new(
// //         &s,
// //         &mut ps,
// //         &index
// //     );
// //
// //     let val0 = valarative::Number(NumberToken { value: 6 });
// //     let val = valarative::Number(NumberToken { value: 9999 });
// //     let val2 = valarative::String(StringToken { value: String::from("gggg")});
// //
// //     a.add_frame();
// //
// //     let c = a.add_var("asdf", &val0);
// //
// //
// //     println!("Context orig: {:?}", a);
// //     a.add_frame();
// //     a.add_var("ree", &val);
// //
// //     println!("Context orig2: {:?}", a);
// //
// //     a.modify_var("ree", &val2);
// //
// //     println!("Context 2: {:?}", a);
// //     exit(0);
// // }
// //
// // #[enum_dispatch(CommonTrait)]
// // enum MyEnum {
// //     A(Struct1),
// //     B(Struct2)
// // }
// //
// // struct Struct2 {
// //     attr2: bool
// // }
// //
// // struct Struct1 {
// //     attr1: i32
// // }
// //
// // #[enum_dispatch]
// // trait CommonTrait {
// //     fn myfunc(self);
// // }
// //
// // impl CommonTrait for Struct1 {
// //     fn myfunc(self) {
// //         println!("myfunc Struct1 {}", self.attr1);
// //     }
// // }
// //
// // impl CommonTrait for Struct2 {
// //     fn myfunc(self) {
// //         println!("myfunc Struct2 {}", self.attr2);
// //     }
// // }
// //
// // pub fn test_instance_checking() {
// //     let a = MyEnum::A(Struct1{attr1: 6});
// //     let b = MyEnum::B(Struct2{attr2: false});
// //
// //     a.myfunc();
// //     b.myfunc();
// // }
//
// #[derive(Debug)]
// pub struct BasicExpression {
//     a: i32,
//     b: bool,
// }
//
// #[enum_dispatch(TypedSecondaryExpression)]
// #[derive(Debug)]
// pub enum Expr<'a> {
//     Expr1(BasicExpression),
//     Expr2(Box<ContainerExpression<'a>>),
// }
//
// #[derive(Debug)]
// pub struct ContainerExpression<'a> {
//     name: String,
//     data: &'a Expr<'a>,
// }
// impl<'a> TypedSecondaryExpression<'a> for BasicExpression {
//     fn get_type(&self) -> Result<valarativeTypes, String> {
//         todo!()
//     }
//
//     fn get_references(&self) -> Vec<RelationEntry> {
//         todo!()
//     }
//
//     fn run(&mut self, frame: &Frame) -> valarative {
//         return valarative::Number(NumberToken{value: 5});
//     }
// }
//
// impl<'a> TypedSecondaryExpression<'a> for Box<ContainerExpression<'a>> {
//     fn get_type(&self) -> Result<valarativeTypes, String> {
//         todo!()
//     }
//
//     fn get_references(&self) -> Vec<RelationEntry> {
//         todo!()
//     }
//
//     fn run(&mut self, frame: &Frame) -> valarative {
//         todo!()
//     }
// }
// //
//
// // pub struct VariableTable<'a> {
// //     all_data: HashMap<String, Expr<'a>>,
// // }
// //
// //
// // pub fn parse_expr<'a>(h: &'a mut VariableTable<'a>) -> Expr<'a> {
// //
// //     let ex = Expr::Expr1(BasicExpression {
// //         a: 5,
// //         b: false
// //     });
// //     let nm = String::from("asdf");
// //
// //     h.all_data.insert(String::from(&nm), ex);
// //
// //     Expr::Expr2(Box::from(ContainerExpression {
// //         name: String::from(&nm),
// //         data: &h.all_data.get(nm.as_str()).unwrap()
// //     }))
// // }
// //
//
//
//
//
// //
// // pub fn test_data_containment<'a>(h: &'a mut VariableTable<'a>) -> Vec<Expr<'a>> {
// //     let expr: Option<Expr> = Some(parse_expr(h));
// //     let mut v:Vec<Expr> = Vec::new();
// //
// //     match expr {
// //         Some(x) => {
// //             v.push(x);
// //         },
// //         None => ()
// //     }
// //
// //
// //     return v;
// // }
// //
// // pub fn test_data_containment_start() {
// //     let mut h = VariableTable{ all_data: HashMap::new() };
// //     let mut vv = test_data_containment(&mut h);
// //     vv.pop();
// // }
//
// // use std::collections::HashMap;
// // use crate::eight::common::tokenizing::symbols::{Symbols, SymbolType};
// // use std::str::FromStr;
// // use crate::eight::common::running::frame::Frame;
// // use crate::eight::{valarative, valarativeTypes, TypedSecondaryExpression};
// // use crate::eight::common::parsing::ast::RelationEntry;
//
//
// // pub fn test_symbol_stuff() {
// // println!("{:?}", Symbols::Add);
// // println!("{:?}", Symbols::from_str("::"));
// // println!("Add {:?}", Symbols::Add.to_str());
// // println!("{:?}", Symbols::get_all());
// // println!("Map: {:?}", Symbols::get_symbol_hashmap());
// // println!("Operators: {:?}", Symbols::get_symbols_by_type(SymbolType::Operator));
// // println!("Operators chars: {:?}", Symbols::to_strs(SymbolType::Operator));
//
// // let mut vv = vec![
// //     Symbols::Add,
// //     Symbols::ClassAccessor,
// //     Symbols::Sub,
// //     Symbols::EqualityCheck,
// //     Symbols::WindowsNewLine
// // ];
// //
// // println!("before {:?}", vv);
// // let nv = Symbols::sort_by_longest(vv);
// // println!("after {:?}", nv);
// // }
//
// // pub enum TestMatch {
// //     A(String),
// //     B(String)
// // }
// // pub fn test_match() {
// //     let a = TestMatch::A(String::from("asdf"));
// //
// //     match a {
// //         TestMatch::A(ss) => {
// //             match ss.as_str() {
// //                 "asdf" => {
// //                     println!("Matched 1");
// //                 },
// //                 _ => {
// //                     println!("No match");
// //                 }
// //             }
// //         },
// //         TestMatch::B(bb) => {
// //             println!("Matched 2 - probs?");
// //         }
// //     }
// // }
// use enum_dispatch::enum_dispatch;
// use crate::eight::{valarative, valarativeTypes, TypedSecondaryExpression};
// use crate::eight::common::parsing::ast::RelationEntry;
// use crate::eight::common::running::frame::Frame;
// use crate::eight::values::basic::number::NumberToken;
//
//
// pub fn gen_exprs<'a>() -> Vec<Expr<'a>> {
//     let expr1 = Expr::Expr1(BasicExpression {
//         a: 0,
//         b: false,
//     });
//
//     vec![
//         expr1,
//         Expr::Expr1(BasicExpression{
//             a: 1,
//             b: true
//         }),
//     ]
// }

/////////////////////////////////
//
// use enum_dispatch::enum_dispatch;

// use crate::eight::valarative;
// use crate::eight::expressions::secondary::operators::add::Add;
//
// #[derive(Debug)]
// pub struct VarTok<'a> {
//     pub(crate) name: String,
//     pub(crate) value: Rc<SecExpr<'a>>,
// }
//
// impl<'a> DistrExpr<'a> for Box<VarTok<'a>> {
//     fn run_exprr(&mut self, frame: &mut Frame) {
//         let mut new = Rc::new(SecExpr::valEx(valEx { value: val::Boolean(BoolT { value: false }) }));
//         let rpl = std::mem::replace(
//             &mut self.value,
//             new,
//         );
//         let mut newrc: SecExpr = Rc::try_unwrap(rpl).unwrap();
//
//         let v = match newrc {
//             SecExpr::valEx(mut d) => {
//                 println!("val");
//                 std::mem::replace(&mut d.value, val::Boolean(BoolT{value:false}))
//             }
//             mut o => {
//                 println!("Second '{:?}'", o);
//                 o.runn(frame)
//             }
//         };
//         println!("HERE {:?}", v);
//
//         // let v = match Rc::try_unwrap(std::mem::replace(
//         //     &mut self.value,
//         //     Rc::new(SecExpr::valEx(valEx{value: val::Boolean(BoolT{value: false})})))
//         // ).unwrap() {
//         //     SecExpr::valEx(d) => {
//         //         println!("val");
//         //         d.value
//         //     },
//         //     mut o => {
//         //         println!("Second '{:?}'", o);
//         //         o.runn(frame)
//         //     }
//         // };
//         // println!("HERE {:?}", v);
//     }
// }
//
// #[enum_dispatch]
// pub trait DistrExpr<'a> {
//     fn run_exprr(&mut self, frame: &mut Frame);
// }
//
// #[enum_dispatch(DistrExpr)]
// #[derive(Debug)]
// pub enum Expr<'a> {
//     VarTok(Box<VarTok<'a>>)
// }
//
// #[enum_dispatch]
// pub trait TypedSecExpr<'a> {
//     fn runn(&mut self, frame: &mut Frame) -> val<'a>;
// }
//
// #[derive(Debug)]
// pub struct valEx<'a> {
//     value: val<'a>,
// }
//
// impl<'a> TypedSecExpr<'a> for valEx<'a> {
//     fn runn(&mut self, frame: &mut Frame) -> val<'a> {
//         let mut src: val = val::Boolean(BoolT { value: false });
//         println!("Running basic val {:?}", self);
//         let d = std::mem::replace(
//             &mut self.value,
//             src,
//         );
//         return d;
//
//         // return match d {
//         //     val::Funct(f) => {
//         //         val::DD(frame.check_out(f.s.as_str()))
//         //     }
//         //     o => o
//         // };
//     }
// }
//
// #[derive(Debug)]
// pub struct AddEx<'a> {
//     v1: SecExpr<'a>,
//     v2: SecExpr<'a>,
// }
//
// impl<'a> TypedSecExpr<'a> for Box<AddEx<'a>> {
//     fn runn(&mut self, frame: &mut Frame) -> val<'a> {
//         let aa: val = self.v1.runn(frame);
//         let bb: val = self.v2.runn(frame);
//         println!("Running Add {:?}", self);
//         return val::Funct(Fun {
//             pt: |x: &mut i32| {
//                 5
//             },
//             s: String::from("a"),
//             vv: vec![aa, bb],
//         });
//     }
// }
//
// #[enum_dispatch(TypedSecExpr)]
// #[derive(Debug)]
// pub enum SecExpr<'a> {
//     valEx(valEx<'a>),
//     AddEx(Box<AddEx<'a>>),
// }
//
// #[derive(Debug)]
// pub struct BoolT {
//     value: bool,
// }
//
// #[derive(Debug)]
// pub struct Fun<'a> {
//     pt: fn(&'a mut i32) -> i32,
//     vv: Vec<val<'a>>,
//     s: String,
// }
//
// #[derive(Debug)]
// pub enum val<'b> {
//     Boolean(BoolT),
//     Funct(Fun<'b>),
//     DD(valarative<'b>),
// }
//
// pub fn gen_exprs<'a>() -> Vec<Expr<'a>> {
//     vec![
//         Expr::VarTok(Box::from(VarTok {
//             name: String::from("asdf"),
//             value: Rc::new(
//                 SecExpr::valEx(valEx {
//                     value:
//                     val::Boolean(BoolT { value: true })
//                 })
//             ),
//         })),
//
//         Expr::VarTok(Box::from(VarTok{
//             name: String::from("aaa"),
//             value: Rc::new(
//                 SecExpr::AddEx(Box::from(AddEx {
//                     v1: SecExpr::valEx(valEx {
//                         value:
//                         val::Boolean(BoolT { value: true })
//                     }),
//                     v2: SecExpr::valEx(valEx {
//                         value:
//                         val::Boolean(BoolT { value: true })
//                     })
//                 }))
//             )
//         }))
//     ]
// }
//
// pub fn test_frame_lifetimes() {
//     let mut exs = gen_exprs();
//     let mut frame = Frame::new();
//     for ex in exs.iter_mut() {
//         ex.run_exprr(&mut frame);
//     }
// }

//
// use enum_dispatch::enum_dispatch;
// use std::collections::HashMap;
// use std::rc::Rc;
// // use crate::eight::common::running::frame::Frame;
//
// #[derive(Debug)]
// pub struct VarTok<'a> {
//     pub(crate) name: String,
//     pub(crate) value: Rc<SecExpr<'a>>,
// }
//
// impl<'a> DistrExpr<'a> for Box<VarTok<'a>> {
//     fn run_exprr(&mut self, frame: &mut HashMap<String, val<'a>>) {
//         let mut new = Rc::new(SecExpr::valEx(valEx { value: val::Boolean(BoolT { value: false }) }));
//         let rpl = std::mem::replace(
//             &mut self.value,
//             new,
//         );
//         let mut newrc: SecExpr = Rc::try_unwrap(rpl).unwrap();
//
//         let v = match newrc {
//             SecExpr::valEx(mut d) => {
//                 println!("val");
//                 std::mem::replace(&mut d.value, val::Boolean(BoolT{value:false}))
//             }
//             mut o => {
//                 println!("Second '{:?}'", o);
//                 o.runn(frame)
//             }
//         };
//         println!("HERE {:?}", v);
//
//         // let v = match Rc::try_unwrap(std::mem::replace(
//         //     &mut self.value,
//         //     Rc::new(SecExpr::valEx(valEx{value: val::Boolean(BoolT{value: false})})))
//         // ).unwrap() {
//         //     SecExpr::valEx(d) => {
//         //         println!("val");
//         //         d.value
//         //     },
//         //     mut o => {
//         //         println!("Second '{:?}'", o);
//         //         o.runn(frame)
//         //     }
//         // };
//         // println!("HERE {:?}", v);
//     }
// }
//
// #[enum_dispatch]
// pub trait DistrExpr<'a> {
//     fn run_exprr(&mut self, frame: &mut HashMap<String, val<'a>>);
// }
//
// #[enum_dispatch(DistrExpr)]
// #[derive(Debug)]
// pub enum Expr<'a> {
//     VarTok(Box<VarTok<'a>>)
// }

// impl<'a> DistrExpr<'a> for Expr<'a> {
//     fn run_exprr(&mut self, frame: &mut HashMap<String, val<'a>>) {
//         match self {
//             Expr::VarTok(v) => {
//                 v.run_exprr(frame)
//             }
//         }
//     }
// }
//
// #[enum_dispatch]
// pub trait TypedSecExpr<'a> {
//     fn runn(&mut self, frame: &mut HashMap<String, val<'a>>) -> val<'a>;
// }
//
// #[derive(Debug)]
// pub struct valEx<'a> {
//     value: val<'a>,
// }
//
// impl<'a> TypedSecExpr<'a> for valEx<'a> {
//     fn runn(&mut self, frame: &mut HashMap<String, val<'a>>) -> val<'a> {
//         let mut src: val = val::Boolean(BoolT { value: false });
//         println!("Running basic val {:?}", self);
//         let d = std::mem::replace(
//             &mut self.value,
//             src,
//         );
//         // return d;
//
//         return match d {
//             val::Refr(f) => {
//                 // let ff = frame.check_out(f.varname.as_str());
//                 frame.remove(f.varname.as_str()).unwrap()
//             }
//             o => o
//         };
//     }
// }
//
// #[derive(Debug)]
// pub struct AddEx<'a> {
//     v1: SecExpr<'a>,
//     v2: SecExpr<'a>,
// }
//
// impl<'a> TypedSecExpr<'a> for Box<AddEx<'a>> {
//     fn runn(&mut self, frame: &mut HashMap<String, val<'a>>) -> val<'a> {
//         let aa: val = self.v1.runn(frame);
//         let bb: val = self.v2.runn(frame);
//         println!("Running Add {:?} -> {:?} & {:?}", self, aa, bb);
//         return val::Funct(Fun {
//             pt: |x: &mut i32| {
//                 5
//             },
//             s: String::from("a"),
//             vv: vec![aa, bb],
//         });
//     }
// }
//
// #[enum_dispatch(TypedSecExpr)]
// #[derive(Debug)]
// pub enum SecExpr<'a> {
//     valEx(valEx<'a>),
//     AddEx(Box<AddEx<'a>>),
// }
//
// // impl<'a> TypedSecExpr<'a> for SecExpr<'a> { // this is basically what enum_dispatch does
// //     fn runn(&mut self, frame: &mut HashMap<String, val<'a>>) -> val<'a> {
// //         return match self {
// //             SecExpr::valEx(d) => {
// //                 d.runn(frame)
// //             },
// //             SecExpr::AddEx(a) => {
// //                 a.runn(frame)
// //             }
// //         }
// //     }
// // }
//
// #[derive(Debug)]
// pub struct BoolT {
//     value: bool,
// }
//
// #[derive(Debug)]
// pub struct Fun<'a> {
//     pt: fn(&'a mut i32) -> i32,
//     vv: Vec<val<'a>>,
//     s: String,
// }
//
// #[derive(Debug)]
// pub struct Ref {
//     varname: String
// }
//
// #[derive(Debug)]
// pub struct IntT {
//     val: i32
// }
//
// #[derive(Debug)]
// pub enum val<'b> {
//     Boolean(BoolT),
//     Int(IntT),
//     Funct(Fun<'b>),
//     Refr(Ref)
// }
//
// pub fn gen_exprs<'a>() -> Vec<Expr<'a>> {
//     vec![
//         Expr::VarTok(Box::from(VarTok {
//             name: String::from("asdf"),
//             value: Rc::new(
//                 SecExpr::valEx(valEx {
//                     value:
//                     val::Boolean(BoolT { value: true })
//                 })
//             ),
//         })),
//
//         Expr::VarTok(Box::from(VarTok{
//             name: String::from("aaa"),
//             value: Rc::new(
//                 SecExpr::AddEx(Box::from(AddEx {
//                     v1: SecExpr::valEx(valEx {
//                         value:
//                         val::Boolean(BoolT { value: true })
//                     }),
//                     v2: SecExpr::valEx(valEx {
//                         value:
//                         val::Refr(Ref{varname: String::from("test-var-name")})
//                     })
//                 }))
//             )
//         }))
//     ]
// }
//
// pub fn test_frame_lifetimes() {
//
//     let mut frame: HashMap<String, val> = HashMap::new();
//     frame.insert("test-var-name".to_string(), val::Int(IntT{val: 666}));
//     {
//         let mut exs = gen_exprs();
//         for ex in exs.iter_mut() {
//             ex.run_exprr(&mut frame);
//         }
//     }
//     println!("FRAME {:?}", frame);
// }

///////////////////
// #[derive(Debug)]
// pub struct Data {
//     value: i32,
//     d1: Option<Box<Data>>,
//     d2: Option<Box<Data>>,
// }
//
// impl Data {
//     fn empty() -> Self {
//         Data { value: 0, d1: None, d2: None }
//     }
//
//     fn withval(v: i32) -> Self {
//         Data {value: v, d1: None, d2: None}
//     }
// }
//
// #[derive(Debug)]
// pub enum Container {
//     V1(Data)
// }
//
// impl Container {
//     pub fn run(&mut self, htable: &mut HashMap<String, Data>) -> Data {
//         match self {
//             Container::V1(d) => {
//                 std::mem::replace(d, Data::empty())
//             }
//         }
//     }
// }
//
// pub fn test_frame_lifetimes() {
//     let mut h: HashMap<String, Data> = HashMap::new();
//
//     let mut g = Container::V1(Data {
//         value: 5,
//         d1: Some(Box::from(Data::withval(1))),
//         d2: Some(Box::from(Data::withval(2)))
//     });
//
//     g.swapout(&mut h);
//     print!("Orig {:?} Swapped {:?}", g, h.get("tt"));
//}

// pub struct Entry<'a> {
//     val: &'a str
// }
//
// impl<'a> Entry<'a> {
//
// }
//
// use std::collections::HashMap;
// use crate::eight::common::running::frame::Frame;
// use crate::eight::Expression;
//
// #[derive(Debug)]
// enum Data {
//     One,
//     Two
// }
//
// impl Data {
//     // fn run_expr<'framelif: 'a>(mut self, frame: &'framelif mut Frame<'framelif>) -> Self
//     pub fn run<'f>(mut self, store: &'f mut Frame) {
//         println!("Here");
//     }
// }
//
// struct Store {
//     data: HashMap<String, Data>
// }
//
// fn test_run(mut exprs: Vec<Vec<Expression>>, mut frame: Frame) {
//     for expr in exprs.iter_mut() {
//         let l = expr.len();
//         for idx in 0..l {
//             let mut ex = expr.remove(0);
//             println!("Running '{:?}'", ex);
//             ex.run_expr(&mut frame);
//         }
//     }
// }
//
// pub fn test_expr_running() {
//     let mut exprs = Vec::from([]);
//     let mut frame = Frame::new();
//     test_run(exprs, frame);
// }
///////////////////////////////
// use std::borrow::BorrowMut;
// use std::cell::{RefCell, RefMut};
// use std::collections::HashMap;
//
// #[derive(Debug)]
// enum BasicVal<'a> {
//     Ref(&'a BasicVal<'a>),
//     Val1(BasicStruct),
// }
//
// #[derive(Debug)]
// struct Holder<'b> {
//     hold: HashMap<String, RefCell<BasicVal<'b>>>,
// }
//
// #[derive(Debug)]
// struct BasicStruct {
//     val: i32,
// }
//
// impl<'a> BasicVal<'a> {
//     pub fn empty() -> Self { BasicVal::Val1(BasicStruct { val: 0 }) }
// }
//
// // must match sig of modify_val_ref
// fn modify_val<'f>(holder: &'f mut Holder<'f>, mut ret: RefMut<BasicVal<'f>>) {
//     *ret = BasicVal::Val1(BasicStruct { val: 5 });
// }
//
// // must match sig of modify_val
// fn modify_val_ref<'f>(holder: &'f mut Holder<'f>, mut ret: RefMut<BasicVal<'f>>) {
//     ret = holder.hold.get("reference_val").unwrap().borrow_mut();
// }
//
//
// fn do_modify<'f>(holder: &'f mut Holder<'f>) {
//     let mut v = RefCell::new(BasicVal::empty());
//     println!("Original {:?}", v);
//
//     modify_val(holder, v.borrow_mut());
//     holder.hold.insert("Data".to_string(), v);
//
//     println!("Modified {:?}", holder.hold.get("Data"));
// }
//
// pub fn test_dropborrow() {
//     let mut holder = Holder { hold: HashMap::new() };
//     holder.hold.insert(
//         "reference_val".to_string(),
//         RefCell::new(BasicVal::Val1(BasicStruct { val: 8 })),
//     );
//     do_modify(&mut holder);
// }
//
// pub fn main() {
//     test_dropborrow();
//}
///////////////////////////////
// use std::collections::HashMap;
//
// #[derive(Debug)]
// enum BasicVal<'a> {
//     Ref(&'a BasicVal<'a>),
//     Val1(BasicStruct)
// }
//
// #[derive(Debug)]
// struct Holder<'b> {
//     hold: HashMap<String, BasicVal<'b>>
// }
//
// #[derive(Debug)]
// struct BasicStruct {
//     val: i32
// }
//
// impl<'a> BasicVal<'a> {
//     pub fn empty() -> Self { BasicVal::Val1(BasicStruct{val: 0})}
// }
//
// fn modify_val<'f>(holder: &'f mut Holder<'f>) {
//     holder.hold.insert("$return".to_string(), BasicVal::Val1(BasicStruct{val: 5}));
// }
//
// fn do_modify<'f>(holder: &'f mut Holder<'f>) {
//
//     modify_val(holder);
//     let mut v = holder.hold.remove("$return").unwrap();
//     holder.hold.insert("Data".to_string(), v);
//
//     println!("Modified {:?}", v);
// }
//
// pub fn test_dropborrow() {
//     let mut holder = Holder { hold: HashMap::new() };
//     do_modify(&mut holder);
// }
//
// pub fn main() {
//     test_dropborrow();
//}
//////////////////////////
//
// use std::collections::HashMap;
//
// trait FixComplexVal {
//     fn fix_complex<'f>(&mut self, holder: &'f mut Holder<'f>);
// }
//
// trait FixSimpleVal {
//     fn fix_simple<'f>(&mut self, holder: &'f mut Holder<'f>) -> BasicVal<'f>;
// }
//
// #[derive(Debug)]
// struct ComplexStruct {
//     a: SimpleVal,
//     b: SimpleVal
// }
//
// impl FixComplexVal for ComplexStruct {
//     fn fix_complex<'f>(&mut self, holder: &'f mut Holder<'f>) {
//         //println!("Adding '{:?}' to holder", v1);
//         holder.hold.insert("testing".to_string(), Simplifier::step(&mut self.a, holder));
//     }
// }
//
// #[derive(Debug)]
// struct SimplStruct {
//     c: Box<SimpleVal>
// }
// impl FixSimpleVal for SimplStruct {
//     fn fix_simple<'f>(&mut self, holder: &'f mut Holder<'f>) -> BasicVal<'f> {
//         let mut v = BasicVal::empty();
//         Simplifier::step(&mut self.c, holder);
//
//         return match v {
//             BasicVal::Val1(s) => {
//                 BasicVal::Val1(BasicStruct{val: s.val +1})
//             },
//             BasicVal::Ref(r) => {
//                 BasicVal::Ref(r)
//             }
//         };
//     }
// }
//
// #[derive(Debug)]
// struct BasicStruct {
//     val: i32
// }
//
// #[derive(Debug)]
// enum ComplexVal {
//     Cmplx1(ComplexStruct)
// }
//
// #[derive(Debug)]
// enum SimpleVal {
//     Smpl1(SimplStruct),
//     Smpl2(BasicStruct)
// }
//
// #[derive(Debug)]
// enum BasicVal<'a> {
//     Ref(&'a BasicVal<'a>),
//     Val1(BasicStruct)
// }
// impl<'a> BasicVal<'a> {
//     pub fn empty() -> Self { BasicVal::Val1(BasicStruct{val: 0})}
// }
//
// #[derive(Debug)]
// struct Holder<'b> {
//     hold: HashMap<String, BasicVal<'b>>
// }
//
// struct Simplifier {}
//
// impl Simplifier {
//     pub fn step<'e, 'f>(val: &'e mut SimpleVal, holder: &'f mut Holder<'f>) -> BasicVal<'f> {
//         println!("Stepping {:?}", val);
//         return match val {
//             SimpleVal::Smpl1(s) => {
//                 s.fix_simple(holder)
//             },
//             SimpleVal::Smpl2(b) => {
//                 BasicVal::Ref(holder.hold.get(b.val.to_string().as_str()).unwrap())
//             }
//         }
//     }
//
//     pub fn run(&mut self, mut vals: Vec<ComplexVal>) {
//         let mut holder = Holder{hold:HashMap::new()};
//         holder.hold.insert("5".to_string(), BasicVal::Val1(BasicStruct{val: 5}));
//         holder.hold.insert("2".to_string(), BasicVal::Val1(BasicStruct{val: 2}));
//         let len = vals.len();
//         for _ in 0..len {
//             let mut val = vals.remove(0);
//             println!("Running {:?}", val);
//             match val {
//                 ComplexVal::Cmplx1(mut c) => {
//                     c.fix_complex(&mut holder)
//                 },
//                 //... more cases of different types of values omitted for simplicity
//             }
//             // val *should* be dropped here, and therefore the mutable borrow of holder?
//         }
//
//         println!("Holder: {:?}", holder);
//     }
// }
//
// pub fn test_multiborrow() {
//     let complexes: Vec<ComplexVal> = vec![
//         ComplexVal::Cmplx1(ComplexStruct{
//             a: SimpleVal::Smpl2(BasicStruct{val: 5}),
//             b: SimpleVal::Smpl1(SimplStruct{
//                 c: Box::from(SimpleVal::Smpl2(BasicStruct{val: 2}))
//             })
//         })
//     ];
//
//     Simplifier{}.run(complexes);
// }


///////////////////////////////
// use std::collections::HashMap;
//
// #[derive(Debug)]
// enum BasicVal<'a> {
//     Ref(&'a BasicVal<'a>),
//     Val1(BasicStruct)
// }
//
// #[derive(Debug)]
// struct Holder<'b> {
//     hold: HashMap<String, BasicVal<'b>>
// }
//
// #[derive(Debug)]
// struct BasicStruct {
//     val: i32
// }
//
// impl<'a> BasicVal<'a> {
//     pub fn empty() -> Self { BasicVal::Val1(BasicStruct{val: 0})}
// }
//
// fn modify_val<'f>(holder: &'f mut Holder<'f>) {
//     holder.hold.insert("$return".to_string(), BasicVal::Val1(BasicStruct{val: 5}));
// }
//
// fn do_modify<'f>(holder: &'f mut Holder<'f>) {
//
//     modify_val(holder);
//     let mut v = holder.hold.remove("$return").unwrap();
//     holder.hold.insert("Data".to_string(), v);
//
//     println!("Modified {:?}", v);
// }
//
// pub fn test_dropborrow() {
//     let mut holder = Holder { hold: HashMap::new() };
//     do_modify(&mut holder);
// }
//
// pub fn main() {
//     test_dropborrow();
// }
// use std::borrow::BorrowMut;
// ///////////////////////////////
// use std::cell::{RefCell, RefMut};
// use std::collections::HashMap;
//
// #[derive(Debug)]
// enum BasicVal<'a> {
//     Ref(&'a BasicVal<'a>),
//     Val1(BasicStruct),
// }
//
// #[derive(Debug)]
// struct Holder<'b> {
//     hold: HashMap<String, RefCell<BasicVal<'b>>>,
// }
//
// #[derive(Debug)]
// struct BasicStruct {
//     val: i32,
// }
//
// impl<'a> BasicVal<'a> {
//     pub fn empty() -> Self { BasicVal::Val1(BasicStruct { val: 0 }) }
// }
//
// // must match sig of modify_val_ref
// fn modify_val<'f>(holder: &'f mut Holder<'f>, mut ret: RefMut<BasicVal<'f>>) {
//     *ret = BasicVal::Val1(BasicStruct { val: 5 });
// }
//
// // must match sig of modify_val
// fn modify_val_ref<'f>(holder: &'f mut Holder<'f>, mut ret: RefMut<BasicVal<'f>>) {
//     ret = holder.hold.get("reference_val").unwrap().borrow_mut();
// }
//
//
// fn do_modify<'f>(holder: &'f mut Holder<'f>) {
//     let mut v = RefCell::new(BasicVal::empty());
//     println!("Original {:?}", v);
//
//     modify_val(holder, v.borrow_mut());
//     holder.hold.insert("Data".to_string(), v);
//
//     println!("Modified {:?}", holder.hold.get("Data"));
// }
//
// pub fn test_dropborrow() {
//     let mut holder = Holder { hold: HashMap::new() };
//     holder.hold.insert(
//         "reference_val".to_string(),
//         RefCell::new(BasicVal::Val1(BasicStruct { val: 8 })),
//     );
//     do_modify(&mut holder);
// }
//
// pub fn main() {
//     test_dropborrow();
// }