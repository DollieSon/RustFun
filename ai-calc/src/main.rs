use dotenv::dotenv;
use google_generative_ai_rs::v1::{
    api::Client,
    gemini::{Content, Model, Part, Role, request::Request},
};
use std::{env, io::stdin};

pub fn create_request(question: String) -> Request {
    let final_string = question + ",Answer the best you can and with only a number";
    Request {
        contents: vec![Content {
            role: Role::User,
            parts: vec![Part {
                text: Some(final_string),
                inline_data: None,
                file_data: None,
                video_metadata: None,
            }],
        }],
        tools: vec![],
        safety_settings: vec![],
        generation_config: None,
        system_instruction: None,
    }
}

pub async fn post_request(client: &Client, request: Request) -> String {
    let req = client.post(30, &request).await.unwrap();
    let mut response = req.rest().unwrap();
    // println!("{:#?}", response);

    let candidate = response.candidates.get_mut(0).expect("candidate Error");
    let content = candidate.content.parts.get_mut(0).expect("content Error");
    let message = content.text.clone().expect("Something");
    message
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let api_key = env::var("KEY").expect("API KEY NOT FOUND");
    let client = Client::new_from_model(Model::Gemini1_5Flash, api_key);
    // let request = create_request("From now on, I will give you mathematical equations, do your best to answer them, and you should only give the final answer, which is a number and preceed it with \"Ans.\" so if I ask \"1+1\" you answer with \"Ans.2\", if you understand, respond with 1.".to_string());
    // let res = post_request(&client, request).await;
    // println!("{res}");
    for _ in 0..10 {
        let mut question = String::new();
        println!("Enter Equation?:");
        stdin().read_line(&mut question).unwrap();
        let request = create_request(question);
        let res = post_request(&client, request).await;
        print!(":{res}");
    }

    // let request = create_request("2-2".to_string());
    // let res = post_request(&client, request).await;
    // println!("{res}");

    // let request = create_request("3*3".to_string());
    // let res = post_request(&client, request).await;
    // println!("{res}");

    // let request = create_request("5/5".to_string());
    // let res = post_request(&client, request).await;
    // println!("{res}");

    Ok(())
}
