fn encode(string: Vec<String>) -> String {
    let mut encoded = String::new() ;

    for s in string {
        encoded.push_str(&format!("{}:{}", s.len(), s)) ;   // ["rust", "dsa"] -> "4:rust3:dsa" 
    }
    encoded
}

fn decode(ecnoded: String) -> Vec<String> {
    let mut result = Vec::new() ;
    let mut i = 0 ;
    let chars: Vec<char> = ecnoded.chars().collect() ;


    while i < chars.len() {
        let mut length_str = String::new() ;
        while chars[i] != ':' {
            length_str.push(chars[i]) ;
            i += 1 ;
        }
        i += 1 ;    // skip the colon
        let lenght: usize = length_str.parse().unwrap() ;   // converting string length to integer value 

        let decoded_str: String = chars[i.. i + lenght].iter().collect() ;
        result.push(decoded_str) ;

        i += lenght ;
    } 
    result 
}

fn main() {
    let strings = vec![
        "hello".to_string(),
        "world".to_string(),
        "rust".to_string(),
        "is".to_string(),
        "awesome".to_string(),
    ];

    // Encode the list of strings
    let encoded = encode(strings.clone());
    println!("Encoded: {}", encoded);

    // Decode the encoded string
    let decoded = decode(encoded);
    println!("Decoded: {:?}", decoded);

    assert_eq!(strings, decoded); // Verify correctness
}
