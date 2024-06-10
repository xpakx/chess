import { Field } from "./field";

export interface BoardMessage {
    username1: String;
    username2: String;
    ai: boolean;

    state: Field[][];
    currentPlayer: String;
    firstUserStarts: boolean;

    error?: String;
}