fn matching_parentheses(s:&str) ->bool //noting return type of this function
{
    if s.len() == 0 {return true;}
    let mut Loclist: Vec<char> = Vec::new(); // using a vector of char instead of vector of int
                                             // as before

    let mut LPnum = 0;
    let mut RPnum = 0;

    for i in s.chars()
    {
        if i == '(' || i == '[' || i == '{'
        {
            Loclist.push(i);
            LPnum += 1;
        }
        else if i == ')' || i == ']' || i == '}'
        {
            Loclist.push(i);
            RPnum += 1;
        }
    }

    if LPnum != RPnum {return false;} // comparing the number of right parentheses and left parentheses
    let mut n = Loclist.len();
    let mut i = 0;
    while n > 0
    {
        if Loclist[i] == '('
        {
            let mut hit = 0;
            for j in i..Loclist.len() // different way of using for loop
            {
                if Loclist[j] == ')' && (j-i)%2 != 0
                {
                    hit = 1;
                    Loclist.remove(i);
                    Loclist.remove(j-1);
                    break;
                }
            }
            if hit == 0 {return false;}
        }
        else if Loclist[i] == '['
        {
            let mut hit = 0;
            for j in i..Loclist.len()
            {
                if Loclist[j] == ']' && (j-i)%2 != 0
                {
                    hit = 1;
                    Loclist.remove(i);
                    Loclist.remove(j-1);
                    break;
                }
            }
            if hit == 0 {return false;}
        }

        else if Loclist[i] == '{'
        {
            let mut hit = 0;
            for j in i..Loclist.len()
            {
                if Loclist[j] == '}' && (j-i)%2 != 0
                {
                    hit = 1;
                    Loclist.remove(i);
                    Loclist.remove(j-1);
                    break;
                }
            }
            if hit == 0 {return false;}
        }
        else {i = i+1;}
        n = Loclist.len();
    }
    return true;
}

fn main() 
{
    println!("{}",matching_parentheses("([})"));
}
