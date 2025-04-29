#![feature(never_type)]



/// Prints the given message to the console.
pub fn print(message : String) -> () { docs_only() }


/// Returns the current instruction queue.
pub fn queue() -> &'static mut Vec<String> { docs_only() }


/// If the given condition is true, when_true is returned, else when_false.
pub fn r#if<T>(condition : bool, when_true : T, when_false : T) -> T { docs_only() }


/// Gets a single entry from the given array.
pub fn get<T>(array : Array<T>, line : usize) -> String { docs_only() }

/// Gets multiple entries from the given array.
///
/// Start is inclusinve, end is exclusive.
pub fn gets<T>(array : Array<T>, start_line : usize, end_line : UInt) -> String { docs_only() }


/// Pushes a single entry at the end of the given array.
///
/// Returns the index at which it was placed.
pub fn push<T>(array : &mut Array<T>, instruction : String) -> UInt { docs_only() }

/// Pushes multiple entries at the end of the given array.
///
/// Returns the index at which the entry was placed.
pub fn pushes<T>(array : &mut Array<T>, instructions : Vec<String>) -> UInt { docs_only() }


/// Inserts a single entry at some position in the given array.
pub fn insert<T>(array : &mut Array<T>, line : usize, instruction : String) -> () { docs_only() }

/// Inserts multiple entries at some position in the given
///  array.
pub fn inserts<T>(array : &mut Array<T>, line : usize, instructions : Vec<String>) -> () { docs_only() }


/// Overwrites a single entry at some position in the given
///  array.
///
/// Returns the old instruction.
pub fn set<T>(array : &mut Array<T>, line : usize, instruction : String) -> String { docs_only() }

/// Overwrites multiple entries starting at some position in
///  the given array.
///
/// Returns the old instructions.
pub fn sets<T>(array : &mut Array<T>, start_line : usize, instructions : Vec<String>) -> Vec<String> { docs_only() }


/// Returns the number of elements in the given array.
pub fn len(array : Array<String>) -> UInt { docs_only() }


/// Reads the entirety of a file as a string.
pub fn fsread(fname : String) -> String { docs_only() }

/// Writes the string to a file, overwriting it.
pub fn fswrite(fname : String, content : String) { docs_only() }

/// Reads the entirety of a file as an array of bytes.
pub fn fbread(fname : String) -> Array<UInt> { docs_only() }

/// Writes the an array of bytes to a file, overwriting it.
pub fn fbwrite(fname : String, content : Array<UInt>) { docs_only() }



fn docs_only() -> ! { unreachable!() }

pub type Array<T> = Vec<T>;

pub type UInt  = usize;
pub type Int   = isize;
pub type Float = f64;
