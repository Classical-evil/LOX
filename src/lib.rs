pub mod Scanner;
mod Token;
mod TokenType;

pub fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, postion: &str, message: &str) {
    println!("[line{}Error]{}:{}", line, postion, message);
}

#[cfg(test)]
mod tests {
    #[test]
    fn identifiers_test() {
        println!("#####Identifiers test begin...............");
        scanner_test("test_lox/ch4/identifiers.lox");
        println!("#####Identifiers test ok..................");
    }

    #[test]
    fn keywords_test() {
        println!("#####keywords test begin...............");
        scanner_test("test_lox/ch4/keywords.lox");
        println!("#####keywords test ok..................");
    }

    #[test]
    fn numbers_test() {
        println!("#####numbers test begin...............");
        scanner_test("test_lox/ch4/numbers.lox");
        println!("#####numbers test ok..................");
    }

    #[test]
    fn punctuators_test() {
        println!("#####punctuators test begin...............");
        scanner_test("test_lox/ch4/punctuators.lox");
        println!("#####punctuators test ok..................");
    }

    #[test]
    fn strings_test() {
        println!("#####strings test begin...............");
        scanner_test("test_lox/ch4/strings.lox");
        println!("#####strings test ok..................");
    }

    #[test]
    fn whitespaces_test() {
        println!("#####whitespaces test begin...............");
        scanner_test("test_lox/ch4/whitespaces.lox");
        println!("#####whitespaces test ok..................");
    }

    fn scanner_test(path: &str) {
        use crate::Scanner::Scanner;
        use std::fs;
        use std::path::Path;

        let file_path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join(path);
        let file_content = fs::read_to_string(&file_path)
            .unwrap_or_else(|e| panic!("Read file:{} failed. Path:{:?}", e, file_path,));
        let mut contents = Vec::new();
        for line in file_content.lines() {
            let trimmed_line = line.trim();
            if trimmed_line.starts_with("// expect:") {
                let expect_content = trimmed_line.strip_prefix("// expect:").unwrap().trim();
                contents.push(expect_content.to_string());
            }
        }
               let mut scanner = Scanner::new(&file_content);
        let tokens = scanner.scanTokens();
        for (idx, (token, line)) in tokens.into_iter().zip(contents.into_iter()).enumerate() {
            let s = format!("{token}");
            assert_eq!(
                s,
                line,
                "Line {} match failed, output:{}, expected:{}", 
                idx + 1,
                s,
                line
            )
        }
    }
}
