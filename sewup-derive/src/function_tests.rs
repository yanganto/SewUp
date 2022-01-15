use crate::*;
use hex_literal::*;

#[test]
fn test_function_signature() {
    let mut sig: [u8; 4] = hex!("c48d6d5e");
    assert_eq!(get_function_signature("sendMessage(string,address)"), sig);
    sig = hex!("70a08231");
    assert_eq!(get_function_signature("balanceOf(address)"), sig);
}

#[test]
fn test_parse_fn_attr() {
    // without attr
    assert_eq!(
        parse_fn_attr("".to_string(), "".to_string()),
        Ok((None, "{}".to_string()))
    );

    // with hex attr only
    assert_eq!(
        parse_fn_attr("".to_string(), "a9059cbb".to_string()),
        Ok((Some("a9059cbb".to_string()), "{}".to_string()))
    );

    // with hex attr only
    assert_eq!(
        parse_fn_attr("".to_string(), "a9059cbb, ".to_string()),
        Ok((Some("a9059cbb".to_string()), "{}".to_string()))
    );

    // with hex attr and full abijson
    assert_eq!(parse_fn_attr("".to_string(), r#"
      a9059cbb,
      constant=false,
      inputs=[
            { "internalType": "address", "name": "recipient", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
      ],
      name=transfer,
      outputs=[
            { "internalType": "bool", "name": "", "type": "bool" }
      ],
      payable=false,
      stateMutability=nonpayable,
      type=function
    "#.to_string()), Ok((Some("a9059cbb".to_string()),
    r#"{"constant":false,"inputs":[{"internalType":"address","name":"recipient","type":"address"},"#.to_owned() +
    r#"{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"transfer","# +
    r#""outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"# +
    r#""stateMutability":"nonpayable","type":"function"}"#)));

    // with hex attr and full abijson
    assert_eq!(parse_fn_attr("transfer_to".to_string(), r#"
      a9059cbb,
      inputs=[
            { "internalType": "address", "name": "recipient", "type": "address" },
            { "internalType": "uint256", "name": "amount", "type": "uint256" }
      ],
      outputs=[
            { "internalType": "bool", "name": "", "type": "bool" }
      ],
      stateMutability=nonpayable
    "#.to_string()), Ok((Some("a9059cbb".to_string()),
    r#"{"constant":false,"inputs":[{"internalType":"address","name":"recipient","type":"address"},"#.to_owned() +
    r#"{"internalType":"uint256","name":"amount","type":"uint256"}],"name":"transferTo","# +
    r#""outputs":[{"internalType":"bool","name":"","type":"bool"}],"payable":false,"# +
    r#""stateMutability":"nonpayable","type":"function"}"#)));
}

#[test]
fn test_parse_fn_attr_validation() {
    assert_eq!(
        parse_fn_attr(
            "transfer_to".to_string(),
            r#"
      a9059cbb,
      inputs=[{}]
    "#
            .to_string()
        ),
        Err("inputs are not valid format")
    );

    assert_eq!(
        parse_fn_attr(
            "transfer_to".to_string(),
            r#"
      a9059cbb,
      outputs=[{"should_not_heler": true}]
    "#
            .to_string()
        ),
        Err("outputs are not valid format")
    );
}