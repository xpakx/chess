import { Piece } from "./piece";

export interface Move {
    piece: Piece;
    capture: boolean;
    check: boolean;
    mate: boolean;
    
    target: number[];
    start: number[];

    promotion?: Piece;
    enpassant: boolean,

}