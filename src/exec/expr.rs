use crate::expr::{ Expr, Lit };
use crate::exec::{ Executor, Value };
use crate::iter::IteratorExt;
use crate::parser;
use std::{ fs, process };


pub trait Execute {
    fn execute(&self, e : &mut Executor) -> Value;
}


impl Execute for Expr {
    fn execute(&self, e : &mut Executor) -> Value {
        match (self) {
            Self::Print(expr) => {
                let v = expr.execute(e);
                println!("{}", v);
                v
            },
            Self::Add(args) => args.0.execute(e) + args.1.execute(e),
            Self::Sub(args) => args.0.execute(e) - args.1.execute(e),
            Self::Mul(args) => args.0.execute(e) * args.1.execute(e),
            Self::Div(args) => args.0.execute(e) / args.1.execute(e),
            Self::Rem(args) => args.0.execute(e) % args.1.execute(e),
            Self::Get(args) => {
                let q = args.0.execute(e);
                let i = args.1.execute(e);
                Self::exec_get(e, q, i)
            },
            Self::Gets(args) => {
                let q  = args.0.execute(e);
                let i0 = args.1.execute(e);
                let i1 = args.2.execute(e);
                Self::exec_gets(e, q, i0, i1)
            },
            Self::Push(args) => {
                let q = args.0.execute(e);
                let val = args.1.execute(e);
                Self::exec_push(e, q, val)
            },
            Self::Pushes(args) => {
                let q = args.0.execute(e);
                let val = args.1.execute(e);
                Self::exec_pushes(e, q, val)
            },
            Self::Insert(args) => {
                let q = args.0.execute(e);
                let i = args.1.execute(e);
                let v = args.2.execute(e);
                Self::exec_insert(e, q, i ,v)
            },
            Self::Inserts(args) => {
                let q = args.0.execute(e);
                let i = args.1.execute(e);
                let v = args.2.execute(e);
                Self::exec_inserts(e, q, i ,v)
            },
            Self::Set(args) => {
                let q = args.0.execute(e);
                let i = args.1.execute(e);
                let v = args.2.execute(e);
                Self::exec_set(e, q, i ,v)
            },
            Self::Sets(args) => {
                let q = args.0.execute(e);
                let i = args.1.execute(e);
                let v = args.2.execute(e);
                Self::exec_sets(e, q, i ,v)
            },
            Self::Len(args) => {
                let q = args.execute(e);
                Self::exec_len(e, q)
            },
            Self::FSRead(args) => {
                let fname = args.execute(e);
                Self::exec_fsread(e, fname)
            },
            Self::If(args) => {
                let c = args.0.execute(e);
                let Value::Bool(c) = c
                    else { return Value::Error; };
                if c { return args.1.execute(e); }
                else { return args.2.execute(e); }
            },
            Self::Range(args) => {
                let i0 = args.0.execute(e);
                let i1 = args.1.execute(e);
                Self::exec_range(e, i0, i1)
            },
            Self::Str(arg) => Value::String(arg.execute(e).to_string()),
            Self::Int(arg) => {
                if let Ok(i) = arg.execute(e).to_string().parse::<i128>() {
                    Value::Int(i)
                } else { Value::Error }
            },
            Self::Lit(lit) => lit.execute(e),
            Self::Not(arg) => !arg.0.execute(e),
            Self::Equals(args) => {
                let l = args.0.execute(e);
                let r = args.1.execute(e);
                //println!("{} {}", l, r); // TODO: why is there a print here?
                Value::Bool(l == r)
            },
            Self::Greater(args) => {
                let l = args.0.execute(e);
                let r = args.1.execute(e);
                Value::Bool(l > r)
            },
            Self::GreaterEquals(args) => {
                let l = args.0.execute(e);
                let r = args.1.execute(e);
                Value::Bool(l >= r)
            }
            Self::Less(args) => {
                let l = args.0.execute(e);
                let r = args.1.execute(e);
                //println!("{} {}", l, r);
                Value::Bool(l < r)
            }
            Self::LessEquals(args) => {
                let l = args.0.execute(e);
                let r = args.1.execute(e);
                Value::Bool(l <= r)
            }
        }
    }
}
impl Expr {

