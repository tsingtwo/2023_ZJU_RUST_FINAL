#![feature(impl_trait_in_assoc_type)]

use std::env;
use std::net::SocketAddr;
use volo_example::FilterLayer;
use volo_example::S;
// use volo_thrift::server;
// use std::fs::File;
//use volo_gen::volo::example::GetItemRequest;
// use lazy_static::lazy_static;
// use pilota::lazy_static;
//use std::io::Error;
//use std::io::Write;
// lazy_static! {
//     static ref CLIENT: volo_gen::volo::example::ItemServiceClient = {
//         let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
//         volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
//             .layer_outer(FilterLayer)
//             .address(addr)
//             .build()
//     };
// }

#[volo::main]
async fn main() {
    // let addr: SocketAddr = "[::]:8080".parse().unwrap();
    // let addr = volo::net::Address::from(addr);

    // volo_gen::volo::example::ItemServiceServer::new(S::new())
    //     .layer_front(FilterLayer)
    //     .run(addr)
    //     .await
    //     .unwrap();
    let args: Vec<String> = env::args().collect();
    let proxy_addr = args[1].clone();
    let mut mas:Vec<Vec<String>> = Vec::new();
    let mut mst: Vec<String>= Vec::new();
    let mut i = 2;
    // println!("{}", args.len());
    while i < args.len(){

        if args[i]=="114514".to_string(){
            // println!("{}", args[i].clone());
            i+=1;
        }

        if i == args.len(){ break; }

        mst.push(args[i].clone());
        // println!("{}", args[i].clone());
        i+=1;

        if i == args.len(){ break; }

        mas.push(Vec::new());
        while args[i].clone()!="114514".to_string() && i < args.len() {
            mas[mst.len()-1].push(args[i].clone());
            // println!("{} {}", mst[mst.len()-1].clone(), args[i].clone());
            i+=1;
        }
        if i == args.len(){ break; }
    }
    // let mut x =0;
	// 	let mut y = 0;
	// 	// println!("mst's num: {}", mst.len().clone());
	// 	while x < mst.len().clone(){
	// 		while y < mas.clone()[x].len().clone(){
	// 			print!("{} {} : {} ", mst.len().clone(), mas.clone()[x].len().clone(), mas[x][y]);
    //             y+=1;
	// 		}
    //         y = 0;
	// 		println!("");
	// 		x += 1;
	// 	}
    let server = S::new();
    for ip in mst{
        server.mst.lock().unwrap().push({
            {
                let addr:SocketAddr = ip.parse().unwrap();
                volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
                                                                        .layer_outer(FilterLayer)
                                                                        .address(addr)
                                                                        .build()
            }
        });
    }
    let mut i = 0;
    for slave in mas{
        {
            {server.mas.lock().unwrap().push(Vec::new());}
            for ip in slave{
                server.mas.lock().unwrap()[i].push({
                    let addr:SocketAddr = ip.parse().unwrap();
                    volo_gen::volo::example::ItemServiceClientBuilder::new("volo-example")
                                                                        .layer_outer(FilterLayer)
                                                                        .address(addr)
                                                                        .build()
                });
            }
        }
        i+=1;
    }
    
    let addr:SocketAddr = proxy_addr.parse().unwrap();
    let addr = volo::net::Address::from(addr);
    volo_gen::volo::example::ItemServiceServer::new(server)
        .layer_front(FilterLayer)
        .run(addr)
        .await
        .unwrap();
}
