fn comp(a: Vec<i64>, b: Vec<i64>) -> bool 
{
    let mut count  = 0;
    let mut b_copy = b;
    
    for val in &a
    {
        if let Some(index) = b_copy.iter().position(|&s| s == val*val) 
        {
            b_copy.remove(index);
        }
    }
        
    if b_copy.is_empty() { true } else { false }
}
