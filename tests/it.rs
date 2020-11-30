use std::{
    io::Write,
    process::{Command, Stdio},
};

#[test]
fn test_calc_bin() {
    let path = env!("CARGO_BIN_EXE_calc");
    let mut cmd = Command::new(path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let mut stdin = cmd.stdin.take().unwrap();
        writeln!(stdin, "3 sqr 4 sqr +").unwrap();
        stdin.flush().unwrap();
    }

    let output = cmd.wait_with_output().unwrap();
    assert!(output.status.success());
    assert_eq!(output.stdout.as_slice(), b" = 25\n");
}
