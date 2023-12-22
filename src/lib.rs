#![cfg(test)]

use risc0_zkvm::{default_executor, ExecutorEnv, ExitCode};
use serde::Deserialize;

#[test]
fn problem1() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 1 =====
    // The most basic IO for Risc Zero programs is in the form of reading and writing data from
    // inside the VM. For more information around Risc Zero's IO, see the following links:
    //   - https://dev.risczero.com/api/next/zkvm/developer-guide/guest-code-101#basic-guest-functionality-reading-writing-and-committing
    //   - https://dev.risczero.com/api/next/zkvm/tutorials/hello-world#step-2-host-share-private-data-as-input-with-the-guest
    //
    // This raw data can represent anything, most commonly encoded/serialized data. Can you figure
    // out what type of data this program expects?

    // TODO define the bytes to use as input to make this program execute successfully
    let input_bytes: &[u8] = todo!();
    let env = ExecutorEnv::builder().write_slice(input_bytes).build()?;

    // ===== do not change anything below this line =====
    executor.execute_elf(env, include_bytes!("./bin/problem1.elf"))?;

    Ok(())
}

#[test]
fn problem2() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 2 =====
    // This program expects input in some form to be able to execute. I wonder what options there
    // are available to the VM...

    // TODO update anything about the `ExecutorEnv` to make this pass
    let env = ExecutorEnv::builder().build()?;

    // ===== do not change anything below this line =====
    executor.execute_elf(env, include_bytes!("./bin/problem2.elf"))?;

    Ok(())
}

#[test]
fn problem3() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 3 =====
    // This program has the ability to commit to data, representing the public output of a proof.
    // Can you figure out the pattern of how this data is output?

    // TODO define the input to make this program return only a single byte, 200
    let input_bytes: &[u8] = todo!();
    let env = ExecutorEnv::builder().write_slice(input_bytes).build()?;

    // ===== do not change anything below this line =====
    let session_info = executor.execute_elf(env, include_bytes!("./bin/problem3.elf"))?;
    assert_eq!(session_info.journal.bytes, [200]);

    Ok(())
}

#[test]
fn problem4() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 4 =====
    // The most common way to handle IO between host and guest is to serialize the data. This
    // program can output data that can be deserialized in the specific form, `OutputType`.
    // As long as the data can be deserialized, this is a valid solution!

    #[derive(Deserialize)]
    struct OutputType {
        a: [u8; 32],
        b: (String, u64),
    }

    // TODO find out which inputs will make this program commit data that can be
    // deserialized as OutputType
    let input = todo!();
    let env = ExecutorEnv::builder().write(&input)?.build()?;

    // ===== do not change anything below this line =====
    let session_info = executor.execute_elf(env, include_bytes!("./bin/problem4.elf"))?;
    let output: OutputType = session_info.journal.decode()?;

    Ok(())
}

#[test]
fn problem5() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 5 =====
    // This program just returns back a number? What does it represent??

    // TODO update anything about the `ExecutorEnv` to make this pass
    let input: &[u8] = todo!();
    let env = ExecutorEnv::builder().write_slice(input).build()?;

    // ===== do not change anything below this line =====
    let session_info = executor.execute_elf(env, include_bytes!("./bin/problem5.elf"))?;
    let output: u32 = session_info.journal.decode()?;
    assert_eq!(output, 24040);

    Ok(())
}

#[test]
fn problem6() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 6 =====
    // Making programs execute correctly is too easy, can you find a way to make this one error?
    //
    // Note: There are at least two different ways to cause this to error, can you think of
    //       what another way would be after you find one?
    //
    // WARNING: This program will execute for a long time, try to find a solution to exit it early
    //          before you remove the `todo!()`, it uses a lot of storage space!

    // TODO update anything about the `ExecutorEnv` to make this pass
    todo!();
    let env = ExecutorEnv::builder().build()?;

    // ===== do not change anything below this line =====
    assert!(
        executor
            .execute_elf(env, include_bytes!("./bin/problem6.elf"))
            .is_err(),
        "This program should error for this puzzle to be complete"
    );

    Ok(())
}

#[test]
fn problem7() -> anyhow::Result<()> {
    let executor = default_executor();

    // ===== PROBLEM 7 =====
    // This program should pause? What does that mean? Might be simpler than you think to make this
    // program pause.

    // TODO update anything about the `ExecutorEnv` to make this pass
    let input: &[u8] = todo!();
    let env = ExecutorEnv::builder().write_slice(input).build()?;

    // ===== do not change anything below this line =====
    let session_info = executor.execute_elf(env, include_bytes!("./bin/problem7.elf"))?;
    assert_eq!(session_info.exit_code, ExitCode::Paused(0));

    Ok(())
}
