#[ derive( PartialEq, Eq, Clone ) ]
/// Represents value which is either `any` or some number
enum Value
{
  Number( i32 ),
  Any,
}

impl std::fmt::Debug for Value
{
  fn fmt( &self, f : &mut std::fmt::Formatter< '_ > ) -> std::fmt::Result
  {
    match self
    {
      Value::Number( n ) => write!( f, "{}", n ),
      Value::Any => write!( f, "`any`" ),
    }
  }
}

impl Value
{
  /// Returns number contained in `Number` consuming `self`.
  /// 
  /// # Panics
  /// 
  /// Panics if self value equals `Any`
  /// 
  /// # Examples
  /// 
  /// ```
  /// let value = Value::Number( 1 );
  /// assert_eq!
  /// (
  ///   value.assume_number(),
  ///   1
  /// );
  /// ```
  /// 
  /// ```should_panic
  /// let value = Value::Any;
  /// assert_eq! // panics
  /// (
  ///   value.assume_number(),
  ///   1
  /// );
  /// ```
  fn assume_number( self ) -> i32
  {
    Option::unwrap(Some(1));

    match self
    {
      Value::Number( n ) => n,
      Value::Any => unreachable!( "Assuming value is a number" ),
    }
  }
}

/// Accepts **sorted** slices of values and returns
/// vector of a numbers in `available` slice that
/// present in `allowed` and closest to `preferred`.
/// 
/// # Examples
/// 
/// ```
/// # use Value::*;
/// assert_eq!
/// (
///   attempt
///   (
///     &[ 240, 360, 720 ],
///     &[ Number( 360 ), Number( 720 ) ],
///     &[ Number( 1080 ) ],
///   ),
///   vec![ 720 ],
/// );
/// ```
fn attempt( available : &[ i32 ], allowed : &[ Value ], preferred : & [ Value ] ) -> Vec< i32 >
{
  find_preferred( filter_allowed( available.to_vec(), allowed.to_vec() ), preferred.to_vec() )
}

/// Accepts **sorted** `Vec`s of values and returns `Vec` of numbers
/// present in both `available` and `allowed`. If `allowed` contains
/// [`Value::Any`], all numbers are allowed.
/// 
/// # Examples
/// 
/// ```
/// # use Value::*;
/// assert_eq!
/// (
///   filter_allowed
///   (
///     vec![ 240, 360, 720 ],
///     vec![ Number( 360 ), Number( 720 ) ],
///   ),
///   vec![ 720 ],
/// );
/// ```
/// 
/// ```
/// # use Value::*;
/// assert_eq!
/// (
///   filter_allowed
///   (
///     vec![ 240, 360, 720 ],
///     vec![ Number( 360 ), Any ],
///   ),
///   vec![ 240, 360, 720 ],
/// );
/// ```
fn filter_allowed( available : Vec< i32 >, allowed : Vec< Value > ) -> Vec< i32 >
{
  if allowed.iter().any( | x | *x == Value::Any )
  {
    available
  }
  else
  {
    let mut result = vec![];
    let mut available = available.into_iter().peekable();
    let mut allowed = allowed.into_iter().map( | x | x.assume_number() ).peekable();

    loop
    {
      match ( available.peek(), allowed.peek(), )
      {
        ( Some( &av ), Some( &al ), ) => match av.cmp( &al )
        {
          std::cmp::Ordering::Greater =>
          {
            allowed.next();
          }
          std::cmp::Ordering::Less =>
          {
            available.next();
          }
          std::cmp::Ordering::Equal =>
          {
            result.push( av );
            available.next();
            allowed.next();
          }
        },
        _ => break,
      }
    }

    result
  }
}

/// Accepts **sorted** `Vec`s of values and returns `Vec` of numbers
/// present in `available` and equal or greater (or smaller if no
/// such values are present) to those in `preferred`. If `preferred`
/// contains [`Value::Any`], all numbers are preferred.
/// 
/// # Examples
/// 
/// ```
/// # use Value::*;
/// assert_eq!
/// (
///   filter_preferred
///   (
///     vec![ 240, 360, 1080 ],
///     vec![ Number( 360 ), Number( 720 ) ],
///   ),
///   vec![ 360, 1080 ],
/// );
/// ```
/// 
/// ```
/// # use Value::*;
/// assert_eq!
/// (
///   filter_preferred
///   (
///     vec![ 240, 360, 720 ],
///     vec![ Number( 360 ), Any ],
///   ),
///   vec![ 240, 360, 720 ],
/// );
/// ```
fn find_preferred( available : Vec< i32 >, preferred : Vec< Value > ) -> Vec< i32 >
{
  if preferred.iter().any( | x | *x == Value::Any )
  {
    available.to_vec()
  }
  else
  {
    let mut result = vec![];

    for pref in preferred.into_iter().map( | x | x.assume_number() )
    {
      let mut index = available.partition_point( | x | *x < pref );
      if index > 0 && available.len() == index
      {
        index -= 1;
      }
      if !available.is_empty()
      {
        result.push( available[ index ] );
      }
    }

    result.dedup();
    result
  }
}

/// Prints out function arguments and result returned by
/// `attempt` function with given arguments.
fn print_attempt( available : &[ i32 ], allowed : &[ Value ], preferred : &[ Value ] )
{
  print!( "available : [ " );
  available.iter().for_each( | x | print!( "{:?}, ", x ) );
  println!( "]" );
  print!( "allowed   : [ " );
  allowed.iter().for_each( | x | print!( "{:?}, ", x ) );
  println!( "]" );
  print!( "preferred : [ " );
  preferred.iter().for_each( | x | print!( "{:?}, ", x ) );
  println!( "]" );
  let output = attempt(available, allowed, preferred);
  print!( "returns   : [ " );
  output.iter().for_each( | x | print!( "{:?}, ", x ) );
  println!( "]" );
  println!();
}

fn main()
{
  use Value::*;

  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 360 ), Number( 720 ) ],
    &[ Number( 1080 ) ],
  );
  print_attempt
  (
    &[ 240, 720 ],
    &[ Number( 360 ), Number( 720 ) ],
    &[ Number( 1080 ) ],
  );
  print_attempt
  (
    &[ 240 ],
    &[ Number( 360 ), Number( 720 ) ],
    &[ Number( 1080 ) ],
  );
  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 240 ), Number( 360 ), Number( 720 ), Number( 1080 ) ],
    &[ Number( 240 ), Number( 360 ) ],
  );
  print_attempt
  (
    &[ 240, 720 ],
    &[ Number( 240 ), Number( 360 ), Number( 720 ), Number( 1080 ) ],
    &[ Number( 240 ), Number( 360 ) ],
  );
  print_attempt
  (
    &[ 240, 720 ],
    &[ Number( 240 ), Number( 360 ), Number( 1080 ) ],
    &[ Number( 240 ), Number( 360 ) ],
  );
  print_attempt
  (
    &[ 720 ],
    &[ Number( 240 ), Number( 360 ), Number( 1080 ) ],
    &[ Number( 240 ), Number( 360 ) ],
  );
  print_attempt
  (
    &[ 240, 360 ],
    &[ Number( 240 ), Number( 360 ) ],
    &[ Number( 720 ), Number( 1080 ) ],
  );
  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 360 ), Any ],
    &[ Number( 360 ), Number( 720 ) ],
  );
  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 240 ), Number( 360 ), Number( 720 ) ],
    &[ Any, Number( 720 ) ],
  );
  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 360 ), Number( 1080 ) ],
    &[ Any, Number( 720 ) ],
  );
  print_attempt
  (
    &[ 240, 360, 720 ],
    &[ Number( 1080 ) ],
    &[ Any, Number( 720 ) ],
  );
}

