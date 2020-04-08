use serenity::{
    client::Context,
    framework::standard::{
        macros::command,
        Args,
        CommandResult,
    },
    model::prelude::Message,
};

use rand;
use rand::Rng;
use regex::Regex;

#[command]
#[description = "The `roll` command rolls a specified number of die with a specified number of sides."]
#[usage = "<number of die> <sides per die>"]
pub async fn roll(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    let mut dice_input = args.single::<u32>();
    let mut sides_input = args.single::<u32>();

    if args.message().is_empty() || dice_input.is_err() || sides_input.is_err() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("No possible rolls could be determined from that combination.");
                e.color(0x1355A4)
            })
        }).await;
        return Ok(());
    }

    let dice = dice_input.unwrap();
    let sides = sides_input.unwrap();

    if dice > 15 || sides > 120 {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Please provide a valid amount of dice and sides. (no more than 15 die and/or 120 sides)");
                e.color(0x1355A4)
            })
        }).await;
        return Ok(())
    }

    if dice <= 0 || sides <= 0 {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Can\'t roll non-existent die with/or non-existent sides");
                e.color(0x1355A4)
            })
        }).await;
        return Ok(())
    }

    let mut results = Vec::new();

    for i in 0..dice {
        results.push(((rand::random::<f64>() * (sides as f64)) + 1 as f64).floor());
    }

    let c_reg = Regex::new("(.*), (.*)").unwrap();

    let mut result = &results.into_iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", ");

    let result = c_reg.replace(&result, "$1 and $2");

    message.channel_id.send_message(&context, |msg| {
        msg.embed(|e| {
            e.description(format!("You rolled: {}", result));
            e.color(0x1355A4)
        })
    }).await;
    Ok(())
}

#[command]
#[description = "The `owofy` command takes text and owofies it! Just try it out and you'll get the gist of it."]
#[usage = "<text to put through owofication process"]
pub async fn owofy(context: &mut Context, message: &Message, args: Args) -> CommandResult {

    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("I can't owo-fy an empty message! uwu");
                e.color(0x1355A4)
            })
        }).await;
        return Ok(());
    }

    let mut sentence = args.raw().collect::<Vec<&str>>().join(" ");
    let faces = vec!["(・`ω´・)",";;w;;","owo","UwU",">w<","^w^"];

    let regex_vec = vec![
        (Regex::new("[lr]").unwrap(), "w"),
        (Regex::new("(?:r|l)").unwrap(), "w"),
        (Regex::new("(?:R|L)").unwrap(), "W"),
        (Regex::new("n([aeiou])").unwrap(), "ny$1"),
        (Regex::new("N([aeiou])").unwrap(), "Ny$1"),
        (Regex::new("N([AEIOU])").unwrap(), "NY$1"),
        (Regex::new("ove").unwrap(), "uv"),
    ];

    for i in regex_vec {
        let mut temp_sentence = i.0.replace_all(&sentence, i.1);
        sentence = temp_sentence.to_string();
    }

    let random_face = faces[(rand::random::<f64>() * faces.len() as f64).floor() as usize];
    sentence = format!("{} {}", sentence, random_face);

    message.channel_id.send_message(&context, |msg| {
        msg.embed(|e| {
            e.description(&sentence);
            e.color(0x1355A4)
        })
    }).await;

    Ok(())
}

// TODO: refactor for negative number exceptions
#[command]
#[description = "Flips a coin (or coins)"]
#[usage = "<number of coins> (leave empty for one coin"]
pub async fn flip(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    if args.message().is_empty() {

        let random = (rand::thread_rng().gen_range(1, 3) as f64).floor();
        let mut coin_side = String::with_capacity(6);

        println!("{}", random);

        if random as u8 == 1 {
            coin_side.push_str("Heads!");
        } else {
            coin_side.push_str("Tails!");
        }

        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description(&coin_side);
                e.color(0x1355A4)
            })
        }).await;

        return Ok(());
    }

    let coins_input = args.single::<u32>();
    if coins_input.is_err() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Please enter a number between 1 and 10 (inclusive)");
                e.color(0x1355A4)
            })
        }).await;

        return Ok(());
    }

    let coins = coins_input.unwrap();

    if coins < 1 || coins > 10 {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Please enter a number between 1 and 10 (inclusive)");
                e.color(0x1355A4)
            })
        }).await;

        return Ok(());
    } else {
        let mut heads_count = 0;
        let mut tails_count = 0;

        let mut coin_history: Vec<&str> = Vec::new();

        for i in 0..coins {
            let random = (rand::thread_rng().gen_range(1, 3) as f64).floor() as u32;

            if random as u8 == 1 {
                coin_history.push("Heads");
                heads_count += 1;
            } else {
                coin_history.push("Tails");
                tails_count += 1;
            }
        }

        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description(
                    format!("Results: {}\n\nHeads: {}\nTails: {}",
                        &coin_history
                            .into_iter()
                            .map(|i| i.to_string())
                            .collect::<Vec<String>>()
                            .join(", "),
                        heads_count,
                        tails_count
                    )
                );
                e.color(0x1355A4)
            })
        }).await;

        Ok(())
    }
}

#[command]
#[description = "Gives a totally ~~random~~ calculated answer to your question"]
#[usage ="<question>"]
#[aliases("8ball")]
pub async fn eightball(context: &mut Context, message: &Message, args: Args) -> CommandResult {
    if args.message().is_empty() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Give me a question to predict the answer for!");
                e.color(0x1355A4)
            })
        }).await;

        return Ok(());
    }

    let answers = vec![
        "That is it chief.", "Can I get an amen?", "OwO", "UwU", "Indede, it is so.", "Mmm, yes.",
        "YES!!!!", "Yeah, why not.", "Only if you say please.",

        "Ask canarado.", "I guess??", "With the way things are, who knows?", "¯\\_(ツ)_/¯", "Not enough info...",
        "I don't want to answer that.", "How would I know that?", "You might wanna reconsider your life choices.",
        "How about you watch anime instead of asking me questions.", "Maybe.", "If you pray hard enough.",

        "This is not it chief.", "That is a no from me chief.", "Leave me alone, I am tired.", "Turn the simp down a notch and I'll answer ya.",
        "Are you...serious?", "How about you shut up.", "Simply put, no.", "Not going to happen.", "Ask again later.", "Pffft."
    ];

    let answer = answers[(rand::random::<f64>() * answers.len() as f64).floor() as usize];

    message.channel_id.send_message(&context, |msg| {
        msg.embed(|e| {
            e.description(answer);
            e.color(0x1355A4)
        })
    }).await;

    Ok(())
}