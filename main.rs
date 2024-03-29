use std::fs;
use std::thread;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::net::{UdpSocket, SocketAddr};
use std::str;
use std::time::Instant;
use image::EncodableLayout;//DynamicImage, ImageBuffer, ImageFormat, EncodableLayout};
use base64;
use rand::Rng;
//use rayon::vec;
use serde_json;
use image::RgbaImage;//Rgba, };
use serde::{Serialize, Deserialize};
use bincode;
use image::{Rgba,ImageBuffer, imageops::FilterType};
//use std::io::Write;

extern crate image;

extern crate steganography;


use steganography::encoder::*;
use steganography::decoder::*;
use steganography::util::*;

const MAX_PACKET_SIZE: usize = 65000;

#[derive(Debug, Deserialize, Serialize)]
struct ImageWithIP {
 id: usize,
 image_data: Option<Vec<u8>>,
}


fn main(){
 // Addresses of the current server
 let local_addr_servers = "192.168.43.69:8080";//for servers
 let local_addr_failure = "192.168.43.69:8079";//for servers
 let local_addr_clients = "192.168.43.69:8081";//for clients
 
 //create a vector containing ips of remaining 2 servers
 let mut servers: Vec<String> = Vec::new();
 servers.push("192.168.43.116:8080".to_string());
 servers.push("192.168.43.52:8080".to_string());

 //create a vector containing ips of remaining 2 servers
 let mut failure_servers: Vec<String> = Vec::new();
 failure_servers.push("192.168.43.116:8079".to_string());
 failure_servers.push("192.168.43.52:8079".to_string());

 //Converting the string ips to SocketAddr
 let server_addr:SocketAddr = local_addr_servers.parse().expect("Invalid IP address or port");
 let failure_addr:SocketAddr = local_addr_failure.parse().expect("Invalid IP address or port");
 let client_addr:SocketAddr = local_addr_clients.parse().expect("Invalid IP address or port");
 
 //create a socket for the server and set timeout
 let server_socket = UdpSocket::bind(server_addr).expect("Failed to bind server socket");
 server_socket.set_read_timeout(Some(Duration::new(0,100000))).expect("Failed to set read timeout");
 let fail_socket = UdpSocket::bind(failure_addr).expect("Failed to bind server socket");
 //fail_socket.set_read_timeout(Some(Duration::from_secs(1))).expect("Failed to set read timeout");

 //Clone sockets for later use
 let server_socket_clone = server_socket.try_clone().unwrap();
 let server_socket_fail = fail_socket.try_clone().unwrap();

 //create a socket for the client
 let client_socket = UdpSocket::bind(client_addr).expect("Failed to bind client socket");
 client_socket.set_read_timeout(Some(Duration::from_secs(1))).expect("Failed to set read timeout");
 //create buffers for incoming packets
 //let client_buffer = [0;60000];
 //let client_buffer_image_data = [0;60000];
 let mut client_buffer_encrypt_Msg = [0;60000];
 let mut server_buffer = [0;60000];
 
 //Flag for the leader and ip of the current leader
 let is_leader = Arc::new(Mutex::new(false));
 let is_failing = Arc::new(Mutex::new(false));
 let mut count = 0;
 let mut result: SocketAddr = "192.168.43.69:8080".parse().unwrap();
 
 //initialize the DOS vector of strings
 let mut DOS: Vec<String> = Vec::new();
 let mut DOS_buffer: Vec<(String, String, u32)> = Vec::new();
 
 
 //Candidate ID to be changed and calculated dynamically
 let mut candidate_ID:f64 = 7.0;

 // let (result_sender, result_receiver) = mpsc::channel(); 
 let mut is_failing_clone = Arc::clone(&is_failing);
 let mut is_failing_clone_2 = Arc::clone(&is_failing);
 let mut is_failing_clone_3 = Arc::clone(&is_failing);

 let is_leader_clone = Arc::clone(&is_leader);
 let elect = thread::spawn(move || {
 loop{ 
 let mut clolo = is_failing_clone_2.clone();
 if(!*clolo.lock().unwrap()){
 let server_socket_elect_clone = server_socket.try_clone().unwrap();
 let server_elect_clone = servers.clone();
 candidate_ID = system_load();
 result = elect_Leader(server_socket_elect_clone, server_elect_clone, candidate_ID, *is_failing.lock().unwrap()).parse().unwrap();
 let start: Instant = Instant::now();

 if result == local_addr_servers.parse().unwrap(){
 //println!("We are the leader {}", result);
 //println!("The system load is {}", candidate_ID);
 let mut leader = is_leader_clone.lock().unwrap();
 *leader = true; 
 }
 else {
 let mut leader = is_leader_clone.lock().unwrap();
 *leader = false;
 }

 while(start.elapsed().as_millis()%50 < 49) {
 }
 }
 }
 }
 );
 ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////




//  let fail = thread::spawn(move || {
//  loop{
//  let server_socket_fail_clone = server_socket_fail.try_clone().unwrap();
//  let server_fail_clone = failure_servers.clone();
 
//  let afifi = elect_failure(server_socket_fail_clone, server_fail_clone);
//  let start: Instant = Instant::now();

//  if afifi {
//  //println!("We are the leader {}", result);
//  let mut failure = is_failing_clone.lock().unwrap();
//  *failure = true;
//  // I need to check first that I am done with requests
//  while(count>0){
//  println!("I will fail after finishing {} tasks", count);
//  }
//  //println!("I will fail");
//  while(start.elapsed().as_secs()%10 < 9) {
//  }
//  //thread::sleep(Duration::new(15, 0)); 
//  *failure=false; 
//  }
//  else {
//  let mut failure = is_failing_clone.lock().unwrap();
//  *failure = false;
//  }
//  }
//  }
//  );
 //////////////////////////////////////////////////////////////////////////////////////////////////////////////////
 
 
 
 
 
 let is_leader_clone = Arc::clone(&is_leader);
 let serve = thread::spawn(move || {
 println!("this is the serve thread");
 loop{
 let mut clolo = is_failing_clone_3.clone();
 if(!*clolo.lock().unwrap())
 {
 let client_socket_clone = client_socket.try_clone().unwrap();
 let mut response;
 let mut curr_client_addr;
 let mut temp_buffer = [0; 65000];
 let mut DOS_buffer_clone = DOS_buffer.clone();
 let mut DOS_clone = DOS.clone();
 loop{
 match client_socket_clone.recv_from(&mut client_buffer_encrypt_Msg){
 Ok((number_of_bytes, curr_client_adr)) => {
 let mut temp_buffer = client_buffer_encrypt_Msg.clone();
 response = String::from_utf8_lossy(&temp_buffer[0..number_of_bytes]).to_string();
 curr_client_addr = curr_client_adr;
 break;
 }
 Err(e) => {
 //println!("Awaiting requests: {}", e);
 //println!("E7na 3etelna hena");
 }
 }
 }
 count +=1;

 
 
 //println!("{}",response);
 //////////////////// Starting call
 if(response == "UP"){
 println!("We are in up and this is the dos {:?}", DOS);
 match find_entry_dos(&DOS_clone, curr_client_addr) {
 Some((addr)) => {
 }
 None => {
 DOS_clone.push(curr_client_addr.to_string());
 }
 }
 match find_entry(&DOS_buffer_clone, curr_client_addr) {
 Some((addr, id, count)) => {
 let leader = is_leader_clone.lock().unwrap();
 if *leader{
 let socket = UdpSocket::bind("192.168.43.69:0").expect("Failed to bind");
 socket.send_to(format!("An image with id {} was requested with {} views from 192.168.43.196", id, count).as_bytes(), curr_client_addr).unwrap(); 
 }
 }
 None => {
 let leader = is_leader_clone.lock().unwrap();
 if *leader{
 let socket = UdpSocket::bind("192.168.43.69:0").expect("Failed to bind");
 socket.send_to("EMPTY".as_bytes(), curr_client_addr).unwrap();
 } 
 }
 }
 DOS_buffer_clone.retain(|(addr, _message, _count)| *addr != curr_client_addr.to_string());
 DOS = DOS_clone;
 DOS_buffer = DOS_buffer_clone;

 println!("We are leaving up and this is the dos {:?}", DOS);
 }

 ////////////////////// Updating Clients
 else if(response.contains("UPDATE")){
 let leader = is_leader_clone.lock().unwrap();
 if *leader{
 let mut update = thread::spawn(move || {
 let encoded: Vec<u8> = bincode::serialize(&DOS_clone).unwrap();
 let socket = UdpSocket::bind("192.168.43.69:0").expect("Failed to bind");
 socket.send_to(encoded.as_bytes(), curr_client_addr).unwrap();
 });
 update.join();
 }
 }
 ////////////////////////Shutting down a user, Make the client send DOWN IPAddress by (format!"DOWN {}", ip)
 
 else if response.contains("DOWN"){
 println!("We are in DOWN");
 //DOS_buffer.retain(|(addr, _message, _count)| *addr != curr_client_addr);
 let parts: Vec<&str> = response.split(',').collect();
 let mut function: Option<String> = None;
 let mut ip: Option<String> = None;
 if parts.len() >= 2 {
 // Extract function name (assuming it's the first part)
 function = Some(parts[0].trim().to_string());

 // Parse the second part as the vector size
 if let Ok(ip) = parts[1].trim().parse::<String>() {
 //Solve this
 println!("The ip is {}", ip);
 //let mut ip_Socket: SocketAddr = ip.to_string().parse().unwrap();
 DOS.retain(|x| x != &ip);
 println!("{:?}", DOS);
 println!("{:?}", DOS_clone);

 } else {
 // Handle the case where the IP address is invalid
 println!("Invalid IP address format");
 }
 }
 
 }

 /////////////////// Buffer requests
 else if response.contains("BUFFER"){
 println!("We are in BUFFER");
 println!("We are in up and this is the buffer {:?}", DOS_buffer);

 //We receive Ip of the failed client, the image id, and the number of views
 let parts: Vec<&str> = response.split(',').collect();
 
 if parts.len() >= 4 {
 // Parse the SocketAddr
 if let Ok(socket_addr) = parts[1].parse::<String>() {
 // Extract the image id and view count
 let image_id = parts[2].to_string();
 if let Ok(view_count) = parts[3].parse::<String>() {
 // Construct the tuple and push it to the vector
 DOS_buffer_clone.push((socket_addr.to_string(), image_id, view_count.parse::<u32>().unwrap()));
 } else {
 println!("Failed to parse view count");
 }
 } else {
 println!("Failed to parse SocketAddr");
 }
 }
 DOS_buffer = DOS_buffer_clone;
 println!("We are leaving BUFFER");
 println!("We are in up and this is the buffer {:?}", DOS_buffer);

 }

 //////////////////////////////// Encrypt request
 else if(response.contains("ENCRYPT"))
 { 
 let leader = is_leader_clone.lock().unwrap();
 if *leader{
    println!("I am Sending {}",curr_client_addr );
 
 let mut image = thread::spawn(move || {
 let socket = UdpSocket::bind("192.168.43.69:0").expect("Failed to bind");
 socket.send_to(&mut "YES".as_bytes(), curr_client_addr).unwrap();
 //println!("{}",curr_client_addr);
 let mut function: Option<String> = None;
 let mut vectorsize: Option<u32> = None;
 let mut recieved_image_data: Vec<u8> = Vec::new();
 let parts: Vec<&str> = response.split(',').collect();

 // Ensure that the message has at least two parts (function name and vector size)
 if parts.len() >= 2 {
 // Extract function name (assuming it's the first part)
 function = Some(parts[0].trim().to_string());

 // Parse the second part as the vector size
 if let Ok(size) = parts[1].trim().parse::<u32>() {
 vectorsize = Some(size);
 }
 }

 // Check if both function and vectorsize have been successfully extracted
 if let Some(func) = &function {
 //println!("Function: {}", func); // Printing the extracted function name
 } else {
 println!("Function not found in the message");
 }

 if let Some(size) = vectorsize {
 //println!("Vector Size: {}", size); // Printing the extracted vector size
 } else {
 println!("Vector size not found or invalid");
 }
 let mut received_images = Vec::new();
 let mut total_chunks = 0;
 //while loop that will run until the entire image is recieved and has a timeout of 5 seconds
 let mut encryption_addr = "0.0.0.0:8080".parse().unwrap();
 while total_chunks < vectorsize.unwrap(){

 let mut image_buffer = [0;MAX_PACKET_SIZE];
 
 //println!("The total chunks is {}", total_chunks);
 match socket.recv_from(&mut image_buffer){
 Ok((data_size, addr)) => {
 received_images.extend_from_slice(&image_buffer[0..data_size]);
 encryption_addr = addr;
 //println!("recieved {} with the current data {}",total_chunks, image_data.len());
 }
 Err(e) => {
 println!("Lost connection. Image Packets Dropped: {}", e);
 continue;
 }
 }
 total_chunks += 1;
 //rintln!("received {} with the data {}",total_chunks, received_images(image_data));
 
 }
 let response = str::from_utf8(&received_images).expect("Failed to convert to string");
 
 let response_image:ImageWithIP = serde_json::from_str(response).expect("Failed to convert to string");

 let mut global_image: Vec<u8>= Vec::new();
 if let Some(image_data) = response_image.image_data{
 let decode_image_data = base64::decode(&image_data).unwrap();
 //let save_im = decode_image_data.clone();
 global_image=decode_image_data;
 //std::fs::write("recieved_image.jpeg", &save_im);
 //println!("Image data saved");
 }
 else{
 println!("No data is received");
 }
 
 let image_buffer = image::load_from_memory(&global_image).unwrap();
 let temp = image_buffer.to_rgba8();
 // Create a random cover image with dimensions based on the hidden image
 //let mut cover_image = generate_random_image(temp.width(), temp.height());
 let mut cover_image = image::open("palestine.jpg").unwrap();
 cover_image = cover_image.resize_exact(temp.width(), temp.height(), FilterType::Nearest);

 let img_buffer = cover_image.to_rgba8();
 // Hide the image in the cover image
 //hide_image(&mut cover_image, &temp);
 let cover_image = encypt_image(&temp, &img_buffer);
 // Save the modified cover image
 cover_image.save("output.png").expect("Failed to save steganographed image");
 let cover_image_data = fs::read("output.png").expect("failed to read");
 fs::remove_file("output.png");
 let cover_encoded_data = base64::encode(&cover_image_data);
 let cover_image_with_ip = ImageWithIP { 
 id: 1,
 image_data: Some(cover_encoded_data.into_bytes()),
 };
 let cover_json_data = serde_json::to_string(&cover_image_with_ip).expect("Failed to serialize to JSON");
 let cover_chunks: Vec<&[u8]> = cover_json_data.as_bytes().chunks(MAX_PACKET_SIZE).collect();
 //println!("The number of chunks is {}", cover_chunks.len());
 socket.send_to(&cover_chunks.len().to_string().as_bytes(), encryption_addr).unwrap();
 for chunk in &cover_chunks {
 socket.send_to(chunk, encryption_addr).expect("Failed to send data");
 }
 //socket.send_to( "CLOSE".as_bytes(), encryption_addr).unwrap();
 //cover_image.save("path/to/your/stegano_image.png").expect("Failed to save steganographed image");
 //cover_image.save("output.jpeg").expect("Failed to save steganographed image");
 
 });
 image.join();
 }
 }
 count-=1;
 }
 }
 });
 /////////////////////////////////////////////////////////////////////////////////////////////////////////////////



 
 serve.join().unwrap();
 //fail.join().unwrap();
 elect.join().unwrap();
 // let final_recieved_image_data = serve.join().unwrap();
 // println!("The final recieved image data is {:?}", final_recieved_image_data);
 println!("This is the main thread");
 ///////////////////
 loop{
 let (number_of_bytes, curr_server_addr) = server_socket_clone.recv_from(&mut server_buffer).unwrap();
 let response = str::from_utf8(&server_buffer[0..number_of_bytes]).expect("Invalid response");
 println!("{}",response);
 server_socket_clone.send_to(candidate_ID.to_string().as_bytes(), curr_server_addr).unwrap();
 //thread::sleep(Duration::new(1, 0))
 }
}




