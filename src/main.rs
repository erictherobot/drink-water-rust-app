use mac_notification_sys::*;
use std::time::Duration;

fn main() {
    let bundle = get_bundle_identifier_or_default("com.apple.Safari");

    loop {
        let response = send_notification(
            "Drink Water NOW!",
            Some("DO IT"),
            "Or else you will get headaches and feel tired.",
            Some(
                Notification::new()
                    .main_button(MainButton::DropdownActions(
                        "Dropdown",
                        &["Remind me in 5 minutes", "Remind me in 10 minutes", "Skip"],
                    ))
                    .sound("Blow")
                    .close_button("Nah, I'm good!"),
            ),
        )
        .unwrap();

        match response {
            NotificationResponse::ActionButton(action_name) => {
                if action_name == "Action 1" {
                    println!("Clicked on Action 1")
                } else if action_name == "Action 2" {
                    println!("Clicked on Action 2")
                } else if action_name == "Skip" {
                    println!("Clicked on Skip")
                }
            }
            NotificationResponse::Click => println!("Clicked on the notification itself"),
            NotificationResponse::CloseButton(close_name) => println!(
                "Dismissed the notification with the close button called {}",
                close_name
            ),

            NotificationResponse::Reply(response) => {
                println!("Replied to the notification with {}", response)
            }
            NotificationResponse::None => println!("No interaction with the notification occured"),
        };
    }
}
