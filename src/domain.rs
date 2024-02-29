use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct NewSubscriber {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 256, message = "Can not be empty"))]
    pub name: String,
}