fn elect_Leader(server_socket: UdpSocket, servers:Vec<String>, candidate_ID:f64, is_failing:bool) -> String{
 let mut serverz_buffer = [0;6500];
 //send the candidate id to the to the addresses in the vector
 let clone = servers.clone();
 let mut responses: Vec<(f64,SocketAddr)> = Vec::new();
 for server in servers{
 server_socket.send_to(candidate_ID.to_string().as_bytes(), server).expect("Failed to send data");
 }
 
 for server in clone{
 match server_socket.recv_from(&mut serverz_buffer){
 Ok((n, addr)) => {
 let response = str::from_utf8(&serverz_buffer[0..n]).expect("Invalid response");
 //println!("The leader elect {}",response);
 let response_id:f64 = response.parse().expect("Invalid response");
 responses.push((response_id, addr));
 //server_buffer = [0; 6500];
 }
 Err(e) => {
 //println!("Other server failed: {}", e);
 }
 }
 }
 let mut lowest = (candidate_ID,"192.168.43.69:8080".to_string());
 //let mut lowest_ip = "192.168.43.69:8080".to_string();
 for (response_id, response_ip) in responses{
 //println!("The response is {} from {}", response_id, response_ip);
 if response_id < lowest.0 {
 lowest = (response_id, response_ip.to_string());
 }
 }
 //println!("The lowest ip is {}", lowest.1);

 // Return the (candidate_id, ip) of the highest id
 return lowest.1;
 //check if the candidate id is the highest and store the ip of the highest id
}

