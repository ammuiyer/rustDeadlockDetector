#[macro_use] extern crate text_io;

   
use std::io;
const N: i32 = 5;
static mut arrmax: [[i32; 1]; 100] =  [[100]; 100];
static  mut allocated: [[i32; 1]; 100] =  [[100]; 100];
static mut need: [[i32; 1]; 100] =  [[100]; 100];
static mut available: [i32; 1] =  [100];
static mut n: String = String::new();
static mut r: String = String::new();









fn input()
{
    let mut i = 0;
    let mut j = 0;
    
    
    println!("Enter the no of Processes\t"); 
    
    io::stdin().read_line(unsafe { &mut n }) 
              .ok() 
              .expect("Failed to read amount"); 
    
    println!("Enter the no of Resource Instances\t"); 
     
    io::stdin().read_line(unsafe { &mut r }) 
              .ok() 
              .expect("Failed to read amount"); 
    let n1:usize = unsafe { n.parse().unwrap() };
    let r1:usize = unsafe { r.parse().unwrap() };
    println!( "Enter the Max Matrix\n");
    while true
    {
        while true
        {
            let b: i32 = read!();
            b >> unsafe { arrmax }[i][j];
            
            j=j+1;
            if(j>= r1){
            break;
            } 
        }
        i=i+1;
        if(i>= n1){
            break;
        } 
    }
    i = 0;
    j = 0;
    println!("Enter the Allocation Matrix\n");
    while true
    {
        while true
        {   
            let b: i32 = read!();
            b >> unsafe { allocated }[i][j];
            j=j+1;
            if(j>= r1){
            break;
            } 
        }
        i=i+1;
        if(i>= n1){
            break;
        } 
    }
    i=0;
    j=0;
    println!("Enter the available Resources\n");
    while true
        {
            let b: i32 = read!();
            b >> unsafe { available }[j];
            j=j+1;
            if(j>= r1){
            break;
            } 
        }
}
fn show()
{
    let n1:usize = unsafe { n.parse().unwrap() };
    let r1:usize = unsafe { r.parse().unwrap() };
    let mut i = 0;
    let mut j = 0;
    println!("Process\t Allocation\t Max\t Available\t");
    while true
    {
        println!("\nP {} \t", i+1); 
        while true
        {
            println!("{} ", unsafe { allocated }[i][j]);
            j=j+1;
            if(j>= r1){
                j=0;
            break;
            } 
        }
        println!("\t\t");
        while true
        {
            println!("{} ", unsafe { arrmax }[i][j]);
            j=j+1;
            if(j>= r1){
                j=0;
            break;
            } 
        }
        println!("\t ");
        if (i == 0)
        {
            while true{
                println!("{} ", unsafe { available }[j]);
                j=j+1;
            if(j>= r1){
                j=0;
                break;
            } 
        }
        }
        i=i+1;
        if(i>= n1){
            break;
        } 
    }
}
fn cal()
{
    let mut finish =  [100];
    for q in 1..100{
        finish[q] = 0; 
    }
    let temp = 0;
    let mut flag = 1;
    let mut k = 0;
    let mut c1 = 0;
    let mut dead =  [100];
    let mut safe =  [100];
    let mut i = 0;
    let mut j = 0;
    let n1:usize = unsafe { n.parse().unwrap() };
    let r1:usize = unsafe { r.parse().unwrap() };
    //find need matrix
    while true
    {
        while true
        {
            let z2 = unsafe { arrmax }[i][j] - unsafe { allocated }[i][j];
            z2 >> unsafe { need }[i][j];
            if(j>= r1){
                j=0;
                break;
            } 
        }
        if(i>= n1){
            i=0;
            break;
        } 
    }
    while flag != 0
    {
        flag = 0;
        while true
        {
            let mut c = 0;
            while true
            {
                if ((finish[i] == 0) && (unsafe { need }[i][j] <= unsafe { available }[j]))
                {
                    c=c+1;
                    if (c == r1)
                    {
                        while true
                        {   
                            let z = &unsafe { available }[k] + &unsafe { allocated }[i][j];
                            z >> unsafe { available }[k];
                            finish[i] = 1;
                            flag = 1;
                            if(k>= r1){
                                k=0;
                                break;
                            } 
                        }
                        //cout<<"\nP%d",i;
                        if (finish[i] == 1)
                        {
                            i = n1;
                        }
                    }
                }
                if(j>= r1){
                    j=0;
                    break;
                } 
            }
            if(i>= n1){
                i=0;
                break;
            } 
        }
    }
    j = 0;
    flag = 0;
    i=0;
    while true
    {
        if (finish[i] == 0)
        {
            dead[j] = i;
            j=j+1;
            flag = 1;
        }
        if(i>= n1){
            i=0;
            break;
        } 
    }
    if (flag == 1)
    {
        println!("\n\nSystem is in Deadlock and the Deadlock process are\n");
        while true
        {
            println!("P {} \t", dead[i]);
            if(i>= n1){
                i=0;
                break;
            } 
        }
    }
    else
    {
        println!("\nNo Deadlock Occur");
    }
}
fn main()
{
    
    
    input();
    show();
    cal();
    return;
}





