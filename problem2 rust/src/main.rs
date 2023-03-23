fn longest_prefix(v:Vec<String>) ->String
{
    let mut mins = &String ::new();
    let mut minl = 100000;

    for i in &v //find the smallest size string in the vector
    {
        if i.len()<minl
        {
            mins = i;
            minl = mins.len();
        }
    }

    let mut rets = String ::new(); //create return string 
    let mut I = 0;
    let mut n = 0;
    while n == 0 && I<minl //char comparision
    {
        let s = mins.as_str().chars().nth(I).unwrap();
        for i in &v
        {
            if i.as_str().chars().nth(I).unwrap() != s
            {
                n = 1;
                break;
            }
        } 
        if n == 0
        {
            rets.push(s); //append the char to the return string, the longest prefix
            I = I+1;
        }
    }
    return rets;
}

fn main() {
    let mut v : Vec<String> = Vec::new();
    let s = "allsdkasd";
    let a = "allla";
    let c = "alll";
    let d = "allc";
    v.push(s.to_string());
    v.push(a.to_string());
    v.push(d.to_string());
    v.push(c.to_string());
    
    println!("{}",longest_prefix(v));
}
