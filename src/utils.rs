macro_rules! single {
  ($( $first:ident $($second:ident)? ),* $(,)?) => { $($crate::utils::single!(@self $first $($second)?);)* };
  (@self $query:ident) => {
    let Ok($query) = $query.get_single() else {
      return;
    };
  };
  (@self mut $query:ident) => {
    let Ok(mut $query) = $query.get_single_mut() else {
      return;
    };
  };
}

pub(crate) use single;
