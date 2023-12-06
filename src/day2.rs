use fancy_regex::Regex;

pub fn part1(lines: Vec<String>) {
    let blue_regex = Regex::new(r"\w*(?= blue)").unwrap();
    let green_regex = Regex::new(r"\w*(?= green)").unwrap();
    let red_regex = Regex::new(r"\w*(?= red)").unwrap();

    let max_blue = 14;
    let max_green = 13;
    let max_red = 12;

    let mut id_sum = 0;

    'main_loop: for (id, line) in lines.into_iter().enumerate() {
        let mut matching_blue = blue_regex.find_iter(&line);

        while let Some(blue_iter) = matching_blue.next() {
            let blue_amount = blue_iter
                .unwrap()
                .as_str()
                .to_string()
                .parse::<i32>()
                .unwrap();
            if blue_amount > max_blue {
                continue 'main_loop;
            }
        }

        let mut matching_red = red_regex.find_iter(&line);

        while let Some(red_iter) = matching_red.next() {
            let red_amount = red_iter
                .unwrap()
                .as_str()
                .to_string()
                .parse::<i32>()
                .unwrap();
            if red_amount > max_red {
                continue 'main_loop;
            }
        }

        let mut matching_green = green_regex.find_iter(&line);

        while let Some(green_iter) = matching_green.next() {
            let green_amount = green_iter
                .unwrap()
                .as_str()
                .to_string()
                .parse::<i32>()
                .unwrap();
            if green_amount > max_green {
                continue 'main_loop;
            }
        }

        println!("Game: {}", id + 1);

        id_sum += id + 1;
    }
    println!("{id_sum}");
}
