use rand::Rng;
use resend_rs::types::CreateEmailBaseOptions;
use resend_rs::{Resend, Result};
use std::cmp::Ordering;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Enter your email to send otp");
    let mut email = String::new();

    std::io::stdin()
        .read_line(&mut email)
        .expect("No email found");

    let otp: u32 = rand::thread_rng().gen_range(100000..=999999);

    println!("Sending your otp to: {}", email);

    let resend = Resend::new("resend_api");

    let from: &str = "Rust Mail <noreply@example.com>";
    let to = [email.clone()];
    let subject: &str = "Rust Mail Otp";

    let send_email: CreateEmailBaseOptions = CreateEmailBaseOptions::new(from, to, subject)
        .with_html(&format!(
            r#" <body
      style="
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      height: 50vh;
      background-color: rgba(0, 47, 75, 0.31);
    "
  >
    <div
      style="
        padding: 20px;
        border: solid 2px #7c7878;
        width: 100%;
        max-width: 400px;
        height: 200px;
        background-color: #fff;
        margin:auto;
      "
    >
      <h3 style="text-align: center">Rust Mail</h3>
      <p>Your Login Otp is</p>
      <h1 style="padding: 4px 10xp">{}</h1>
    </div>
  </body>"#,
            otp
        ));

    let _email = (resend.emails.send(send_email)).await;

    println!(" ______________________________________");
    println!("|                                      |");
    println!("|       Email Sent Successfully        |");
    println!("|______________________________________|");

    loop {
        let mut confirm_otp: String = String::new();

        println!("Enter the Otp sent to your email");
        std::io::stdin()
            .read_line(&mut confirm_otp)
            .expect("Incorrect or No otp found");

        let confirm_otp: u32 = confirm_otp.trim().parse().expect("Enter your otp");

        match confirm_otp.cmp(&otp) {
            Ordering::Less => println!("incorrect Otp"),
            Ordering::Equal => {
                println!("Otp Verified");
                break;
            }
            Ordering::Greater => println!("Incorrect Opt"),
        };
    }

    Ok(())
}
