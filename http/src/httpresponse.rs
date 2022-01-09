use std::{
    collections::HashMap,
    io::{Result, Write},
};
#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HashMap<&'a str, &'a str>>,
    body: Option<String>,
}

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1",
            status_code: "200",
            status_text: "OK",
            headers: None,
            body: None,
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(resp: HttpResponse) -> Self {
        let resp_tmp = resp.clone();
        format! {
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &resp_tmp.version(),
            &resp_tmp.status_code(),
            &resp_tmp.status_text(),
            &resp_tmp.headers(),
            &resp_tmp.body().len(),
            &resp_tmp.body(),
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HashMap<&'a str, &'a str>>,
        body: Option<String>,
    ) -> Self {
        let mut response = HttpResponse::default();
        response.status_code = status_code;
        response.body = body;
        response.headers = match &headers {
            Some(_) => headers,
            None => {
                let mut new_header = HashMap::new();
                new_header.insert("Content-Type", "text/html");
                Some(new_header)
            }
        };
        response.status_text = match response.status_code {
            "200" => "OK",
            "400" => "Bad Request",
            "404" => "Not Found",
            "500" => "Internal Server Error",
            _ => "Not Found",
        };
        response
    }

    pub fn send_response(&self, write_stream: &mut impl Write) -> Result<()> {
        let res = self.clone();
        let resp_string = String::from(res);
        let _ = write!(write_stream, "{}", resp_string);
        Ok(())
    }

    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let map = self.headers.clone().unwrap();
        let mut header_string = "".into();
        for (k, v) in map {
            header_string = format!("{}{}:{}\r\n", header_string, k, v);
        }
        header_string
    }

    pub fn body(&self) -> &str {
        match &self.body {
            Some(v) => v.as_str(),
            None => "",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_struct_creation_200() {
        let response_actual = HttpResponse::new("200", None, Some("xxxx".into()));
        let response_expected = HttpResponse {
            version:"HTTP/1.1",
            status_code:"200",
            status_text:"OK",
            headers: Some(HashMap::from([
                ("Content-Type", "text/html")
            ])),
            body: Some("xxxx".into())
        };
        assert_eq!(response_expected, response_actual);
        assert_eq!(String::from(response_expected), String::from(response_actual));
    }
}
