extern crate jsonwebtoken;
#[macro_use]
extern crate serde_derive;

use jsonwebtoken::{encode, decode, Algorithm, Header, sign, verify};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String
}

#[test]
fn sign_hs256() {
    let result = sign("hello world", b"secret", Algorithm::HS256);
    let expected = "c0zGLzKEFWj0VxWuufTXiRMk5tlI5MbGDAYhzaxIYjo";
    assert_eq!(result, expected);
}

#[test]
fn verify_hs256() {
    let sig = "c0zGLzKEFWj0VxWuufTXiRMk5tlI5MbGDAYhzaxIYjo";
    let valid = verify(sig, "hello world", b"secret", Algorithm::HS256);
    assert!(valid);
}

#[test]
fn encode_with_custom_header() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string()
    };
    let mut header = Header::default();
    header.kid = Some("kid".to_string());
    let token = encode(header, &my_claims, "secret".as_ref()).unwrap();
    let token_data = decode::<Claims>(&token, "secret".as_ref(), Algorithm::HS256).unwrap();
    assert_eq!(my_claims, token_data.claims);
    assert_eq!("kid", token_data.header.kid.unwrap());
}

#[test]
fn round_trip_claim() {
    let my_claims = Claims {
        sub: "b@b.com".to_string(),
        company: "ACME".to_string()
    };
    let token = encode(Header::default(), &my_claims, "secret".as_ref()).unwrap();
    let token_data = decode::<Claims>(&token, "secret".as_ref(), Algorithm::HS256).unwrap();
    assert_eq!(my_claims, token_data.claims);
    assert!(token_data.header.kid.is_none());
}

#[test]
fn decode_token() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUifQ.I1BvFoHe94AFf09O6tDbcSB8-jp8w6xZqmyHIwPeSdY";
    let claims = decode::<Claims>(token, "secret".as_ref(), Algorithm::HS256);
    claims.unwrap();
}

#[test]
#[should_panic(expected = "InvalidToken")]
fn decode_token_missing_parts() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";
    let claims = decode::<Claims>(token, "secret".as_ref(), Algorithm::HS256);
    claims.unwrap();
}

#[test]
#[should_panic(expected = "InvalidSignature")]
fn decode_token_invalid_signature() {
    let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUifQ.wrong";
    let claims = decode::<Claims>(token, "secret".as_ref(), Algorithm::HS256);
    claims.unwrap();
}

#[test]
#[should_panic(expected = "WrongAlgorithmHeader")]
fn decode_token_wrong_algorithm() {
    let token = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJiQGIuY29tIiwiY29tcGFueSI6IkFDTUUifQ.pKscJVk7-aHxfmQKlaZxh5uhuKhGMAa-1F5IX5mfUwI";
    let claims = decode::<Claims>(token, "secret".as_ref(), Algorithm::HS256);
    claims.unwrap();
}

#[test]
fn decode_token_with_bytes_secret() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwiY29tcGFueSI6Ikdvb2dvbCJ9.27QxgG96vpX4akKNpD1YdRGHE3_u2X35wR3EHA2eCrs";
    let claims = decode::<Claims>(token, b"\x01\x02\x03", Algorithm::HS256);
    assert!(claims.is_ok());
}

#[test]
fn decode_token_with_shuffled_header_fields() {
    let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb21wYW55IjoiMTIzNDU2Nzg5MCIsInN1YiI6IkpvaG4gRG9lIn0.SEIZ4Jg46VGhquuwPYDLY5qHF8AkQczF14aXM3a2c28";
    let claims = decode::<Claims>(token, "secret".as_ref(), Algorithm::HS256);
    assert!(claims.is_ok());
}
