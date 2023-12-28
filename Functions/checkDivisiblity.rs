fn test_divisibility_by_3_4(a:i32) -> i32{
    if a % 3 == 0 && a %  4 == 0{
        return 0;
    } 
    else if a % 3 == 0 {
        return 1;
    }
    else if a % 4 == 0 {
        return 2;
    }
    else {
        return -1;
    }
    
}
