

    
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
    let i = 0;
    let j = 0;
    
    
    println!("Enter the no of Processes\t"); 
    
    io::stdin().read_line(&mut n) 
              .ok() 
              .expect("Failed to read amount"); 
    
    println!("Enter the no of Resource Instances\t"); 
     
    io::stdin().read_line(&mut r) 
              .ok() 
              .expect("Failed to read amount"); 
    let n1:usize = n.parse().unwrap();
    let r1:usize = r.parse().unwrap();
    cout << "Enter the Max Matrix\n";
    while true
    {
        while true
        {
            cin >> unsafe { arrmax }[i][j];
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
    cout << "Enter the Allocation Matrix\n";
    while true
    {
        while true
        {
            cin >> unsafe { allocated }[i][j];
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
    cout << "Enter the available Resources\n";
    while true
        {
            cin >> unsafe { available }[j];
            j=j+1;
            if(j>= r1){
            break;
            } 
        }
}
fn show()
{
    let n1:usize = n.parse().unwrap();
    let r1:usize = r.parse().unwrap();
    let i = 0;
    let j = 0;
    println!("Process\t Allocation\t Max\t Available\t");
    while true
    {
        cout << "\nP" << i + 1 << "\t ";
        while true
        {
            cout << unsafe { allocated }[i][j] << " ";
            j=j+1;
            if(j>= r1){
                j=0;
            break;
            } 
        }
        println!("\t\t");
        while true
        {
            cout << unsafe { arrmax }[i][j] << " ";
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
                cout << unsafe { available }[j] << " ";
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
    let mut need =  [[100]; 100];
    let flag = 1;
    let k = 0;
    let c1 = 0;
    let mut dead =  [100];
    let mut safe =  [100];
    let i = 0;
    let j = 0;
    let n1:usize = n.parse().unwrap();
    let r1:usize = r.parse().unwrap();
    //find need matrix
    while true
    {
        while true
        {
            need[i][j] = unsafe { arrmax }[i][j] - unsafe { allocated }[i][j];
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
            let c = 0;
            while true
            {
                if ((finish[i] == 0) && (need[i][j] <= unsafe { available }[j]))
                {
                    c=c+1;
                    if (c == r1)
                    {
                        while true
                        {
                            unsafe { available }[k] =  unsafe { available }[k] + unsafe { allocated }[i][j];
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
            cout << "P" << dead[i] << "\t";
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





