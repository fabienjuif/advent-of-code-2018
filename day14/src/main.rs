fn main() {
    let mut scores = vec![3, 7];
    let mut first_elf = 0;
    let mut second_elf = 1;

    let stop_at = 637061;

    while stop_at + 10 > scores.len() {
        // add new scores
        let mut new_score = scores[first_elf] + scores[second_elf];
        let mut to_concat = vec![];
        while new_score > 9 {
            to_concat.push(new_score % 10);
            new_score = new_score / 10;
        };
        to_concat.push(new_score);
        to_concat.reverse();
        scores.append(&mut to_concat);

        // move elfs
        first_elf += 1 + scores[first_elf];
        second_elf += 1 + scores[second_elf];
        while first_elf >= scores.len() {
            first_elf %= scores.len();
        }
        while second_elf >= scores.len() {
            second_elf %= scores.len();
        }
    }

    let result = scores.iter().skip(stop_at).collect::<Vec<_>>();

    println!("--part 1--");
    for (index, c) in result.iter().enumerate() {
        if index > 9 { break; }
        print!("{}", c);
    }
    println!("");
}