fn elect_failure(server_socket: UdpSocket, servers:Vec<String>)->bool{
 let mut server_buffer = [0;6500];
 //send the candidate id to the to the addresses in the vector
 let clone = servers.clone();
 let mut rng = rand::thread_rng();
 let mut is_failing = false;
 let random_number = rng.gen_range(1..=100);
 for server in servers{
 server_socket.send_to(random_number.to_string().as_bytes(), server).expect("Failed to send data");
 }
 let mut responses: Vec<(u32)> = Vec::new();
 for server in clone{
 match server_socket.recv_from(&mut server_buffer){
 Ok((n, addr)) => {
 let response = str::from_utf8(&server_buffer[0..n]).expect("Invalid response");
 //println!("failure ID is is {} from server {}",response, addr);
 let response_id:u32 = response.parse().expect("Invalid response");
 responses.push((response_id));
 }
 Err(e) => {
 //println!("Server Unreachable: {}", e);
 }
 }
 }
 //check if the candidate id is the highest and store the ip of the highest id
 let mut lowest = random_number;
 for (response_id) in responses{
 //println!("The response is {} from {}", response_id, response_ip);
 if response_id < lowest {
 lowest = response_id;
 //println!("The lowest ip is {}", lowest.1);
 }
 }
 if lowest == random_number{
 is_failing = true;
 }
 return is_failing;
}

