
#[macro_export]
macro_rules! deref {
    ($ty:ty => $out:ty) => {
        impl std::ops::Deref for $ty {
            type Target = $out;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $ty {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}


#[macro_export]
macro_rules! from {
    ({$($from:ty => $var:expr),+} =>$ty:ty) 
    => {
        $(impl From<$from> for $ty {
            fn from(value: $from) -> Self {
                $var(value)
            }
        })+
    };
}

