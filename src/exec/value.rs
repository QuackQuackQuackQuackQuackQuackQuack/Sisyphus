use core::fmt;
use core::ops::{ Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not };
use f128::f128;


#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Unit,
    Bool(bool),
    Int(i128),
    Float(f128),
    String(String),
    Error,
    ExprQueue,
    Array(Vec<Value>)
}

impl Add for Value {
    type Output = Value;
    fn add(self, rhs : Self) -> Self::Output {
        match ((self, rhs)) {
            (Self::Unit, _)                  | (_, Self::Unit)                 => Self::Error,
            (Self::Bool(a), Self::Bool(b))                                     => Self::Bool(a || b),
            (Self::Bool(a), Self::Int(b))    | (Self::Int(b), Self::Bool(a))   => Self::Int((a as i128) + b),
            (Self::Bool(a), Self::Float(b))  | (Self::Float(b), Self::Bool(a)) => Self::Float(f128::from(a as i128) + b),
            (Self::Int(a), Self::Int(b))                                       => Self::Int(a + b),
            (Self::Int(a), Self::Float(b))   | (Self::Float(b), Self::Int(a))  => Self::Float(f128::from(a) + b),
            (Self::Float(a), Self::Float(b))                                   => Self::Float(a + b),
            (a@Self::String(_), b)           | (a, b@Self::String(_))          => Self::String(format!("{}{}", a, b)),
            (Self::Error, _)                 | (_, Self::Error)                => Self::Error,
            (Self::ExprQueue, _)             | (_, Self::ExprQueue)            => Self::Error,
            (Self::Array(mut a), Self::Array(mut b))                           => Self::Array( {
                a.append(&mut b);
                a
            } ),
            (Self::Array(_), _)              | (_, Self::Array(_))             => Self::Error,
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs : Self) -> Self::Output {
        match ((&self, &rhs)) {
            (Self::Unit, _)                  | (_, Self::Unit)      => Self::Error,
            (Self::Bool(a), Self::Bool(b))                          => Self::Bool(*a && (! *b)),
            (Self::Bool(a), Self::Int(b))                           => Self::Int((*a as i128) - b),
            (Self::Bool(a), Self::Float(b))                         => Self::Float(f128::from(*a as i128) - b),
            (Self::Int(a), Self::Bool(b))                           => Self::Int(*a - (*b as i128)),
            (Self::Int(a), Self::Int(b))                            => Self::Int(*a - *b),
            (Self::Int(a), Self::Float(b))                          => Self::Float(f128::from(*a) - b),
            (Self::Float(a), Self::Bool(b))                         => Self::Float(*a - f128::from(*b as i128)),
            (Self::Float(a), Self::Int(b))                          => Self::Float(*a - f128::from(*b)),
            (Self::Float(a), Self::Float(b))                        => Self::Float(*a - *b),
            (Self::String(_), _)             | (_, Self::String(_)) => Self::Error,
            (Self::Error, _)                 | (_, Self::Error)     => Self::Error,
            (Self::ExprQueue, _)             | (_, Self::ExprQueue) => Self::Error,
            (Self::Array(_), _)              | (_, Self::Array(_))  => Self::Error
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs : Self) -> Self::Output {
        match ((&self, &rhs)) {
            (Self::Unit, _)                  | (_, Self::Unit)                 => Self::Error,
            (Self::Bool(a), Self::Bool(b))                                     => Self::Bool(*a && *b),
            (Self::Bool(a), Self::Int(b))    | (Self::Int(b), Self::Bool(a))   => Self::Int((*a as i128) * b),
            (Self::Bool(a), Self::Float(b))  | (Self::Float(b), Self::Bool(a)) => Self::Float(f128::from(*a as i128) * b),
            (Self::Int(a), Self::Int(b))                                       => Self::Int(*a * *b),
            (Self::Int(a), Self::Float(b))   | (Self::Float(b), Self::Int(a))  => Self::Float(f128::from(*a) * *b),
            (Self::Float(a), Self::Float(b))                                   => Self::Float(*a * *b),
            (Self::String(a), Self::Int(b))  | (Self::Int(b), Self::String(a)) => if (*b >= 0) { Self::String(a.repeat(*b as usize)) } else { Self::Error },
            (Self::String(_), _)             | (_, Self::String(_))            => Self::Error,
            (Self::Error, _)                 | (_, Self::Error)                => Self::Error,
            (Self::ExprQueue, _)             | (_, Self::ExprQueue)            => Self::Error,
            (Self::Array(a), Self::Int(b))   | (Self::Int(b), Self::Array(a))  => {
                let b = *b as usize;
                let mut out = Vec::with_capacity(a.len() * b);
                for _ in 0..b { out.extend(a.iter().cloned()); }
                Self::Array(out)
            },
            (Self::Array(_), _)              | (_, Self::Array(_))             => Self::Error
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs : Self) -> Self::Output {
        match ((&self, &rhs)) {
            (Self::Unit, _)                  | (_, Self::Unit)                 => Self::Error,
            (Self::Bool(a), Self::Bool(b))                                     => if (*b ) { Self::Bool(*a) } else { Self::Error }, 
            (Self::Bool(_), Self::Int(_))                                      => Self::Error, 
            (Self::Int(a), Self::Bool(b))                                      => if (*b) { Self::Int(*a) } else { Self::Error }, 
            (Self::Bool(_), Self::Float(_))                                    => Self::Error, 
            (Self::Float(a), Self::Bool(b))                                    => if (*b) { Self::Float(*a) } else { Self::Error }, 
            (Self::Int(a), Self::Int(b))                                       => Self::Int(*a / *b), 
            (Self::Int(a), Self::Float(b))                                     => Self::Float(f128::from(*a) / *b), 
            (Self::Float(a), Self::Int(b))                                     => Self::Float(*a / f128::from(*b)), 
            (Self::Float(a), Self::Float(b))                                   => Self::Float(*a / *b), 
            (Self::String(_), _)             | (_, Self::String(_))            => Self::Error,
            (Self::Error, _)                 | (_, Self::Error)                => Self::Error,
            (Self::ExprQueue, _)             | (_, Self::ExprQueue)            => Self::Error,
            (Self::Array(_), _)              | (_, Self::Array(_))             => Self::Error
        }
    }
}

impl Not for Value {
    type Output = Value;
    fn not(self) -> Self::Output {
        match (self) {
            Self::Bool(b) => Self::Bool(!b),
            Self::Unit => Self::Error,
            Self::Int(i) => Self::Int(-i),
            Self::Float(f) => Self::Float(-f),
            Self::String(s) => Self::String(s.chars().rev().collect::<String>()),
            Self::Error => Self::Unit, // TODO: yes, we're doing this
            Self::ExprQueue => Self::Error,
            Self::Array(mut arr) => {
                arr.reverse();
                Self::Array(arr)
            }
        }
    }
}

impl Rem for Value {
    type Output = Value;
    fn rem(self, rhs : Self) -> Self::Output {
        match ((&self, &rhs)) {
            (Self::Unit, _)                  | (_, Self::Unit)                 => Self::Error,
            (Self::Bool(_), _)               | (_, Self::Bool(_))              => Self::Error,
            (Self::Int(a), Self::Int(b))                                       => Self::Int(*a % *b), 
            (Self::Int(a), Self::Float(b))                                     => Self::Float(f128::from(*a) % *b), 
            (Self::Float(a), Self::Int(b))                                     => Self::Float(*a % f128::from(*b)), 
            (Self::Float(a), Self::Float(b))                                   => Self::Float(*a % *b), 
            (Self::String(_), _)             | (_, Self::String(_))            => Self::Error,
            (Self::Error, _)                 | (_, Self::Error)                => Self::Error,
            (Self::ExprQueue, _)             | (_, Self::ExprQueue)            => Self::Error,
            (Self::Array(_), _)              | (_, Self::Array(_))             => Self::Error
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        match (self) {
            Self::Unit      => write!(f, "unit"),
            Self::Bool(v)   => if (*v) { write!(f, "true") } else { write!(f, "false") },
            Self::Int(v)    => write!(f, "{}", v),
            Self::Float(v)  => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Error     => write!(f, "error"),
            Self::ExprQueue => write!(f, "exprqueue"),
            Self::Array(v)  => {
                write!(f, "[")?;
                for (i, u,) in v.iter().enumerate() {
                    if (i != 0) { write!(f, ", ")?; }
                    write!(f, "{}", u)?;
                }
                write!(f, "]")
            }
        }
    }
}
