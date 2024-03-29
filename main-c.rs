use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use serde_json;
use std::str;
use base64;
use std::fs;

const MAX_PACKET_SIZE: usize = 60000;

// Define your custom struct with serde attributes
#[derive(Serialize, Deserialize, Debug)]
struct ImageWithIP {
    allowed_ips: Vec<String>,
    image_data: Option<Vec<u8>>,
}

fn main() {
    // Bind the socket
    let socket = UdpSocket::bind("192.168.43.196:7878").expect("Failed to bind socket");

    // Prepare a request message with your IP address
    let request_message = "192.168.1.6"; // The IP address you want to request an image from

    // Send the request to the server
    socket.send_to(request_message.as_bytes(), "192.168.43.196:7879")
        .expect("Failed to send the request");

    // Prepare a buffer for receiving the response image
    let mut response_buffer = Vec::new();

    loop {
        let mut chunk = vec![0; MAX_PACKET_SIZE];
        let (bytes_received, _) = socket.recv_from(&mut chunk).expect("Failed to receive data");
        response_buffer.extend_from_slice(&chunk[0..bytes_received]);

        // Check if this is the last chunk
        if bytes_received < MAX_PACKET_SIZE {
            break;
        }
    }

    // Convert the received bytes to a string
    let response = str::from_utf8(&response_buffer)
        .expect("Failed to convert to string");

    // Deserialize the response into your custom struct
    let response_image: ImageWithIP = serde_json::from_str(response)
        .expect("Failed to deserialize the response");

    // Check if an image is included in the response
    if let Some(image_data) = response_image.image_data {
        // The received data is base64 encoded, decode it
        let decoded_image_data = base64::decode(&image_data)
            .expect("Failed to decode image data");

        // Save the received image to a file
        std::fs::write("received_image.jpg", &decoded_image_data)
            .expect("Failed to save image");

        println!("Image saved as 'received_image.jpg'");
    } else {
        println!("No image data received. You may have received a default image.");
    }
}