    fn exec_get(e : &mut Executor, q : Value, i : Value) -> Value {
        let Value::Int(i) = i
            else { return Value::Error; };
        if (i < 0) { return Value::Error; }
        let i = i as usize;
        match (q) {
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::String    (v) => v.chars().nth(i).map_or(Value::Error, |ch| Value::String(ch.to_string())),
            Value::Error         => Value::Error,
            Value::ExprQueue     => e.get_expr(i).map_or(Value::Error, |v| Value::String(v.to_string())),
            Value::Array     (q) => q.get(i).map_or(Value::Error, |v| v.clone())
        }
    }

    fn exec_gets(e : &mut Executor, q : Value, i0 : Value, i1 : Value) -> Value {
        let Value::Int(i0) = i0
            else { return Value::Error };
        let Value::Int(i1) = i1
            else { return Value::Error };
        let i0 = i0 as usize;
        let i1 = i1 as usize;
        match (q) {
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::String    (v) => v.chars()
                                        .map(|ch| Value::String(ch.to_string()))
                                        .skip(i0).next_n_exact(i1 - i0)
                                        .map_or(Value::Error, |v| Value::Array(v)),
            Value::Error         => Value::Error,
            Value::ExprQueue     => {
                let Some(exprs) = e.get_exprs_values(i0, i1)
                    else { return Value::Error; };
                Value::Array(exprs)
            },
            Value::Array     (q) => q.get(i0..i1).map_or(Value::Error, |v| Value::Array(v.to_vec()))
        }

    }

    fn exec_len(e : &mut Executor, q : Value) -> Value {
        match (q) {
            Value::String(v)     => Value::Int( v.len() as i128 ),
            Value::Array(v)      => Value::Int( v.len() as i128 ),
            Value::ExprQueue     => Value::Int( e.len_exprs() as i128 ),
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::Error         => Value::Error
        }
    }

    // Returns the resulting array/string/queue
    fn exec_push(e : &mut Executor, q : Value, v : Value) -> Value {
        match (q) {
            Value::String(str)     => Value::String(str + &v.to_string()),
            Value::Array(mut arr)  => { arr.push(v); Value::Array(arr) },
            Value::Unit            => Value::Error,
            Value::ExprQueue       => { 
                match parser::parse(&v.to_string()) {
                    Ok(val) => e.push_exprs(val),
                    Err(err) => {
                        err.print_formatted();
                        process::exit(1);
                    },
                }
                Value::ExprQueue
            },
            Value::Bool      (_)   => Value::Error,
            Value::Int       (_)   => Value::Error,
            Value::Float     (_)   => Value::Error,
            Value::Error           => Value::Error
        }
    }

    fn exec_pushes(e : &mut Executor, q : Value, v : Value) -> Value {
        let Value::Array(mut v) = v
            else { return Value::Error; };
        match (q) {
            Value::String(mut str)     => {
                for val in v {
                    str += &val.to_string();
                }
                Value::String(str)
            },
            Value::Array(mut arr)  => { arr.append(&mut v); Value::Array(arr) },
            Value::Unit            => Value::Error,
            Value::ExprQueue       => {
                let v = v.into_iter().map(|v| {
                    match parser::parse(&v.to_string()) {
                        Ok(val) => val,
                        Err(err) => {
                            err.print_formatted();
                            process::exit(1);
                        },
                    }
                }).flatten();
                e.push_exprs(v);
                Value::ExprQueue
            },
            Value::Bool      (_)   => Value::Error,
            Value::Int       (_)   => Value::Error,
            Value::Float     (_)   => Value::Error,
            Value::Error           => Value::Error
        }
    }

    fn exec_set (e : &mut Executor, q : Value, i : Value, v : Value) -> Value {
        let Value::Int(i) = i
            else { return Value::Error; };
        if (i < 0) { return Value::Error; }
        let i = i as usize;
        match (q) {
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::String  (str) => {
                let Value::String(v) = v
                    else { return Value::Error; };
                if (v.len() != 1) { return Value::Error; }
                let Some(first_slice) = str.get(..i) else { return Value::Error };
                let Some(second_slice) = str.get((i+1)..) else { return Value::Error };
                Value::String(String::from(first_slice) + &v + second_slice)
            },
            Value::Error         => Value::Error,
            Value::ExprQueue     => {
                let parsed_val = match (parser::parse(&v.to_string())) {
                    Ok(val) => val,
                    Err(err) => {
                        err.print_formatted();
                        process::exit(1);
                    },
                };
                match (e.sets_expr(i, parsed_val)) {
                    Ok(_) => Value::ExprQueue,
                    Err(_) => Value::Error
                }
            },
            Value::Array   (mut arr) => {
                let Some(arr_at_i) = arr.get_mut(i) 
                    else { return Value::Error; };
                *arr_at_i = v;
                Value::Array(arr)
            }
        }
    }

