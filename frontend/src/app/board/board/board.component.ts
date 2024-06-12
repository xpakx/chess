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

  makeMove(move: MoveMessage) {
    // TODO
    const pattern = /([KQRBN]?)([a-h]?)([1-8]?)(x?)([a-h][1-8])(=[KQRBN])?([+#]?)/;
    const match = move.move.match(pattern);

    if (!match) {
      alert('Invalid move format');
      return;
    }

    const [_,
      pieceLetter,
      disambiguationFile,
      disambiguationRank,
      capture,
      destination,
      promotion,
      check
    ] = match;

    const piece = this.getPiece(pieceLetter);
    const target = [this.charToNumber(destination.charCodeAt(0)), this.charToNumber(destination.charCodeAt(1))];
    const startFile = disambiguationFile ? this.charToNumber(disambiguationFile.charCodeAt(0)) : undefined;
    const startRank = disambiguationRank ? this.charToNumber(disambiguationRank.charCodeAt(0)) : undefined;
    const start = this.findStart(target, "White", piece, startFile, startRank); //TODO
    if (!start) {
      return;
    }


    let transMove: Move = {
      piece: piece,
      promotion: promotion ? this.getPiece(promotion.slice(1)) : undefined,
      capture: capture ? true : false,
      check: check ? true : false,
      mate: check && check == "#" ? true : false,
      target: target,
      start: start,
    }

    console.log(piece, disambiguationFile, disambiguationRank, capture, destination, promotion, check)
    console.log(transMove);
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

    return candidates.find((c) => this.checkCapture(c, target, piece));
  }

  checkCapture(start: number[], target: number[], piece: Piece): boolean {
    return true;
  }
}
