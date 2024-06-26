use lapin::{message::DeliveryResult, options::BasicAckOptions, Channel};
use serde::{Deserialize, Serialize};

use crate::{engine::{generate_bit_board, rules::{game_state, string_to_move, GameState}}, rabbit::DESTINATION_EXCHANGE, Color};

pub fn set_move_delegate(consumer: lapin::Consumer, channel: Channel) {
    consumer.set_delegate({
        move |delivery: DeliveryResult| {
            println!("New move verification request");
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
                let message: MoveEvent = match serde_json::from_str(message) {
                    Ok(msg) => msg,
                    Err(err) => {
                        println!("Failed to deserialize game event: {:?}", err);
                        return;
                    }
                };
                println!("Received message: {:?}", &message);


                let response = process_move(message);
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
struct MoveEvent {
    game_id: usize,
    game_state: String,
    #[serde(rename = "move")]
    mov: String,
    noncapture_moves: usize,
    color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineEvent {
    pub game_id: usize,
    pub legal: bool,
    pub new_state: String,
    #[serde(rename = "move")]
    pub mov: String,
    pub finished: bool,
}

fn process_move(message: MoveEvent) -> EngineEvent {
    let mut board = generate_bit_board(message.game_state.clone()).unwrap(); // TODO
    let mov = string_to_move(&mut board, message.mov.clone(), &message.color);
    let legal = mov.is_ok();
    let (new_state, finished) = match mov {
        Ok(mov) => {
            board.apply_move(&mov, &message.color);
            let state = game_state(&mut board, &message.color);
            let won = match state {
                GameState::Checkmate => true,
                _ => false,
            };
            let drawn = match state {
                GameState::Stalemate => true,
                _ => false,
            };
            (board.to_fen(), won || drawn)
        },
        Err(_) => (message.game_state, false),
    };

    EngineEvent {
        game_id: message.game_id,
        new_state,
        mov: message.mov,
        legal,
        finished,
    }
}
