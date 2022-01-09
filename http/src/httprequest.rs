use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Post,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            // "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl From<String> for HttpRequest {
    fn from(req: String) -> Self {
        let mut parsed_method = Method::Uninitialized;
        let mut parsed_version = Version::Uninitialized;
        let mut parsed_resource = Resource::Path("".to_string());
        let mut parsed_headers = HashMap::new();
        let mut parsed_body = "";

        for line in req.lines() {
            if line.contains("HTTP") {
                let (method, resource, version) = process_req_line(line);
                parsed_method = method;
                parsed_resource = resource;
                parsed_version = version;
            } else if line.contains(":") {
                let (key, value) = process_header_line(line);
                parsed_headers.insert(key, value);
            } else {
                parsed_body = line;
            }
        }
        HttpRequest {
            method: parsed_method,
            version: parsed_version,
            resource: parsed_resource,
            headers: parsed_headers,
            body: parsed_body.to_string(),
        }
    }
}

fn process_req_line(s: &str) -> (Method, Resource, Version) {
    let mut words = s.split_whitespace();
    let method = words.next().unwrap();
    let path = words.next().unwrap();
    let version = words.next().unwrap();
    (
        method.into(),
        Resource::Path(path.to_string()),
        version.into(),
    )
}
fn process_header_line(s: &str) -> (String, String) {
    let headers_items = s.split_once(":");
    let mut key = String::from("");
    let mut value = String::from("");

    if let Some((k, v)) = headers_items {
        key = k.trim().to_string();
        value = v.trim().to_string();
    }

    (key, value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }

    #[test]
    fn test_version_into() {
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
    }
    #[test]
    fn test_read_http() {
        let content = String::from("GET /sd HTTP/1.1\r\nUser-Agent: PostmanRuntime/7.28.4\r\nAccept: */*\r\nPostman-Token: 529edce1-9249-4a63-96b4-127cb0f57a83\r\nHost: 159.75.96.101:7879\r\nAccept-Encoding: gzip, deflate, br\r\nConnection: keep-alive\r\n\r\n");
        let headers_expected = HashMap::from([
            ("Host".into(), "159.75.96.101:7879".into()),
            ("User-Agent".into(), "PostmanRuntime/7.28.4".into()),
            (
                "Postman-Token".into(),
                "529edce1-9249-4a63-96b4-127cb0f57a83".into(),
            ),
            ("Accept-Encoding".into(), "gzip, deflate, br".into()),
            ("Connection".into(), "keep-alive".into()),
            ("Accept".into(), "*/*".into()),
        ]);

        let req: HttpRequest = content.into();
        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/sd".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}
