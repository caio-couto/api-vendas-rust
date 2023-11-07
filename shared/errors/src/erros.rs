use std::num::NonZeroU16;

#[derive(Debug, Clone)]
pub struct  Errors(NonZeroU16);

impl Errors 
{
    pub fn canonical_reason(&self) -> Option<&'static str> 
    {
        canonical_reason(self.0.get())
    }    
}

impl From<Errors> for u16 
{
    #[inline]
    fn from(status: Errors) -> u16 
    {
        status.0.get()
    }
}

macro_rules! errors_codes 
{
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => 
    {
        impl Errors
        {
            $(
                $(#[$docs])*
                pub const $konst: Errors = Errors(unsafe { NonZeroU16::new_unchecked($num) });
            )+
        }

        fn canonical_reason(num: u16) -> Option<&'static str> 
        {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

errors_codes!
{
    (1, INVALID_UUID, "Invalid Uuid.");
    (2, SERVER_ERROR, "Internal Server Error.");
    (3, USER_NOT_FOUND, "User Not Found.");
}