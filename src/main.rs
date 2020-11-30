use calc::eval_str;
use std::{fs::File, io, iter, rc::Rc, sync::Arc, sync::Mutex, thread, time::Duration};

fn main() -> io::Result<()> {
    let mut buf = String::new();

    loop {
        buf.clear();
        let n = io::stdin().read_line(&mut buf)?;
        if n == 0 {
            break;
        }
        match eval_str(&buf) {
            Ok(value) => println!(" = {}", value),
            Err(err) => println!(" error: {}", err),
        }
    }

    Ok(())
}
