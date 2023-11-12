use std::num::NonZeroU16;

#[derive(Debug, Clone)]
pub struct  Errors(NonZeroU16);

impl Errors 
{
    pub fn message(&self) -> Option<&'static str> 
    {
        message(self.0.get())
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

        fn message(num: u16) -> Option<&'static str> 
        {
            match num 
            {
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
    (4, INVALID_PASSWORD, "Invalid Password.");
    (5, DATA_ALREDY_IN_USE, "Data Already In Use");
    (6, INVALIDE_CREDENTIALS, "Invalid Credentials");

    (101, INVALID_DATABASE_CONNECTION, "Problem With The Database Connection.");
    (102, INTERNAL_SERVER_ERROR, "Internal Server Error.");
    (103, FAILED_TO_READ_FILE, "Failed To Read File");
    (104, NOT_FOUND, "File Not Found.");
}