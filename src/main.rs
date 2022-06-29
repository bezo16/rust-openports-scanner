use port_scanner::*;
use core::panic;
use std::env;
use regex::Regex;

fn main() {
    let (option,value) = match parse_arguments(env::args().collect()) {
        Ok(res) => {res},
        Err(err) => {panic!("{}", err)},
    };

    
    if option == "all" {scan_all_ports()}
    else if option == "p" && !value.is_empty() {scan_one_port(&value)}
    else if option == "a" && !value.is_empty() {scan_adress(&value)}
    else {println!("invalid option or value inserted")}
}


fn parse_arguments(args: Vec<String>) -> Result<(String,String), String> {

    if args.len() < 2 {
        return Err(String::from("not enough arguments!"))
    }
    if args.len() >= 4 {
        return Err(String::from("too many arguments!"))
    }
    
    let value = if args.len() == 3 {args[2].clone()} else {String::from("")};
    let option = args[1].clone();

    if option != "all" && option != "p" && option != "a" {panic!("invalid option")}


    Ok((option,value))
}

fn scan_adress (addr: &String) {
    let regex = Regex::new(r"^\d{1,3}\.\d{1,3}.\d{1,3}.\d{1,3}:\d{1,5}$").unwrap();
    if !regex.is_match(&addr) {
        panic!("{} is not valid ip adress", &addr)
    }

    let result = scan_port_addr(&addr);
    let result_str = if result {"opened"} else {"closed"};

    println!("IP {} has port {}", &addr, result_str);
}

fn scan_one_port(port: &String) {
    println!("scanning one port");
    let port = port.parse::<u16>().unwrap_or_else(|_| panic!("wrong parameter value, expecting number or number is too big (max 65.535,min: 1)"));

    let result = scan_port(port);
    let result_str = if result {"opened"} else {"closed"};


    println!("sucessfully scanned! port {} is {}", port, result_str);

}


fn scan_all_ports() {
    let mut port: u16 = 0; // max 65 535
    let mut open_ports: u16 = 0;
    let mut closed_ports: u16 = 0;
    println!("scanning all ports");

    loop {
        if port == 65_535 {break}
        let result = scan_port(port);

        if result {
            open_ports += 1;
            println!("carefull! port {} is opened, close it ASAP", port)
        } 
        
        else {
            closed_ports += 1;
        }

        port += 1;
    }

    if open_ports == 0 {
        println!("good job all your ports ale closed! you are safe")
    }

    println!("open ports: {}", open_ports);
    println!("closed ports: {}", closed_ports);

}
