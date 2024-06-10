import { Field } from "src/app/board/dto/field";

export interface Game {
    id: number;
    invitation: "Issued" | "Accepted" | "Rejected";
    gameType: "User" | "AI";
    aiType: "Random" | "None";
    gameStatus: "NotFinished" | "Won" | "Lost" | "Drawn";
    currentState: Field[][];
    lastMoveRow: number; // TODO
    lastMoveColumn: number; // TODO
    userStarts: boolean;
    userTurn: boolean;
    user: String;
    opponent: String; 
    user_id: number;
    opponent_id: number; 
    invert: boolean;
}