export interface BoardMessage {
    username1: String;
    username2: String;
    ai: boolean;

    state: String[][];
    currentPlayer: String;
    gameStarted: boolean;

    error?: String;
}