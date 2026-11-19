use std::io::{self};

fn main() {
   
    let mut running = true;
    let mut fruitBasket = Vec::new();

    let mut input = String::new();

    while running == true {   
        let mut stopList = false;
        running = false;
        while stopList == false  {
            clearscreen::clear().expect("Failed to clear screen");
            println!("Type a food name and type 'stop' to stop");
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input.trim();
            if input.trim().to_lowercase() == "stop" {
                stopList = true;
                input.clear();
                continue;
            }
            fruitBasket.push(input.clone());
            input.clear();
        }

        clearscreen::clear().expect("Failed to clear screen");
        println!("List of Foods:\n");
        
        let mut fruitBasketLength = fruitBasket.len();
        if fruitBasketLength > 2 {
            loop {
                let mut swapped = false;
                for i in 1..fruitBasketLength {
                    if fruitBasket[i-1] > fruitBasket[i] {
                        fruitBasket.swap(i-1, i);
                        swapped = true;
                    }
                }

                fruitBasketLength-=1;
                if !swapped {
                    break;
                }
            }
        }

        for fruit in &fruitBasket {
            print!("{}", fruit);
        }
        io::stdin().read_line(&mut input).expect("Failed to read line");

        running = false;
    }

}
