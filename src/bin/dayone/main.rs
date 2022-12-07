use std::fs;

fn main() {
    // Who should the elves prioritise going to for snacks? How many calories are these particular elves carrying?

    // Part 1:
    // Which elf was most prepared?

    // Import data from the file the elves have provided and split into what each elf is carrying
    let calorie_data =
        fs::read_to_string("./src/bin/dayone/calorie_data.txt").expect("Problem reading file");

    // Sum the calories contained in each elf's sack
    let calorie_sums: Vec<u32> = calorie_data
        .split("\r\n\r\n") // Windows sucks :(
        .map(|sack| {
            sack.lines()
                .filter_map(|items| items.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect();

    find_first_place(&calorie_sums);

    // Part 2:
    // Recognising the top 3 most prepared elves

    find_top_three(&calorie_sums);

    // Check for ties??
}

// Part 1 function

// Locate the elf with the most calories and tell everyone how much they're carrying!
fn find_first_place(sums: &Vec<u32>) {
    let (first_place_index, first_place_sum) = sums
        .into_iter()
        .enumerate() // Elves need an index to identify them
        .reduce(
            |(current_max_index, current_max_sum), (next_index, next_sum)| {
                if current_max_sum >= next_sum {
                    (current_max_index, current_max_sum)
                } else {
                    (next_index, next_sum)
                }
            },
        )
        .unwrap();

    // Declare the "winner"
    println!("The elf carrying the most calories is the elf with badge number {}, with a total of {} calories!", first_place_index, first_place_sum);
}

// Part 2 function

// Find the elves with the second and third most calories in their snack collection
fn find_top_three(sums: &Vec<u32>) {
    let top_three = sums.into_iter().copied().enumerate().fold(
        [(0, 0); 3],
        |[mut first_place, mut second_place, mut third_place], (next_index, next_sum)| {
            if first_place.1 < next_sum {
                third_place = second_place;
                second_place = first_place;
                first_place = (next_index, next_sum);
            } else if second_place.1 < next_sum {
                third_place = second_place;
                second_place = (next_index, next_sum);
            } else if third_place.1 < next_sum {
                third_place = (next_index, next_sum);
            }
            [first_place, second_place, third_place]
        },
    );

    // Declare 2nd place
    println!(
        "The elf carrying the second most calories is badge number {}, with {} calories.",
        top_three[1].0, top_three[1].1
    );

    // Declare 3rd place
    println!(
        "The elf carrying the third most calories is badge number {}, with {} calories.",
        top_three[2].0, top_three[2].1
    );

    // Sum the calorie values carried by these three elves
    let top_three_calories_total = top_three
        .into_iter()
        .fold(0, |total, (_next_index, next_sum)| total + next_sum);

    println!(
        "Total calories carried between these three: {}",
        top_three_calories_total
    );
}
