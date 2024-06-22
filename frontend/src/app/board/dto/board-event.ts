import { BoardMessage } from "./board-message";

export interface BoardEvent {
    board: BoardMessage;
    inverted: boolean;
    color?: "White" | "Black";
}