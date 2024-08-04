import { Component, Input, Output, OnInit, EventEmitter } from '@angular/core';
import { Piece } from '../dto/piece';

interface Choice {
  image: string;
  piece: Piece;
}

@Component({
  selector: 'app-promotion-modal',
  templateUrl: './promotion-modal.component.html',
  styleUrls: ['./promotion-modal.component.css']
})
export class PromotionModalComponent implements OnInit {
  choices: Choice[] = [];
  @Input() color: "White" | "Black" = "White";
  @Output() choice: EventEmitter<Piece> = new EventEmitter<Piece>();

  constructor() { }

  ngOnInit(): void {
    const pieces: Piece[] = ["Rook", "Bishop", "Knight", "Queen"];
    for(let piece of pieces) {
      this.choices.push({piece, image: `${this.color}${piece}`});
    }
  }

  onClick(choice: Piece) {
    this.choice.emit(choice);
  }

}
