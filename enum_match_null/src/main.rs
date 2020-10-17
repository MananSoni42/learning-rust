use std::option::Option;

enum IP {
    V4(u8,u8,u8,u8),
    V6(u8,u8,u8,u8,u8,u8),
}

fn print_ip(ip: &Option<IP>) {
    match ip { // rust match is exhaustive!
        None => println!("No IP specified"),
        &Some(IP::V4(a,b,c,d)) => println!("IPv4: {}.{}.{}.{}",a,b,c,d),
        &Some(IP::V6(a,b,c,d,e,f)) => println!("IPv6: {}.{}.{}.{}.{}.{}",a,b,c,d,e,f),
    }
}

fn main() {
    let user_ip1: Option<IP> = Some(IP::V6(0,1,2,3,4,5));
    let user_ip2: Option<IP> = Some(IP::V4(4,3,2,1));
    let user_ip3: Option<IP> = None;
    print_ip(&user_ip1);
    print_ip(&user_ip2);
    print_ip(&user_ip3);

    // how to print an enum
    if let Some(IP::V4(a,b,c,d)) = user_ip2 {
        println!("{}.{}.{}.{}",a,b,c,d);
    } else {
        println!("Not in ipv4 format");
    }

    if let Some(IP::V4(a,b,c,d)) = user_ip1 {
        println!("{}.{}.{}.{}",a,b,c,d);
    } else {
        println!("Not in ipv4 format");
    }
}
