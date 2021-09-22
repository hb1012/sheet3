//! Task 3.1: Rule 90

/// Reads a valid initial configuration for our automaton from the terminal.
fn read_input() -> Vec<bool> {
    // This tries to read a string from the terminal, checks whether it's
    // valid (only contains 1's and 0's). If the user fails to input a correct
    // string, this routine will ask again until the user finally manages to
    // give us a correct string.
    //
    // You don't need to understand this routine yet; that's why I've written
    // it already ;-)
    //
    // You only need to use the `input` variable (of type `String`). You can
    // also assume that it only contains '0' and '1' chars.
    let input = {
        let mut buffer = String::new();

        loop {
            println!("Please give me the initial configuration (a string of '0' and '1'!):");
            buffer.clear();

            // `read_line` returns an error if the input isn't valid UTF8 or if
            // a strange IO error occured. We just panic in that case...
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("something went seriously wrong :O");

            if buffer.trim().chars().all(|c| c == '1' || c == '0') {
                break;
            }
        }

        buffer.trim().to_string()
    };

    // TODO: Task 1a)
    // primitive solution
    // let mut out: Vec<bool> = vec![];
    // for c in input.chars() {
    //     if c == '0' {
    //         out.push(false);
    //     }
    //     else {
    //         out.push(true);
    //     }
    // }

    // better solution
    let mut out = Vec::with_capacity(input.len());
    for c in input.chars() {
        out.push(c == '1');
    }
    out
}

// TODO: Task 1b)
fn next_step(old_state: &[bool]) -> Vec<bool> {
    let mut new_state = Vec::with_capacity(old_state.len());
    let s = old_state.len();
    for i in 0..s {
        // if i == 0 {
        //     new_state.push(old_state[s - 1] ^ old_state[i + 1]);
        // } else if i == s - 1 {
        //     new_state.push(old_state[i - 1] ^ old_state[0]);
        // } else {
        //     new_state.push(old_state[i - 1] ^ old_state[i + 1]);
        // }
        let right_index = (i + 1) % s;
        let left_index = (i + s - 1) % s;
        new_state.push(old_state[left_index] ^ old_state[right_index]);
    }
    new_state
}

fn print_state(state: &[bool]) {
    // for i in 0..state.len() {
    let a_iter = state.iter();
    for b in a_iter {
        if *b == true {
            print!("██");
        } else {
            print!("  ");
        }
    }
    println!("");
}

fn main() {
    // TODO: Task 1c)
    let mut old_state = read_input();
    print_state(&old_state);
    // let a_iter = a.iter();
    // for i in a_iter {
    //     println!("{}", i);
    // }
    for _i in 1..=31 {
        let new_state: Vec<bool> = next_step(&old_state);
        print_state(&new_state);
        old_state = new_state;
    }
}

#[test]
fn rule90_rules() {
    assert_eq!(next_step(&[false, false, false]), vec![false, false, false]);
    assert_eq!(next_step(&[ true, false, false]), vec![false,  true,  true]);
    assert_eq!(next_step(&[ true,  true, false]), vec![ true,  true, false]);
    assert_eq!(next_step(&[ true,  true,  true]), vec![false, false, false]);
}