fn generate_random_image(width: u32, height: u32) -> RgbaImage {
 let mut rng = rand::thread_rng();
 let mut image = RgbaImage::new(width, height);

 for (_, _, pixel) in image.enumerate_pixels_mut() {
 // Generate random RGBA values
 for i in 0..4 {
 pixel[i] = rng.gen_range(0..=255);
 }
 }

 image
}



fn encypt_image(image1: &ImageBuffer<image::Rgba<u8>, Vec<u8>>, image2: &ImageBuffer<image::Rgba<u8>, Vec<u8>>) -> RgbaImage {
 let mut encoded = RgbaImage::new(image1.width(), image1.height());
 for (x, y, pixel1) in image1.enumerate_pixels() {
 let pixel2 = image2.get_pixel(x, y);

 let mut encoded_pixel = encoded.get_pixel_mut(x, y);

 // Blend each color channel
 for i in 0..3 {
 let image1_channel = pixel1[i] >> 5; // Take the 3 most significant bits
 let image2_channel = pixel2[i] >> 5;
 encoded_pixel[i] = (image2_channel << 5) | image1_channel; // Blend the channels
 }

 // Set alpha to full
 encoded_pixel[3] = 255;
 }
 //println!("image encoded");
 encoded
}







// fn hide_image(cover_image: &mut RgbaImage, hidden_image: &RgbaImage) {
// let mut rng = rand::thread_rng();

