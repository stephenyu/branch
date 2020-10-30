use rand::Rng;

fn get_random_number() -> i8 {
   let mut rng = rand::thread_rng();
   rng.gen_range(1, 91)
}

fn unique_number(existing: &Vec<i8>) -> i8 {
   let mut random_number = get_random_number();

   while existing.contains(&random_number) {
       random_number = get_random_number();
   }

   random_number
}

fn main() {
    let mut collected_iterator: Vec<i8> = Vec::with_capacity(25);

    while collected_iterator.len() < 25 {
        collected_iterator.push(unique_number(&collected_iterator))
    }

    let mut y = 5;
    while y <= 25 {
        println!("{:?}", &collected_iterator[y-5..y]);
        y += 5;
    }
}
