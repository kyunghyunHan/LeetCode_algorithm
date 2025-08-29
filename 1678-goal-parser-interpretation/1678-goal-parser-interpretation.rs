impl Solution {
    pub fn interpret(command: String) -> String {
            let command = command.replace("()", "o");
    let command = command.replace("(al)", "al");
    command
    }
}