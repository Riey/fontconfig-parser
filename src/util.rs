macro_rules! try_opt {
    ($opt:expr, $($tt:tt)*) => {
        match $opt {
            Some(t) => t,
            None => Err(crate::Error::UnexpectedEof(format!($($tt)*))),
        }
    };
}

macro_rules! parse_attrs {
    ($tokens:expr, { $($key:expr => $lvalue:expr,)+ } $(, { $($str_key:expr => $str_lvalue:expr,)+ } )?) => {
        for attr in take_attrs!($tokens) {
            let (key, value) = attr?;

            match key {
                $(
                    $key => $lvalue = value.parse()?,
                )+
                $(
                    $(
                        $str_key => $str_lvalue = value,
                    )+
                )?
                _ => {}
            }
        }
    };
}

macro_rules! take_attrs {
    ($tokens:expr) => {
        $tokens
            .take_while(|t| {
                !matches!(
                    t,
                    Ok(Token::ElementEnd {
                        end: ElementEnd::Open,
                        ..
                    })
                )
            })
            .filter_map(|t| match t {
                Ok(Token::Attribute { local, value, .. }) => {
                    Some(Ok((local.as_str(), value.as_str())))
                }
                Ok(_) => None,
                Err(err) => Some(Err(err)),
            })
    };
}

macro_rules! take_while_end {
    ($tokens:expr, $tag:expr) => {
        let mut depth = 1;
        loop {
            match try_opt!($tokens.next(), "Expect: {}", $tag)? {
                Token::ElementStart { local, .. } if local.as_str() == $tag => {
                    depth += 1;
                }
                Token::ElementEnd {
                    end: ElementEnd::Close(_, e),
                    ..
                } if e.as_str() == $tag => {
                    depth -= 1;

                    if depth == 0 {
                        break;
                    }
                }
                _ => {}
            }
        }
    };
}

macro_rules! parse_enum {
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
        |$arg:ident| $fallback:expr,
    ) => {
        impl core::str::FromStr for $ty {
            type Err = crate::Error;

            fn from_str($arg: &str) -> crate::Result<$ty> {
                match $arg {
                    $(
                        $text => Ok(<$ty>::$variant),
                    )+
                    _ => {
                        $fallback
                    }
                }
            }
        }
    };
    (
        $ty:ty,
        $(
            ($variant:ident, $text:expr),
        )+
    ) => {
        parse_enum! {
            $ty,
            $(
                ($variant, $text),
            )+
            |s| Err(crate::Error::ParseEnumError(core::any::type_name::<$ty>(), s.into())),
        }
    };
}
