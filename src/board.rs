pub fn testing() {
    println!("  ┌───┬───┬───┬───┬───┬───┬───┬───┬───┬───┐");

    let mut x = 0;
    while x != 10 {
        x+=1;
        let ntlr = numtolet(x);
        println!("{} │{}-1│{}-2│{}-3│{}-4│{}-5│{}-6│{}-7│{}-8│{}-9│{}10│",ntlr,ntlr,ntlr,ntlr,ntlr,ntlr,ntlr,ntlr,ntlr,ntlr,ntlr);
        if x < 10 {
            println!("  ├───┼───┼───┼───┼───┼───┼───┼───┼───┼───┤");
        }
    }

    println!("  └───┴───┴───┴───┴───┴───┴───┴───┴───┴───┘");
    println!("    1   2   3   4   5   6   7   8   9   10");
}

pub fn numtolet(x: i16) -> String {
    if x == 1 {
        return "A".to_string();
    } else if x == 2 {
        return "B".to_string();
    } else if x == 3 {
        return "C".to_string();
    } else if x == 4 {
        return "D".to_string();
    } else if x == 5 {
        return "E".to_string();
    } else if x == 6 {
        return "F".to_string();
    } else if x == 7 {
        return "G".to_string();
    } else if x == 8 {
        return "H".to_string();
    } else if x == 9 {
        return "I".to_string();
    } else if x == 10 {
        return "J".to_string();
    }
    return x.to_string();
}
