// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!


pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;
    fn myuppercase(string: &str) -> String {
        string.to_uppercase()
    }

    fn mytrim(string: &str) -> String {
        string.trim().to_string()
    }

    fn append_bar(string: &str, count: usize) -> String {
        let bar_list: Vec<&str> = Vec::new(); //empty vec
        let mut result = String::from(string);
        for _ in 0..count {
            result += "bar";
        }
        result
    }

    // TODO: Complete the function signature!
    // - The input is going to be a Vector of a 2-length tuple,
    // - the first element is the string, the second one is the command.
    // - The output element is going to be a Vector of strings.
    pub fn transformer(input: Vec<(String,Command)>) -> Vec<String> {
        // TODO: Complete the output declaration!
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // TODO: Complete the function body. You can do it!
            // https://doc.rust-lang.org/book/ch06-02-match.html
            match command {
                Command::Uppercase => output.push(myuppercase(&string)),
                Command::Trim => output.push(mytrim(&string)),
                Command::Append(usize) => output.push(append_bar(&string,*usize)),
                // Command::Append(usize) =>  {
                //     let mut ans = String::new();
                //     for i in 0..*usize {
                //         ans += &string.clone();
                //     }
                //     output.push(format!("{}bar", ans));
                // }
            }

        }
        output
    }
}

#[cfg(test)]
mod tests {
    // TODO: What do we need to import to have `transformer` in scope?
    use crate::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        //The output element is going to be a Vector of strings.
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}