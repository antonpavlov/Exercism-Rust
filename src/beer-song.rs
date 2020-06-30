pub fn verse(n: u32) -> String {
    let mut ret_str = String::new();
    let bottles = n;

    if bottles == 2 {
        ret_str = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottle of beer on the wall.", 
        bottles.to_string(), bottles.to_string(), (bottles-1).to_string());
    } else if bottles == 1 {
        ret_str = format!("{} bottle of beer on the wall, {} bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.", 1, 1);             
    } else {
        ret_str = format!("{} bottles of beer on the wall, {} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.", 
        bottles.to_string(), bottles.to_string(), (bottles-1).to_string());
    }
    
    ret_str    
}

pub fn sing(start: u32, end: u32) -> String {
    let mut ret_string = String::new();
    let mut fn_string = String::new();

    let mut counter: u32 = start;
    loop{
        println!("{}", counter);
        fn_string = verse(counter);
        ret_string.push_str(&fn_string);
        //ret_string.push_str("\n");
        counter -= 1;
        if counter == end {
            break;
        }
        ret_string.push_str("\n");
    }
    ret_string
}
