enum LogLevel{
    Info,
    Warning,
    Error,
}
struct LogMsg<'a> {
    level: LogLevel,
    msg: &'a str,
}
fn log(msg: LogMsg){
    match msg.level {
        LogLevel::Info => println!("[INFO]: {}",msg.msg),
        LogLevel::Error => println!("[Error]: {}",msg.msg),
        LogLevel::Warning => println!("[Warning]: {}",msg.msg),
    }
}
fn main() {
    let info = LogMsg{level: LogLevel::Info,msg: "Server listening on port 8080"};
    log(info);
    let err = LogMsg{level: LogLevel::Error,msg: "Segmentation fault"};
    log(err);
    let warn = LogMsg{level: LogLevel::Warning,msg: "Config file not found, proceeding with default values"};
    log(warn);

}
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn log_an_error() {
//         let msg = LogMsg{level: LogLevel::Error,msg: "Segmentation Fault"};
//         assert_eq!(result, 4);
//     }
// }