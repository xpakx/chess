import { Component, ElementRef, Input, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { FieldPipe } from '../field.pipe';
import { ToastService } from 'src/app/elements/toast.service';
import { Field } from '../dto/field';
import { WebsocketService } from '../websocket.service';
import { Subscription } from 'rxjs';
import { MoveMessage } from '../dto/move-message';
import { BoardEvent } from '../dto/board-event';
import { Move } from '../dto/move';
import { Piece } from '../dto/piece';

@Component({
  selector: 'app-board',
  templateUrl: './board.component.html',
  styleUrls: ['./board.component.css']
})
export class BoardComponent implements OnInit, OnDestroy {
  board: Field[][] = 
  [
    ["BlackRook", "BlackKnight", "BlackBishop", "BlackQueen", "BlackKing", "BlackBishop", "BlackKnight", "BlackRook"],
    ["BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn", "BlackPawn"],
    ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
    ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
    ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
    ["Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty", "Empty"],
    ["WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn", "WhitePawn"],
    ["WhiteRook", "WhiteKnight", "WhiteBishop", "WhiteQueen", "WhiteKing", "WhiteBishop", "WhiteKnight", "WhiteRook"],
  ];

  classList: String[][] = Array(this.board.length).fill(null).map(() => Array(this.board.length).fill(""));
  @ViewChild('ghost') ghost?: ElementRef;
  ghostClass: String = "";
  invert: boolean = false;
  _gameId: number | undefined = undefined;
  finished: boolean = false;

  private moveSub?: Subscription;
  private boardSub?: Subscription;

  constructor(private fieldPipe: FieldPipe, private toast: ToastService, private websocket: WebsocketService) { }

  ngOnInit(): void {
    this.boardSub = this.websocket.board$
      .subscribe((board: BoardEvent) => this.updateBoard(board));

    this.moveSub = this.websocket.move$
    .subscribe((move: MoveMessage) => this.makeMove(move));
  }

  ngOnDestroy() {
    this.websocket.unsubscribe();
    this.websocket.disconnect();
    this.boardSub?.unsubscribe();
    this.moveSub?.unsubscribe();
  }

  @Input() set id(value: number | undefined) {
    this._gameId = value;
    this.finished = false;
    this.invert = false;
    if (this._gameId) {
      this.websocket.connect();
      this.websocket.subscribeGame(this._gameId);
    }
  }

  onDragStart(event: DragEvent, i: number, j: number) {
    this.toast.createToast({id: `dragging${i}${j}`, type: "info", message:`Dragging ${i}, ${j}`});
    this.classList[i][j] = "dragging";
    if (!this.ghost) {
      return;
    }
    this.ghostClass = this.fieldPipe.transform(this.board[i][j]);
    var offset = this.ghost.nativeElement.offsetWidth/2;
    event.dataTransfer?.setDragImage(this.ghost.nativeElement, offset, offset);
  }

  onDragEnd(event: DragEvent, i: number, j: number) {
    this.toast.createToast({id: `dragend${i}${j}`, type: "info", message:`Stop dragging ${i}, ${j}`});
    this.classList[i][j] = "";
  }

  onDragOver(event: DragEvent, i: number, j: number) {
  }

  onDrop(event: DragEvent, i: number, j: number) {
    this.toast.createToast({id: `drop${i}${j}`, type: "info", message:`Stop dragging ${i}, ${j}`});
    this.classList[i][j] = "";
    event.preventDefault();
  }

  onDragEnter(event: DragEvent, i: number, j: number) {
    if (this.classList[i][j] != "") {
      return;
    }
    this.classList[i][j] = "dragover";
  }

  onDragLeave(event: DragEvent, i: number, j: number) {
    if (this.classList[i][j] != "dragover") {
      return;
    }
    this.classList[i][j] = "";
  }

  makeMove(message: MoveMessage) {
    // TODO
    const color = "White"; // TODO

    //  TODO: castling
    let move = this.parseMove(message.move, color);
    if(!move) {
      return;
    }

    console.log(move);

    this.board[move.start[0]][move.start[1]] = "Empty";
    const pieceAfterMove = `${color}${move.promotion ? move.promotion : move.piece}` as Field;
    this.board[move.target[0]][move.target[1]] = pieceAfterMove;
    // TODO: if enpassant…
  }


  parseMove(move: String, color: "Black" | "White"): Move | undefined {
    const pattern = /([KQRBN]?)([a-h]?)([1-8]?)(x?)([a-h][1-8])(=[KQRBN])?([+#]?)( e\.p\.)?/;
    const match = move.match(pattern);

    if (!match) {
      return undefined;
    }

    const [_,
      pieceLetter,
      disambiguationFile,
      disambiguationRank,
      capture,
      destination,
      promotion,
      check,
      enpassant
    ] = match;

    const piece = this.getPiece(pieceLetter);
    const target = [this.charToNumber(destination.charCodeAt(0)), this.charToNumber(destination.charCodeAt(1))];
    const startFile = disambiguationFile ? this.charToNumber(disambiguationFile.charCodeAt(0)) : undefined;
    const startRank = disambiguationRank ? this.charToNumber(disambiguationRank.charCodeAt(0)) : undefined;
    const start = this.findStart(target, color, piece, startFile, startRank); //TODO
    if (!start) {
      return;
    }
    const isEnpassant = enpassant ? true : false;

    return {
      piece: piece,
      promotion: promotion ? this.getPiece(promotion.slice(1)) : undefined,
      capture: capture ? true : false,
      check: check ? true : false,
      mate: check && check == "#" ? true : false,
      target: target,
      start: start,
      enpassant: isEnpassant,
    }
  }

  getPiece(piece: string): Piece {
    switch (piece) {
      case "K": return "King";
      case "Q": return "Queen";
      case "R": return "Rook";
      case "B": return "Bishop";
      case "N": return "Knight";
      default: return "Pawn";
    }
  }

  charToNumber(charCode: number): number {
    if (charCode >= 97 && charCode <= 104) {
      return charCode - 97;
    } else if (charCode >= 49 && charCode <= 56) {
      return charCode - 49;
    } 
    return -1;
  }

  updateBoard(board: BoardEvent) {
    this.board = board.board.state;
    this.invert = board.inverted;
  }

  findStart(target: number[], color: "Black" | "White", piece: Piece, startFile?: number, startRank?: number): number[] | undefined {
    if(startFile && startRank) {
      return [startFile, startRank];
    }

    let type = `${color}${piece}`;
    let candidates: number[][] = [];

    if(startRank) {
      for(let i = 0; i<8; i++) {
        if(this.board[startRank][i] == type) {
          candidates.push([startRank, i]);
        }
      }
    } else if(startFile) {
      for(let i = 0; i<8; i++) {
        if(this.board[i][startFile] == type) {
          candidates.push([i, startFile]);
        }
      }
    } else {
      for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
          if (this.board[i][j] == type) {
            candidates.push([i, j]);
          }
        }
      }
    }

    return candidates.find((c) => this.checkCapture(c, target, piece, color));
  }

  checkCapture(start: number[], target: number[], piece: Piece, color: "White" | "Black"): boolean {
    if (piece == "Rook") {
      return this.checkRookCapture(start, target);
    } else if (piece == "Queen") {
      return this.checkRookCapture(start, target) || this.checkBishopCapture(start, target);
    } else if (piece == "Bishop") {
      return this.checkBishopCapture(start, target);
    } else if (piece == "Knight") {
      return this.checkKnightCapture(start, target);
    } else if (piece == "King") {
      return this.checkKingCapture(start, target);
    } else if (piece == "Pawn") {
      return this.checkPawnCapture(start, target, color)
    }
    
    return false;
  }

  checkRookCapture(start: number[], target: number[]): boolean {
    if (start[0] != target[0] && start[1] != target[1]) {
      return false;
    }
    if (start[0] == target[0]) {
      const first = Math.min(start[1], target[1]);
      const second = Math.max(start[1], target[1]);
      for (let i = first + 1; i < second; i++) {
        if (this.board[start[0]][i] != "Empty") {
          return false;
        }
      }
      return true;
    }
    if (start[1] == target[1]) {
      const first = Math.min(start[0], target[0]);
      const second = Math.max(start[0], target[0]);
      for (let i = first + 1; i < second; i++) {
        if (this.board[i][start[1]] != "Empty") {
          return false;
        }
      }
      return true;
    }
    return false;
  }

  checkBishopCapture(start: number[], target: number[]): boolean {
    const dx = Math.abs(start[0] - target[0]);
    const dy = Math.abs(start[1] - target[1]);

    if (dx != dy) {
      return false;
    }

    const xDirection = start[0] < target[0] ? 1 : -1;
    const yDirection = start[1] < target[1] ? 1 : -1;

    for (let i = 1; i < dx; i++) {
      const x = start[0] + i * xDirection;
      const y = start[1] + i * yDirection;
      if (this.board[x][y] !== "Empty") {
        return false;
      }
    }
    return true;
  }

  checkKnightCapture(start: number[], target: number[]): boolean {
    const dx = Math.abs(start[0] - target[0]);
    const dy = Math.abs(start[1] - target[1]);
    return (dx === 2 && dy === 1) || (dx === 1 && dy === 2);
  }

  checkKingCapture(start: number[], target: number[]): boolean {
    const dx = Math.abs(start[0] - target[0]);
    const dy = Math.abs(start[1] - target[1]);
    return dx <= 1 && dy <= 1 && !(dx === 0 && dy === 0);
  }

  checkPawnCapture(start: number[], target: number[], color: "White" | "Black"): boolean {
    // TODO: en passant
    const dx = Math.abs(start[0] - target[0]);
    const dy = target[1] - start[1];
    if (color == "White") {
      return dx === 1 && dy === 1;
    } else {
      return dx === 1 && dy === -1;
    }
  }
}
