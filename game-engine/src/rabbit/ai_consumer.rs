use lapin::{message::DeliveryResult, options::BasicAckOptions, Channel};
use serde::{Deserialize, Serialize};

use crate::{engine::{generate_bit_board, get_engine, rules::{is_game_drawn, is_game_won, move_to_string}, EngineType}, rabbit::DESTINATION_EXCHANGE, Color};

use super::move_consumer::EngineEvent;


pub fn set_ai_delegate(consumer: lapin::Consumer, channel: Channel) {
    consumer.set_delegate({
        move |delivery: DeliveryResult| {
            println!("New ai move request");
            let channel = channel.clone();
            async move {
                let channel = channel.clone();
                let delivery = match delivery {
                    Ok(Some(delivery)) => delivery,
                    Ok(None) => return,
                    Err(error) => {
                        println!("Failed to consume queue message {}", error);
                        return;
                    }
                };

                let message = std::str::from_utf8(&delivery.data).unwrap();
                let message: AIEvent = match serde_json::from_str(message) {
                    Ok(msg) => msg,
                    Err(err) => {
                        println!("Failed to deserialize game event: {:?}", err);
                        return;
                    }
                };
                println!("Received message: {:?}", &message);


                let response = process_ai_event(message);
                println!("Response: {:?}", &response);
                let response = serde_json::to_string(&response).unwrap();

                if let Err(err) = channel
                    .basic_publish(
                        DESTINATION_EXCHANGE,
                        "engine",
                        Default::default(),
                        response.into_bytes().as_slice(),
                        Default::default(),
                        )
                        .await {
                            println!("Failed to publish message to destination exchange: {:?}", err);
                        };

                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("Failed to acknowledge message");
            }
        }
    }
    );
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AIEvent {
    game_id: usize,
    game_state: String,
    noncapture_moves: usize,
    #[serde(rename = "type")]
    ai_type: String,
    color: Color,
}

fn process_ai_event(message: AIEvent) -> EngineEvent {
    let mut board = generate_bit_board(message.game_state.clone()).unwrap().board; // TODO
    let mut engine = get_engine(EngineType::Random);
    let mov = engine.get_move(&board, &message.color);
    board.apply_move(&mov, &message.color);
    let mov_string = move_to_string(&mut board, &mov, &message.color, false, false);

    let won = is_game_won(&board, &message.color);
    let drawn = !won && is_game_drawn(&board, &message.color);
    let finished = won || drawn;

    EngineEvent {
        game_id: message.game_id,
        new_state: board.to_fen(),
        mov: mov_string,
        legal: true,
        finished,
    }
}
