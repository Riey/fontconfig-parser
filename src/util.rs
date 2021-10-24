macro_rules! try_opt {
    ($opt:expr, $($tt:tt)*) => {
        match $opt {
            Some(t) => t,
            None => return Err(crate::Error::UnexpectedEof(format!($($tt)*))),
        }
    };
}

macro_rules! try_text {
	($node:expr) => {
        match $node.text() {
            Some(t) => t,
            None => return Err(crate::Error::InvalidFormat),
        }
	};
}

macro_rules! parse_attrs {
    ($node:expr, { $($key:expr => $lvalue:expr,)+ } $(, { $($str_key:expr => $str_lvalue:expr,)+ } )?) => {
        for attr in $node.attributes() {
            match attr.name() {
                $(
                    $key => $lvalue = attr.value().parse()?,
                )+
                $(
                    $(
                        $str_key => $str_lvalue = attr.value().into(),
                    )+
                )?
                _ => {}
            }
        }
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
