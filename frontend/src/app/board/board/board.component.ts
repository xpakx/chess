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
import { Castling } from '../dto/castling';

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
  lastMove: number[] = [];
  dragged?: number[];
  color?: "White" | "Black";

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
    if (!this.color || !this.board[i][j].startsWith(this.color)) {
      event.preventDefault();
      return;
    }
    this.classList[i][j] = "dragging";
    if (!this.ghost) {
      return;
    }
    this.dragged = [i, j];
    this.ghostClass = this.fieldPipe.transform(this.board[i][j]);
    var offset = this.ghost.nativeElement.offsetWidth/2;
    event.dataTransfer?.setDragImage(this.ghost.nativeElement, offset, offset);
  }

  onDragEnd(event: DragEvent, i: number, j: number) {
    this.classList[i][j] = "";
    this.dragged = undefined;
  }

  onDragOver(event: DragEvent, i: number, j: number) {
    event.preventDefault();
  }

  onDrop(event: DragEvent, i: number, j: number) {
    this.classList[i][j] = "";
    event.preventDefault();
    if (!this.dragged || !this.color || !this._gameId) {
      return;
    }
    const start = [this.dragged[0], this.dragged[1]];
    const end = [i, j];
    const capture = this.board[i][j] != "Empty"; // TODO: en passant
    const piece = this.board[start[0]][start[1]].replace(this.color, "") as Piece;

    if (piece == "King") {
      const rank = this.color == "White" ? 7 : 0;
      if (start[0] == rank && start[1] == 4 && end[0] == rank) {
        if (end[1] == 2) {
          this.toast.createToast({ id: `drop${i}${j}`, type: "info", message: `move O-O-O` });
          this.websocket.makeMove(this._gameId!, {move: "O-O-O"});
          return;
        } else if (end[1] == 6) {
          this.toast.createToast({ id: `drop${i}${j}`, type: "info", message: `move O-O` });
          this.websocket.makeMove(this._gameId!, {move: "O-O"});
          return;
        }
      }
    }

    let candidates = this.findCandidates(start, end, this.color, piece, capture);
    const sameFile = candidates.find((a) => a[0] == start[0] && a[1] != start[1]);
    const sameRank = candidates.find((a) => a[1] == start[1] && a[0] != start[0]);

    const pieceLetter = this.getPieceLetter(piece);
    const startFile = sameFile ? String.fromCharCode(this.letterToCharCode(start[1])) : "";
    const startRank = sameRank ? String.fromCharCode(this.numberToCharCode(start[0])) : "";
    const captureString = capture ? "x" : "";
    const targetFile = String.fromCharCode(this.letterToCharCode(end[1]));
    const targetRank = String.fromCharCode(this.numberToCharCode(end[0]));

    const moveString = "".concat(pieceLetter, startFile, startRank, captureString, targetFile, targetRank);
    this.toast.createToast({id: `drop${i}${j}`, type: "info", message:`move ${moveString}`});
    this.websocket.makeMove(this._gameId, {move: moveString});

    this.dragged = undefined;
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
    if(message.move.startsWith("O")) {
      let move = this.parseCastling(message.move, message.color);
      if (!move) {
        return;
      }

      let king = this.board[move.kingMove.start[0]][move.kingMove.start[1]];
      this.board[move.kingMove.start[0]][move.kingMove.start[1]] = "Empty";
      this.board[move.kingMove.target[0]][move.kingMove.target[1]] = king;

      let rook = this.board[move.rookMove.start[0]][move.rookMove.start[1]];
      this.board[move.rookMove.start[0]][move.rookMove.start[1]] = "Empty";
      this.board[move.rookMove.target[0]][move.rookMove.target[1]] = rook;
      return;
    }
    let move = this.parseMove(message.move, message.color);
    if(!move) {
      return;
    }

    this.board[move.start[0]][move.start[1]] = "Empty";
    const pieceAfterMove = `${message.color}${move.promotion ? move.promotion : move.piece}` as Field;
    this.board[move.target[0]][move.target[1]] = pieceAfterMove;


    if (move.piece != "Pawn") {
      this.lastMove = [];
      return;
    }

    if (move.enpassant && this.lastMove.length == 2) {
      this.board[this.lastMove[0]][this.lastMove[1]] = "Empty";
    }
    const dx = Math.abs(move.start[0] - move.target[0]);
    if (dx == 2) {
      this.lastMove = move.target;
    } else {
      this.lastMove = [];
    }
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
    const target = [7-this.charToNumber(destination.charCodeAt(1)), this.charToNumber(destination.charCodeAt(0))];
    const startFile = disambiguationFile ? this.charToNumber(disambiguationFile.charCodeAt(0)) : undefined;
    const startRank = disambiguationRank ? 7-this.charToNumber(disambiguationRank.charCodeAt(0)) : undefined;
    const start = this.findStart(target, color, piece, startFile, startRank, capture ? true : false);
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

  getPieceLetter(piece: Piece): string {
    switch (piece) {
      case "King": return "K";
      case "Queen": return "Q";
      case "Rook": return "R";
      case "Bishop": return "B";
      case "Knight": return "N";
      default: return "";
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
    this.color = board.color;
  }

  findStart(target: number[], color: "Black" | "White", piece: Piece, startFile?: number, startRank?: number, capture: boolean = false): number[] | undefined {
    if(startFile && startRank) {
      return [startRank, startFile];
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

    return candidates.find((c) => this.checkCapture(c, target, piece, color, capture));
  }

  checkCapture(start: number[], target: number[], piece: Piece, color: "White" | "Black", capture: boolean): boolean {
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
      if (capture) {
        return this.checkPawnCapture(start, target, color)
      }
      return this.checkPawnMove(start, target, color)
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

  checkPawnMove(start: number[], target: number[], color: "White" | "Black"): boolean {
    if (start[1] != target[1]) {
      return false;
    }
    const startRank = color == "White" ? 6 : 1;
    const dir = color == "White" ? -1 : 1;
    if (startRank == start[0]) {
      return start[0] + dir == target[0] || start[0] + 2*dir == target[0];
    }
    return start[0] + dir == target[0];
  }


  parseCastling(move: String, color: "Black" | "White"): Castling | undefined {
    const pattern = /(O-O(-O)?)([+#]?)/;
    const match = move.match(pattern);

    if (!match) {
      console.log("No match")
      return undefined;
    }

    const [_,
      _shortCastle,
      longCastle,
      check,
    ] = match;
    const isLongCastle = longCastle ? true : false;
    const isCheck = check ? true : false;
    const isMate = check && check == "#" ? true : false;

    const rank = color == "White" ? 7 : 0;
    const kingStart = [rank, 4];
    const kingTarget = [rank, isLongCastle ? 2 : 6];
    const rookStart = [rank, isLongCastle ? 0 : 7];
    const rookTarget = [rank, isLongCastle ? 3 : 5];

    return {
      kingMove: {
        piece: "King",
        capture: false,
        check: isCheck,
        mate: isMate,
        enpassant: false,
        start: kingStart,
        target: kingTarget,
      },
      rookMove: {
        piece: "Rook",
        capture: false,
        check: isCheck,
        mate: isMate,
        enpassant: false,
        start: rookStart,
        target: rookTarget,
      },
    };
  }

  findCandidates(start: number[], target: number[], color: "Black" | "White", piece: Piece, capture: boolean = false): number[][] {
    let type = `${color}${piece}`;
    let candidates: number[][] = [];

    for (let i = 0; i < 8; i++) {
      for (let j = 0; j < 8; j++) {
        if (this.board[i][j] == type) {
          candidates.push([i, j]);
        }
      }
    }

    return candidates.filter((c) => this.checkCapture(c, target, piece, color, capture));
  }

  numberToCharCode(num: number): number {
    return (7- num) + 49;
  }

  letterToCharCode(num: number): number {
    return num + 97;
  }
}