    // q is an array, start_index is an int, and v is an array
    fn exec_sets (e : &mut Executor, mut q : Value, start_index : Value, v : Value) -> Value {
        let Value::Array(v) = v
            else { return Value::Error; };
        let Value::Int(start_index) = start_index 
            else { return Value::Error; };
        for i in 0..v.len() {
            let Some(val) = v.get(i) else 
                { return Value::Error };
            let new_q = Self::exec_set(e, q, Value::Int(start_index + i as i128), val.clone());
            if let Value::Error = new_q {
                return Value::Error;
            }
            q = new_q
        }
        q
    }

    fn exec_fsread(_e : &mut Executor, fname : Value) -> Value {
        let fname = fname.to_string();
        fs::read_to_string(fname).map_or(Value::Error, |v| Value::String(v))
    }

    fn exec_if(_e : &mut Executor, c : Value, t : Value, f : Value) -> Value {
        let c = match (c) {
            Value::Unit          => { return Value::Error; },
            Value::Bool      (v) => v,
            Value::Int       (v) => v != 0,
            Value::Float     (_) => { return Value::Error; },
            Value::String    (_) => { return Value::Error; },
            Value::Error         => { return Value::Error; },
            Value::ExprQueue     => { return Value::Error; },
            Value::Array     (_) => { return Value::Error; },
        };
        if (c) { t } else { f }
    }

    fn exec_range (_e : &mut Executor, i0 : Value, i1 : Value) -> Value {
        let Value::Int(i0) = i0
            else { return Value::Error };
        let Value::Int(i1) = i1
            else { return Value::Error };
        Value::Array((i0..i1).map(|v| Value::Int(v)).collect())
    }

    fn exec_insert (e : &mut Executor, q : Value, i : Value, v : Value) -> Value {
        let Value::Int(i) = i
            else { return Value::Error; };
        if (i < 0) { return Value::Error; }
        let i = i as usize;
        match (q) {
            Value::Unit          => Value::Error,
            Value::Bool      (_) => Value::Error,
            Value::Int       (_) => Value::Error,
            Value::Float     (_) => Value::Error,
            Value::String  (str) => {
                let Value::String(v) = v
                    else { return Value::Error; };
                let Some(first_slice) = str.get(..i) else { return Value::Error };
                let Some(second_slice) = str.get(i..) else { return Value::Error };
                Value::String(String::from(first_slice) + &v + second_slice)
            },
            Value::Error         => Value::Error,
            Value::ExprQueue     => {
                let parsed_val = match (parser::parse(&v.to_string())) {
                    Ok(val) => val,
                    Err(err) => {
                        err.print_formatted();
                        process::exit(1);
                    },
                };
                match (e.inserts_expr(i, parsed_val)) {
                    Ok(_) => Value::ExprQueue,
                    Err(_) => Value::Error
                }
            },
            Value::Array   (mut arr) => {
                if (i > arr.len()) { return Value::Error }
                arr.insert(i, v);
                Value::Array(arr)
            }
        }
    }

    fn exec_inserts (e : &mut Executor, mut q : Value, start_index : Value, v : Value) -> Value {
        let Value::Array(v) = v
            else { return Value::Error; };
        let Value::Int(start_index) = start_index 
            else { return Value::Error; };
        for i in 0..v.len() {
            let Some(val) = v.get(i) else 
                { return Value::Error };
            let new_q = Self::exec_insert(e, q, Value::Int(start_index + i as i128), val.clone());
            if let Value::Error = new_q {
                return Value::Error;
            }
            q = new_q
        }
        q
    }

}


impl Execute for Lit {
    fn execute(&self, _e : &mut Executor) -> Value {
        match (self) {
            Self::Bool      (v) => Value::Bool(*v),
            Self::Int       (v) => Value::Int(*v),
            Self::Float     (v) => Value::Float(*v),
            Self::String    (v) => Value::String(v.clone()),
            Self::ExprQueue     => Value::ExprQueue
        }
    }
}