// // Get the dimensions of the cover image
// let (width, height) = cover_image.dimensions();

// // Ensure that the hidden image can fit in the cover image
// if hidden_image.width() > width || hidden_image.height() > height {
// panic!("Hidden image is too large to be hidden in the cover image");
// }

// // Iterate over the pixels of the hidden image
// for (x, y, hidden_pixel) in hidden_image.enumerate_pixels() {
// let cover_pixel = cover_image.get_pixel_mut(x, y);

// // Modify the least significant bits of each channel in the cover pixel
// for i in 0..4 {
// cover_pixel[i] = (cover_pixel[i] & 0b11111110) | ((hidden_pixel[i] >> 7) & 1);
// }
// }
// }


fn system_load()-> f64{
 // Get the system load average
 if let Ok(load_avg) = sys_info::loadavg() {
 return load_avg.one;
 } else {
 return 3000.0;
 }
}

fn find_entry<'a>(dos_buffer: &'a Vec<(String, String, u32)>, client_addr: SocketAddr) -> Option<&'a (String, String, u32)> {
 dos_buffer.iter().find(|(addr, _message, _count)| *addr == client_addr.to_string())
}

fn find_entry_dos<'a>(dos: &'a Vec<(String)>, client_addr: SocketAddr) -> Option<&'a (String)> {
 dos.iter().find(|(addr)| *addr == &client_addr.to_string())
}