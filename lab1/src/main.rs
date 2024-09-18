use colored::Colorize;

fn input_digit() -> f32 {
    let mut temp_str = String::new(); 
    loop {
        std::io::stdin().read_line(&mut temp_str).expect("Failed to read line");
        match temp_str.trim().parse::<f32>(){
            Ok(_n) => {
                let temp = temp_str.trim().parse::<f32>().unwrap();
                return temp;
            }
            Err(_e) => {
                println!("Please input a number");
                temp_str.clear();
                continue;
            }
        }
    }
}
fn one_root(root: f32){
    let temp = f32::sqrt(root);
    if root > 0.0{
        println!("2 roots: \n root 1 = {}, root 2 = {}", temp.to_string().green(), (-temp).to_string().green());
    }
     else if root == 0.0{
        println!("1 root: {}", temp);
     }
}


fn two_roots(root1: f32, root2: f32){
    let mut roots_arr = Vec::new();
    if root1 > 0.0{
        let temp = f32::sqrt(root1);
        roots_arr.push(temp);
        roots_arr.push(-temp);
    } else if root1 == 0.0 {roots_arr.push(0.0)}
    if root2 > 0.0{
        let temp = f32::sqrt(root2);
        roots_arr.push(temp);
        roots_arr.push(-temp);
    } else if root2 == 0.0 {roots_arr.push(0.0)}

    if roots_arr.len() > 0{
    print!("{} roots; ", roots_arr.len().to_string().green());
    for i in 0..roots_arr.len(){
        print!("root {} = {}, ", i + 1, roots_arr[i].to_string().green())
        }
    } else {println!("{}", "No roots".red())}
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let (a, b, c);
    if args.len() == 4{
    let (a_str, b_str, c_str) = (&args[1], &args[2], &args[3]);
    (a, b, c) = (a_str.trim().parse::<f32>().unwrap(), b_str.trim().parse::<f32>().unwrap(), c_str.trim().parse::<f32>().unwrap());
    }
    else{
    (a, b, c) = (input_digit(), input_digit(), input_digit());
    }
    let descriptor=  b * b - 4.0 * a * c;
    if descriptor > 0.0{
        let r1 = ((-1.0) * b - f32::sqrt(descriptor)) / (2.0 * a);
        let r2 = ((-1.0) * b + f32::sqrt(descriptor)) / (2.0 * a);
        two_roots(r1, r2)
    } else if descriptor == 0.0 {
        let r = -b/(2.0 * a);
        one_root(r);
    } else {
        println!("{}", "No roots".red())
    }
}