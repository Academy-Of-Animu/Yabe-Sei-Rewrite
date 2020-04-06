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
use regex::Regex;

#[command]
#[description = "The `roll` command rolls a specified number of die with a specified number of sides."]
#[usage = "<number of die> <sides per die>"]
pub fn roll(context: &mut Context, message: &Message, mut args: Args) -> CommandResult {
    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("No possible rolls could be determined from that combination.");
                e.color(0x1355A4)
            })
        })?;
        return Ok(());
    }

    let mut dice = args.single::<u32>().unwrap();
    let mut sides = args.single::<u32>().unwrap();

    if dice > 15 || sides > 120 {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Please provide a valid amount of dice and sides. (no more than 15 die and/or 120 sides)");
                e.color(0x1355A4)
            })
        })?;
        return Ok(())
    }

    if dice <= 0 || sides <= 0 {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("Can\'t roll non-existent die with/or non-existent sides");
                e.color(0x1355A4)
            })
        })?;
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
    })?;
    Ok(())
}

#[command]
#[description = "The `owofy` command takes text and owofies it! Just try it out and you'll get the gist of it."]
#[usage = "<text to put through owofication process"]
pub fn owofy(context: &mut Context, message: &Message, args: Args) -> CommandResult {

    if args.rest().is_empty() {
        message.channel_id.send_message(&context, |msg| {
            msg.embed(|e| {
                e.description("I can't owo-fy an empty message! uwu");
                e.color(0x1355A4)
            })
        })?;
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
        // (Regex::new("!+").unwrap(), (format!(" {} ", faces[(rand::random::<f64>() * faces.len() as f64).floor() as usize])).as_str()),
    ];

    for i in regex_vec {
        let mut temp_sentence = i.0.replace_all(&sentence, i.1);
        sentence = temp_sentence.to_string();
    }

    let random_face = faces[(rand::random::<f64>() * faces.len() as f64).floor() as usize];
    sentence = format!("{} {}", sentence, random_face);
    
    // sentence = sentence.push_str(&format!(" {}", faces[(rand::random::<f64>() * faces.len() as f64).floor() as usize])).to_string();

    // sentence = sentence
    //     .replace("[lr]", "w")
    //     .replace("(?:r|l)", "w")
    //     .replace("(?:R|L)", "W")
    //     .replace("n([aeiou])", "ny$1")
    //     .replace("N([aeiou])", "Ny$1")
    //     .replace("N([AEIOU])", "NY$1")
    //     .replace("ove", "uv")
    //     .replace("!+", &(format!(" {} ", faces[(rand::random::<f64>() * faces.len() as f64).floor() as usize])));

    message.channel_id.send_message(&context, |msg| {
        msg.embed(|e| {
            e.description(&sentence);
            e.color(0x1355A4)
        })
    })?;

    Ok(())
}