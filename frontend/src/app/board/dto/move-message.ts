export interface MoveMessage {
    player: String;
    move: String;

    legal: boolean;

    message?: String;
    finished: boolean ;
    won: boolean ;
    winner?: String;
}