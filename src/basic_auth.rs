use rocket::{request::{FromRequest, Outcome}, http::Status};

#[derive(Debug)]
pub struct BasicAuthStruct {
    pub username: String,
    pub password: String,
}

impl BasicAuthStruct {
    fn from_header(header: &str) -> Option<BasicAuthStruct> {
        let split = header.split(" ").collect::<Vec<_>>();
        if split.len() != 2 {
            None
        }else if split[0] != "Basic" {
            None
        }else {
            Self::from_base64(split[1])
        }
    }

    fn from_base64(base64_string: &str) -> Option<BasicAuthStruct> {
        let decoder = base64::decode(base64_string).ok()?;
        let decoder_str = String::from_utf8(decoder).ok()?;
        let split = decoder_str.split(":").collect::<Vec<_>>();
        if split.len() != 2 {
            None
        }else {
            let (username, password) = (split[0], split[1]);
            Some(BasicAuthStruct { username: username.to_string(), password: password.to_string() })
        }
    }

}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuthStruct {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
        if let Some(header_auth) = header_auth {
            if let Some(auth) = Self::from_header(header_auth) {
                return Outcome::Success(auth);
            }
        }
        Outcome::Failure((Status::Unauthorized, ()))
    }
}