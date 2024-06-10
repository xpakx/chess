import { Component, ElementRef, Input, OnDestroy, OnInit, ViewChild } from '@angular/core';
import { FieldPipe } from '../field.pipe';
import { ToastService } from 'src/app/elements/toast.service';
import { Field } from '../dto/field';
import { WebsocketService } from '../websocket.service';
import { Subscription } from 'rxjs';
import { BoardMessage } from '../dto/board-message';
import { MoveMessage } from '../dto/move-message';
import { BoardEvent } from '../dto/board-event';

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
  }

  updateBoard(board: BoardEvent) {
    this.board = board.board.state;
    this.invert = board.inverted;
  }
}
