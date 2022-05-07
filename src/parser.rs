use log::*;

// Regex wasent cutting it, and this is pretty simple parsing
pub struct Parser;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct FoundToken {
    pub start: usize,
    pub end: usize, 
}

impl Parser {
    #[allow(dead_code)]
    pub fn find_all_tokens(string: &str) -> Vec<FoundToken> {
        let mut start_index: usize = 0;
        let mut found_tokens: Vec<FoundToken> = Vec::new();

        let mut first_token = Parser::find_first_token(string);
        if first_token == None {
            return found_tokens;
        }
       
        while first_token.is_some() {
            let token = first_token.unwrap();

            start_index += token.end;

            found_tokens.push(token);
            first_token = Parser::find_first_token(&string[start_index..]);
        }

        found_tokens
    }

    pub fn find_first_token(string: &str) -> Option<FoundToken> {

        // This parsing is a little confusing to read, this is organized as a 
        //  small state machine. We start looking for a token when encountering
        //  an open square bracket, then when we keep chugging along until we get
        //  an open curly bracket, or a character that's not alphabetic or '_'. 
        //  When we get into the parameter section of the variable {} we count the 
        //  number of open curlies we get so that we can allow the user to use {} 
        //  in their variables.                                 - Austin Haskell  
        let mut open_square_bracket_found: bool = false;
        let mut is_in_token: bool = false;
        let mut is_in_parameters: bool = false;

        let mut user_curly_brackets_count: usize = 0;

        let mut start_index: usize = 0;

        let mut index: usize = 0;
        for character in string.chars() {
            match character {
                '{' => { 
                    if is_in_parameters {
                        user_curly_brackets_count+=1;
                    } else {
                        if is_in_token {
                            is_in_parameters = true;
                        }
                    }
                }, 
                '}' => { 
                    if is_in_parameters {
                        if user_curly_brackets_count != 0 {
                            user_curly_brackets_count -= 1;
                        } else {
                            is_in_parameters = false;
                        }
                    }
                }, 
                '[' => { 
                    open_square_bracket_found = true; 
                    if !is_in_token {
                        start_index = index;
                    }
                },
                ']' => { 
                    if open_square_bracket_found {
                        if is_in_token && !is_in_parameters{
                            info!("Found token!!!! S:{:}, E:{:}", start_index, index + 1);
                            return Some(FoundToken {
                                start: start_index,
                                end: index + 1
                            });
                        } else {
                            is_in_token = true;
                        }
                    } else {
                        if !is_in_parameters {
                            open_square_bracket_found = false;
                            is_in_token = false;
                        }
                    }
                },
                _ => {

                    if !(character.is_alphabetic() || character == '_') && is_in_token {
                        is_in_token = false;
                    }

                    // Reset the hunt for the starting or ending '['
                    open_square_bracket_found = false;
                }
            }
            index += 1;
        }

        None
    }
}

#[test]
fn no_token_exists_returns_none() {
    let test_string = "Some text but no token is present. ";

    assert!(Parser::find_all_tokens(test_string).is_empty());
}

#[test]
fn no_token_exists_but_brackets_do() {
    let test_string = "some_array[] { // With a function }";

    assert!(Parser::find_all_tokens(test_string).is_empty());
} 

#[test]
fn token_with_no_parameters() {
    let test_string = "[]FILE_NAME_AS_TYPE[]";

    assert_eq!(vec![FoundToken {
        start: 0,
        end: 21
    }], Parser::find_all_tokens(test_string))
}

#[test]
fn token_with_no_parameters_but_with_curly_brackets() {
    let test_string = "[]FILE_NAME_AS_TYPE{}[]";

    assert_eq!(vec![FoundToken {
        start: 0,   
        end: 23
    }], Parser::find_all_tokens(test_string))
}

#[test]
fn big_test_multiple_tokens() {
    let test_string = r"#include '[]PARTNER_FILE[]'
    []FILE_NAME_AS_TYPE[]::[]FILE_NAME_AS_TYPE[]() {
    }
    []FILE_NAME_AS_TYPE[]::~[]FILE_NAME_AS_TYPE[]() {
    }
    []FILE_NAME_AS_TYPE{-Accessor}[]
    ";

    assert_eq!(6, Parser::find_all_tokens(test_string).len())
}

#[test]
fn token_with_sub_tokens_parses() {
    let test_string = "Some other text before []FOR_EACH_FILE_IN_DIR{qml, h|||
        []FILE_NAME_WITHOUT_EXTENSION[] 1.0 []FILE_NAME[]}[] Some other text after".to_string();

    let found_tokens = Parser::find_all_tokens(&test_string);

    //println!("{:}", test_string[found_tokens[0].start..found_tokens[0].end].to_string());
    
    assert_eq!(vec![ FoundToken {
        start: 23,
        end: 93 + 23
    }], found_tokens);
}


#[test]
fn token_with_sub_tokens_and_brackets_parses() {
    let test_string = "Some other text before []FOR_EACH_FILE_IN_DIR{qml, h|||
        []FILE_NAME_WITHOUT_EXTENSION[] {1.0} []FILE_NAME[]}[] Some other text after".to_string();

    let found_tokens = Parser::find_all_tokens(&test_string);

    //println!("{:}", test_string[found_tokens[0].start..found_tokens[0].end].to_string());
    
    assert_eq!(vec![ FoundToken {
        start: 23,
        end: 95 + 23
    }], found_tokens);
}