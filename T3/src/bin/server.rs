#![feature(impl_trait_in_assoc_type)]

use std::env;
use std::net::SocketAddr;
use volo_example::FilterLayer;
use volo_example::S;


#[volo::main]
async fn main() {

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
