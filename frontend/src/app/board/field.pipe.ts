import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'field'
})
export class FieldPipe implements PipeTransform {

  transform(value: String): String {
    if (value == "WhiteKing") {
      return "♔";
    }
    if (value == "WhiteQueen") {
      return "♕";
    }
    if (value == "WhiteBishop") {
      return "♗";
    }
    if (value == "WhiteKnight") {
      return "♘";
    }
    if (value == "WhiteRook") {
      return "♖";
    }
    if (value == "WhitePawn") {
      return "♙";
    }

    if (value == "BlackKing") {
      return "♚";
    }
    if (value == "BlackQueen") {
      return "♛";
    }
    if (value == "BlackBishop") {
      return "♝";
    }
    if (value == "BlackKnight") {
      return "♞";
    }
    if (value == "BlackRook") {
      return "♜";
    }
    if (value == "BlackPawn") {
      return "♟";
    }
    return "";
  }
}