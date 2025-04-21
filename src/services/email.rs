#![allow(dead_code)]
#![allow(unused_variables)]
// Email service.
// Provide the email service in here
pub struct EmailService {

}

impl EmailService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn send_email<T>(&self, to: T, cc: Vec<T>, bcc: Vec<T>, subject: T ,content: T) -> Result<(), ()>
    where T: ToString
    {
        Ok(())
    }
}