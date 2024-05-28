import { Pipe, PipeTransform } from '@angular/core';

@Pipe({
  name: 'field'
})
export class FieldPipe implements PipeTransform {

  transform(value: String): String {
    if (value == "WhiteKing") {
      return "white king";
    }
    if (value == "WhiteQueen") {
      return "white queen";
    }
    if (value == "WhiteBishop") {
      return "white bishop";
    }
    if (value == "WhiteKnight") {
      return "white knight";
    }
    if (value == "WhiteRook") {
      return "white rook";
    }
    if (value == "WhitePawn") {
      return "white pawn";
    }

    if (value == "BlackKing") {
      return "black king";
    }
    if (value == "BlackQueen") {
      return "black queen";
    }
    if (value == "BlackBishop") {
      return "black bishop";
    }
    if (value == "BlackKnight") {
      return "black knight";
    }
    if (value == "BlackRook") {
      return "black rook";
    }
    if (value == "BlackPawn") {
      return "black pawn";
    }
    return "";
  }
}