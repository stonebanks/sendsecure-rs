pub mod SendSecure {
    
    use std::cmp::Ordering;
    use hyper::{client,error,status};
    use std::io::Read;
    use std::result::Result;
    use error::SendSecure;

    pub struct Client;

    impl Client {
        pub fn get_user_token (
            enterprise_account: &str,
            username: &str,
            password: &str,
            device_id: &str,
            device_name: &str,
            application_type: &str,
            endpoint: &str,
            one_time_password: bool ) -> SendSecure::SendSecureResult<String>
        {
            // let formatted_string = format!("{0}/services/{1}/portal/host", endpoint, enterprise_account);
            let url: &str = "https://httpbin.org/status/404";//"https://secure.bixi.com/data/stations.json";
            let client = client::Client::new();
            let mut res = try!(client.get(url).send()
                .map_err(|error| SendSecure::SendSecureError::new("0".to_string(), "Unexpected server Error".to_string(), SendSecure::SendSecureKind::UnexpectedServerError(error))));
            let status_code = res.status.class().default_code();
            res = try!(match status_code.cmp(&status::StatusCode::BadRequest) {
                Ordering::Less => Ok(res),
                Ordering::Greater | Ordering::Equal => Err(SendSecure::SendSecureError::new(res.status.canonical_reason().unwrap().to_string(), "message: String".to_string(), SendSecure::SendSecureKind::RequestError(status_code))/*SendSecureKind::RequestError(res.status.class().default_code())*/)
            });
            let mut body = String::new();
            try!(res.read_to_string(&mut body)
                    .map_err(|err| SendSecure::SendSecureError::new(res.status.canonical_reason().unwrap().to_string(), "message: String".to_string(), SendSecure::SendSecureKind::IoError(err))));
            Ok::<String, SendSecure::SendSecureError>(body)
        }
    }
}
