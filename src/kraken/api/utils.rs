use data_encoding::{BASE64, HEXUPPER};
use log::{debug, trace};
use ring::hmac;
use ring::{
    digest::{Context, Digest, SHA256},
    hmac::Tag,
};
use std::io::{BufRead, Read, Write};
use std::{collections::HashMap, fs::File};

pub fn create_signature(
    url_path: &str,
    params: &HashMap<String, String>,
    secret: &[u8],
) -> Result<String, String> {
    let nonce: &str = if params.contains_key("nonce") {
        params.get("nonce").unwrap()
    } else {
        return Err(String::from("Nonce is not set"));
    };

    // Get Sha Sum
    let params = urlencode_hashmap(&params);
    let mut message = String::new();
    message.push_str(nonce);
    message.push_str(&params);
    let sha_sum = get_sha256(message.as_bytes());

    // Get HMAC Sum
    let hmac_msg_bytes: Vec<u8> = url_path
        .as_bytes()
        .to_owned()
        .into_iter()
        .chain(sha_sum.as_ref().to_owned().into_iter())
        .collect();

    let hmac_sum = get_hmac_sha512(&hmac_msg_bytes, secret);

    Ok(BASE64.encode(hmac_sum.as_ref()))
}

pub fn get_sha256(input: &[u8]) -> Digest {
    let mut context = Context::new(&SHA256);
    context.update(input);
    context.finish()
}

pub fn get_hmac_sha512(message: &[u8], secret: &[u8]) -> Tag {
    let key = hmac::Key::new(hmac::HMAC_SHA512, secret);
    hmac::sign(&key, message)
}

pub fn urlencode_hashmap(params: &HashMap<String, String>) -> String {
    params
        .iter()
        .map(|(key, val)| format!("{}={}", urlencoding::encode(key), urlencoding::encode(val)))
        .collect::<Vec<String>>()
        .join("&")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_encode_hash_map() {
        let mut params = HashMap::new();
        params.insert("bar".to_owned(), "baz".to_owned());
        params.insert("foo".to_owned(), "quux".to_owned());
        assert!(
            urlencode_hashmap(&params) == "bar=baz&foo=quux"
                || urlencode_hashmap(&params) == "foo=quux&bar=baz"
        );
    }

    #[test]
    fn test_url_encode_hash_map_with_space() {
        let mut params = HashMap::new();
        params.insert("bar".to_owned(), "baz ".to_owned());
        params.insert("foo".to_owned(), "quux".to_owned());
        assert!(
            urlencode_hashmap(&params) == "bar=baz%20&foo=quux"
                || urlencode_hashmap(&params) == "foo=quux&bar=baz%20"
        );
    }

    #[test]
    fn test_sha256() {
        let msg = "We will generate a digest of this text";
        assert_eq!(
            HEXUPPER.encode(get_sha256(msg.as_bytes()).as_ref()),
            "81700022B5CAB8EFC79F276B69D17251B03FFCDAB61C026B75F783B55E3953CB"
        );
    }

    #[test]
    fn test_hmac512() {
        let msg = "the message to hash here";
        let secret = "the shared secret key here";
        assert_eq!(
            HEXUPPER.encode(get_hmac_sha512(msg.as_bytes(), secret.as_bytes()).as_ref()),
            "7A0888E5BFFEE55D524189C936D2DC2BB4A770F58705F3722561736BAF40C7C1648E101DACEC932293BB127F65D3F7E4F0EB0613DEB5B9C8795C730DD756025F"
        );
    }

    #[test]
    fn test_hmac521_base64_key() {
        let msg = "hello";
        let secret_bytes = BASE64
            .decode("P3sd+5Bj8aZEyyMTmKi1WUIucox64jeOt1lRNEPYBdgLWwnxRMVd5JJa".as_bytes())
            .expect("Not able to decode Kraken api secret");

        assert_eq!(
            BASE64.encode(get_hmac_sha512(msg.as_bytes(), &secret_bytes).as_ref()),
            "u/GRN8FSVdT5QG0xBmH6MxU6gfPaeQBY7DxKIa3BuzbEzi7GVcFl8PteGxXBhetO5feAkFcPVWIw7Z1Dj0KrQA=="
        );
    }

    #[test]
    fn test_create_signature() {
        let mut params = HashMap::new();
        params.insert("nonce".to_owned(), "233".to_owned());

        let secret_bytes = BASE64
            .decode("P3sd+5Bj8aZEyyMTmKi1WUIucox64jeOt1lRNEPYBdgLWwnxRMVd5JJa".as_bytes())
            .expect("Not able to decode Kraken api secret");
        assert_eq!(
            create_signature("private/Balance", &params, &secret_bytes).unwrap(),
            "NptMbFe4CUW53NMPTPlbHxZXFFEpaN+5iPD00e9IL2ydNrf4P1pbEm4Vdr5gLLn8ZnorZFVzyHCgPCkYih1tJA=="
        );
    }
}