#[ cfg( test ) ]
mod tests
{
  use crate::attempt;
  use crate::Value::*;

  #[ test ]
  fn test1()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 360 ), Number( 720 ) ],
        &[ Number( 1080 ) ],
      ),
      vec![ 720 ],
    );
  }

  #[ test ]
  fn test2()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 720 ],
        &[ Number( 360 ), Number( 720 ) ],
        &[ Number( 1080 ) ]
      ),
      vec![ 720 ],
    );
  }

  #[ test ]
  fn test3()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240 ],
        &[ Number( 360 ), Number( 720 ) ],
        &[ Number( 1080 ) ]
      ),
      vec![],
    );
  }

  #[ test ]
  fn test4()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 240 ), Number( 360 ), Number( 720 ), Number( 1080 ) ],
        &[ Number( 240 ), Number( 360 ) ],
      ),
      vec![ 240, 360 ],
    );
  }

  #[ test ]
  fn test5()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 720 ],
        &[ Number( 240 ), Number( 360 ), Number( 720 ), Number( 1080 ) ],
        &[ Number( 240 ), Number( 360 ) ],
      ),
      vec![ 240, 720 ],
    );
  }

  #[ test ]
  fn test6()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 720 ],
        &[ Number( 240 ), Number( 360 ), Number( 1080 ) ],
        &[ Number( 240 ), Number( 360 ) ],
      ),
      vec![ 240 ],
    );
  }
  
  #[ test ]
  fn test7()
  {
    assert_eq!
    (
      attempt
      (
        &[ 720 ],
        &[ Number( 240 ), Number( 360 ), Number( 1080 ) ],
        &[ Number( 240 ), Number( 360 ) ],
      ),
      vec![],
    );
  }

  #[ test ]
  fn test8()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360 ],
        &[ Number( 240 ), Number( 360 ) ],
        &[ Number( 720 ), Number( 1080 ) ],
      ),
      vec![ 360 ],
    );
  }

  // `any` tests
  #[ test ]
  fn test_any1()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 360 ), Any ],
        &[ Number( 360 ), Number( 720 ) ],
      ),
      vec![ 360, 720 ],
    );
  }

  #[ test ]
  fn test_any2()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 240 ), Number( 360 ), Number( 720 ) ],
        &[ Any, Number( 720 ) ],
      ),
      vec![ 240, 360, 720 ],
    );
  }

  #[ test ]
  fn test_any3()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 360 ), Number( 1080 ) ],
        &[ Any, Number( 720 ) ],
      ),
      vec![ 360 ],
    );
  }

  #[ test ]
  fn test_any4()
  {
    assert_eq!
    (
      attempt
      (
        &[ 240, 360, 720 ],
        &[ Number( 1080 ) ],
        &[ Any, Number( 720 ) ],
      ),
      vec![],
    );
  }
}
